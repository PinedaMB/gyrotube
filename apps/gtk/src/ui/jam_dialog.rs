use std::cell::RefCell;
use std::rc::Rc;
use std::sync::{Arc, RwLock};

use adw::prelude::*;
use gtk4::gdk;
use gtk4::glib;
use gtk4::prelude::*;
use gtk4::{
    Align, Box as GtkBox, Button, Entry, FlowBox, Image, Label, Orientation, Picture, PolicyType,
    ScrolledWindow, SelectionMode, Stack, ToggleButton,
};
use libadwaita as adw;

use crate::i18n::tr;
use crate::jam::{
    JamCommand, JamRequest, JamServerHandle, JamSnapshot, JamSource, JamTrackInfo, generate_qr_svg,
    start_jam_server,
};
use crate::spin_remote::{
    GuestSession, HostPublisher, RemoteRole, RemoteSyncPayload, RemoteUpdate,
    local_display_name, normalize_room_code, now_ms, start_guest_sync, ticket_qr_svg,
};
use crate::ui::player::{clear_playback, load_queue_index, play_queue_index, refresh_queue_ui};
use crate::ui::state::{AppState, next_queue_index};
use crate::ui::widgets::{clear_box, spawn_bg};

pub struct JamRuntime {
    pub snapshot: Arc<RwLock<JamSnapshot>>,
    pub server: Option<JamServerHandle>,
    pub cmd_tx: async_channel::Sender<JamRequest>,
    pub remote_role: RemoteRole,
    pub guest_session: Option<GuestSession>,
    host_starting: bool,
    pub host_publisher: Option<HostPublisher>,
    pub remote_tx: async_channel::Sender<RemoteUpdate>,
    pub latest_state: Option<RemoteSyncPayload>,
}

impl JamRuntime {
    fn accepts_command(&self, source: &JamSource) -> bool {
        let host_room = match &self.remote_role {
            RemoteRole::Host(room) => Some(room.as_str()),
            _ => None,
        };
        source.is_current(
            self.server.as_ref().map(|server| server.session_id),
            host_room,
        )
    }

    pub fn new(
        cmd_tx: async_channel::Sender<JamRequest>,
        remote_tx: async_channel::Sender<RemoteUpdate>,
    ) -> Self {
        Self {
            snapshot: Arc::new(RwLock::new(JamSnapshot::default())),
            server: None,
            cmd_tx,
            remote_role: RemoteRole::Idle,
            guest_session: None,
            host_starting: false,
            host_publisher: None,
            remote_tx,
            latest_state: None,
        }
    }
}

pub fn sync_jam_snapshot(s: &AppState) {
    let is_playing = s.active_media().is_playing();
    let ts_us = s.active_media().timestamp();
    let position_secs = if ts_us > 0 {
        ts_us as f64 / 1_000_000.0
    } else {
        s.seek_scale.value().max(0.0)
    };
    let cur_track = s.current_track().cloned();
    let allow_controls = if let Ok(mut snap) = s.jam.snapshot.write() {
        snap.is_playing = is_playing;
        snap.position_secs = position_secs;
        snap.now_playing = cur_track.as_ref().map(|t| JamTrackInfo {
            id: t.id.clone(),
            title: t.title.clone(),
            artist: t.artist.clone(),
            album: t.album.clone(),
            duration_secs: t.duration_secs,
            cover_url: t.cover_url.clone(),
            is_current: true,
        });
        snap.queue = s
            .queue
            .iter()
            .enumerate()
            .map(|(i, t)| JamTrackInfo {
                id: t.id.clone(),
                title: t.title.clone(),
                artist: t.artist.clone(),
                album: t.album.clone(),
                duration_secs: t.duration_secs,
                cover_url: t.cover_url.clone(),
                is_current: s.current_index == Some(i),
            })
            .collect();
        snap.allow_controls
    } else {
        false
    };

    if let Some(publisher) = &s.jam.host_publisher {
        publisher.publish(RemoteSyncPayload {
            track: cur_track,
            position_secs,
            is_playing,
            sent_at_ms: now_ms(),
            allow_controls,
            source_name: s.source_name.clone(),
            host_name: local_display_name(),
        });
    }
    refresh_spin_ui(s);
}

