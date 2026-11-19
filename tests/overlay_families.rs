use astryx_gpui::{
    BottomSheet, ContextMenu, Drawer, DropdownMenu, HoverCard, InfoTip, MenuItem, Toast,
};
use gpui::{
    App, Bounds, Context, Entity, FocusHandle, InteractiveElement as _, IntoElement, KeyDownEvent,
    KeyUpEvent, Keystroke, Modifiers, MouseButton, MouseDownEvent, MouseUpEvent, ParentElement,
    Pixels, Render, Styled, TestAppContext, VisualTestContext, WeakEntity, Window, canvas, div,
    point, prelude::FluentBuilder as _, px,
};
use std::{cell::Cell, rc::Rc};

#[derive(Clone, Copy)]
enum Kind {
    Drawer,
    BottomSheet,
    Menu,
    ContextMenu,
    InfoTip,
    Toast,
    HoverCard,
}
struct Harness {
    kind: Kind,
    open: bool,
    disabled: bool,
    highlighted: usize,
    values: Vec<String>,
    trigger: FocusHandle,
    body: Rc<Cell<Bounds<Pixels>>>,
}

fn opened(owner: &WeakEntity<Harness>, open: bool, cx: &mut App) {
    owner
        .update(cx, |this, cx| {
            this.open = open;
            cx.notify();
        })
        .unwrap();
}

impl Render for Harness {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let owner = cx.entity().downgrade();
        let body = self.body.clone();
        let measured = canvas(move |bounds, _, _| body.set(bounds), |_, _, _, _| {})
            .w_full()
            .h(px(80.));
        let items = vec![
            MenuItem::new("a", "Alpha"),
            MenuItem::new("b", "Beta").disabled(true),
            MenuItem::new("c", "Gamma"),
        ];
        let control = match self.kind {
            Kind::Drawer => Drawer::new("drawer")
                .open(self.open)
                .title("Inspector")
                .on_open_change(move |open, _, _, cx| opened(&owner, open, cx))
                .child(measured)
                .into_any_element(),
            Kind::BottomSheet => BottomSheet::new("sheet")
                .open(self.open)
                .title("Details")
                .on_open_change(move |open, _, _, cx| opened(&owner, open, cx))
                .child(measured)
                .into_any_element(),
            Kind::Menu => {
                let selected = owner.clone();
                let highlight = owner.clone();
                DropdownMenu::new("actions")
                    .open(self.open)
                    .disabled(self.disabled)
                    .highlighted(self.highlighted)
                    .items(items)
                    .trigger(
                        gpui_base::Button::new("menu-trigger")
                            .track_focus(&self.trigger)
                            .w(px(120.))
                            .h(px(30.))
                            .child("Options"),
                    )
                    .on_open_change(move |open, _, cx| opened(&owner, open, cx))
                    .on_highlight(move |index, _, cx| {
                        highlight
                            .update(cx, |this, cx| {
                                this.highlighted = index;
                                cx.notify();
                            })
                            .unwrap();
                    })
                    .on_select(move |value, _, cx| {
                        selected
                            .update(cx, |this, cx| {
                                this.values.push(value.to_string());
                                cx.notify();
                            })
                            .unwrap();
                    })
                    .into_any_element()
            }
            Kind::ContextMenu => ContextMenu::new("context")
                .open(self.open)
                .items(items)
                .trigger(
                    gpui_base::Button::new("target")
                        .track_focus(&self.trigger)
                        .w(px(120.))
                        .h(px(30.))
                        .child("Target"),
                )
                .on_open_change(move |open, _, cx| opened(&owner, open, cx))
                .into_any_element(),
            Kind::InfoTip => InfoTip::new("info", "More information")
                .open(self.open)
                .on_open_change(move |open, _, cx| opened(&owner, open, cx))
                .into_any_element(),
            Kind::Toast => div()
                .when(self.open, |el| {
                    el.child(
                        Toast::new("toast", "Saved")
                            .description("Changes persisted")
                            .on_dismiss(move |_, cx| opened(&owner, false, cx)),
                    )
                })
                .into_any_element(),
            Kind::HoverCard => HoverCard::new("preview")
                .open_delay(std::time::Duration::from_millis(10))
                .close_delay(std::time::Duration::from_millis(10))
                .trigger(
                    gpui_base::Button::new("preview-trigger")
                        .w(px(120.))
                        .h(px(30.))
                        .child("Preview"),
                )
                .on_open_change(move |open, _, cx| opened(&owner, open, cx))
                .content(move |_, _, _| measured)
                .into_any_element(),
        };
        div()
            .id("family-owner")
            .size_full()
            .when(
                matches!(self.kind, Kind::Drawer | Kind::BottomSheet),
                |el| {
                    el.child(
                        gpui_base::Button::new("show-modal")
                            .track_focus(&self.trigger)
                            .w(px(120.))
                            .h(px(30.))
                            .child("Open")
                            .on_click(cx.listener(|this, _, _, cx| {
                                this.open = true;
                                cx.notify();
                            })),
                    )
                },
            )
            .child(control)
    }
}
fn mount(cx: &mut TestAppContext, kind: Kind) -> (Entity<Harness>, &mut VisualTestContext) {
    cx.update(astryx_gpui::init);
    let (view, cx) = cx.add_window_view(move |_, cx| Harness {
        kind,
        open: matches!(kind, Kind::Toast),
        disabled: false,
        highlighted: 0,
        values: vec![],
        trigger: cx.focus_handle(),
        body: Rc::new(Cell::new(Bounds::default())),
    });
    draw(cx);
    (view, cx)
}
fn draw(cx: &mut VisualTestContext) {
    cx.update(|w, cx| w.draw(cx).clear(cx));
}
fn key(cx: &mut VisualTestContext, name: &str) {
    let keystroke = Keystroke::parse(name).unwrap();
    cx.simulate_event(KeyDownEvent {
        keystroke: keystroke.clone(),
        is_held: false,
        prefer_character_input: false,
    });
    cx.simulate_event(KeyUpEvent { keystroke });
    draw(cx);
}

