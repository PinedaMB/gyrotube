use std::cell::RefCell;
use std::rc::Rc;

use gtk4::prelude::*;
use gtk4::{
    Align, Box as GtkBox, Button, Image, Justification, Label, ListBox, MenuButton, Orientation,
    Picture, PolicyType, Popover, Scale, ScrolledWindow, SelectionMode, Spinner, Stack,
    StackTransitionType, ToggleButton,
};
use libadwaita as adw;

use crate::i18n::tr;
use crate::ui::state::AppState;
use crate::ui::widgets::clamped_cover;

pub struct SidebarPlayerWidgets {
    pub sidebar_toolbar: adw::ToolbarView,
    pub sidebar_collapse_btn: Button,
    pub tab_player: ToggleButton,
    pub tab_lyrics: ToggleButton,
    pub player_source_label: Label,
    pub player_cover: Picture,
    pub player_cover_spinner: Spinner,
    pub player_title: Label,
    pub player_artist: Label,
    pub player_album_label: Label,
    pub player_fav_btn: ToggleButton,
    pub player_radio_btn: Button,
    pub seek_scale: Scale,
    pub time_label: Label,
    pub duration_label: Label,
    pub quality_label: Label,
    pub playback_status: Label,
    pub play_pause_btn: Button,
    pub play_spinner: Spinner,
    pub prev_btn: Button,
    pub next_btn: Button,
    pub shuffle_btn: ToggleButton,
    pub repeat_btn: ToggleButton,
    pub volume_scale: Scale,
    pub volume_btn: MenuButton,
    pub sleep_timer_btn: MenuButton,
    pub party_btn: Button,
    pub lyrics_title_label: Label,
    pub lyrics_artist_label: Label,
    pub lyrics_sync_badge: Label,
    pub lyrics_body_label: Label,
    pub lyrics_lines_box: GtkBox,
    pub lyrics_scroll: ScrolledWindow,
    pub lyrics_resync_btn: Button,
    pub queue_list: ListBox,
    pub queue_empty_label: Label,
    pub queue_title_label: Label,
    pub queue_save_btn: Button,
    pub queue_clear_btn: Button,
    pub queue_autoplay_switch: gtk4::Switch,
    pub queue_suggestions_box: GtkBox,
    pub queue_suggestions_list: ListBox,
    pub queue_suggestions_refresh_btn: Button,
    pub spin_participants_box: GtkBox,
}