pub fn build_spin_bottom_bar(on_open: Rc<dyn Fn()>) -> (GtkBox, Label, Label) {
    let icon = Image::builder()
        .icon_name("people-community-regular")
        .pixel_size(16)
        .valign(Align::Center)
        .build();
    let label = Label::builder()
        .xalign(0.0)
        .hexpand(true)
        .ellipsize(gtk4::pango::EllipsizeMode::End)
        .css_classes(["heading", "caption"])
        .build();
    let count_lbl = Label::builder()
        .css_classes(["spin-host-pill"])
        .valign(Align::Center)
        .build();
    let manage_btn = Button::builder()
        .label(tr("Gestionar", "Manage"))
        .valign(Align::Center)
        .css_classes(["flat", "pill-switch"])
        .build();
    manage_btn.set_cursor_from_name(Some("pointer"));
    manage_btn.connect_clicked(move |_| {
        on_open();
    });

    let bar = GtkBox::new(Orientation::Horizontal, 10);
    bar.add_css_class("spin-bottom-bar");
    bar.set_visible(false);
    bar.append(&icon);
    bar.append(&label);
    bar.append(&count_lbl);
    bar.append(&manage_btn);
    (bar, label, count_lbl)
}

pub fn refresh_spin_ui(s: &AppState) {
    let jukebox_active = s.jam.server.is_some();
    let (guest_list, host_from_snap) = s
        .jam
        .snapshot
        .read()
        .ok()
        .map(|sn| (sn.guest_list.clone(), sn.host_name.clone()))
        .unwrap_or_default();

    let host_display = if !host_from_snap.is_empty() {
        host_from_snap
    } else {
        local_display_name()
    };

    let (active, summary, count_text, participants_host, guests): (
        bool,
        String,
        String,
        String,
        Vec<String>,
    ) = match &s.jam.remote_role {
        RemoteRole::Host(_) => {
            let total = 1 + guest_list.len();
            let mode = if jukebox_active {
                String::from("Jukebox + SPIN")
            } else {
                String::from("SPIN")
            };
            (
                true,
                format!(
                    "{mode} • {}: {host_display}",
                    tr("Anfitrión", "Host")
                ),
                format!(
                    "{total} {}",
                    if total == 1 {
                        tr("conectado", "connected")
                    } else {
                        tr("conectados", "connected")
                    }
                ),
                host_display,
                guest_list,
            )
        }
        RemoteRole::Guest(_) => {
            let remote_host = s
                .remote_host_name
                .clone()
                .unwrap_or_else(|| tr("Conectando…", "Connecting…").to_string());
            (
                true,
                format!(
                    "SPIN • {}: {remote_host}",
                    tr("Anfitrión", "Host")
                ),
                tr("Sincronizado", "Synced").to_string(),
                remote_host,
                vec![local_display_name()],
            )
        }
        RemoteRole::Idle if jukebox_active => {
            let total = 1 + guest_list.len();
            (
                true,
                format!(
                    "SPIN Jukebox • {}: {host_display}",
                    tr("Anfitrión", "Host")
                ),
                format!(
                    "{total} {}",
                    if total == 1 {
                        tr("conectado", "connected")
                    } else {
                        tr("conectados", "connected")
                    }
                ),
                host_display,
                guest_list,
            )
        }
        RemoteRole::Idle => (
            false,
            String::new(),
            String::new(),
            String::new(),
            Vec::new(),
        ),
    };

    if s.spin_bar.is_visible() == active
        && (!active
            || (s.spin_bar_label.label() == summary && s.spin_bar_count.label() == count_text))
    {
        return;
    }

    s.spin_bar.set_visible(active);
    s.spin_participants_box.set_visible(active);
    if !active {
        return;
    }

    s.spin_bar_label.set_label(&summary);
    s.spin_bar_count.set_label(&count_text);

    clear_box(&s.spin_participants_box);
    let header = Label::builder()
        .label(tr("Conectados en SPIN", "Connected in SPIN"))
        .xalign(0.0)
        .css_classes(["caption", "dim-label"])
        .build();
    let flow = FlowBox::builder()
        .selection_mode(SelectionMode::None)
        .column_spacing(6)
        .row_spacing(6)
        .min_children_per_line(1)
        .max_children_per_line(4)
        .build();
    flow.insert(
        &Label::builder()
            .label(&format!("★ {} ({})", participants_host, tr("Anfitrión", "Host")))
            .css_classes(["spin-host-pill"])
            .build(),
        -1,
    );
    for g in guests {
        flow.insert(
            &Label::builder()
                .label(&g)
                .css_classes(["spin-guest-pill"])
                .build(),
            -1,
        );
    }
    s.spin_participants_box.append(&header);
    s.spin_participants_box.append(&flow);
}

async fn verify_music_track_bg(
    backend: crate::backend::Backend,
    video_id: String,
) -> Result<crate::models::Track, String> {
    let (tx, rx) = async_channel::bounded(1);
    crate::backend::runtime().spawn(async move {
        let res = backend.verify_music_track(&video_id).await;
        let _ = tx.send(res).await;
    });
    rx.recv()
        .await
        .unwrap_or_else(|_| Err(String::from("Error verificando canción.")))
}

