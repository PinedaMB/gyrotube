use std::cell::{Cell, RefCell};
use std::rc::Rc;

use gtk4::prelude::*;
use gtk4::{Align, Box as GtkBox, Button, FlowBox, Label, SearchEntry};
use libadwaita as adw;
use adw::prelude::AdwApplicationWindowExt;

use crate::ui::state::AppState;

pub struct ResponsiveTargets {
    pub window: adw::ApplicationWindow,
    pub split_view: adw::OverlaySplitView,
    pub sidebar_toolbar: adw::ToolbarView,
    pub bottom_drawer_revealer: gtk4::Revealer,
    pub mini_player_bar: GtkBox,
    pub mobile_mini_bar: GtkBox,
    pub mini_open_btn: Button,
    pub mobile_open_btn: Button,
    pub sidebar_collapse_btn: Button,
    pub brand_title: Label,
    pub brand_badge: Label,
    pub top_search_entry: SearchEntry,
    pub search_focus_ctrl: gtk4::EventControllerFocus,
    pub quick_grid: FlowBox,
    pub state: Rc<RefCell<AppState>>,
}

pub fn build_main_overlay(
    split_view: &adw::OverlaySplitView,
    mini_player_bar: &GtkBox,
) -> (gtk4::Overlay, gtk4::Revealer) {
    let bottom_drawer_revealer = gtk4::Revealer::builder()
        .transition_type(gtk4::RevealerTransitionType::SlideUp)
        .transition_duration(240)
        .reveal_child(false)
        .visible(false)
        .valign(Align::Fill)
        .halign(Align::Fill)
        .vexpand(true)
        .hexpand(true)
        .build();

    let main_overlay = gtk4::Overlay::builder()
        .child(split_view)
        .vexpand(true)
        .hexpand(true)
        .build();
    main_overlay.add_overlay(mini_player_bar);
    main_overlay.add_overlay(&bottom_drawer_revealer);

    (main_overlay, bottom_drawer_revealer)
}