#[gpui::test]
fn modal_inspector_is_at_the_right_edge_and_returns_focus(cx: &mut TestAppContext) {
    let (view, cx) = mount(cx, Kind::Drawer);
    assert_eq!(view.read_with(cx, |v, _| v.body.get().size.height), px(0.));
    cx.simulate_click(point(px(10.), px(10.)), Modifiers::default());
    draw(cx);
    let bounds = view.read_with(cx, |v, _| v.body.get());
    let viewport = cx.update(|window, _| window.viewport_size());
    assert!(bounds.left() > viewport.width / 2.);
    assert!(bounds.right() > viewport.width - px(24.));
    key(cx, "escape");
    assert!(!view.read_with(cx, |v, _| v.open));
    assert!(cx.update(|window, cx| view.read(cx).trigger.is_focused(window)));
}

#[gpui::test]
fn bottom_sheet_is_bottom_anchored_and_escape_closes(cx: &mut TestAppContext) {
    let (view, cx) = mount(cx, Kind::BottomSheet);
    cx.simulate_click(point(px(10.), px(10.)), Modifiers::default());
    draw(cx);
    let bounds = view.read_with(cx, |v, _| v.body.get());
    let viewport = cx.update(|window, _| window.viewport_size());
    assert!(bounds.bottom() > viewport.height - px(24.));
    assert!(bounds.size.width > viewport.width - px(40.));
    key(cx, "escape");
    assert!(!view.read_with(cx, |v, _| v.open));
}

#[gpui::test]
fn menu_keyboard_skips_disabled_and_confirms_once(cx: &mut TestAppContext) {
    let (view, cx) = mount(cx, Kind::Menu);
    cx.simulate_click(point(px(10.), px(10.)), Modifiers::default());
    draw(cx);
    assert!(view.read_with(cx, |v, _| v.open));
    key(cx, "down");
    assert_eq!(view.read_with(cx, |v, _| v.highlighted), 2);
    key(cx, "enter");
    assert!(!view.read_with(cx, |v, _| v.open));
    assert_eq!(view.read_with(cx, |v, _| v.values.clone()), vec!["c"]);
    key(cx, "enter");
    assert!(view.read_with(cx, |v, _| v.open));
    key(cx, "escape");
    assert!(!view.read_with(cx, |v, _| v.open));
}

#[gpui::test]
fn disabled_menu_does_not_request_open(cx: &mut TestAppContext) {
    let (view, cx) = mount(cx, Kind::Menu);
    view.update(cx, |v, cx| {
        v.disabled = true;
        cx.notify();
    });
    draw(cx);
    cx.simulate_click(point(px(10.), px(10.)), Modifiers::default());
    key(cx, "enter");
    assert!(!view.read_with(cx, |v, _| v.open));
    assert!(view.read_with(cx, |v, _| v.values.is_empty()));
}