pub fn wire_remote_sync(
    state: Rc<RefCell<AppState>>,
    remote_rx: async_channel::Receiver<RemoteUpdate>,
) {
    glib::spawn_future_local(async move {
        while let Ok(update) = remote_rx.recv().await {
            let role = state.borrow().jam.remote_role.clone();
            let target = match &role {
                RemoteRole::Host(target) | RemoteRole::Guest(target) => target,
                RemoteRole::Idle => continue,
            };
            if target != &update.target {
                continue;
            }
            let payload = match update.result {
                Ok(payload) if matches!(role, RemoteRole::Guest(_)) => payload,
                Err(err) => {
                    state.borrow().show_toast(&err);
                    continue;
                }
                _ => continue,
            };
            {
                let mut s = state.borrow_mut();
                if !payload.host_name.is_empty()
                    && s.remote_host_name.as_deref() != Some(payload.host_name.as_str())
                {
                    s.remote_host_name = Some(payload.host_name.clone());
                    refresh_spin_ui(&s);
                }
            }
            let target_pos = payload.position_at(now_ms());
            let same_track = {
                let s = state.borrow();
                s.current_track()
                    .zip(payload.track.as_ref())
                    .is_some_and(|(current, track)| {
                        current.id == track.id
                            && (s.active_media().has_stream() || s.play_spinner.is_spinning())
                            && s.jam
                                .latest_state
                                .as_ref()
                                .and_then(|old| old.track.as_ref())
                                .is_some_and(|old| old.id == track.id)
                    })
            };
            state.borrow_mut().jam.latest_state = Some(payload.clone());
            let Some(ref track) = payload.track else {
                clear_playback(Rc::clone(&state));
                continue;
            };

            if same_track {
                let media = state.borrow().active_media().clone();
                if !media.is_prepared() {
                    state.borrow_mut().pending_autoplay = payload.is_playing;
                    continue;
                }
                if payload.is_playing && !media.is_playing() {
                    media.play();
                } else if !payload.is_playing && media.is_playing() {
                    media.pause();
                }
                if media.is_prepared() {
                    let cur_pos = media.timestamp().max(0) as f64 / 1_000_000.0;
                    if (cur_pos - target_pos).abs() > 2.5 {
                        media.seek((target_pos * 1_000_000.0) as i64);
                    }
                }
            } else {
                let (backend, role) = {
                    let s = state.borrow();
                    (s.backend.clone(), s.jam.remote_role.clone())
                };
                let track = match verify_music_track_bg(backend, track.id.clone()).await {
                    Ok(track) => track,
                    Err(err) => {
                        state.borrow().show_toast(&err);
                        continue;
                    }
                };
                if state.borrow().jam.remote_role != role {
                    continue;
                }
                {
                    let mut s = state.borrow_mut();
                    s.replace_queue(vec![track], format!("SPIN • {}", payload.source_name));
                }
                let sync_pos = payload.position_at(now_ms());
                load_queue_index(Rc::clone(&state), 0, payload.is_playing, Some(sync_pos));
            }
        }
    });
}

