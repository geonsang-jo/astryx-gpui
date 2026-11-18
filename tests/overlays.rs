use std::{cell::Cell, rc::Rc};

use astryx_gpui::{Dialog, Popover};
use gpui::{
    Context, Entity, FocusHandle, InteractiveElement, IntoElement, KeyDownEvent, KeyUpEvent,
    Keystroke, Modifiers, ParentElement, Render, Styled, TestAppContext, VisualTestContext, Window,
    div, point, px,
};
use gpui_base::ElementExt as _;

#[derive(Clone, Copy)]
enum Kind {
    Dialog,
    Popover,
}

struct Harness {
    kind: Kind,
    open: bool,
    veto: bool,
    trigger: FocusHandle,
    inner: FocusHandle,
    inner_clicks: usize,
    confirms: usize,
    confirm_enabled: bool,
    requests: Vec<bool>,
    layouts: Rc<Cell<usize>>,
}

impl Render for Harness {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let layouts = self.layouts.clone();
        let content = div()
            .w(px(120.))
            .h(px(50.))
            .child("Overlay body")
            .on_prepaint(move |_, _, _| layouts.set(layouts.get() + 1));
        let content = content.child(
            gpui_base::Button::new("inner")
                .track_focus(&self.inner)
                .w(px(80.))
                .h(px(24.))
                .child("Inner action")
                .on_click(cx.listener(|this, _, _, cx| {
                    this.inner_clicks += 1;
                    cx.notify();
                })),
        );
        let owner = cx.entity().downgrade();
        let veto_owner = owner.clone();
        let confirm_owner = owner.clone();
        let overlay = match self.kind {
            Kind::Dialog => Dialog::new("modal")
                .open(self.open)
                .title("Review")
                .on_cancel(move |_, _, cx| {
                    veto_owner.update(cx, |this, _| !this.veto).unwrap_or(false)
                })
                .on_confirm(move |_, _, cx| {
                    confirm_owner
                        .update(cx, |this, _| {
                            if this.confirm_enabled {
                                this.confirms += 1;
                                true
                            } else {
                                false
                            }
                        })
                        .unwrap_or(false)
                })
                .on_open_change(move |open, _, _, cx| {
                    owner
                        .update(cx, |this, cx| {
                            this.open = open;
                            this.requests.push(open);
                            cx.notify();
                        })
                        .unwrap();
                })
                .child(content)
                .into_any_element(),
            Kind::Popover => Popover::new("popover")
                .open(self.open)
                .trigger(
                    gpui_base::Button::new("popover-trigger")
                        .track_focus(&self.trigger)
                        .w(px(120.))
                        .h(px(30.))
                        .child("Open popover"),
                )
                .on_open_change(move |open, _, cx| {
                    owner
                        .update(cx, |this, cx| {
                            this.open = open;
                            this.requests.push(open);
                            cx.notify();
                        })
                        .unwrap();
                })
                .content(move |_, _, _| content)
                .into_any_element(),
        };
        div()
            .id("overlay-owner")
            .size_full()
            .when(matches!(self.kind, Kind::Dialog), |root| {
                root.child(
                    gpui_base::Button::new("modal-trigger")
                        .track_focus(&self.trigger)
                        .w(px(120.))
                        .h(px(30.))
                        .child("Open dialog")
                        .on_click(cx.listener(|this, _, _, cx| {
                            this.open = true;
                            cx.notify();
                        })),
                )
            })
            .child(overlay)
    }
}

use gpui::prelude::FluentBuilder as _;

fn mount(cx: &mut TestAppContext, kind: Kind) -> (Entity<Harness>, &mut VisualTestContext) {
    cx.update(astryx_gpui::init);
    let (view, cx) = cx.add_window_view(move |_, cx| Harness {
        kind,
        open: false,
        veto: false,
        trigger: cx.focus_handle(),
        inner: cx.focus_handle(),
        inner_clicks: 0,
        confirms: 0,
        confirm_enabled: false,
        requests: vec![],
        layouts: Rc::new(Cell::new(0)),
    });
    draw(cx);
    (view, cx)
}

