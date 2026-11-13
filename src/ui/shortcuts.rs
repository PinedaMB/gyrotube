use std::cell::Cell;
use std::rc::Rc;

use gtk4::gdk;
use gtk4::glib;
use gtk4::prelude::*;
use gtk4::{Button, SearchEntry, ToggleButton};
use libadwaita as adw;

pub struct ShortcutTargets {
    pub window: adw::ApplicationWindow,
    pub search_entry: SearchEntry,
    pub allow_search_focus: Rc<Cell<bool>>,
    pub tab_player: ToggleButton,
    pub tab_lyrics: ToggleButton,
    pub play_pause_btn: Button,
    pub prev_btn: Button,
    pub next_btn: Button,
    pub content_stack: adw::ViewStack,
}

pub fn install_shortcuts(targets: ShortcutTargets) {
    let key_ctrl = gtk4::EventControllerKey::new();
    let win_ref = targets.window.clone();
    key_ctrl.connect_key_pressed(move |_, key, _, mods| {
        let ctrl = mods.contains(gdk::ModifierType::CONTROL_MASK);
        let alt = mods.contains(gdk::ModifierType::ALT_MASK);
        if alt && key == gdk::Key::_1 {
            targets.content_stack.set_visible_child_name("home");
            return glib::Propagation::Stop;
        }
        if alt && key == gdk::Key::_2 {
            targets.content_stack.set_visible_child_name("explore");
            return glib::Propagation::Stop;
        }
        if alt && key == gdk::Key::_3 {
            targets.content_stack.set_visible_child_name("collection");
            return glib::Propagation::Stop;
        }
        if ctrl && (key == gdk::Key::f || key == gdk::Key::F) {
            targets.allow_search_focus.set(true);
            targets.search_entry.set_can_focus(true);
            targets.search_entry.grab_focus();
            return glib::Propagation::Stop;
        }
        if ctrl && (key == gdk::Key::l || key == gdk::Key::L) {
            if targets.tab_lyrics.is_active() {
                targets.tab_player.set_active(true);
            } else {
                targets.tab_lyrics.set_active(true);
            }
            return glib::Propagation::Stop;
        }
        if ctrl && key == gdk::Key::Right {
            targets.next_btn.emit_clicked();
            return glib::Propagation::Stop;
        }
        if ctrl && key == gdk::Key::Left {
            targets.prev_btn.emit_clicked();
            return glib::Propagation::Stop;
        }
        if key == gdk::Key::space {
            let in_text = gtk4::prelude::GtkWindowExt::focus(&win_ref)
                .is_some_and(|w| w.is::<gtk4::Text>() || w.is::<gtk4::Entry>() || w.is::<SearchEntry>());
            if !in_text {
                targets.play_pause_btn.emit_clicked();
                return glib::Propagation::Stop;
            }
        }
        glib::Propagation::Proceed
    });
    targets.window.add_controller(key_ctrl);
}
