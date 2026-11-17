use std::cell::{Cell, RefCell};
use std::rc::Rc;

use adw::prelude::*;
use gtk4::gdk;
use gtk4::glib;
use gtk4::prelude::*;
use gtk4::{
    Align, Box as GtkBox, Button, Image, Label, MenuButton, Orientation, Popover, SearchEntry,
};
use libadwaita as adw;

use crate::i18n::{AppLanguage, set_language_preference, tr};
use crate::models::save_search_history;
use crate::ui::state::AppState;
use crate::ui::widgets::spawn_bg;

pub struct HeaderWidgets {
    pub header_bar: adw::HeaderBar,
    pub brand_title: Label,
    pub brand_badge: Label,
    pub search_entry: SearchEntry,
    pub settings_btn: Button,
    pub menu_popover: Popover,
    pub party_btn: Button,
    pub jam_btn: Button,
}

pub fn build_header_bar(_content_view_stack: &adw::ViewStack) -> HeaderWidgets {
    let brand_icon = match gdk::Texture::from_bytes(&glib::Bytes::from_static(include_bytes!(
        "../../gyrotube.svg"
    ))) {
        Ok(tex) => Image::builder()
            .paintable(&tex)
            .pixel_size(28)
            .halign(Align::Center)
            .valign(Align::Center)
            .css_classes(["brand-icon"])
            .build(),
        Err(_) => Image::builder()
            .icon_name(crate::ui::APP_ID)
            .pixel_size(28)
            .halign(Align::Center)
            .valign(Align::Center)
            .css_classes(["brand-icon"])
            .build(),
    };

    let brand_title = Label::builder()
        .label("GyroTube")
        .css_classes(["brand-title"])
        .build();
    let brand_badge = Label::builder()
        .label(tr("Música", "Music"))
        .css_classes(["brand-badge"])
        .build();

    let brand_box = GtkBox::new(Orientation::Horizontal, 8);
    brand_box.set_valign(Align::Center);
    brand_box.append(&brand_icon);
    brand_box.append(&brand_title);
    brand_box.append(&brand_badge);

    let search_entry = SearchEntry::builder()
        .placeholder_text(tr(
            "Buscar canciones, álbumes, artistas…",
            "Search songs, albums, artists…",
        ))
        .width_chars(8)
        .hexpand(true)
        .can_focus(false)
        .css_classes(["header-search"])
        .build();
    let top_search_clamp = adw::Clamp::builder()
        .maximum_size(360)
        .child(&search_entry)
        .build();

    let menu_box = GtkBox::new(Orientation::Vertical, 4);
    menu_box.set_margin_top(6);
    menu_box.set_margin_bottom(6);
    menu_box.set_margin_start(6);
    menu_box.set_margin_end(6);

    let settings_btn = Button::builder()
        .label(tr("Configuración", "Settings"))
        .css_classes(["flat"])
        .halign(Align::Fill)
        .build();
    settings_btn.set_cursor_from_name(Some("pointer"));

    let about_btn = Button::builder()
        .label(tr("Acerca de GyroTube", "About GyroTube"))
        .css_classes(["flat"])
        .halign(Align::Fill)
        .build();
    about_btn.set_cursor_from_name(Some("pointer"));

    menu_box.append(&settings_btn);
    menu_box.append(&about_btn);

    let menu_popover = Popover::builder().child(&menu_box).build();
    let menu_btn = MenuButton::builder()
        .icon_name("open-menu-symbolic")
        .tooltip_text(tr("Opciones", "Options"))
        .popover(&menu_popover)
        .css_classes(["flat", "circular"])
        .build();
    menu_btn.set_cursor_from_name(Some("pointer"));

    {
        let pop = menu_popover.clone();
        about_btn.connect_clicked(move |btn| {
            pop.popdown();
            let dialog = adw::AboutDialog::builder()
                .application_name("GyroTube")
                .application_icon(crate::ui::APP_ID)
                .version(env!("CARGO_PKG_VERSION"))
                .comments(tr(
                    "Reproductor de música para YouTube Music",
                    "Music player for YouTube Music",
                ))
                .build();
            let root = btn.root().and_then(|r| r.downcast::<gtk4::Window>().ok());
            dialog.present(root.as_ref());
        });
    }

    let party_btn = Button::builder()
        .icon_name("sparkle-regular")
        .tooltip_text(tr("Modo Fiesta (F11)", "Party Mode (F11)"))
        .css_classes(["flat", "circular"])
        .build();
    party_btn.set_cursor_from_name(Some("pointer"));

    let jam_btn = Button::builder()
        .icon_name("people-community-regular")
        .tooltip_text(tr("Modo SPIN", "SPIN Mode"))
        .css_classes(["flat", "circular"])
        .build();
    jam_btn.set_cursor_from_name(Some("pointer"));

    let header_bar = adw::HeaderBar::builder()
        .centering_policy(adw::CenteringPolicy::Loose)
        .title_widget(&top_search_clamp)
        .build();
    header_bar.pack_start(&brand_box);
    header_bar.pack_end(&menu_btn);
    header_bar.pack_end(&party_btn);
    header_bar.pack_end(&jam_btn);

    HeaderWidgets {
        header_bar,
        brand_title,
        brand_badge,
        search_entry,
        settings_btn,
        menu_popover,
        party_btn,
        jam_btn,
    }
}