pub fn wire_jam_commands(
    state: Rc<RefCell<AppState>>,
    cmd_rx: async_channel::Receiver<JamRequest>,
) {
    glib::spawn_future_local(async move {
        while let Ok(request) = cmd_rx.recv().await {
            if !state.borrow().jam.accepts_command(&request.source) {
                continue;
            }
            match request.command {
                JamCommand::AddTrack(video_id, _guest) => {
                    let backend = state.borrow().backend.clone();
                    let state_add = Rc::clone(&state);
                    let source = request.source;
                    glib::spawn_future_local(async move {
                        let track = match verify_music_track_bg(backend, video_id).await {
                            Ok(track) => track,
                            Err(err) => {
                                state_add.borrow().show_toast(&err);
                                return;
                            }
                        };
                        if !state_add.borrow().jam.accepts_command(&source) {
                            return;
                        }
                        let should_start = {
                            let mut s = state_add.borrow_mut();
                            let was_empty = s.queue.is_empty() || s.current_index.is_none();
                            let start_idx = s.queue.len();
                            if let Ok(mut snap) = s.jam.snapshot.write() {
                                snap.added_count += 1;
                            }
                            s.queue.push(track.clone());
                            s.update_nav_buttons();
                            s.save_session();
                            s.show_toast(&format!(
                                "{}: {}",
                                tr("SPIN • Agregada", "SPIN • Added"),
                                track.title
                            ));
                            if was_empty { Some(start_idx) } else { None }
                        };
                        refresh_queue_ui(Rc::clone(&state_add));
                        if let Some(idx) = should_start {
                            play_queue_index(state_add, idx);
                        }
                    });
                }
                JamCommand::TogglePlayPause => {
                    let allowed = state
                        .borrow()
                        .jam
                        .snapshot
                        .read()
                        .map(|sn| sn.allow_controls)
                        .unwrap_or(false);
                    if allowed {
                        let btn = state.borrow().play_pause_btn.clone();
                        btn.emit_clicked();
                    }
                }
                JamCommand::NextTrack => {
                    let allowed = state
                        .borrow()
                        .jam
                        .snapshot
                        .read()
                        .map(|sn| sn.allow_controls)
                        .unwrap_or(false);
                    if allowed {
                        let target = next_queue_index(&state.borrow());
                        if let Some(idx) = target {
                            play_queue_index(Rc::clone(&state), idx);
                        }
                    }
                }
                JamCommand::PrevTrack => {
                    let allowed = state
                        .borrow()
                        .jam
                        .snapshot
                        .read()
                        .map(|sn| sn.allow_controls)
                        .unwrap_or(false);
                    if allowed {
                        let target = state
                            .borrow()
                            .current_index
                            .and_then(|idx| idx.checked_sub(1));
                        if let Some(idx) = target {
                            play_queue_index(Rc::clone(&state), idx);
                        }
                    }
                }
                JamCommand::MoveTrack(from, to) => {
                    let allowed = state
                        .borrow()
                        .jam
                        .snapshot
                        .read()
                        .map(|sn| sn.allow_controls)
                        .unwrap_or(false);
                    if allowed {
                        state.borrow_mut().move_queue_item(from, to);
                        refresh_queue_ui(Rc::clone(&state));
                    }
                }
            }
        }
    });
}

fn update_jam_btn_highlight(s: &AppState, jam_btn: &Button) {
    let any_active = s.jam.server.is_some() || s.jam.remote_role != RemoteRole::Idle;
    if any_active {
        jam_btn.add_css_class("suggested-action");
    } else {
        jam_btn.remove_css_class("suggested-action");
    }
}

