use astryx_gpui::{
    Breadcrumb, Breadcrumbs, Button, Input, InputState, Link, MobileNav, NavItem, Pagination,
    SideNav, Step, Stepper, Toolbar, TopNav,
};
use gpui::{
    AppContext, Context, Entity, InteractiveElement as _, IntoElement, KeyDownEvent, KeyUpEvent,
    Keystroke, Modifiers, ParentElement, Render, Styled, TestAppContext, VisualTestContext, Window,
    div, point, px,
};
#[derive(Clone, Copy)]
enum Kind {
    Link,
    Breadcrumbs,
    Top,
    Side,
    Mobile,
    Pagination,
    Stepper,
    Toolbar,
}
struct Navigation {
    kind: Kind,
    selected: gpui::SharedString,
    page: usize,
    step: usize,
    changes: usize,
    disabled: bool,
    input: Entity<InputState>,
}
impl Render for Navigation {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let owner = cx.entity().downgrade();
        let select = move |key: gpui::SharedString, _: &mut Window, cx: &mut gpui::App| {
            let _ = owner.update(cx, |s, cx| {
                s.selected = key;
                s.changes += 1;
                cx.notify();
            });
        };
        let items = vec![
            NavItem::new("a", "Alpha"),
            NavItem::new("b", "Beta").disabled(true),
            NavItem::new("c", "Gamma"),
        ];
        let view = match self.kind {
            Kind::Link => Link::new("link", "Details")
                .href("internal:details")
                .disabled(self.disabled)
                .open_with(move |href, _, w, cx| select(href.to_string().into(), w, cx))
                .into_any_element(),
            Kind::Breadcrumbs => Breadcrumbs::new("trail")
                .items(vec![
                    Breadcrumb::new("home", "Home"),
                    Breadcrumb::new("settings", "Settings"),
                ])
                .on_select(select)
                .into_any_element(),
            Kind::Top => TopNav::new("nav")
                .items(items)
                .selected(self.selected.clone())
                .on_select(select)
                .into_any_element(),
            Kind::Side => SideNav::new("nav")
                .items(items)
                .selected(self.selected.clone())
                .on_select(select)
                .into_any_element(),
            Kind::Mobile => MobileNav::new("nav")
                .items(items)
                .selected(self.selected.clone())
                .on_select(select)
                .into_any_element(),
            Kind::Pagination => {
                let owner = cx.entity().downgrade();
                Pagination::new("pages", self.page, 3)
                    .disabled(self.disabled)
                    .on_change(move |page, _, cx| {
                        let _ = owner.update(cx, |s, cx| {
                            s.page = page;
                            s.changes += 1;
                            cx.notify();
                        });
                    })
                    .into_any_element()
            }
            Kind::Stepper => {
                let owner = cx.entity().downgrade();
                Stepper::new("steps")
                    .steps(vec![
                        Step::new("One"),
                        Step::new("Two").disabled(true),
                        Step::new("Three"),
                    ])
                    .current(self.step)
                    .on_select(move |step, _, cx| {
                        let _ = owner.update(cx, |s, cx| {
                            s.step = step;
                            s.changes += 1;
                            cx.notify();
                        });
                    })
                    .into_any_element()
            }
            Kind::Toolbar => {
                let a = cx.entity().downgrade();
                let c = cx.entity().downgrade();
                Toolbar::new("tools")
                    .child(Button::new("a", "Alpha").on_click(move |_, _, cx| {
                        let _ = a.update(cx, |s, cx| {
                            s.selected = "a".into();
                            s.changes += 1;
                            cx.notify();
                        });
                    }))
                    .child(Button::new("b", "Beta").disabled(true))
                    .child(Button::new("c", "Gamma").on_click(move |_, _, cx| {
                        let _ = c.update(cx, |s, cx| {
                            s.selected = "c".into();
                            s.changes += 1;
                            cx.notify();
                        });
                    }))
                    .child(Input::new(&self.input).w(px(120.)))
                    .into_any_element()
            }
        };
        div()
            .id("navigation-owner")
            .p(px(10.))
            .w(px(500.))
            .h(px(300.))
            .child(view)
    }
}
fn mount(cx: &mut TestAppContext, kind: Kind) -> (Entity<Navigation>, &mut VisualTestContext) {
    cx.update(astryx_gpui::init);
    let (v, cx) = cx.add_window_view(move |w, cx| Navigation {
        kind,
        selected: "a".into(),
        page: 1,
        step: 0,
        changes: 0,
        disabled: false,
        input: cx.new(|cx| InputState::new(w, cx)),
    });
    cx.update(|w, cx| w.draw(cx).clear(cx));
    (v, cx)
}
fn key(cx: &mut VisualTestContext, key: &str) {
    let k = Keystroke::parse(key).unwrap();
    cx.simulate_event(KeyDownEvent {
        keystroke: k.clone(),
        is_held: false,
        prefer_character_input: false,
    });
    cx.simulate_event(KeyUpEvent { keystroke: k });
    cx.update(|w, cx| w.draw(cx).clear(cx));
}
#[gpui::test]
fn link_uses_injected_navigation_and_disabled_never_opens(cx: &mut TestAppContext) {
    let (v, cx) = mount(cx, Kind::Link);
    cx.simulate_click(point(px(20.), px(20.)), Modifiers::default());
    assert_eq!(
        v.read_with(cx, |s, _| s.selected.to_string()),
        "internal:details"
    );
    v.update(cx, |s, cx| {
        s.disabled = true;
        cx.notify();
    });
    cx.update(|w, cx| w.draw(cx).clear(cx));
    cx.simulate_click(point(px(20.), px(20.)), Modifiers::default());
    key(cx, "enter");
    assert_eq!(v.read_with(cx, |s, _| s.changes), 1);
}
#[gpui::test]
fn breadcrumb_current_location_is_not_an_action(cx: &mut TestAppContext) {
    let (v, cx) = mount(cx, Kind::Breadcrumbs);
    cx.simulate_click(point(px(20.), px(20.)), Modifiers::default());
    assert_eq!(v.read_with(cx, |s, _| s.selected.to_string()), "home");
    cx.simulate_click(point(px(125.), px(20.)), Modifiers::default());
    assert_eq!(v.read_with(cx, |s, _| s.changes), 1);
}
fn nav_case(cx: &mut TestAppContext, kind: Kind, key_name: &str) {
    let (v, cx) = mount(cx, kind);
    cx.update(|w, cx| w.focus_next(cx));
    key(cx, key_name);
    key(cx, "enter");
    assert_eq!(
        v.read_with(cx, |s, _| (s.selected.to_string(), s.changes)),
        ("c".into(), 1)
    );
}
#[gpui::test]
fn top_navigation_skips_disabled_items(cx: &mut TestAppContext) {
    nav_case(cx, Kind::Top, "right");
}
#[gpui::test]
fn side_navigation_uses_vertical_arrows(cx: &mut TestAppContext) {
    nav_case(cx, Kind::Side, "down");
}
#[gpui::test]
fn mobile_navigation_remains_keyboard_operable(cx: &mut TestAppContext) {
    nav_case(cx, Kind::Mobile, "end");
}
#[gpui::test]
fn pagination_stops_at_bounds_and_disabled_blocks_requests(cx: &mut TestAppContext) {
    let (v, cx) = mount(cx, Kind::Pagination);
    cx.simulate_click(point(px(170.), px(25.)), Modifiers::default());
    cx.update(|w, cx| w.draw(cx).clear(cx));
    assert_eq!(v.read_with(cx, |s, _| s.page), 2);
    key(cx, "enter");
    assert_eq!(v.read_with(cx, |s, _| s.page), 3);
    cx.simulate_click(point(px(170.), px(25.)), Modifiers::default());
    assert_eq!(v.read_with(cx, |s, _| s.changes), 2);
    v.update(cx, |s, cx| {
        s.disabled = true;
        cx.notify();
    });
    cx.update(|w, cx| w.draw(cx).clear(cx));
    cx.simulate_click(point(px(20.), px(25.)), Modifiers::default());
    assert_eq!(v.read_with(cx, |s, _| s.changes), 2);
}
#[gpui::test]
fn stepper_disabled_steps_do_not_receive_keyboard_activation(cx: &mut TestAppContext) {
    let (v, cx) = mount(cx, Kind::Stepper);
    cx.update(|w, cx| w.focus_next(cx));
    cx.update(|w, cx| w.focus_next(cx));
    key(cx, "enter");
    assert_eq!(v.read_with(cx, |s, _| (s.step, s.changes)), (2, 1));
}
#[gpui::test]
fn toolbar_roves_enabled_controls_but_input_keeps_caret_arrows(cx: &mut TestAppContext) {
    let (v, cx) = mount(cx, Kind::Toolbar);
    cx.update(|w, cx| w.focus_next(cx));
    key(cx, "right");
    key(cx, "enter");
    assert_eq!(v.read_with(cx, |s, _| s.selected.to_string()), "c");
    let input = v.read_with(cx, |s, _| s.input.clone());
    cx.update(|w, cx| {
        input.update(cx, |s, cx| {
            s.set_value("abc", w, cx);
            s.focus(w, cx);
        })
    });
    key(cx, "left");
    cx.simulate_input("!");
    assert_eq!(input.read_with(cx, |s, _| s.value().to_string()), "ab!c");
}
