use astryx_gpui::{Button, Checkbox, Collapsible, Switch, Tab, Tabs};
use gpui::{
    Context, Entity, InteractiveElement, IntoElement, KeyDownEvent, KeyUpEvent, Keystroke,
    Modifiers, ParentElement, Render, StatefulInteractiveElement, Styled, TestAppContext,
    VisualTestContext, Window, div, point, px,
};
use gpui_base::ElementExt as _;
use std::{cell::Cell, rc::Rc};

#[derive(Clone, Copy)]
enum Kind {
    Button,
    IconButton,
    Checkbox,
    Tabs,
    Collapsible,
    Switch,
}

struct Harness {
    kind: Kind,
    disabled: bool,
    clicks: usize,
    button_focus: gpui::FocusHandle,
    parent_clicks: usize,
    checked: bool,
    changes: Vec<bool>,
    selected: gpui::SharedString,
    selections: Vec<gpui::SharedString>,
    focused_selections: Vec<bool>,
    open: bool,
    content_layouts: Rc<Cell<usize>>,
}

impl Render for Harness {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let owner = cx.entity().downgrade();
        let control = match self.kind {
            Kind::Button => Button::new("button", "Activate")
                .focus_handle(&self.button_focus)
                .disabled(self.disabled)
                .w(px(140.))
                .on_click(cx.listener(|this, _, _, cx| {
                    this.clicks += 1;
                    cx.notify();
                }))
                .into_any_element(),
            Kind::IconButton => Button::new("icon-button", "")
                .accessibility_label("Activate")
                .leading(div().size(px(14.)).child("✦"))
                .trailing(div().size(px(14.)).child("→"))
                .disabled(self.disabled)
                .w(px(80.))
                .on_click(cx.listener(|this, _, _, cx| {
                    this.clicks += 1;
                    cx.notify();
                }))
                .into_any_element(),
            Kind::Checkbox => Checkbox::new("checkbox", "Subscribe")
                .checked(self.checked)
                .disabled(self.disabled)
                .w(px(180.))
                .on_change(move |checked, _, cx| {
                    let _ = owner.update(cx, |this, cx| {
                        this.checked = checked;
                        this.changes.push(checked);
                        cx.notify();
                    });
                })
                .into_any_element(),
            Kind::Tabs => Tabs::new("tabs")
                .items(vec![
                    Tab::new("a", "Alpha"),
                    Tab::new("b", "Beta").disabled(true),
                    Tab::new("c", "Gamma"),
                ])
                .selected(self.selected.clone())
                .on_select_with_focus(move |selected, focused, _, cx| {
                    let _ = owner.update(cx, |this, cx| {
                        this.selected = selected.clone();
                        this.selections.push(selected);
                        this.focused_selections.push(focused);
                        cx.notify();
                    });
                })
                .into_any_element(),
            Kind::Switch => Switch::new("switch", "Notifications")
                .checked(self.checked)
                .disabled(self.disabled)
                .on_change(move |checked, _, cx| {
                    let _ = owner.update(cx, |this, cx| {
                        this.checked = checked;
                        this.changes.push(checked);
                        cx.notify();
                    });
                })
                .into_any_element(),
            Kind::Collapsible => {
                let layouts = self.content_layouts.clone();
                Collapsible::new("details", "Details")
                    .open(self.open)
                    .disabled(self.disabled)
                    .on_open_change(move |open, _, cx| {
                        let _ = owner.update(cx, |this, cx| {
                            this.open = open;
                            cx.notify();
                        });
                    })
                    .child(
                        div()
                            .h(px(20.))
                            .child("Only laid out while expanded")
                            .on_prepaint(move |_, _, _| layouts.set(layouts.get() + 1)),
                    )
                    .into_any_element()
            }
        };
        div()
            .id("owner")
            .tab_group()
            .w(px(400.))
            .h(px(280.))
            .on_click(cx.listener(|this, _, _, cx| {
                this.parent_clicks += 1;
                cx.notify();
            }))
            .child(control)
    }
}

fn mount(
    cx: &mut TestAppContext,
    kind: Kind,
    disabled: bool,
) -> (Entity<Harness>, &mut VisualTestContext) {
    cx.update(astryx_gpui::init);
    let (view, cx) = cx.add_window_view(move |_, cx| Harness {
        kind,
        disabled,
        clicks: 0,
        button_focus: cx.focus_handle(),
        parent_clicks: 0,
        checked: false,
        changes: Vec::new(),
        selected: "a".into(),
        selections: Vec::new(),
        focused_selections: Vec::new(),
        open: false,
        content_layouts: Rc::new(Cell::new(0)),
    });
    cx.update(|window, cx| window.draw(cx).clear(cx));
    (view, cx)
}

fn key(cx: &mut VisualTestContext, name: &str) {
    let keystroke = Keystroke::parse(name).expect("valid test key");
    cx.simulate_event(KeyDownEvent {
        keystroke: keystroke.clone(),
        is_held: false,
        prefer_character_input: false,
    });
    cx.simulate_event(KeyUpEvent { keystroke });
}

#[gpui::test]
fn button_pointer_enter_and_space_activate_once_each(cx: &mut TestAppContext) {
    let (view, cx) = mount(cx, Kind::Button, false);
    cx.simulate_click(point(px(10.), px(10.)), Modifiers::default());
    assert_eq!(view.read_with(cx, |view, _| view.clicks), 1);
    key(cx, "enter");
    key(cx, "space");
    assert_eq!(view.read_with(cx, |view, _| view.clicks), 3);
}