pub fn build_spin_sidebar(
    state: Rc<RefCell<AppState>>,
    jam_btn: Button,
    on_close: Rc<dyn Fn()>,
) -> (adw::ToolbarView, Rc<dyn Fn()>) {
    let status_text = |active: bool, guests: usize, added: usize| -> String {
        if !active {
            String::from(tr("Inactivo", "Inactive"))
        } else {
            format!(
                "{guests} {} • {added} {}",
                tr("invitados", "guests"),
                tr("agregadas", "added")
            )
        }
    };

    // Header bar
    let spin_icon = Image::builder()
        .icon_name("people-community-regular")
        .pixel_size(17)
        .valign(Align::Center)
        .build();
    let spin_title = Label::builder()
        .label(tr("Modo SPIN", "SPIN Mode"))
        .xalign(0.0)
        .css_classes(["title-4"])
        .build();
    let live_badge = Label::builder()
        .label(tr("● En vivo", "● Live"))
        .valign(Align::Center)
        .visible(false)
        .css_classes(["lyrics-badge-synced"])
        .build();
    let header_left = GtkBox::new(Orientation::Horizontal, 8);
    header_left.set_hexpand(true);
    header_left.append(&spin_icon);
    header_left.append(&spin_title);
    header_left.append(&live_badge);

    let close_btn = Button::builder()
        .icon_name("window-close-symbolic")
        .tooltip_text(tr("Cerrar panel SPIN", "Close SPIN panel"))
        .valign(Align::Center)
        .css_classes(["flat", "circular"])
        .build();
    close_btn.set_cursor_from_name(Some("pointer"));
    close_btn.connect_clicked(move |_| on_close());

    let top_bar = GtkBox::new(Orientation::Horizontal, 8);
    top_bar.add_css_class("spin-sidebar-header");
    top_bar.append(&header_left);
    top_bar.append(&close_btn);

    // Compact mode switcher: Wi-Fi Jukebox | App to App
    let tab_local = ToggleButton::builder()
        .label(tr("Wi‑Fi Jukebox", "Wi-Fi Jukebox"))
        .active(true)
        .hexpand(true)
        .css_classes(["flat", "pill-switch"])
        .build();
    let tab_remote = ToggleButton::builder()
        .label(tr("App a App", "App to App"))
        .group(&tab_local)
        .hexpand(true)
        .css_classes(["flat", "pill-switch"])
        .build();
    let tab_bar = GtkBox::new(Orientation::Horizontal, 4);
    tab_bar.set_homogeneous(true);
    tab_bar.add_css_class("pill-switch-group");
    tab_bar.append(&tab_local);
    tab_bar.append(&tab_remote);

    let mode_stack = Stack::builder()
        .transition_type(gtk4::StackTransitionType::Crossfade)
        .transition_duration(140)
        .vhomogeneous(false)
        .build();

    // --- Page 1: Local Wi-Fi Jukebox ---
    let active_row = adw::SwitchRow::builder()
        .title(tr("Activar Jukebox local", "Enable local Jukebox"))
        .subtitle(tr(
            "Servidor web en tu red Wi‑Fi",
            "Local web server on your Wi-Fi",
        ))
        .build();
    let controls_row = adw::SwitchRow::builder()
        .title(tr("Permitir controles", "Allow controls"))
        .subtitle(tr(
            "Pausar, saltar y reordenar la cola",
            "Pause, skip, and reorder the queue",
        ))
        .build();
    let status_row = adw::ActionRow::builder()
        .title(tr("Estado", "Status"))
        .subtitle(&status_text(false, 0, 0))
        .build();

    let jukebox_list = gtk4::ListBox::builder()
        .selection_mode(SelectionMode::None)
        .css_classes(["boxed-list"])
        .build();
    jukebox_list.append(&active_row);
    jukebox_list.append(&controls_row);
    jukebox_list.append(&status_row);

    let qr_picture = Picture::builder()
        .width_request(176)
        .height_request(176)
        .halign(Align::Center)
        .valign(Align::Center)
        .build();
    let qr_card = GtkBox::new(Orientation::Vertical, 0);
    qr_card.add_css_class("spin-qr-card");
    qr_card.set_overflow(gtk4::Overflow::Hidden);
    qr_card.set_halign(Align::Center);
    qr_card.append(&qr_picture);

    let url_label = Label::builder()
        .label("")
        .selectable(true)
        .xalign(0.0)
        .hexpand(true)
        .ellipsize(gtk4::pango::EllipsizeMode::Middle)
        .css_classes(["caption", "heading"])
        .build();
    let copy_btn = Button::builder()
        .label(tr("Copiar", "Copy"))
        .valign(Align::Center)
        .css_classes(["flat", "collection-play-all-btn"])
        .build();
    copy_btn.set_cursor_from_name(Some("pointer"));
    {
        let url_lbl = url_label.clone();
        let st = Rc::clone(&state);
        copy_btn.connect_clicked(move |_| {
            if let Some(display) = gdk::Display::default() {
                display.clipboard().set_text(&url_lbl.text());
                st.borrow()
                    .show_toast(tr("Enlace SPIN copiado", "SPIN link copied"));
            }
        });
    }
    let url_row = GtkBox::new(Orientation::Horizontal, 8);
    url_row.append(&url_label);
    url_row.append(&copy_btn);

    let connect_card = GtkBox::new(Orientation::Vertical, 12);
    connect_card.add_css_class("spin-sidebar-card");
    connect_card.set_visible(false);
    connect_card.append(
        &Label::builder()
            .label(tr("Escanea para unirte", "Scan to join"))
            .xalign(0.5)
            .css_classes(["heading", "caption"])
            .build(),
    );
    connect_card.append(&qr_card);
    connect_card.append(&url_row);

    let local_page = GtkBox::new(Orientation::Vertical, 14);
    local_page.append(&jukebox_list);
    local_page.append(&connect_card);

    // --- Page 2: Remote App-to-App Sync ---
    let host_row = adw::SwitchRow::builder()
        .title(tr("Transmitir como anfitrión", "Broadcast as host"))
        .subtitle(tr("Crea una invitación", "Create an invitation"))
        .build();

    let host_list = gtk4::ListBox::builder()
        .selection_mode(SelectionMode::None)
        .css_classes(["boxed-list"])
        .build();
    host_list.append(&host_row);

    let room_code_label = Label::builder()
        .label("")
        .ellipsize(gtk4::pango::EllipsizeMode::Middle)
        .xalign(0.0)
        .hexpand(true)
        .selectable(true)
        .css_classes(["caption", "heading"])
        .build();
    let copy_code_btn = Button::builder()
        .label(tr("Copiar", "Copy"))
        .valign(Align::Center)
        .css_classes(["flat", "collection-play-all-btn"])
        .build();
    copy_code_btn.set_cursor_from_name(Some("pointer"));
    {
        let lbl = room_code_label.clone();
        let st = Rc::clone(&state);
        copy_code_btn.connect_clicked(move |_| {
            if let Some(display) = gdk::Display::default() {
                display.clipboard().set_text(&lbl.text());
                st.borrow()
                    .show_toast(tr("Invitación copiada", "Invitation copied"));
            }
        });
    }
    let ticket_qr = Picture::builder()
        .width_request(176)
        .height_request(176)
        .halign(Align::Center)
        .valign(Align::Center)
        .build();
    let ticket_qr_card = GtkBox::new(Orientation::Vertical, 0);
    ticket_qr_card.add_css_class("spin-qr-card");
    ticket_qr_card.set_overflow(gtk4::Overflow::Hidden);
    ticket_qr_card.set_halign(Align::Center);
    ticket_qr_card.append(&ticket_qr);

    let code_row = GtkBox::new(Orientation::Horizontal, 8);
    code_row.append(&room_code_label);
    code_row.append(&copy_code_btn);

    let host_invite_card = GtkBox::new(Orientation::Vertical, 10);
    host_invite_card.add_css_class("spin-sidebar-card");
    host_invite_card.set_visible(false);
    host_invite_card.append(
        &Label::builder()
            .label(tr("Invitación activa", "Active invitation"))
            .xalign(0.5)
            .css_classes(["heading", "caption"])
            .build(),
    );
    host_invite_card.append(&ticket_qr_card);
    host_invite_card.append(&code_row);

    let join_subtitle = Label::builder()
        .label(tr(
            "Pega una invitación SPIN para escuchar sincronizado.",
            "Paste a SPIN invitation to listen in sync.",
        ))
        .xalign(0.0)
        .wrap(true)
        .css_classes(["caption", "dim-label"])
        .build();
    let join_entry = Entry::builder()
        .placeholder_text(tr(
            "Invitación SPIN o URL del anfitrión…",
            "SPIN invitation or host URL…",
        ))
        .hexpand(true)
        .build();
    let join_btn = Button::builder()
        .label(tr("Unirse", "Join"))
        .halign(Align::End)
        .css_classes(["suggested-action", "collection-new-pl-btn"])
        .build();
    join_btn.set_cursor_from_name(Some("pointer"));

    let join_card = GtkBox::new(Orientation::Vertical, 10);
    join_card.add_css_class("spin-sidebar-card");
    join_card.append(
        &Label::builder()
            .label(tr("Unirse a una sala", "Join a room"))
            .xalign(0.0)
            .css_classes(["heading"])
            .build(),
    );
    join_card.append(&join_subtitle);
    join_card.append(&join_entry);
    join_card.append(&join_btn);

    let remote_page = GtkBox::new(Orientation::Vertical, 14);
    remote_page.append(&host_list);
    remote_page.append(&host_invite_card);
    remote_page.append(&join_card);

    mode_stack.add_named(&local_page, Some("local"));
    mode_stack.add_named(&remote_page, Some("remote"));
    mode_stack.set_visible_child_name("local");

    {
        let stack = mode_stack.clone();
        tab_local.connect_toggled(move |b| {
            if b.is_active() {
                stack.set_visible_child_name("local");
            }
        });
    }
    {
        let stack = mode_stack.clone();
        tab_remote.connect_toggled(move |b| {
            if b.is_active() {
                stack.set_visible_child_name("remote");
            }
        });
    }

    let update_qr = |pic: &Picture, target_url: &str| {
        if let Some(svg) = generate_qr_svg(target_url)
            && let Ok(tex) = gdk::Texture::from_bytes(&glib::Bytes::from_owned(svg.into_bytes()))
        {
            pic.set_paintable(Some(&tex));
        }
    };
    let show_ticket_qr = |picture: &Picture, card: &GtkBox, ticket: &str| {
        let texture = ticket_qr_svg(ticket).and_then(|svg| {
            gdk::Texture::from_bytes(&glib::Bytes::from_owned(svg.into_bytes())).ok()
        });
        picture.set_paintable(texture.as_ref());
        card.set_visible(texture.is_some());
    };

    let sync_live_badge: Rc<dyn Fn()> = {
        let state = Rc::clone(&state);
        let live_badge = live_badge.clone();
        Rc::new(move || {
            let s = state.borrow();
            let active = s.jam.server.is_some() || s.jam.remote_role != RemoteRole::Idle;
            live_badge.set_visible(active);
        })
    };

    // Local Jukebox switch
    {
        let state_sw = Rc::clone(&state);
        let connect_card = connect_card.clone();
        let status_row = status_row.clone();
        let url_label = url_label.clone();
        let qr_picture = qr_picture.clone();
        let jam_btn = jam_btn.clone();
        let sync_badge = Rc::clone(&sync_live_badge);
        active_row.connect_active_notify(move |row| {
            let want = row.is_active();
            if want {
                let (backend, snapshot, cmd_tx, already) = {
                    let s = state_sw.borrow();
                    (
                        s.backend.clone(),
                        Arc::clone(&s.jam.snapshot),
                        s.jam.cmd_tx.clone(),
                        s.jam.server.is_some(),
                    )
                };
                if already {
                    return;
                }
                match start_jam_server(backend, Arc::clone(&snapshot), cmd_tx) {
                    Ok(handle) => {
                        let url = handle.url.clone();
                        if let Ok(mut sn) = snapshot.write() {
                            sn.active = true;
                            sn.url = url.clone();
                            sn.host_name = local_display_name();
                            sn.guests.clear();
                            sn.guest_list.clear();
                            sn.guest_count = 0;
                            sn.added_count = 0;
                        }
                        {
                            let mut s = state_sw.borrow_mut();
                            s.jam.server = Some(handle);
                            sync_jam_snapshot(&s);
                            update_jam_btn_highlight(&s, &jam_btn);
                        }
                        url_label.set_label(&url);
                        update_qr(&qr_picture, &url);
                        connect_card.set_visible(true);
                        status_row.set_subtitle(&status_text(true, 0, 0));
                        sync_badge();
                    }
                    Err(err) => {
                        row.set_active(false);
                        state_sw.borrow().show_toast(&err);
                    }
                }
            } else {
                {
                    let mut s = state_sw.borrow_mut();
                    if let Some(mut srv) = s.jam.server.take() {
                        srv.stop();
                    }
                    if let Ok(mut sn) = s.jam.snapshot.write() {
                        sn.active = false;
                    }
                    refresh_spin_ui(&s);
                    update_jam_btn_highlight(&s, &jam_btn);
                }
                connect_card.set_visible(false);
                status_row.set_subtitle(&status_text(false, 0, 0));
                sync_badge();
            }
        });
    }

    {
        let state_ctrl = Rc::clone(&state);
        controls_row.connect_active_notify(move |row| {
            let allow = row.is_active();
            let s = state_ctrl.borrow();
            if let Ok(mut sn) = s.jam.snapshot.write() {
                sn.allow_controls = allow;
            }
            sync_jam_snapshot(&s);
        });
    }

    // Remote Host switch
    {
        let state_h = Rc::clone(&state);
        let host_invite_card = host_invite_card.clone();
        let room_code_label = room_code_label.clone();
        let join_subtitle = join_subtitle.clone();
        let join_btn = join_btn.clone();
        let jam_btn = jam_btn.clone();
        let ticket_qr = ticket_qr.clone();
        let ticket_qr_card = ticket_qr_card.clone();
        let sync_badge = Rc::clone(&sync_live_badge);
        host_row.connect_active_notify(move |row| {
            if row.is_active() {
                let commands = {
                    let mut s = state_h.borrow_mut();
                    if s.jam.host_starting || matches!(s.jam.remote_role, RemoteRole::Host(_)) {
                        return;
                    }
                    s.jam.host_starting = true;
                    s.jam.cmd_tx.clone()
                };
                row.set_sensitive(false);
                join_btn.set_sensitive(false);
                row.set_subtitle(tr("Preparando la sala…", "Preparing room…"));
                let (
                    row,
                    state_h,
                    host_invite_card,
                    room_code_label,
                    join_btn,
                    join_subtitle,
                    jam_btn,
                    ticket_qr,
                    ticket_qr_card,
                    sync_badge,
                ) = (
                    row.clone(),
                    state_h.clone(),
                    host_invite_card.clone(),
                    room_code_label.clone(),
                    join_btn.clone(),
                    join_subtitle.clone(),
                    jam_btn.clone(),
                    ticket_qr.clone(),
                    ticket_qr_card.clone(),
                    Rc::clone(&sync_badge),
                );
                spawn_bg(
                    move || HostPublisher::new(commands),
                    move |result| {
                        state_h.borrow_mut().jam.host_starting = false;
                        row.set_sensitive(true);
                        join_btn.set_sensitive(true);
                        row.set_subtitle(tr("Crea una invitación", "Create an invitation"));
                        let publisher = match result {
                            Ok(publisher) => publisher,
                            Err(err) => {
                                row.set_active(false);
                                state_h.borrow().show_toast(&format!("SPIN: {err}"));
                                return;
                            }
                        };
                        let code = publisher.invitation.clone();
                        {
                            let mut s = state_h.borrow_mut();
                            s.jam.guest_session = None;
                            s.jam.host_publisher = Some(publisher);
                            s.jam.remote_role = RemoteRole::Host(code.clone());
                            s.jam.latest_state = None;
                            s.remote_host_name = None;
                            sync_jam_snapshot(&s);
                            update_jam_btn_highlight(&s, &jam_btn);
                        }
                        let ticket = format!("GYRO-{code}");
                        room_code_label.set_label(&ticket);
                        host_invite_card.set_visible(true);
                        show_ticket_qr(&ticket_qr, &ticket_qr_card, &ticket);
                        join_btn.set_label(tr("Unirse", "Join"));
                        join_btn.remove_css_class("destructive-action");
                        join_btn.add_css_class("suggested-action");
                        join_subtitle.set_label(tr(
                            "Pega una invitación SPIN para escuchar sincronizado.",
                            "Paste a SPIN invitation to listen in sync.",
                        ));
                        sync_badge();
                    },
                );
            } else {
                {
                    let mut s = state_h.borrow_mut();
                    if matches!(s.jam.remote_role, RemoteRole::Host(_)) {
                        s.jam.host_publisher = None;
                        s.jam.remote_role = RemoteRole::Idle;
                        s.jam.latest_state = None;
                    }
                    refresh_spin_ui(&s);
                    update_jam_btn_highlight(&s, &jam_btn);
                }
                host_invite_card.set_visible(false);
                sync_badge();
            }
        });
    }

    // Remote Guest Join / Disconnect
    {
        let state_g = Rc::clone(&state);
        let join_entry = join_entry.clone();
        let join_subtitle = join_subtitle.clone();
        let host_row = host_row.clone();
        let host_invite_card = host_invite_card.clone();
        let jam_btn = jam_btn.clone();
        let sync_badge = Rc::clone(&sync_live_badge);
        join_btn.connect_clicked(move |btn| {
            if state_g.borrow().jam.host_starting {
                return;
            }
            let is_currently_guest =
                matches!(state_g.borrow().jam.remote_role, RemoteRole::Guest(_));
            if is_currently_guest {
                {
                    let mut s = state_g.borrow_mut();
                    s.jam.guest_session = None;
                    s.jam.remote_role = RemoteRole::Idle;
                    s.jam.latest_state = None;
                    s.remote_host_name = None;
                    refresh_spin_ui(&s);
                    update_jam_btn_highlight(&s, &jam_btn);
                }
                btn.set_label(tr("Unirse", "Join"));
                btn.remove_css_class("destructive-action");
                btn.add_css_class("suggested-action");
                join_subtitle.set_label(tr(
                    "Pega una invitación SPIN para escuchar sincronizado.",
                    "Paste a SPIN invitation to listen in sync.",
                ));
                sync_badge();
                return;
            }
            let raw = join_entry.text().to_string();
            let target = match normalize_room_code(&raw) {
                Ok(target) => target,
                Err(err) => {
                    state_g.borrow().show_toast(&err);
                    return;
                }
            };
            if host_row.is_active() {
                host_row.set_active(false);
            }
            host_invite_card.set_visible(false);
            {
                let mut s = state_g.borrow_mut();
                s.jam.host_publisher = None;
                s.jam.remote_role = RemoteRole::Guest(target.clone());
                s.jam.latest_state = None;
                s.remote_host_name = None;
                let tx = s.jam.remote_tx.clone();
                s.jam.guest_session = Some(start_guest_sync(target, tx));
                refresh_spin_ui(&s);
                update_jam_btn_highlight(&s, &jam_btn);
                s.show_toast(tr("Conectando a sala SPIN…", "Connecting to SPIN room…"));
            }
            btn.set_label(tr("Desconectar", "Disconnect"));
            btn.remove_css_class("suggested-action");
            btn.add_css_class("destructive-action");
            join_subtitle.set_label(tr("Siguiendo al anfitrión", "Following the host"));
            sync_badge();
        });
    }

    let body = GtkBox::new(Orientation::Vertical, 14);
    body.set_margin_top(12);
    body.set_margin_bottom(24);
    body.set_margin_start(14);
    body.set_margin_end(14);
    body.append(&tab_bar);
    body.append(&mode_stack);

    let scroller = ScrolledWindow::builder()
        .hscrollbar_policy(PolicyType::Never)
        .vexpand(true)
        .child(&body)
        .build();

    let sidebar_toolbar = adw::ToolbarView::new();
    sidebar_toolbar.add_top_bar(&top_bar);
    sidebar_toolbar.set_content(Some(&scroller));
    sidebar_toolbar.add_css_class("spin-sidebar-pane");

    let refresh_sidebar: Rc<dyn Fn()> = {
        let state = Rc::clone(&state);
        let status_row = status_row.clone();
        let sync_badge = Rc::clone(&sync_live_badge);
        Rc::new(move || {
            let (is_active, guests, added) = {
                let s = state.borrow();
                let snap = s.jam.snapshot.read().ok();
                (
                    s.jam.server.is_some(),
                    snap.as_ref().map(|n| n.guest_count).unwrap_or(0),
                    snap.as_ref().map(|n| n.added_count).unwrap_or(0),
                )
            };
            status_row.set_subtitle(&status_text(is_active, guests, added));
            sync_badge();
        })
    };

    (sidebar_toolbar, refresh_sidebar)
}