#[gpui::test]
fn disabling_an_open_menu_restores_focus_and_blocks_reopening(cx: &mut TestAppContext) {
    let (view, cx) = mount(cx, Kind::Menu);
    cx.simulate_click(point(px(10.), px(10.)), Modifiers::default());
    draw(cx);
    assert!(view.read_with(cx, |v, _| v.open));
    assert!(!cx.update(|window, cx| view.read(cx).trigger.is_focused(window)));
    view.update(cx, |v, cx| {
        v.disabled = true;
        cx.notify();
    });
    draw(cx);
    // Disabling presentation does not mutate the owner's controlled open value.
    assert!(view.read_with(cx, |v, _| v.open));
    assert!(cx.update(|window, cx| view.read(cx).trigger.is_focused(window)));
    assert!(!cx.update(|_, cx| gpui_base::GlobalState::is_in_deferred_context(cx)));
    key(cx, "enter");
    cx.simulate_click(point(px(10.), px(10.)), Modifiers::default());
    draw(cx);
    assert!(view.read_with(cx, |v, _| v.values.is_empty()));
    assert!(!cx.update(|_, cx| gpui_base::GlobalState::is_in_deferred_context(cx)));
    view.update(cx, |v, cx| {
        v.disabled = false;
        v.open = false;
        cx.notify();
    });
    draw(cx);
    key(cx, "enter");
    assert!(view.read_with(cx, |v, _| v.open));
}

#[gpui::test]
fn context_menu_uses_right_click_and_keyboard_context_gesture(cx: &mut TestAppContext) {
    let (view, cx) = mount(cx, Kind::ContextMenu);
    cx.simulate_click(point(px(10.), px(10.)), Modifiers::default());
    draw(cx);
    assert!(!view.read_with(cx, |v, _| v.open));
    cx.simulate_event(MouseDownEvent {
        button: MouseButton::Right,
        position: point(px(10.), px(10.)),
        modifiers: Modifiers::default(),
        click_count: 1,
        first_mouse: false,
    });
    cx.simulate_event(MouseUpEvent {
        button: MouseButton::Right,
        position: point(px(10.), px(10.)),
        modifiers: Modifiers::default(),
        click_count: 1,
    });
    draw(cx);
    assert!(view.read_with(cx, |v, _| v.open));
    key(cx, "escape");
    cx.update(|window, cx| {
        let trigger = view.read(cx).trigger.clone();
        trigger.focus(window, cx);
    });
    key(cx, "shift-f10");
    assert!(view.read_with(cx, |v, _| v.open));
}

#[gpui::test]
fn info_tip_opens_and_escape_dismisses(cx: &mut TestAppContext) {
    let (view, cx) = mount(cx, Kind::InfoTip);
    cx.simulate_click(point(px(10.), px(10.)), Modifiers::default());
    draw(cx);
    assert!(view.read_with(cx, |v, _| v.open));
    key(cx, "escape");
    assert!(!view.read_with(cx, |v, _| v.open));
    key(cx, "enter");
    assert!(view.read_with(cx, |v, _| v.open));
}

#[gpui::test]
fn toast_dismissal_is_explicit_and_owner_controlled(cx: &mut TestAppContext) {
    let (view, cx) = mount(cx, Kind::Toast);
    assert!(view.read_with(cx, |v, _| v.open));
    cx.simulate_click(point(px(336.), px(24.)), Modifiers::default());
    draw(cx);
    assert!(!view.read_with(cx, |v, _| v.open));
}

#[gpui::test]
fn hover_preview_waits_for_delay_and_closes_after_leaving(cx: &mut TestAppContext) {
    let (view, cx) = mount(cx, Kind::HoverCard);
    assert!(!view.read_with(cx, |v, _| v.open));
    cx.simulate_mouse_move(point(px(10.), px(10.)), None, Modifiers::default());
    assert!(!view.read_with(cx, |v, _| v.open));
    cx.executor()
        .advance_clock(std::time::Duration::from_millis(11));
    cx.run_until_parked();
    draw(cx);
    draw(cx);
    assert!(view.read_with(cx, |v, _| v.open));
    cx.simulate_mouse_move(point(px(600.), px(600.)), None, Modifiers::default());
    cx.executor()
        .advance_clock(std::time::Duration::from_millis(11));
    cx.run_until_parked();
    draw(cx);
    assert!(!view.read_with(cx, |v, _| v.open));
}