pub fn wire_settings_dialog(
    settings_btn: &Button,
    menu_popover: &Popover,
    state: Rc<RefCell<AppState>>,
) {
    let pop = menu_popover.clone();
    settings_btn.connect_clicked(move |btn| {
        pop.popdown();
        let dialog = adw::PreferencesDialog::builder()
            .title(tr("Configuración", "Settings"))
            .build();
        let page = adw::PreferencesPage::new();

        let lang_group = adw::PreferencesGroup::builder()
            .title(tr("Idioma", "Language"))
            .description(tr(
                "Detección automática del sistema o selección manual",
                "Automatic system detection or manual selection",
            ))
            .build();
        let lang_model = gtk4::StringList::new(&[
            tr("Automático (sistema)", "Automatic (system)"),
            "Español",
            "English",
        ]);
        let current_lang_idx = match state.borrow().language {
            AppLanguage::Auto => 0,
            AppLanguage::Es => 1,
            AppLanguage::En => 2,
        };
        let lang_row = adw::ComboRow::builder()
            .title(tr("Idioma de la interfaz", "Interface language"))
            .model(&lang_model)
            .selected(current_lang_idx)
            .build();
        {
            let state_lang = Rc::clone(&state);
            lang_row.connect_selected_notify(move |row| {
                let pref = match row.selected() {
                    1 => AppLanguage::Es,
                    2 => AppLanguage::En,
                    _ => AppLanguage::Auto,
                };
                {
                    let mut s = state_lang.borrow_mut();
                    if s.language == pref {
                        return;
                    }
                    s.language = pref;
                    set_language_preference(pref);
                    s.save_session();
                    s.show_toast(tr(
                        "Idioma guardado. Reinicia la app para aplicar todos los textos.",
                        "Language saved. Restart the app to apply all texts.",
                    ));
                }
            });
        }
        lang_group.add(&lang_row);
        page.add(&lang_group);

        let playback_group = adw::PreferencesGroup::builder()
            .title(tr("Reproducción", "Playback"))
            .description(tr(
                "Transición suave entre pistas",
                "Smooth transition between tracks",
            ))
            .build();

        let current_cf = state.borrow().crossfade_secs.min(10);
        let format_cf = |secs: u32| -> String {
            if secs == 0 {
                String::from(tr("Desactivado (0 s)", "Disabled (0 s)"))
            } else {
                format!("{secs} s")
            }
        };
        let cf_row = adw::ActionRow::builder()
            .title("Crossfade")
            .subtitle(&format_cf(current_cf))
            .build();
        let cf_scale = gtk4::Scale::with_range(Orientation::Horizontal, 0.0, 10.0, 1.0);
        cf_scale.set_round_digits(0);
        cf_scale.set_draw_value(false);
        cf_scale.set_width_request(160);
        cf_scale.set_valign(Align::Center);
        cf_scale.set_value(current_cf as f64);
        {
            let state_cf = Rc::clone(&state);
            let cf_row_ref = cf_row.clone();
            cf_scale.connect_value_changed(move |scale| {
                let secs = (scale.value().round() as u32).min(10);
                cf_row_ref.set_subtitle(&format_cf(secs));
                {
                    let mut s = state_cf.borrow_mut();
                    s.crossfade_secs = secs;
                    s.crossfade_enabled = secs > 0;
                    if secs == 0 {
                        s.crossfade_active = false;
                        s.crossfade_generation = s.crossfade_generation.wrapping_add(1);
                        s.crossfade_triggered_for = None;
                    }
                    s.save_session();
                }
            });
        }
        cf_row.add_suffix(&cf_scale);
        playback_group.add(&cf_row);
        page.add(&playback_group);

        let storage_group = adw::PreferencesGroup::builder()
            .title(tr("Almacenamiento", "Storage"))
            .description(tr(
                "Audios descargados para reproducción instantánea",
                "Downloaded audio files for instant playback",
            ))
            .build();

        let streams_dir = crate::models::cache_dir().map(|d| d.join("streams"));
        let calc_size_text = |dir: Option<&std::path::PathBuf>| -> String {
            let bytes: u64 = dir
                .and_then(|d| std::fs::read_dir(d).ok())
                .into_iter()
                .flatten()
                .filter_map(|e| e.ok())
                .filter_map(|e| e.metadata().ok())
                .map(|m| m.len())
                .sum();
            let mb = bytes as f64 / (1024.0 * 1024.0);
            format!("{mb:.1} MB en caché")
        };

        let cache_row = adw::ActionRow::builder()
            .title("Caché de audio")
            .subtitle(&calc_size_text(streams_dir.as_ref()))
            .build();
        let clear_btn = Button::builder()
            .label("Limpiar")
            .valign(Align::Center)
            .css_classes(["destructive-action"])
            .build();
        clear_btn.set_cursor_from_name(Some("pointer"));
        {
            let cache_row_ref = cache_row.clone();
            let dir_ref = streams_dir.clone();
            clear_btn.connect_clicked(move |b| {
                if let Some(ref d) = dir_ref
                    && let Ok(entries) = std::fs::read_dir(d)
                {
                    for entry in entries.flatten() {
                        let _ = std::fs::remove_file(entry.path());
                    }
                }
                cache_row_ref.set_subtitle("0.0 MB en caché");
                b.set_sensitive(false);
            });
        }
        cache_row.add_suffix(&clear_btn);
        storage_group.add(&cache_row);
        page.add(&storage_group);

        dialog.add(&page);

        let root = btn.root().and_then(|r| r.downcast::<gtk4::Window>().ok());
        dialog.present(root.as_ref());
    });
}