pub fn setup_responsive(t: ResponsiveTargets) {
    let is_mobile = Rc::new(Cell::new(false));

    let update_mini_bar = {
        let mini_bar = t.mini_player_bar.clone();
        let mobile_bar = t.mobile_mini_bar.clone();
        let sv_ref = t.split_view.clone();
        let rev_ref = t.bottom_drawer_revealer.clone();
        let is_mobile_ref = Rc::clone(&is_mobile);
        let state_ref = Rc::clone(&t.state);
        move || {
            let mobile = is_mobile_ref.get();
            let player_visible = if mobile {
                rev_ref.reveals_child()
            } else {
                sv_ref.shows_sidebar()
            };
            let has_track = state_ref
                .try_borrow()
                .map(|s| s.current_index.is_some())
                .unwrap_or(true);
            let show_mini = !player_visible && has_track;
            mini_bar.set_visible(show_mini && !mobile);
            mobile_bar.set_visible(show_mini && mobile);
        }
    };

    let sync_player_container = {
        let sv_ref = t.split_view.clone();
        let rev_ref = t.bottom_drawer_revealer.clone();
        let sidebar_tb = t.sidebar_toolbar.clone();
        let collapse_btn_ref = t.sidebar_collapse_btn.clone();
        let state_ref = Rc::clone(&t.state);
        let is_mobile_ref = Rc::clone(&is_mobile);
        let top_search_ref = t.top_search_entry.clone();
        let focus_ctrl_ref = t.search_focus_ctrl.clone();
        let update_bar = update_mini_bar.clone();
        move || {
            let search_focused =
                top_search_ref.has_focus() || focus_ctrl_ref.contains_focus();
            if !search_focused && let Some(root) = sv_ref.root() {
                root.set_focus(None::<&gtk4::Widget>);
            }

            if is_mobile_ref.get() {
                // Celular: mover reproductor al drawer inferior
                sv_ref.set_show_sidebar(false);
                if sv_ref.sidebar().is_some() {
                    sv_ref.set_sidebar(None::<&gtk4::Widget>);
                }
                if rev_ref.child().is_none() {
                    rev_ref.set_child(Some(&sidebar_tb));
                }
                rev_ref.set_reveal_child(false);
                rev_ref.set_visible(false);
                collapse_btn_ref.set_icon_name("go-down-symbolic");
            } else {
                // Tablet (sidebar modal) o Escritorio (sidebar fijo)
                rev_ref.set_reveal_child(false);
                rev_ref.set_visible(false);
                if rev_ref.child().is_some() {
                    rev_ref.set_child(None::<&gtk4::Widget>);
                }
                collapse_btn_ref.set_icon_name("go-previous-symbolic");
                if sv_ref.sidebar().is_none() {
                    sv_ref.set_sidebar(Some(&sidebar_tb));
                }
                if sv_ref.is_collapsed() {
                    sv_ref.set_show_sidebar(false);
                } else {
                    let show = !state_ref.borrow().sidebar_collapsed;
                    sv_ref.set_show_sidebar(show);
                }
            }

            if !search_focused && let Some(root) = sv_ref.root() {
                root.set_focus(None::<&gtk4::Widget>);
            }
            update_bar();
        }
    };

    {
        let update_rev = update_mini_bar.clone();
        t.bottom_drawer_revealer
            .connect_reveal_child_notify(move |_| update_rev());
        t.bottom_drawer_revealer.connect_child_revealed_notify(|r| {
            if !r.is_child_revealed() {
                r.set_visible(false);
            }
        });

        let update_sv_show = update_mini_bar.clone();
        t.split_view
            .connect_show_sidebar_notify(move |_| update_sv_show());

        let open_player = {
            let sv_for_mini = t.split_view.clone();
            let rev_for_mini = t.bottom_drawer_revealer.clone();
            let state_for_mini = Rc::clone(&t.state);
            let is_mobile_open = Rc::clone(&is_mobile);
            move || {
                if is_mobile_open.get() {
                    rev_for_mini.set_visible(true);
                    rev_for_mini.set_reveal_child(true);
                } else {
                    sv_for_mini.set_show_sidebar(true);
                    if !sv_for_mini.is_collapsed() {
                        let mut s = state_for_mini.borrow_mut();
                        s.sidebar_collapsed = false;
                        s.save_session();
                    }
                }
            }
        };
        {
            let open_fn = open_player.clone();
            t.mini_open_btn.connect_clicked(move |_| open_fn());
        }
        t.mobile_open_btn.connect_clicked(move |_| open_player());

        let sv_for_collapse = t.split_view.clone();
        let rev_for_collapse_btn = t.bottom_drawer_revealer.clone();
        let state_for_collapse = Rc::clone(&t.state);
        let is_mobile_collapse = Rc::clone(&is_mobile);
        t.sidebar_collapse_btn.connect_clicked(move |btn| {
            if let Some(root) = btn.root() {
                root.set_focus(None::<&gtk4::Widget>);
            }
            if is_mobile_collapse.get() {
                rev_for_collapse_btn.set_reveal_child(false);
            } else {
                sv_for_collapse.set_show_sidebar(false);
                if !sv_for_collapse.is_collapsed() {
                    let mut s = state_for_collapse.borrow_mut();
                    s.sidebar_collapsed = true;
                    s.save_session();
                }
            }
        });

        let sync_on_collapse = sync_player_container.clone();
        t.split_view.connect_collapsed_notify(move |_| {
            sync_on_collapse();
        });

        let update_on_track = update_mini_bar.clone();
        t.state
            .borrow_mut()
            .on_track_changed
            .push(Rc::new(move || update_on_track()));

        update_mini_bar();
    }

    // =========================================================================
    // REGLA 1: MODO TABLET / VENTANA MEDIANA (<= 850sp)
    // Sidebar sobrepuesto (modal) + mini reproductor flotante
    // =========================================================================
    let tablet_condition = adw::BreakpointCondition::new_length(
        adw::BreakpointConditionLengthType::MaxWidth,
        850.0,
        adw::LengthUnit::Sp,
    );
    let tablet_bp = adw::Breakpoint::new(tablet_condition);
    tablet_bp.add_setter(&t.split_view, "collapsed", Some(&true.to_value()));
    tablet_bp.add_setter(&t.split_view, "show-sidebar", Some(&false.to_value()));
    tablet_bp.add_setter(&t.quick_grid, "min-children-per-line", Some(&2u32.to_value()));
    tablet_bp.add_setter(&t.quick_grid, "max-children-per-line", Some(&2u32.to_value()));

    // =========================================================================
    // REGLA 2: MODO MÓVIL / VENTANA ESTRECHA (<= 500sp)
    // Barra inferior + reproductor estilo drawer desde abajo
    // =========================================================================
    let mobile_condition = adw::BreakpointCondition::new_length(
        adw::BreakpointConditionLengthType::MaxWidth,
        500.0,
        adw::LengthUnit::Sp,
    );
    let mobile_bp = adw::Breakpoint::new(mobile_condition);
    mobile_bp.add_setter(&t.brand_title, "visible", Some(&false.to_value()));
    mobile_bp.add_setter(&t.brand_badge, "visible", Some(&false.to_value()));
    mobile_bp.add_setter(
        &t.top_search_entry,
        "placeholder-text",
        Some(&"Buscar…".to_value()),
    );
    {
        let is_mobile_apply = Rc::clone(&is_mobile);
        let sync_apply = sync_player_container.clone();
        let win_apply = t.window.clone();
        mobile_bp.connect_apply(move |_| {
            win_apply.add_css_class("mobile-layout");
            is_mobile_apply.set(true);
            sync_apply();
        });
        let is_mobile_unapply = Rc::clone(&is_mobile);
        let sync_unapply = sync_player_container;
        let win_unapply = t.window.clone();
        mobile_bp.connect_unapply(move |_| {
            win_unapply.remove_css_class("mobile-layout");
            is_mobile_unapply.set(false);
            sync_unapply();
        });
    }

    // =========================================================================
    // REGISTRO EN ORDEN DE PRIORIDAD (De mayor a menor umbral)
    // =========================================================================
    t.window.add_breakpoint(tablet_bp);
    t.window.add_breakpoint(mobile_bp);
}
