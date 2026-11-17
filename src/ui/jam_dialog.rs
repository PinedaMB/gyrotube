use std::cell::RefCell;
use std::rc::Rc;
use std::sync::{Arc, RwLock};

use adw::prelude::*;
use gtk4::gdk;
use gtk4::glib;
use gtk4::prelude::*;
use gtk4::{Align, Box as GtkBox, Button, Label, Orientation, Picture};
use libadwaita as adw;

use crate::i18n::tr;
use crate::jam::{
    JamCommand, JamServerHandle, JamSnapshot, JamTrackInfo, generate_qr_svg, start_jam_server,
};
use crate::ui::player::{play_queue_index, refresh_queue_ui};
use crate::ui::state::{AppState, next_queue_index};

pub struct JamRuntime {
    pub snapshot: Arc<RwLock<JamSnapshot>>,
    pub server: Option<JamServerHandle>,
    pub cmd_tx: async_channel::Sender<JamCommand>,
}

impl JamRuntime {
    pub fn new(cmd_tx: async_channel::Sender<JamCommand>) -> Self {
        Self {
            snapshot: Arc::new(RwLock::new(JamSnapshot::default())),
            server: None,
            cmd_tx,
        }
    }
}

pub fn sync_jam_snapshot(s: &AppState) {
    let Ok(mut snap) = s.jam.snapshot.write() else {
        return;
    };
    snap.is_playing = s.active_media().is_playing();
    snap.now_playing = s.current_track().map(|t| JamTrackInfo {
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
}

pub fn wire_jam_commands(
    state: Rc<RefCell<AppState>>,
    cmd_rx: async_channel::Receiver<JamCommand>,
) {
    glib::spawn_future_local(async move {
        while let Ok(cmd) = cmd_rx.recv().await {
            match cmd {
                JamCommand::AddTrack(track, _guest) => {
                    let should_start = {
                        let mut s = state.borrow_mut();
                        let was_empty = s.queue.is_empty() || s.current_index.is_none();
                        let start_idx = s.queue.len();
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
                    refresh_queue_ui(Rc::clone(&state));
                    if let Some(idx) = should_start {
                        play_queue_index(Rc::clone(&state), idx);
                    }
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
            }
        }
    });
}

pub fn open_jam_dialog(parent: &Button, state: Rc<RefCell<AppState>>, jam_btn: Button) {
    let dialog = adw::PreferencesDialog::builder()
        .title(tr("Modo SPIN", "SPIN Mode"))
        .content_width(480)
        .content_height(690)
        .build();
    let page = adw::PreferencesPage::new();

    let (is_active, allow_controls, url, guest_count, added_count) = {
        let s = state.borrow();
        let snap = s.jam.snapshot.read().ok();
        (
            s.jam.server.is_some(),
            snap.as_ref().map(|n| n.allow_controls).unwrap_or(false),
            snap.as_ref().map(|n| n.url.clone()).unwrap_or_default(),
            snap.as_ref().map(|n| n.guest_count).unwrap_or(0),
            snap.as_ref().map(|n| n.added_count).unwrap_or(0),
        )
    };

    let session_group = adw::PreferencesGroup::builder()
        .title(tr("Sesión compartida", "Shared session"))
        .description(tr(
            "Permite que otros dispositivos en la misma red Wi‑Fi agreguen canciones a tu cola",
            "Let other devices on the same Wi-Fi network add songs to your queue",
        ))
        .build();

    let active_row = adw::SwitchRow::builder()
        .title(tr("Activar Modo SPIN", "Enable SPIN Mode"))
        .subtitle(tr(
            "Inicia un servidor web local en tu red",
            "Starts a local web server on your network",
        ))
        .active(is_active)
        .build();

    let controls_row = adw::SwitchRow::builder()
        .title(tr("Permitir controles de reproducción", "Allow playback controls"))
        .subtitle(tr(
            "Los invitados podrán pausar y saltar canciones",
            "Guests can pause and skip tracks",
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

    session_group.add(&active_row);
    session_group.add(&controls_row);
    session_group.add(&status_row);
    page.add(&session_group);

    let connect_group = adw::PreferencesGroup::builder()
        .title(tr("Conectar dispositivos", "Connect devices"))
        .description(tr(
            "Escanea el código QR o abre la dirección en el navegador del celular",
            "Scan the QR code or open the address in a phone browser",
        ))
        .visible(is_active)
        .build();

    let qr_picture = Picture::builder()
        .width_request(184)
        .height_request(184)
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
    qr_frame.set_margin_top(14);
    qr_frame.set_margin_bottom(14);
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
        let st = Rc::clone(&state);
        copy_btn.connect_clicked(move |_| {
            if let Some(display) = gdk::Display::default() {
                display.clipboard().set_text(&url_lbl.text());
                st.borrow()
                    .show_toast(tr("Enlace SPIN copiado", "SPIN link copied"));
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

    {
        let state_sw = Rc::clone(&state);
        let connect_group = connect_group.clone();
        let status_row = status_row.clone();
        let url_label = url_label.clone();
        let qr_picture = qr_picture.clone();
        let jam_btn = jam_btn.clone();
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
                            sn.guests.clear();
                            sn.guest_count = 0;
                            sn.added_count = 0;
                        }
                        {
                            let mut s = state_sw.borrow_mut();
                            s.jam.server = Some(handle);
                            sync_jam_snapshot(&s);
                        }
                        url_label.set_label(&url);
                        update_qr(&qr_picture, &url);
                        connect_group.set_visible(true);
                        status_row.set_subtitle(&status_text(true, 0, 0));
                        jam_btn.add_css_class("suggested-action");
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
                }
                connect_group.set_visible(false);
                status_row.set_subtitle(&status_text(false, 0, 0));
                jam_btn.remove_css_class("suggested-action");
            }
        });
    }

    {
        let state_ctrl = Rc::clone(&state);
        controls_row.connect_active_notify(move |row| {
            let allow = row.is_active();
            if let Ok(mut sn) = state_ctrl.borrow().jam.snapshot.write() {
                sn.allow_controls = allow;
            }
        });
    }

    dialog.add(&page);
    let root = parent.root().and_then(|r| r.downcast::<gtk4::Window>().ok());
    dialog.present(root.as_ref());
}