pub fn wire_search_popover(
    state: Rc<RefCell<AppState>>,
    top_search_entry: &SearchEntry,
    content_view_stack: &adw::ViewStack,
    trigger_explore_search: Rc<dyn Fn(&str)>,
) -> (gtk4::EventControllerFocus, Popover, Rc<Cell<bool>>) {
    let search_popover = Popover::builder().autohide(false).can_focus(false).build();
    search_popover.set_parent(top_search_entry);
    let debounce_id: Rc<RefCell<Option<glib::SourceId>>> = Rc::new(RefCell::new(None));
    let suggest_gen: Rc<RefCell<u64>> = Rc::new(RefCell::new(0));
    let suppress_suggest: Rc<Cell<bool>> = Rc::new(Cell::new(false));
    let allow_search_focus: Rc<Cell<bool>> = Rc::new(Cell::new(false));

    // Only allow focus on the search bar when explicitly clicked (or Ctrl+F)
    {
        let click = gtk4::GestureClick::builder()
            .propagation_phase(gtk4::PropagationPhase::Capture)
            .button(0)
            .build();
        let entry_click = top_search_entry.clone();
        let allow_click = Rc::clone(&allow_search_focus);
        click.connect_pressed(move |_, _, _, _| {
            allow_click.set(true);
            entry_click.set_can_focus(true);
            entry_click.grab_focus();
        });
        top_search_entry.add_controller(click);
    }

    let run_header_search: Rc<dyn Fn(&str)> = {
        let content_view_stack = content_view_stack.clone();
        let top_entry_focus = top_search_entry.clone();
        let state = Rc::clone(&state);
        let search_popover = search_popover.clone();
        let debounce_id = Rc::clone(&debounce_id);
        let suggest_gen = Rc::clone(&suggest_gen);
        let allow_run = Rc::clone(&allow_search_focus);
        Rc::new(move |query: &str| {
            let trimmed = query.trim();
            if trimmed.is_empty() {
                return;
            }
            if let Some(id) = debounce_id.borrow_mut().take() {
                id.remove();
            }
            *suggest_gen.borrow_mut() += 1;
            {
                let mut s = state.borrow_mut();
                s.search_history.retain(|q| !q.eq_ignore_ascii_case(trimmed));
                s.search_history.insert(0, trimmed.to_string());
                s.search_history.truncate(8);
                save_search_history(&s.search_history);
            }
            allow_run.set(false);
            top_entry_focus.set_can_focus(false);
            search_popover.popdown();
            if let Some(root) = top_entry_focus.root() {
                root.set_focus(None::<&gtk4::Widget>);
            }
            content_view_stack.set_visible_child_name("explore");
            trigger_explore_search(trimmed);
        })
    };

    let active_items: Rc<RefCell<Vec<(Button, String)>>> = Rc::new(RefCell::new(Vec::new()));
    let selected_idx: Rc<Cell<Option<usize>>> = Rc::new(Cell::new(None));

    let show_search_popover = {
        let search_popover = search_popover.clone();
        let top_entry_ref = top_search_entry.clone();
        let run_header_search = Rc::clone(&run_header_search);
        let suppress_suggest = Rc::clone(&suppress_suggest);
        let active_items = Rc::clone(&active_items);
        let selected_idx = Rc::clone(&selected_idx);
        let state_pop = Rc::clone(&state);
        move |heading_text: &str, icon_name: &str, items: Vec<String>, is_history: bool| {
            active_items.borrow_mut().clear();
            selected_idx.set(None);
            if items.is_empty() {
                search_popover.popdown();
                return;
            }
            let box_hist = GtkBox::new(Orientation::Vertical, 4);
            box_hist.set_margin_top(6);
            box_hist.set_margin_bottom(6);
            box_hist.set_margin_start(6);
            box_hist.set_margin_end(6);
            box_hist.set_width_request(320);

            let header_row = GtkBox::new(Orientation::Horizontal, 6);
            header_row.append(
                &Label::builder()
                    .label(heading_text)
                    .xalign(0.0)
                    .hexpand(true)
                    .margin_start(6)
                    .css_classes(["dim-label", "caption"])
                    .build(),
            );
            if is_history {
                let clear_all_btn = Button::builder()
                    .label(tr("Limpiar", "Clear"))
                    .can_focus(false)
                    .focus_on_click(false)
                    .css_classes(["flat", "caption"])
                    .build();
                clear_all_btn.set_cursor_from_name(Some("pointer"));
                let st_clear = Rc::clone(&state_pop);
                let pop_clear = search_popover.clone();
                let items_clear = Rc::clone(&active_items);
                clear_all_btn.connect_clicked(move |_| {
                    {
                        let mut s = st_clear.borrow_mut();
                        s.search_history.clear();
                        save_search_history(&s.search_history);
                    }
                    items_clear.borrow_mut().clear();
                    pop_clear.popdown();
                });
                header_row.append(&clear_all_btn);
            }
            box_hist.append(&header_row);

            for term in items {
                let row = GtkBox::new(Orientation::Horizontal, 8);
                row.append(&Image::from_icon_name(icon_name));
                row.append(
                    &Label::builder()
                        .label(&term)
                        .xalign(0.0)
                        .hexpand(true)
                        .ellipsize(gtk4::pango::EllipsizeMode::End)
                        .build(),
                );
                let item = Button::builder()
                    .child(&row)
                    .hexpand(true)
                    .can_focus(false)
                    .focus_on_click(false)
                    .css_classes(["flat", "search-popover-row"])
                    .build();
                let top_entry = top_entry_ref.clone();
                let run_fn = Rc::clone(&run_header_search);
                let suppress = Rc::clone(&suppress_suggest);
                let term_clone = term.clone();
                item.connect_clicked(move |_| {
                    suppress.set(true);
                    top_entry.set_text(&term_clone);
                    suppress.set(false);
                    run_fn(&term_clone);
                });
                active_items.borrow_mut().push((item.clone(), term.clone()));

                if is_history {
                    let row_wrap = GtkBox::new(Orientation::Horizontal, 4);
                    row_wrap.append(&item);
                    let del_btn = Button::builder()
                        .icon_name("window-close-symbolic")
                        .tooltip_text(tr("Eliminar búsqueda", "Remove search"))
                        .can_focus(false)
                        .focus_on_click(false)
                        .valign(Align::Center)
                        .css_classes(["flat", "circular"])
                        .build();
                    del_btn.set_cursor_from_name(Some("pointer"));
                    let st_del = Rc::clone(&state_pop);
                    let box_ref = box_hist.clone();
                    let wrap_ref = row_wrap.clone();
                    let items_del = Rc::clone(&active_items);
                    let pop_del = search_popover.clone();
                    let term_del = term.clone();
                    del_btn.connect_clicked(move |_| {
                        {
                            let mut s = st_del.borrow_mut();
                            s.search_history.retain(|t| t != &term_del);
                            save_search_history(&s.search_history);
                        }
                        items_del.borrow_mut().retain(|(_, t)| t != &term_del);
                        box_ref.remove(&wrap_ref);
                        if items_del.borrow().is_empty() {
                            pop_del.popdown();
                        }
                    });
                    row_wrap.append(&del_btn);
                    box_hist.append(&row_wrap);
                } else {
                    box_hist.append(&item);
                }
            }
            search_popover.set_child(Some(&box_hist));
            if !search_popover.is_visible() {
                search_popover.popup();
            }
        }
    };

    let search_focus_ctrl = gtk4::EventControllerFocus::new();
    let schedule_suggestions = {
        let state = Rc::clone(&state);
        let show_search_popover = show_search_popover.clone();
        let search_popover = search_popover.clone();
        let debounce_id = Rc::clone(&debounce_id);
        let suggest_gen = Rc::clone(&suggest_gen);
        let focus_ctrl = search_focus_ctrl.clone();
        move |entry: &SearchEntry, delay_ms: u64| {
            if let Some(id) = debounce_id.borrow_mut().take() {
                id.remove();
            }
            let current_gen = {
                let mut g = suggest_gen.borrow_mut();
                *g += 1;
                *g
            };

            let text = entry.text().trim().to_string();
            if text.is_empty() {
                let history = state.borrow().search_history.clone();
                show_search_popover(
                    tr("Búsquedas recientes", "Recent searches"),
                    "document-open-recent-symbolic",
                    history,
                    true,
                );
                return;
            }
            if text.chars().count() < 2 {
                search_popover.popdown();
                return;
            }

            let state = Rc::clone(&state);
            let show_search_popover = show_search_popover.clone();
            let suggest_gen_ref = Rc::clone(&suggest_gen);
            let debounce_id_ref = Rc::clone(&debounce_id);
            let entry_clone = entry.clone();
            let focus_ctrl_ref = focus_ctrl.clone();

            let id = glib::timeout_add_local_once(
                std::time::Duration::from_millis(delay_ms),
                move || {
                    *debounce_id_ref.borrow_mut() = None;
                    if *suggest_gen_ref.borrow() != current_gen
                        || !(entry_clone.has_focus() || focus_ctrl_ref.contains_focus())
                    {
                        return;
                    }
                    let backend = state.borrow().backend.clone();
                    let query = text.clone();
                    let show_search_popover = show_search_popover.clone();
                    let suggest_gen_ref = Rc::clone(&suggest_gen_ref);
                    let entry_inner = entry_clone.clone();
                    let focus_ctrl_rx = focus_ctrl_ref.clone();
                    spawn_bg(
                        move || async move { backend.search_suggestions(&query).await },
                        move |res| {
                            if let Ok(terms) = res
                                && *suggest_gen_ref.borrow() == current_gen
                                && (entry_inner.has_focus() || focus_ctrl_rx.contains_focus())
                                && !entry_inner.text().trim().is_empty()
                            {
                                show_search_popover(
                                    tr("Sugerencias", "Suggestions"),
                                    "search-regular",
                                    terms,
                                    false,
                                );
                            }
                        },
                    );
                },
            );
            *debounce_id.borrow_mut() = Some(id);
        }
    };

    {
        let run_header_search = Rc::clone(&run_header_search);
        let active_items = Rc::clone(&active_items);
        let selected_idx = Rc::clone(&selected_idx);
        let search_popover_act = search_popover.clone();
        let suppress_act = Rc::clone(&suppress_suggest);
        top_search_entry.connect_activate(move |entry| {
            if search_popover_act.is_visible()
                && let Some(idx) = selected_idx.get()
                && let Some((_, term)) = active_items.borrow().get(idx).cloned()
            {
                suppress_act.set(true);
                entry.set_text(&term);
                suppress_act.set(false);
                run_header_search(&term);
                return;
            }
            let text = entry.text().to_string();
            run_header_search(&text);
        });
    }
    {
        let key_ctrl = gtk4::EventControllerKey::new();
        key_ctrl.set_propagation_phase(gtk4::PropagationPhase::Capture);
        let active_items = Rc::clone(&active_items);
        let selected_idx = Rc::clone(&selected_idx);
        let search_popover_key = search_popover.clone();
        let entry_tab = top_search_entry.clone();
        key_ctrl.connect_key_pressed(move |_, key, _, _| {
            if !search_popover_key.is_visible() {
                return glib::Propagation::Proceed;
            }
            let items = active_items.borrow();
            let len = items.len();
            if len == 0 {
                return glib::Propagation::Proceed;
            }
            if key == gtk4::gdk::Key::Tab {
                let idx = selected_idx.get().unwrap_or(0).min(len - 1);
                if let Some((_, term)) = items.get(idx) {
                    let completed = term.clone();
                    drop(items);
                    entry_tab.set_text(&completed);
                    entry_tab.set_position(-1);
                    selected_idx.set(None);
                    return glib::Propagation::Stop;
                }
                return glib::Propagation::Proceed;
            }
            let next_idx = match key {
                gtk4::gdk::Key::Down => Some(match selected_idx.get() {
                    None => 0,
                    Some(i) => (i + 1) % len,
                }),
                gtk4::gdk::Key::Up => Some(match selected_idx.get() {
                    None | Some(0) => len - 1,
                    Some(i) => i - 1,
                }),
                _ => return glib::Propagation::Proceed,
            };
            for (i, (btn, _)) in items.iter().enumerate() {
                if Some(i) == next_idx {
                    btn.add_css_class("search-row-selected");
                } else {
                    btn.remove_css_class("search-row-selected");
                }
            }
            selected_idx.set(next_idx);
            glib::Propagation::Stop
        });
        top_search_entry.add_controller(key_ctrl);
    }
    {
        let search_popover = search_popover.clone();
        let debounce_id = Rc::clone(&debounce_id);
        let suggest_gen = Rc::clone(&suggest_gen);
        let allow_stop = Rc::clone(&allow_search_focus);
        top_search_entry.connect_stop_search(move |entry| {
            if let Some(id) = debounce_id.borrow_mut().take() {
                id.remove();
            }
            *suggest_gen.borrow_mut() += 1;
            allow_stop.set(false);
            entry.set_can_focus(false);
            search_popover.popdown();
            if let Some(root) = entry.root() {
                root.set_focus(None::<&gtk4::Widget>);
            }
        });
    }
    {
        let schedule_enter = schedule_suggestions.clone();
        let top_entry_enter = top_search_entry.clone();
        search_focus_ctrl.connect_enter(move |_| {
            schedule_enter(&top_entry_enter, 80);
        });
        let popover_leave = search_popover.clone();
        let top_entry_leave = top_search_entry.clone();
        let focus_ctrl_leave = search_focus_ctrl.clone();
        let allow_leave = Rc::clone(&allow_search_focus);
        search_focus_ctrl.connect_leave(move |_| {
            let pop = popover_leave.clone();
            let entry = top_entry_leave.clone();
            let fc = focus_ctrl_leave.clone();
            let allow = Rc::clone(&allow_leave);
            glib::timeout_add_local_once(std::time::Duration::from_millis(120), move || {
                if !(entry.has_focus() || fc.contains_focus()) {
                    allow.set(false);
                    entry.set_can_focus(false);
                    pop.popdown();
                }
            });
        });
        top_search_entry.add_controller(search_focus_ctrl.clone());
    }
    {
        let suppress_suggest = Rc::clone(&suppress_suggest);
        let focus_ctrl = search_focus_ctrl.clone();
        let schedule_changed = schedule_suggestions.clone();
        top_search_entry.connect_search_changed(move |entry| {
            if suppress_suggest.get() || !(entry.has_focus() || focus_ctrl.contains_focus()) {
                return;
            }
            schedule_changed(entry, 320);
        });
    }

    (search_focus_ctrl, search_popover, allow_search_focus)
}