#[gpui::test]
fn disabled_button_blocks_activation_and_has_no_tab_stop(cx: &mut TestAppContext) {
    let (view, cx) = mount(cx, Kind::Button, true);
    cx.simulate_click(point(px(10.), px(10.)), Modifiers::default());
    cx.update(|window, cx| window.focus_next(cx));
    key(cx, "enter");
    key(cx, "space");
    assert_eq!(view.read_with(cx, |view, _| view.clicks), 0);
    assert_eq!(view.read_with(cx, |view, _| view.parent_clicks), 0);
    cx.update(|window, cx| assert!(window.focused(cx).is_none()));
}

#[gpui::test]
fn icon_children_share_button_activation_and_disabled_semantics(cx: &mut TestAppContext) {
    let (view, cx) = mount(cx, Kind::IconButton, false);
    cx.simulate_click(point(px(16.), px(10.)), Modifiers::default());
    key(cx, "enter");
    assert_eq!(view.read_with(cx, |view, _| view.clicks), 2);
    view.update(cx, |view, cx| {
        view.disabled = true;
        cx.notify();
    });
    cx.update(|window, cx| window.draw(cx).clear(cx));
    cx.simulate_click(point(px(16.), px(10.)), Modifiers::default());
    key(cx, "space");
    assert_eq!(view.read_with(cx, |view, _| view.clicks), 2);
}

#[gpui::test]
fn switch_reports_controlled_activation_and_suppresses_disabled_changes(cx: &mut TestAppContext) {
    let (view, cx) = mount(cx, Kind::Switch, false);
    cx.simulate_click(point(px(10.), px(10.)), Modifiers::default());
    key(cx, "space");
    assert_eq!(
        view.read_with(cx, |view, _| view.changes.clone()),
        vec![true, false]
    );
    view.update(cx, |view, cx| {
        view.disabled = true;
        cx.notify();
    });
    cx.update(|window, cx| window.draw(cx).clear(cx));
    cx.simulate_click(point(px(10.), px(10.)), Modifiers::default());
    key(cx, "enter");
    assert_eq!(view.read_with(cx, |view, _| view.changes.len()), 2);
}

#[gpui::test]
fn checkbox_reports_owner_controlled_pointer_and_keyboard_changes(cx: &mut TestAppContext) {
    let (view, cx) = mount(cx, Kind::Checkbox, false);
    cx.simulate_click(point(px(10.), px(10.)), Modifiers::default());
    key(cx, "enter");
    key(cx, "space");
    assert_eq!(
        view.read_with(cx, |view, _| view.changes.clone()),
        vec![true, false, true]
    );
    assert!(view.read_with(cx, |view, _| view.checked));
}

#[gpui::test]
fn disabled_checkbox_never_reports_a_change(cx: &mut TestAppContext) {
    let (view, cx) = mount(cx, Kind::Checkbox, true);
    cx.simulate_click(point(px(10.), px(10.)), Modifiers::default());
    cx.update(|window, cx| window.focus_next(cx));
    key(cx, "enter");
    key(cx, "space");
    assert!(view.read_with(cx, |view, _| view.changes.is_empty()));
}

#[gpui::test]
fn tabs_use_roving_arrows_home_and_end_while_skipping_disabled(cx: &mut TestAppContext) {
    let (view, cx) = mount(cx, Kind::Tabs, false);
    cx.simulate_click(point(px(10.), px(10.)), Modifiers::default());
    view.update(cx, |view, _| view.selections.clear());
    key(cx, "right");
    key(cx, "right");
    key(cx, "end");
    key(cx, "home");
    let selections = view.read_with(cx, |view, _| view.selections.clone());
    assert_eq!(
        selections
            .iter()
            .map(|value| value.as_ref())
            .collect::<Vec<_>>(),
        vec!["c", "a", "c", "a"]
    );
    key(cx, "ctrl-right");
    assert_eq!(view.read_with(cx, |view, _| view.selections.len()), 4);
    key(cx, "enter");
    key(cx, "space");
    assert_eq!(view.read_with(cx, |view, _| view.selections.len()), 6);
    assert!(view.read_with(cx, |view, _| {
        view.focused_selections.iter().all(|focused| *focused)
    }));
}

#[gpui::test]
fn disclosure_applies_owner_state_and_gates_content_layout(cx: &mut TestAppContext) {
    let (view, cx) = mount(cx, Kind::Collapsible, false);
    assert_eq!(view.read_with(cx, |view, _| view.content_layouts.get()), 0);
    cx.simulate_click(point(px(10.), px(10.)), Modifiers::default());
    assert!(view.read_with(cx, |view, _| view.open));
    let expanded_layouts = view.read_with(cx, |view, _| view.content_layouts.get());
    assert!(expanded_layouts > 0);
    key(cx, "space");
    assert!(!view.read_with(cx, |view, _| view.open));
    let closed_layouts = view.read_with(cx, |view, _| view.content_layouts.get());
    cx.update(|window, cx| window.draw(cx).clear(cx));
    assert_eq!(
        view.read_with(cx, |view, _| view.content_layouts.get()),
        closed_layouts
    );
}

#[gpui::test]
fn disabled_disclosure_cannot_open(cx: &mut TestAppContext) {
    let (view, cx) = mount(cx, Kind::Collapsible, true);
    cx.simulate_click(point(px(10.), px(10.)), Modifiers::default());
    key(cx, "enter");
    assert!(!view.read_with(cx, |view, _| view.open));
    assert_eq!(view.read_with(cx, |view, _| view.content_layouts.get()), 0);
}