pub fn build_sidebar_player() -> SidebarPlayerWidgets {
    let player_source_label = Label::builder()
        .label("Sin reproducción")
        .xalign(0.0)
        .use_markup(true)
        .ellipsize(gtk4::pango::EllipsizeMode::End)
        .css_classes(["heading", "caption", "player-link"])
        .build();

    let context_text = GtkBox::new(Orientation::Vertical, 1);
    context_text.set_hexpand(true);
    context_text.append(
        &Label::builder()
            .label(tr("Reproduciendo desde", "Playing from"))
            .xalign(0.0)
            .css_classes(["dim-label", "caption"])
            .build(),
    );
    context_text.append(&player_source_label);

    let sidebar_collapse_btn = Button::builder()
        .icon_name("go-previous-symbolic")
        .tooltip_text(tr("Ocultar reproductor", "Hide player"))
        .css_classes(["flat", "circular"])
        .build();
    sidebar_collapse_btn.set_cursor_from_name(Some("pointer"));

    let context_row = GtkBox::new(Orientation::Horizontal, 10);
    context_row.add_css_class("sidebar-top-bar");
    context_row.append(&context_text);
    context_row.append(&sidebar_collapse_btn);

    let (player_cover_frame, player_cover) = clamped_cover(250, 280, "cover-art-large");
    let player_cover_spinner = Spinner::builder()
        .halign(Align::Center)
        .valign(Align::Center)
        .width_request(36)
        .height_request(36)
        .visible(false)
        .build();
    let player_cover_overlay = gtk4::Overlay::builder()
        .child(&player_cover_frame)
        .halign(Align::Center)
        .valign(Align::Center)
        .css_classes(["player-cover-glow"])
        .vexpand(true)
        .build();
    player_cover_overlay.add_overlay(&player_cover_spinner);

    let player_title = Label::builder()
        .label(tr("Nada reproduciéndose", "Nothing playing"))
        .halign(Align::Center)
        .ellipsize(gtk4::pango::EllipsizeMode::End)
        .max_width_chars(28)
        .css_classes(["title-3"])
        .build();

    let player_artist = Label::builder()
        .label(tr("Selecciona una canción", "Select a song"))
        .halign(Align::Center)
        .use_markup(true)
        .ellipsize(gtk4::pango::EllipsizeMode::End)
        .max_width_chars(30)
        .css_classes(["dim-label", "player-link"])
        .build();

    let player_album_label = Label::builder()
        .label("")
        .halign(Align::Center)
        .use_markup(true)
        .visible(false)
        .ellipsize(gtk4::pango::EllipsizeMode::End)
        .max_width_chars(32)
        .css_classes(["caption", "dim-label", "player-link"])
        .build();

    let volume_scale = Scale::with_range(Orientation::Vertical, 0.0, 1.0, 0.05);
    volume_scale.set_inverted(true);
    volume_scale.set_draw_value(false);
    volume_scale.set_height_request(120);
    volume_scale.set_value(1.0);

    let volume_popover = Popover::builder().child(&volume_scale).build();
    let volume_btn = MenuButton::builder()
        .icon_name("speaker-2-regular")
        .tooltip_text(tr("Volumen", "Volume"))
        .popover(&volume_popover)
        .css_classes(["flat", "circular"])
        .build();

    let player_fav_btn = ToggleButton::builder()
        .icon_name("heart-regular")
        .tooltip_text(tr("Agregar a favoritos", "Add to favorites"))
        .sensitive(false)
        .css_classes(["flat", "circular", "mini-fav-btn"])
        .build();
    player_fav_btn.connect_active_notify(|btn| {
        if btn.is_active() {
            btn.add_css_class("is-favorite");
        } else {
            btn.remove_css_class("is-favorite");
        }
    });

    let player_radio_btn = Button::builder()
        .icon_name("sound-wave-circle-regular")
        .tooltip_text(tr("Iniciar radio desde esta canción", "Start radio from this song"))
        .sensitive(false)
        .css_classes(["flat", "circular"])
        .build();

    let sleep_timer_btn = MenuButton::builder()
        .icon_name("timer-regular")
        .tooltip_text(tr("Temporizador de apagado", "Sleep timer"))
        .css_classes(["flat", "circular"])
        .build();

    let party_btn = Button::builder()
        .icon_name("sparkle-regular")
        .tooltip_text(tr("Modo Fiesta (F11)", "Party Mode (F11)"))
        .css_classes(["flat", "circular"])
        .build();

    let action_icons_row = GtkBox::new(Orientation::Horizontal, 14);
    action_icons_row.set_halign(Align::Center);
    action_icons_row.append(&volume_btn);
    action_icons_row.append(&player_fav_btn);
    action_icons_row.append(&player_radio_btn);
    action_icons_row.append(&sleep_timer_btn);
    action_icons_row.append(&party_btn);

    let seek_scale = Scale::with_range(Orientation::Horizontal, 0.0, 100.0, 1.0);
    seek_scale.set_draw_value(false);
    seek_scale.set_hexpand(true);
    seek_scale.set_sensitive(false);

    let time_label = Label::builder()
        .label("0:00")
        .css_classes(["caption", "numeric", "dim-label"])
        .build();

    let quality_label = Label::builder()
        .label("YTM")
        .halign(Align::Center)
        .hexpand(true)
        .css_classes(["quality-pill"])
        .build();

    let duration_label = Label::builder()
        .label("0:00")
        .css_classes(["caption", "numeric", "dim-label"])
        .build();

    let playback_status = Label::builder()
        .accessible_role(gtk4::AccessibleRole::Status)
        .ellipsize(gtk4::pango::EllipsizeMode::End)
        .css_classes(["caption", "heading"])
        .build();
    let audio_info = playback_status_stack(&playback_status, &quality_label, false);
    audio_info.set_hexpand(true);

    let time_row = GtkBox::new(Orientation::Horizontal, 8);
    time_row.append(&time_label);
    time_row.append(&audio_info);
    time_row.append(&duration_label);

    let shuffle_btn = ToggleButton::builder()
        .icon_name("arrow-shuffle-regular")
        .tooltip_text(tr("Aleatorio", "Shuffle"))
        .css_classes(["flat", "circular", "sidebar-toggle-btn"])
        .build();

    let prev_btn = Button::builder()
        .icon_name("previous-filled")
        .sensitive(false)
        .css_classes(["flat", "circular"])
        .build();

    let play_spinner = Spinner::builder()
        .halign(Align::Center)
        .valign(Align::Center)
        .width_request(18)
        .height_request(18)
        .build();
    let play_icon_img = Image::from_icon_name("play-filled");
    let play_btn_stack = Stack::new();
    play_btn_stack.add_named(&play_icon_img, Some("icon"));
    play_btn_stack.add_named(&play_spinner, Some("spinner"));
    play_btn_stack.set_visible_child_name("icon");

    let play_pause_btn = Button::builder()
        .child(&play_btn_stack)
        .name("play-filled")
        .sensitive(false)
        .css_classes(["suggested-action", "play-pill"])
        .build();

    let next_btn = Button::builder()
        .icon_name("next-filled")
        .sensitive(false)
        .css_classes(["flat", "circular"])
        .build();

    let repeat_btn = ToggleButton::builder()
        .icon_name("arrow-repeat-all-regular")
        .tooltip_text(tr("Repetir", "Repeat"))
        .css_classes(["flat", "circular", "sidebar-toggle-btn"])
        .build();

    let transport_row = GtkBox::new(Orientation::Horizontal, 10);
    transport_row.set_halign(Align::Center);
    transport_row.append(&shuffle_btn);
    transport_row.append(&prev_btn);
    transport_row.append(&play_pause_btn);
    transport_row.append(&next_btn);
    transport_row.append(&repeat_btn);

    let player_view = GtkBox::new(Orientation::Vertical, 12);
    player_view.set_margin_top(10);
    player_view.set_margin_bottom(12);
    player_view.set_margin_start(18);
    player_view.set_margin_end(18);
    player_view.set_vexpand(true);
    player_view.append(&player_cover_overlay);
    player_view.append(&player_title);
    player_view.append(&player_artist);
    player_view.append(&player_album_label);
    player_view.append(&action_icons_row);
    player_view.append(&seek_scale);
    player_view.append(&time_row);
    player_view.append(&transport_row);

    // Lyrics View
    let lyrics_title_label = Label::builder()
        .label(tr("Letras", "Lyrics"))
        .xalign(0.0)
        .ellipsize(gtk4::pango::EllipsizeMode::End)
        .css_classes(["title-4"])
        .build();
    let lyrics_artist_label = Label::builder()
        .label("")
        .xalign(0.0)
        .ellipsize(gtk4::pango::EllipsizeMode::End)
        .visible(false)
        .css_classes(["caption", "dim-label"])
        .build();
    let lyrics_sync_badge = Label::builder()
        .label("")
        .visible(false)
        .halign(Align::End)
        .valign(Align::Center)
        .build();

    let lyrics_title_col = GtkBox::new(Orientation::Vertical, 2);
    lyrics_title_col.set_hexpand(true);
    lyrics_title_col.append(&lyrics_title_label);
    lyrics_title_col.append(&lyrics_artist_label);

    let lyrics_header = GtkBox::new(Orientation::Horizontal, 8);
    lyrics_header.set_valign(Align::Center);
    lyrics_header.append(&lyrics_title_col);
    lyrics_header.append(&lyrics_sync_badge);

    let lyrics_body_label = Label::builder()
        .label(tr(
            "Reproduce una canción para ver su letra.",
            "Play a song to view its lyrics.",
        ))
        .xalign(0.0)
        .yalign(0.0)
        .wrap(true)
        .selectable(true)
        .css_classes(["lyrics-plain"])
        .build();
    let lyrics_lines_box = GtkBox::new(Orientation::Vertical, 2);
    lyrics_lines_box.set_margin_top(4);
    lyrics_lines_box.append(&lyrics_body_label);

    let lyrics_box = GtkBox::new(Orientation::Vertical, 12);
    lyrics_box.set_margin_top(14);
    lyrics_box.set_margin_bottom(14);
    lyrics_box.set_margin_start(14);
    lyrics_box.set_margin_end(14);
    lyrics_box.append(&lyrics_header);
    lyrics_box.append(&lyrics_lines_box);

    let lyrics_scroll = ScrolledWindow::builder()
        .hscrollbar_policy(PolicyType::Never)
        .vexpand(true)
        .child(&lyrics_box)
        .build();

    let lyrics_resync_btn = Button::builder()
        .halign(Align::Center)
        .valign(Align::End)
        .margin_bottom(14)
        .visible(false)
        .css_classes(["suggested-action", "pill", "lyrics-resync-btn"])
        .build();
    let resync_content = GtkBox::new(Orientation::Horizontal, 6);
    resync_content.append(&Image::from_icon_name("arrow-sync-regular"));
    resync_content.append(&Label::new(Some(tr("Volver a la letra", "Back to lyrics"))));
    lyrics_resync_btn.set_child(Some(&resync_content));

    let lyrics_overlay = gtk4::Overlay::builder()
        .vexpand(true)
        .child(&lyrics_scroll)
        .build();
    lyrics_overlay.add_overlay(&lyrics_resync_btn);

    // Compact now-playing + controls bar for the Lyrics tab
    let (lyrics_np_cover_frame, lyrics_np_cover) = clamped_cover(40, 40, "cover-art");
    let lyrics_np_title = Label::builder()
        .xalign(0.0)
        .ellipsize(gtk4::pango::EllipsizeMode::End)
        .css_classes(["heading"])
        .build();
    let lyrics_np_artist = Label::builder()
        .xalign(0.0)
        .ellipsize(gtk4::pango::EllipsizeMode::End)
        .css_classes(["caption", "dim-label"])
        .build();

    player_cover
        .bind_property("paintable", &lyrics_np_cover, "paintable")
        .sync_create()
        .build();
    player_title
        .bind_property("label", &lyrics_np_title, "label")
        .sync_create()
        .build();
    player_artist
        .bind_property("label", &lyrics_np_artist, "label")
        .transform_to(|_, val: String| {
            Some(crate::ui::widgets::strip_link_markup(&val).to_value())
        })
        .sync_create()
        .build();

    let lyrics_np_text = GtkBox::new(Orientation::Vertical, 1);
    lyrics_np_text.set_hexpand(true);
    lyrics_np_text.set_valign(Align::Center);
    lyrics_np_text.append(&lyrics_np_title);
    lyrics_np_text.append(&playback_status_stack(
        &playback_status,
        &lyrics_np_artist,
        true,
    ));

    let lyrics_np_info_box = GtkBox::new(Orientation::Horizontal, 10);
    lyrics_np_info_box.append(&lyrics_np_cover_frame);
    lyrics_np_info_box.append(&lyrics_np_text);
    let lyrics_np_open_btn = Button::builder()
        .child(&lyrics_np_info_box)
        .hexpand(true)
        .tooltip_text(tr("Ir al reproductor", "Go to player"))
        .css_classes(["flat"])
        .build();

    let lyrics_prev_btn = Button::builder()
        .icon_name("previous-filled")
        .tooltip_text(tr("Anterior", "Previous"))
        .css_classes(["flat", "circular"])
        .build();
    prev_btn
        .bind_property("sensitive", &lyrics_prev_btn, "sensitive")
        .sync_create()
        .build();
    {
        let p = prev_btn.clone();
        lyrics_prev_btn.connect_clicked(move |_| p.emit_clicked());
    }

    let lyrics_play_btn = Button::builder()
        .tooltip_text(tr("Reproducir / Pausar", "Play / Pause"))
        .css_classes(["suggested-action", "circular"])
        .build();
    attach_mirrored_play_stack(&lyrics_play_btn, &play_pause_btn, &play_spinner, 16);
    play_pause_btn
        .bind_property("sensitive", &lyrics_play_btn, "sensitive")
        .sync_create()
        .build();
    {
        let pp = play_pause_btn.clone();
        lyrics_play_btn.connect_clicked(move |_| pp.emit_clicked());
    }

    let lyrics_next_btn = Button::builder()
        .icon_name("next-filled")
        .tooltip_text(tr("Siguiente", "Next"))
        .css_classes(["flat", "circular"])
        .build();
    next_btn
        .bind_property("sensitive", &lyrics_next_btn, "sensitive")
        .sync_create()
        .build();
    {
        let n = next_btn.clone();
        lyrics_next_btn.connect_clicked(move |_| n.emit_clicked());
    }

    let lyrics_controls_box = GtkBox::new(Orientation::Horizontal, 4);
    lyrics_controls_box.set_valign(Align::Center);
    lyrics_controls_box.append(&lyrics_prev_btn);
    lyrics_controls_box.append(&lyrics_play_btn);
    lyrics_controls_box.append(&lyrics_next_btn);

    let lyrics_np_row = GtkBox::new(Orientation::Horizontal, 6);
    lyrics_np_row.append(&lyrics_np_open_btn);
    lyrics_np_row.append(&lyrics_controls_box);

    let lyrics_np_progress = build_bound_progress_bar(&seek_scale.adjustment());

    let lyrics_np_bar = GtkBox::new(Orientation::Vertical, 6);
    lyrics_np_bar.add_css_class("lyrics-np-bar");
    lyrics_np_bar.set_margin_top(4);
    lyrics_np_bar.set_margin_bottom(8);
    lyrics_np_bar.set_margin_start(10);
    lyrics_np_bar.set_margin_end(10);
    lyrics_np_bar.append(&lyrics_np_progress);
    lyrics_np_bar.append(&lyrics_np_row);

    let lyrics_page = GtkBox::new(Orientation::Vertical, 0);
    lyrics_page.set_vexpand(true);
    lyrics_page.append(&lyrics_overlay);
    lyrics_page.append(&lyrics_np_bar);

    // Queue View
    let queue_list = ListBox::builder()
        .selection_mode(SelectionMode::None)
        .activate_on_single_click(true)
        .css_classes(["boxed-list"])
        .build();
    let queue_empty_label = Label::builder()
        .label(tr("La cola está vacía.", "The queue is empty."))
        .justify(Justification::Center)
        .margin_top(36)
        .css_classes(["dim-label"])
        .build();

    let queue_title_label = Label::builder()
        .label(tr("Cola", "Queue"))
        .xalign(0.0)
        .hexpand(true)
        .css_classes(["title-4"])
        .build();
    let queue_save_btn = Button::builder()
        .icon_name("music-note-2-regular")
        .tooltip_text(tr("Guardar cola como playlist", "Save queue as playlist"))
        .css_classes(["flat", "circular"])
        .build();
    let queue_clear_btn = Button::builder()
        .label(tr("Limpiar", "Clear"))
        .valign(Align::Center)
        .css_classes(["flat", "queue-clear-btn"])
        .build();
    queue_clear_btn.set_cursor_from_name(Some("pointer"));
    let queue_header = GtkBox::new(Orientation::Horizontal, 6);
    queue_header.append(&queue_title_label);
    queue_header.append(&queue_save_btn);
    queue_header.append(&queue_clear_btn);

    let queue_box = GtkBox::new(Orientation::Vertical, 10);
    queue_box.set_margin_top(12);
    queue_box.set_margin_bottom(12);
    queue_box.set_margin_start(12);
    queue_box.set_margin_end(12);
    let spin_participants_box = GtkBox::new(Orientation::Vertical, 6);
    spin_participants_box.add_css_class("spin-participants-card");
    spin_participants_box.set_visible(false);

    let queue_autoplay_title = Label::builder()
        .label(tr("Reproducción automática", "Autoplay"))
        .xalign(0.0)
        .css_classes(["heading", "caption"])
        .build();
    let queue_autoplay_sub = Label::builder()
        .label(tr(
            "Añadir canciones similares al elegir una suelta o al terminar",
            "Queue similar songs for single tracks or when queue ends",
        ))
        .xalign(0.0)
        .wrap(true)
        .css_classes(["dim-label", "caption"])
        .build();
    let queue_autoplay_text = GtkBox::new(Orientation::Vertical, 2);
    queue_autoplay_text.set_hexpand(true);
    queue_autoplay_text.append(&queue_autoplay_title);
    queue_autoplay_text.append(&queue_autoplay_sub);

    let queue_autoplay_switch = gtk4::Switch::builder()
        .valign(Align::Center)
        .build();
    let queue_autoplay_row = GtkBox::new(Orientation::Horizontal, 10);
    queue_autoplay_row.add_css_class("queue-autoplay-row");
    queue_autoplay_row.append(&queue_autoplay_text);
    queue_autoplay_row.append(&queue_autoplay_switch);

    let queue_suggestions_title = Label::builder()
        .label(tr("Sugeridas", "Suggested"))
        .xalign(0.0)
        .hexpand(true)
        .css_classes(["title-4"])
        .build();
    let queue_suggestions_refresh_btn = Button::builder()
        .icon_name("view-refresh-symbolic")
        .tooltip_text(tr("Actualizar sugerencias", "Refresh suggestions"))
        .valign(Align::Center)
        .css_classes(["flat", "circular"])
        .build();
    queue_suggestions_refresh_btn.set_cursor_from_name(Some("pointer"));
    let queue_suggestions_header = GtkBox::new(Orientation::Horizontal, 6);
    queue_suggestions_header.append(&queue_suggestions_title);
    queue_suggestions_header.append(&queue_suggestions_refresh_btn);

    let queue_suggestions_list = ListBox::builder()
        .selection_mode(SelectionMode::None)
        .activate_on_single_click(true)
        .css_classes(["boxed-list"])
        .build();

    let queue_suggestions_box = GtkBox::new(Orientation::Vertical, 8);
    queue_suggestions_box.set_margin_top(6);
    queue_suggestions_box.set_visible(false);
    queue_suggestions_box.append(&queue_suggestions_header);
    queue_suggestions_box.append(&queue_suggestions_list);

    queue_box.append(&queue_header);
    queue_box.append(&spin_participants_box);
    queue_box.append(&queue_empty_label);
    queue_box.append(&queue_list);
    queue_box.append(&queue_autoplay_row);
    queue_box.append(&queue_suggestions_box);

    let queue_scroll = ScrolledWindow::builder()
        .hscrollbar_policy(PolicyType::Never)
        .vexpand(true)
        .child(&queue_box)
        .build();

    let sidebar_stack = Stack::builder()
        .transition_type(StackTransitionType::Crossfade)
        .vexpand(true)
        .build();
    sidebar_stack.add_named(&player_view, Some("player"));
    sidebar_stack.add_named(&lyrics_page, Some("lyrics"));
    sidebar_stack.add_named(&queue_scroll, Some("queue"));

    let tab_player = make_sidebar_tab("music-note-2-regular", tr("Reproductor", "Player"), true);
    {
        let tp = tab_player.clone();
        lyrics_np_open_btn.connect_clicked(move |_| tp.set_active(true));
    }
    let tab_lyrics = make_sidebar_tab("comment-quote-regular", tr("Letras", "Lyrics"), false);
    let tab_queue = make_sidebar_tab("document-bullet-list-regular", tr("Cola", "Queue"), false);
    tab_lyrics.set_group(Some(&tab_player));
    tab_queue.set_group(Some(&tab_player));

    tab_player.set_hexpand(true);
    tab_lyrics.set_hexpand(true);
    tab_queue.set_hexpand(true);

    let sidebar_tabs = GtkBox::new(Orientation::Horizontal, 4);
    sidebar_tabs.set_homogeneous(true);
    sidebar_tabs.add_css_class("sidebar-tabs");
    sidebar_tabs.append(&tab_player);
    sidebar_tabs.append(&tab_lyrics);
    sidebar_tabs.append(&tab_queue);

    let sidebar_toolbar = adw::ToolbarView::new();
    sidebar_toolbar.add_top_bar(&context_row);
    sidebar_toolbar.set_content(Some(&sidebar_stack));
    sidebar_toolbar.add_bottom_bar(&sidebar_tabs);
    sidebar_toolbar.add_css_class("sidebar-pane");
    sidebar_toolbar.add_css_class("ambient-player");

    for (tab, page) in [
        (&tab_player, "player"),
        (&tab_lyrics, "lyrics"),
        (&tab_queue, "queue"),
    ] {
        let stack = sidebar_stack.clone();
        tab.connect_toggled(move |btn| {
            if btn.is_active() {
                stack.set_visible_child_name(page);
            }
        });
    }

    SidebarPlayerWidgets {
        sidebar_toolbar,
        sidebar_collapse_btn,
        tab_player,
        tab_lyrics,
        player_source_label,
        player_cover,
        player_cover_spinner,
        player_title,
        player_artist,
        player_album_label,
        player_fav_btn,
        player_radio_btn,
        seek_scale,
        time_label,
        duration_label,
        quality_label,
        playback_status,
        play_pause_btn,
        play_spinner,
        prev_btn,
        next_btn,
        shuffle_btn,
        repeat_btn,
        volume_scale,
        volume_btn,
        sleep_timer_btn,
        party_btn,
        lyrics_title_label,
        lyrics_artist_label,
        lyrics_sync_badge,
        lyrics_body_label,
        lyrics_lines_box,
        lyrics_scroll,
        lyrics_resync_btn,
        queue_list,
        queue_empty_label,
        queue_title_label,
        queue_save_btn,
        queue_clear_btn,
        queue_autoplay_switch,
        queue_suggestions_box,
        queue_suggestions_list,
        queue_suggestions_refresh_btn,
        spin_participants_box,
    }
}