pub fn install_unfocus_on_click(
    window: &adw::ApplicationWindow,
    search_entry: &SearchEntry,
    focus_ctrl: &gtk4::EventControllerFocus,
    search_popover: &Popover,
    allow_search_focus: &Rc<Cell<bool>>,
) {
    // Reject any automatic GTK focus (startup, resize, Tab fallback) unless explicitly allowed
    {
        let entry_guard = search_entry.clone();
        let allow_guard = Rc::clone(allow_search_focus);
        window.connect_focus_widget_notify(move |win| {
            if allow_guard.get() {
                return;
            }
            let focused_search = GtkWindowExt::focus(win).is_some_and(|w| {
                &w == entry_guard.upcast_ref::<gtk4::Widget>() || w.is_ancestor(&entry_guard)
            });
            if focused_search {
                entry_guard.set_can_focus(false);
                GtkWindowExt::set_focus(win, None::<&gtk4::Widget>);
            }
        });
    }

    let click = gtk4::GestureClick::builder()
        .propagation_phase(gtk4::PropagationPhase::Capture)
        .button(0)
        .build();
    let win_ref = window.clone();
    let entry_ref = search_entry.clone();
    let fc_ref = focus_ctrl.clone();
    let pop_ref = search_popover.clone();
    let allow_ref = Rc::clone(allow_search_focus);
    click.connect_pressed(move |_, _, x, y| {
        if !(entry_ref.has_focus() || fc_ref.contains_focus()) {
            return;
        }
        let clicked_inside_search = win_ref
            .pick(x, y, gtk4::PickFlags::DEFAULT)
            .is_some_and(|w| {
                &w == entry_ref.upcast_ref::<gtk4::Widget>()
                    || w.is_ancestor(&entry_ref)
                    || &w == pop_ref.upcast_ref::<gtk4::Widget>()
                    || w.is_ancestor(&pop_ref)
            });
        if !clicked_inside_search {
            allow_ref.set(false);
            entry_ref.set_can_focus(false);
            pop_ref.popdown();
            GtkWindowExt::set_focus(&win_ref, None::<&gtk4::Widget>);
        }
    });
    window.add_controller(click);
}