fn draw(cx: &mut VisualTestContext) {
    cx.update(|window, cx| window.draw(cx).clear(cx));
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
fn dialog_does_not_layout_hidden_content_and_escape_restores_trigger(cx: &mut TestAppContext) {
    let (view, cx) = mount(cx, Kind::Dialog);
    assert_eq!(view.read_with(cx, |v, _| v.layouts.get()), 0);
    cx.simulate_click(point(px(10.), px(10.)), Modifiers::default());
    draw(cx);
    assert!(view.read_with(cx, |v, _| v.open));
    assert!(view.read_with(cx, |v, _| v.layouts.get()) > 0);
    key(cx, "enter");
    assert!(
        view.read_with(cx, |v, _| v.open),
        "Enter does not confirm a dialog without an explicit handler"
    );
    key(cx, "escape");
    assert!(!view.read_with(cx, |v, _| v.open));
    assert_eq!(view.read_with(cx, |v, _| v.requests.clone()), vec![false]);
    assert!(cx.update(|window, cx| view.read(cx).trigger.is_focused(window)));
}

#[gpui::test]
fn dialog_owner_can_veto_backdrop_and_escape_dismissal(cx: &mut TestAppContext) {
    let (view, cx) = mount(cx, Kind::Dialog);
    view.update(cx, |v, cx| {
        v.veto = true;
        cx.notify();
    });
    cx.simulate_click(point(px(10.), px(10.)), Modifiers::default());
    draw(cx);
    cx.simulate_click(point(px(3.), px(70.)), Modifiers::default());
    draw(cx);
    assert!(view.read_with(cx, |v, _| v.open));
    key(cx, "escape");
    assert!(view.read_with(cx, |v, _| v.open));
    assert!(view.read_with(cx, |v, _| v.requests.is_empty()));
    view.update(cx, |v, cx| {
        v.veto = false;
        cx.notify();
    });
    key(cx, "escape");
    assert!(!view.read_with(cx, |v, _| v.open));
}

#[gpui::test]
fn popover_pointer_and_escape_are_controlled_and_return_focus(cx: &mut TestAppContext) {
    let (view, cx) = mount(cx, Kind::Popover);
    assert_eq!(view.read_with(cx, |v, _| v.layouts.get()), 0);
    cx.simulate_click(point(px(10.), px(10.)), Modifiers::default());
    draw(cx);
    assert!(view.read_with(cx, |v, _| v.open));
    assert!(view.read_with(cx, |v, _| v.layouts.get()) > 0);
    key(cx, "escape");
    assert!(!view.read_with(cx, |v, _| v.open));
    assert_eq!(
        view.read_with(cx, |v, _| v.requests.clone()),
        vec![true, false]
    );
    assert!(cx.update(|window, cx| view.read(cx).trigger.is_focused(window)));
}

#[gpui::test]
fn focused_child_button_owns_enter_even_when_dialog_has_a_default_confirm(cx: &mut TestAppContext) {
    let (view, cx) = mount(cx, Kind::Dialog);
    view.update(cx, |v, cx| {
        v.confirm_enabled = true;
        cx.notify();
    });
    cx.simulate_click(point(px(10.), px(10.)), Modifiers::default());
    draw(cx);
    let root_focus = cx.update(|window, cx| {
        let root = window.focused(cx).unwrap();
        let inner = view.read(cx).inner.clone();
        inner.focus(window, cx);
        root
    });
    key(cx, "enter");
    assert_eq!(
        view.read_with(cx, |v, _| (v.inner_clicks, v.confirms, v.open)),
        (1, 0, true)
    );
    cx.update(|window, cx| root_focus.focus(window, cx));
    key(cx, "enter");
    assert_eq!(
        view.read_with(cx, |v, _| (v.inner_clicks, v.confirms, v.open)),
        (1, 1, false)
    );
}