fn playback_status_stack(status: &Label, normal: &Label, compact: bool) -> gtk4::Stack {
    let stack = gtk4::Stack::new();
    let alternate = if compact {
        let label = Label::builder()
            .accessible_role(gtk4::AccessibleRole::Status)
            .xalign(0.0)
            .ellipsize(gtk4::pango::EllipsizeMode::End)
            .css_classes(["caption", "heading"])
            .build();
        status
            .bind_property("label", &label, "label")
            .sync_create()
            .build();
        label
    } else {
        status.clone()
    };
    stack.add_named(normal, Some("normal"));
    stack.add_named(&alternate, Some("status"));
    status
        .bind_property("label", &stack, "visible-child-name")
        .transform_to(|_, text: String| {
            Some(if text.is_empty() { "normal" } else { "status" }.to_value())
        })
        .sync_create()
        .build();
    stack
}

pub fn build_mini_bar(
    state: &Rc<RefCell<AppState>>,
    play_pause_btn: &Button,
    next_btn: &Button,
    compact: bool,
) -> (GtkBox, Button) {
    let (cover_frame, cover) = clamped_cover(40, 40, "cover-art");
    let title_builder = Label::builder()
        .xalign(0.0)
        .ellipsize(gtk4::pango::EllipsizeMode::End)
        .css_classes(["heading"]);
    let title = if compact {
        title_builder.max_width_chars(18).build()
    } else {
        title_builder.build()
    };
    let artist_builder = Label::builder()
        .xalign(0.0)
        .ellipsize(gtk4::pango::EllipsizeMode::End)
        .css_classes(["caption", "dim-label"]);
    let artist = if compact {
        artist_builder.max_width_chars(20).build()
    } else {
        artist_builder.build()
    };

    let mini_fav_btn = Button::builder()
        .icon_name("heart-regular")
        .tooltip_text(tr("Agregar a favoritos", "Add to favorites"))
        .css_classes(["circular", "flat", "mini-fav-btn"])
        .build();
    let mini_prev_btn = Button::builder()
        .icon_name("previous-filled")
        .tooltip_text(tr("Anterior", "Previous"))
        .css_classes(["circular", "flat"])
        .build();
    let mini_play_btn = Button::builder()
        .icon_name("play-filled")
        .tooltip_text(tr("Reproducir / Pausar", "Play / Pause"))
        .css_classes(["circular", "suggested-action"])
        .build();
    let mini_next_btn = Button::builder()
        .icon_name("next-filled")
        .tooltip_text(tr("Siguiente", "Next"))
        .css_classes(["circular", "flat"])
        .build();
    let (prev_btn_ref, fav_btn_ref) = {
        let s = state.borrow();
        attach_mirrored_play_stack(&mini_play_btn, &s.play_pause_btn, &s.play_spinner, 16);
        s.player_cover
            .bind_property("paintable", &cover, "paintable")
            .sync_create()
            .build();
        s.player_title
            .bind_property("label", &title, "label")
            .sync_create()
            .build();
        s.player_artist
            .bind_property("label", &artist, "label")
            .transform_to(|_, val: String| {
                Some(crate::ui::widgets::strip_link_markup(&val).to_value())
            })
            .sync_create()
            .build();
        s.player_fav_btn
            .bind_property("icon-name", &mini_fav_btn, "icon-name")
            .sync_create()
            .build();
        s.player_fav_btn
            .bind_property("sensitive", &mini_fav_btn, "sensitive")
            .sync_create()
            .build();
        if s.player_fav_btn.is_active() {
            mini_fav_btn.add_css_class("is-favorite");
        }
        {
            let fav_mirror = mini_fav_btn.clone();
            s.player_fav_btn.connect_active_notify(move |btn| {
                if btn.is_active() {
                    fav_mirror.add_css_class("is-favorite");
                } else {
                    fav_mirror.remove_css_class("is-favorite");
                }
            });
        }
        s.prev_btn
            .bind_property("sensitive", &mini_prev_btn, "sensitive")
            .sync_create()
            .build();
        s.play_pause_btn
            .bind_property("sensitive", &mini_play_btn, "sensitive")
            .sync_create()
            .build();
        s.next_btn
            .bind_property("sensitive", &mini_next_btn, "sensitive")
            .sync_create()
            .build();
        (s.prev_btn.clone(), s.player_fav_btn.clone())
    };

    let text_col = GtkBox::new(Orientation::Vertical, 1);
    text_col.set_hexpand(true);
    text_col.set_valign(Align::Center);
    text_col.append(&title);
    text_col.append(&playback_status_stack(
        &state.borrow().playback_status,
        &artist,
        true,
    ));

    let open_box = GtkBox::new(Orientation::Horizontal, 10);
    open_box.append(&cover_frame);
    open_box.append(&text_col);
    let open_btn = Button::builder()
        .child(&open_box)
        .hexpand(true)
        .tooltip_text(tr("Abrir reproductor", "Open player"))
        .css_classes(["flat"])
        .build();

    mini_fav_btn.connect_clicked(move |_| fav_btn_ref.set_active(!fav_btn_ref.is_active()));
    mini_prev_btn.connect_clicked(move |_| prev_btn_ref.emit_clicked());
    {
        let play_btn = play_pause_btn.clone();
        mini_play_btn.connect_clicked(move |_| play_btn.emit_clicked());
    }
    {
        let nxt_btn = next_btn.clone();
        mini_next_btn.connect_clicked(move |_| nxt_btn.emit_clicked());
    }

    let mini_progress = build_bound_progress_bar(&state.borrow().seek_scale.adjustment());

    let controls_box = GtkBox::new(Orientation::Horizontal, 4);
    controls_box.set_valign(Align::Center);
    if compact {
        controls_box.append(&mini_fav_btn);
    }
    controls_box.append(&mini_prev_btn);
    controls_box.append(&mini_play_btn);
    controls_box.append(&mini_next_btn);

    let row = GtkBox::builder()
        .orientation(Orientation::Horizontal)
        .spacing(6)
        .build();
    row.append(&open_btn);
    row.append(&controls_box);

    let bar = GtkBox::builder()
        .orientation(Orientation::Vertical)
        .spacing(4)
        .visible(false)
        .build();
    bar.append(&row);
    bar.append(&mini_progress);
    (bar, open_btn)
}

