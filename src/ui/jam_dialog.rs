use std::cell::RefCell;
use std::rc::Rc;
use std::sync::{Arc, RwLock};

use adw::prelude::*;
use gtk4::gdk;
use gtk4::glib;
use gtk4::prelude::*;
use gtk4::{
    Align, Box as GtkBox, Button, Entry, FlowBox, Image, Label, Orientation, Picture, SelectionMode,
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

pub fn build_spin_bottom_bar(
    state: &Rc<RefCell<AppState>>,
    jam_btn: &Button,
) -> (GtkBox, Label, Label) {
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
    {
        let st = Rc::clone(state);
        let jb = jam_btn.clone();
        manage_btn.connect_clicked(move |btn| {
            open_jam_dialog(btn, Rc::clone(&st), jb.clone());
        });
    }

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

pub fn open_jam_dialog(parent: &Button, state: Rc<RefCell<AppState>>, jam_btn: Button) {
    let dialog = adw::PreferencesDialog::builder()
        .title(tr("Modo SPIN", "SPIN Mode"))
        .content_width(500)
        .content_height(720)
        .build();
    let page = adw::PreferencesPage::new();

    let (is_active, allow_controls, url, guest_count, added_count, remote_role) = {
        let s = state.borrow();
        let snap = s.jam.snapshot.read().ok();
        (
            s.jam.server.is_some(),
            snap.as_ref().map(|n| n.allow_controls).unwrap_or(false),
            snap.as_ref().map(|n| n.url.clone()).unwrap_or_default(),
            snap.as_ref().map(|n| n.guest_count).unwrap_or(0),
            snap.as_ref().map(|n| n.added_count).unwrap_or(0),
            s.jam.remote_role.clone(),
        )
    };

    // Control Remoto / Jukebox (Wi-Fi local)
    let jukebox_group = adw::PreferencesGroup::builder()
        .title(tr("Jukebox local (Wi‑Fi)", "Local Jukebox (Wi-Fi)"))
        .description(tr(
            "Agrega canciones a tu cola desde el celular.",
            "Add songs to your queue from a phone.",
        ))
        .build();

    let active_row = adw::SwitchRow::builder()
        .title(tr("Activar Jukebox local", "Enable local Jukebox"))
        .subtitle(tr(
            "Inicia un servidor web local en tu red",
            "Starts a local web server on your network",
        ))
        .active(is_active)
        .build();

    let controls_row = adw::SwitchRow::builder()
        .title(tr(
            "Permitir controles de reproducción",
            "Allow playback controls",
        ))
        .subtitle(tr(
            "Los invitados podrán pausar, saltar y reordenar la cola",
            "Guests can pause, skip, and reorder the queue",
        ))
        .active(allow_controls)
        .build();

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
    let status_row = adw::ActionRow::builder()
        .title(tr("Estado", "Status"))
        .subtitle(&status_text(is_active, guest_count, added_count))
        .build();

    jukebox_group.add(&active_row);
    jukebox_group.add(&controls_row);
    jukebox_group.add(&status_row);
    page.add(&jukebox_group);

    let connect_group = adw::PreferencesGroup::builder()
        .title(tr("Conectar celulares (QR)", "Connect phones (QR)"))
        .description(tr(
            "Escanea el código QR o abre la dirección en el navegador",
            "Scan the QR code or open the address in a browser",
        ))
        .visible(is_active)
        .build();

    let qr_picture = Picture::builder()
        .width_request(180)
        .height_request(180)
        .halign(Align::Center)
        .valign(Align::Center)
        .build();
    let qr_card = GtkBox::new(Orientation::Vertical, 0);
    qr_card.add_css_class("spin-qr-card");
    qr_card.set_overflow(gtk4::Overflow::Hidden);
    qr_card.set_halign(Align::Center);
    qr_card.append(&qr_picture);

    let qr_frame = GtkBox::new(Orientation::Vertical, 8);
    qr_frame.set_halign(Align::Center);
    qr_frame.set_margin_top(12);
    qr_frame.set_margin_bottom(12);
    qr_frame.append(&qr_card);

    let qr_row = adw::ActionRow::new();
    qr_row.set_child(Some(&qr_frame));

    let url_label = Label::builder()
        .label(&url)
        .selectable(true)
        .ellipsize(gtk4::pango::EllipsizeMode::Middle)
        .css_classes(["heading"])
        .build();
    let copy_btn = Button::builder()
        .label(tr("Copiar", "Copy"))
        .valign(Align::Center)
        .css_classes(["flat"])
        .build();
    {
        let url_lbl = url_label.clone();
        let dlg = dialog.clone();
        copy_btn.connect_clicked(move |_| {
            if let Some(display) = gdk::Display::default() {
                display.clipboard().set_text(&url_lbl.text());
                dlg.add_toast(adw::Toast::new(tr("Enlace SPIN copiado", "SPIN link copied")));
            }
        });
    }
    let url_row = adw::ActionRow::builder()
        .title(tr("Dirección local", "Local address"))
        .build();
    url_row.add_suffix(&url_label);
    url_row.add_suffix(&copy_btn);

    connect_group.add(&qr_row);
    connect_group.add(&url_row);
    page.add(&connect_group);

    // Escucha Sincronizada Remota (entre instancias de GyroTube)
    let remote_group = adw::PreferencesGroup::builder()
        .title(tr(
            "Escucha sincronizada (App a App)",
            "Synced listening (App to App)",
        ))
        .description(tr(
            "Escucha lo mismo al mismo tiempo con amigos.",
            "Listen together in real time with friends.",
        ))
        .build();

    let is_host = matches!(remote_role, RemoteRole::Host(_));
    let host_code_init = match &remote_role {
        RemoteRole::Host(c) => format!("GYRO-{c}"),
        _ => String::new(),
    };
    let guest_target_init = match &remote_role {
        RemoteRole::Guest(t) => t.clone(),
        _ => String::new(),
    };

    let host_row = adw::SwitchRow::builder()
        .title(tr("Transmitir como anfitrión", "Broadcast as host"))
        .subtitle(tr("Crea una invitación", "Create an invitation"))
        .active(is_host)
        .build();

    let room_code_label = Label::builder()
        .label(&host_code_init)
        .ellipsize(gtk4::pango::EllipsizeMode::Middle)
        .max_width_chars(24)
        .selectable(true)
        .build();
    let copy_code_btn = Button::builder()
        .label(tr("Copiar", "Copy"))
        .valign(Align::Center)
        .css_classes(["flat"])
        .build();
    {
        let lbl = room_code_label.clone();
        let dlg = dialog.clone();
        copy_code_btn.connect_clicked(move |_| {
            if let Some(display) = gdk::Display::default() {
                display.clipboard().set_text(&lbl.text());
                dlg.add_toast(adw::Toast::new(tr(
                    "Invitación copiada",
                    "Invitation copied",
                )));
            }
        });
    }
    let room_code_row = adw::ActionRow::builder()
        .title(tr("Invitación", "Invitation"))
        .visible(is_host)
        .build();
    room_code_row.add_suffix(&room_code_label);
    room_code_row.add_suffix(&copy_code_btn);

    let join_entry = Entry::builder()
        .placeholder_text(tr(
            "Invitación SPIN o URL del anfitrión...",
            "SPIN invitation or host URL...",
        ))
        .text(&guest_target_init)
        .valign(Align::Center)
        .width_chars(18)
        .build();
    let is_guest = matches!(remote_role, RemoteRole::Guest(_));
    let join_btn = Button::builder()
        .label(if is_guest {
            tr("Desconectar", "Disconnect")
        } else {
            tr("Unirse", "Join")
        })
        .valign(Align::Center)
        .css_classes(if is_guest {
            ["destructive-action"]
        } else {
            ["suggested-action"]
        })
        .build();
    let join_row = adw::ActionRow::builder()
        .title(tr("Unirse a una sala", "Join a room"))
        .subtitle(if is_guest {
            tr("Sincronizado con anfitrión", "Synced with host")
        } else {
            tr("Pega una invitación", "Paste an invitation")
        })
        .build();
    join_row.add_suffix(&join_entry);
    join_row.add_suffix(&join_btn);

    let ticket_qr = Picture::builder()
        .width_request(220)
        .height_request(220)
        .halign(Align::Center)
        .visible(false)
        .build();
    let show_ticket_qr = |picture: &Picture, ticket: &str| {
        let texture = ticket_qr_svg(ticket).and_then(|svg| {
            gdk::Texture::from_bytes(&glib::Bytes::from_owned(svg.into_bytes())).ok()
        });
        picture.set_paintable(texture.as_ref());
        picture.set_visible(texture.is_some());
    };
    if is_host {
        show_ticket_qr(&ticket_qr, &host_code_init);
    }

    remote_group.add(&host_row);
    remote_group.add(&room_code_row);
    remote_group.add(&ticket_qr);
    remote_group.add(&join_row);
    page.add(&remote_group);

    let update_qr = |pic: &Picture, target_url: &str| {
        if let Some(svg) = generate_qr_svg(target_url)
            && let Ok(tex) = gdk::Texture::from_bytes(&glib::Bytes::from_owned(svg.into_bytes()))
        {
            pic.set_paintable(Some(&tex));
        }
    };
    if is_active && !url.is_empty() {
        update_qr(&qr_picture, &url);
    }

    // Modo A switch
    {
        let state_sw = Rc::clone(&state);
        let connect_group = connect_group.clone();
        let status_row = status_row.clone();
        let url_label = url_label.clone();
        let qr_picture = qr_picture.clone();
        let jam_btn = jam_btn.clone();
        let dlg = dialog.clone();
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
                        connect_group.set_visible(true);
                        status_row.set_subtitle(&status_text(true, 0, 0));
                    }
                    Err(err) => {
                        row.set_active(false);
                        dlg.add_toast(adw::Toast::new(&err));
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
                connect_group.set_visible(false);
                status_row.set_subtitle(&status_text(false, 0, 0));
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

    // Modo B Host switch
    {
        let state_h = Rc::clone(&state);
        let room_code_row = room_code_row.clone();
        let room_code_label = room_code_label.clone();
        let join_row = join_row.clone();
        let join_btn = join_btn.clone();
        let jam_btn = jam_btn.clone();
        let ticket_qr = ticket_qr.clone();
        let dlg = dialog.clone();
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
                    room_code_row,
                    room_code_label,
                    join_btn,
                    join_row,
                    jam_btn,
                    ticket_qr,
                    dlg,
                ) = (
                    row.clone(),
                    state_h.clone(),
                    room_code_row.clone(),
                    room_code_label.clone(),
                    join_btn.clone(),
                    join_row.clone(),
                    jam_btn.clone(),
                    ticket_qr.clone(),
                    dlg.clone(),
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
                                dlg.add_toast(adw::Toast::new(&format!("SPIN: {err}")));
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
                        room_code_row.set_visible(true);
                        show_ticket_qr(&ticket_qr, &ticket);
                        join_btn.set_label(tr("Unirse", "Join"));
                        join_btn.remove_css_class("destructive-action");
                        join_btn.add_css_class("suggested-action");
                        join_row.set_subtitle(tr("Pega una invitación", "Paste an invitation"));
                    },
                );
            } else {
                let mut s = state_h.borrow_mut();
                if matches!(s.jam.remote_role, RemoteRole::Host(_)) {
                    s.jam.host_publisher = None;
                    s.jam.remote_role = RemoteRole::Idle;
                    s.jam.latest_state = None;
                }
                refresh_spin_ui(&s);
                update_jam_btn_highlight(&s, &jam_btn);
                room_code_row.set_visible(false);
                ticket_qr.set_visible(false);
            }
        });
    }

    // Modo B Guest Join/Disconnect
    {
        let state_g = Rc::clone(&state);
        let join_entry = join_entry.clone();
        let join_row = join_row.clone();
        let host_row = host_row.clone();
        let room_code_row = room_code_row.clone();
        let jam_btn = jam_btn.clone();
        let dlg = dialog.clone();
        join_btn.connect_clicked(move |btn| {
            if state_g.borrow().jam.host_starting {
                return;
            }
            let is_currently_guest =
                matches!(state_g.borrow().jam.remote_role, RemoteRole::Guest(_));
            if is_currently_guest {
                let mut s = state_g.borrow_mut();
                s.jam.guest_session = None;
                s.jam.remote_role = RemoteRole::Idle;
                s.jam.latest_state = None;
                s.remote_host_name = None;
                refresh_spin_ui(&s);
                update_jam_btn_highlight(&s, &jam_btn);
                btn.set_label(tr("Unirse", "Join"));
                btn.remove_css_class("destructive-action");
                btn.add_css_class("suggested-action");
                join_row.set_subtitle(tr(
                    "Ingresa el código de tu amigo para escuchar igual",
                    "Enter your friend's code to listen together",
                ));
                return;
            }
            let raw = join_entry.text().to_string();
            let target = match normalize_room_code(&raw) {
                Ok(target) => target,
                Err(err) => {
                    dlg.add_toast(adw::Toast::new(&err));
                    return;
                }
            };
            if host_row.is_active() {
                host_row.set_active(false);
            }
            room_code_row.set_visible(false);
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
            }
            dlg.add_toast(adw::Toast::new(tr(
                "Conectando a sala SPIN…",
                "Connecting to SPIN room…",
            )));
            btn.set_label(tr("Desconectar", "Disconnect"));
            btn.remove_css_class("suggested-action");
            btn.add_css_class("destructive-action");
            join_row.set_subtitle(tr("Siguiendo al anfitrión", "Following the host"));
        });
    }

    dialog.add(&page);
    let root = parent.root().and_then(|r| r.downcast::<gtk4::Window>().ok());
    dialog.present(root.as_ref());
}