pub fn build_bound_progress_bar(adj: &gtk4::Adjustment) -> gtk4::ProgressBar {
    let pb = gtk4::ProgressBar::builder()
        .fraction(0.0)
        .css_classes(["mini-progress"])
        .build();
    let update_prog = {
        let pb = pb.clone();
        let adj_ref = adj.clone();
        move || {
            let upper = adj_ref.upper();
            let frac = if upper > 0.0 {
                (adj_ref.value() / upper).clamp(0.0, 1.0)
            } else {
                0.0
            };
            pb.set_fraction(frac);
        }
    };
    update_prog();
    let u1 = update_prog.clone();
    adj.connect_value_changed(move |_| u1());
    adj.connect_upper_notify(move |_| update_prog());
    pb
}

pub fn set_play_button_icon(btn: &Button, icon_name: &str) {
    if let Some(stack) = btn.child().and_downcast::<Stack>() {
        if let Some(img) = stack.child_by_name("icon").and_downcast::<Image>() {
            img.set_icon_name(Some(icon_name));
        }
        stack.set_visible_child_name("icon");
    } else {
        btn.set_icon_name(icon_name);
    }
    btn.set_widget_name(icon_name);
}

pub fn set_play_button_loading(btn: &Button, spinner: &Spinner) {
    if let Some(stack) = btn.child().and_downcast::<Stack>() {
        stack.set_visible_child_name("spinner");
    }
    spinner.set_visible(true);
    spinner.set_spinning(true);
}

pub fn attach_mirrored_play_stack(
    target_btn: &Button,
    main_btn: &Button,
    main_spinner: &Spinner,
    spinner_size: i32,
) {
    let initial_icon = {
        let name = main_btn.widget_name();
        if name.is_empty() {
            String::from("play-filled")
        } else {
            name.to_string()
        }
    };
    let icon_img = Image::from_icon_name(&initial_icon);
    let sub_spinner = Spinner::builder()
        .halign(Align::Center)
        .valign(Align::Center)
        .width_request(spinner_size)
        .height_request(spinner_size)
        .build();
    let stack = Stack::new();
    stack.add_named(&icon_img, Some("icon"));
    stack.add_named(&sub_spinner, Some("spinner"));
    stack.set_visible_child_name(if main_spinner.is_spinning() {
        "spinner"
    } else {
        "icon"
    });
    target_btn.set_child(Some(&stack));

    main_spinner
        .bind_property("spinning", &sub_spinner, "spinning")
        .sync_create()
        .build();

    let stack_for_spin = stack.clone();
    main_spinner.connect_spinning_notify(move |sp| {
        if sp.is_spinning() {
            stack_for_spin.set_visible_child_name("spinner");
        } else {
            stack_for_spin.set_visible_child_name("icon");
        }
    });

    let img_for_name = icon_img;
    main_btn.connect_name_notify(move |b| {
        let name = b.widget_name();
        if !name.is_empty() {
            img_for_name.set_icon_name(Some(name.as_str()));
        }
    });
}

fn make_sidebar_tab(icon_name: &str, label_text: &str, active: bool) -> ToggleButton {
    let content = GtkBox::new(Orientation::Horizontal, 6);
    content.set_halign(Align::Center);
    content.append(&Image::from_icon_name(icon_name));
    content.append(&Label::new(Some(label_text)));

    ToggleButton::builder()
        .child(&content)
        .active(active)
        .css_classes(["flat"])
        .build()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore = "Requires a GTK display"]
    fn playback_phase_reaches_main_and_compact_player_without_losing_metadata() {
        adw::init().unwrap();
        let status = Label::new(None);
        let quality = Label::new(Some("OPUS"));
        let artist = Label::new(Some("Chéjere"));
        let main = playback_status_stack(&status, &quality, false);
        let compact = playback_status_stack(&status, &artist, true);
        assert_eq!(main.visible_child_name().as_deref(), Some("normal"));
        assert_eq!(compact.visible_child_name().as_deref(), Some("normal"));

        for phase in ["Buscando audio…", "Preparando audio…", "No se pudo reproducir"] {
            status.set_label(phase);
            for view in [&main, &compact] {
                let text = view.visible_child().and_downcast::<Label>().unwrap();
                assert_eq!(text.label(), phase);
            }
        }

        status.set_label("");
        assert_eq!(main.visible_child().and_downcast::<Label>(), Some(quality));
        assert_eq!(compact.visible_child().and_downcast::<Label>(), Some(artist));
    }
}
