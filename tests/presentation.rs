use std::{
    cell::{Cell, RefCell},
    rc::Rc,
};

use astryx_gpui::{
    AppShell, AspectRatio, Avatar, Button, ButtonGroup, Center, Grid, Icon, IconButton, Theme,
};
use gpui::{
    Bounds, Context, Entity, FocusHandle, IntoElement, KeyDownEvent, KeyUpEvent, Keystroke,
    Modifiers, ParentElement, Pixels, Render, Styled, TestAppContext, VisualTestContext, Window,
    div, point, px,
};
use gpui_base::ElementExt as _;

#[derive(Clone, Copy)]
enum Mode {
    Layout,
    Media,
    Shell,
    Group,
}
#[derive(Default)]
struct Measures {
    first: RefCell<Option<Bounds<Pixels>>>,
    second: RefCell<Option<Bounds<Pixels>>>,
    ratio: RefCell<Option<Bounds<Pixels>>>,
    center: RefCell<Option<Bounds<Pixels>>>,
    frame: RefCell<Option<Bounds<Pixels>>>,
    nav: Cell<usize>,
}
struct Harness {
    mode: Mode,
    measure: Rc<Measures>,
    open: bool,
    disabled: bool,
    clicks: usize,
    focus_a: FocusHandle,
    focus_b: FocusHandle,
}
impl Render for Harness {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let first = self.measure.clone();
        let second = self.measure.clone();
        let ratio = self.measure.clone();
        let center = self.measure.clone();
        let frame = self.measure.clone();
        let nav = self.measure.clone();
        let owner = cx.entity().downgrade();
        let child=match self.mode{
            Mode::Layout=>div().flex().flex_col().gap(px(10.))
                .child(Grid::new(2).spacing(10.).w(px(200.)).child(div().h(px(20.)).on_prepaint(move|bounds,_,_|*first.first.borrow_mut()=Some(bounds))).child(div().h(px(20.)).on_prepaint(move|bounds,_,_|*second.second.borrow_mut()=Some(bounds))))
                .child(AspectRatio::new(2.).w(px(200.)).child(div().size_full().on_prepaint(move|bounds,_,_|*ratio.ratio.borrow_mut()=Some(bounds))))
                .child(div().w(px(200.)).h(px(80.)).on_prepaint(move|bounds,_,_|*frame.frame.borrow_mut()=Some(bounds)).child(Center::new().size_full().child(div().size(px(20.)).on_prepaint(move|bounds,_,_|*center.center.borrow_mut()=Some(bounds)))))
                .into_any_element(),
            Mode::Media=>div().flex().gap(px(10.)).child(IconButton::new("icon","Favorite",Icon::svg("star","<svg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 16 16'><circle cx='8' cy='8' r='4'/></svg>")).disabled(self.disabled).on_click(cx.listener(|this,_,_,cx|{this.clicks+=1;cx.notify();})))
                .child(Avatar::new("avatar","홍 길동").theme(Theme::dark())).into_any_element(),
            Mode::Shell=>AppShell::new("app","Example").navigation_open(self.open).on_navigation_change(move|open,_,cx|{let _=owner.update(cx,|this,cx|{this.open=open;cx.notify();});})
                .navigation(div().size_full().on_prepaint(move|_,_,_|nav.nav.set(nav.nav.get()+1))).child("Main content").w(px(400.)).h(px(280.)).into_any_element(),
            Mode::Group=>ButtonGroup::new("actions")
                .child(Button::new("a","A").focus_handle(&self.focus_a).on_click(cx.listener(|this,_,_,cx|{this.clicks+=1;cx.notify();})))
                .child(Button::new("b","B").focus_handle(&self.focus_b).on_click(cx.listener(|this,_,_,cx|{this.clicks+=10;cx.notify();}))).into_any_element(),
        };
        div().w(px(600.)).h(px(500.)).child(child)
    }
}
fn mount(cx: &mut TestAppContext, mode: Mode) -> (Entity<Harness>, &mut VisualTestContext) {
    cx.update(astryx_gpui::init);
    let (view, cx) = cx.add_window_view(move |_, cx| Harness {
        mode,
        measure: Rc::new(Measures::default()),
        open: true,
        disabled: false,
        clicks: 0,
        focus_a: cx.focus_handle(),
        focus_b: cx.focus_handle(),
    });
    cx.update(|window, cx| window.draw(cx).clear(cx));
    (view, cx)
}
fn key(cx: &mut VisualTestContext, key: &str) {
    let keystroke = Keystroke::parse(key).unwrap();
    cx.simulate_event(KeyDownEvent {
        keystroke: keystroke.clone(),
        is_held: false,
        prefer_character_input: false,
    });
    cx.simulate_event(KeyUpEvent { keystroke });
    cx.update(|window, cx| window.draw(cx).clear(cx));
}

#[gpui::test]
fn native_layout_applies_grid_ratio_and_centering(cx: &mut TestAppContext) {
    let (view, cx) = mount(cx, Mode::Layout);
    let m = view.read_with(cx, |this, _| this.measure.clone());
    let a = m.first.borrow().unwrap();
    let b = m.second.borrow().unwrap();
    assert_eq!(a.origin.y, b.origin.y);
    assert_eq!(b.origin.x - a.origin.x, px(105.));
    assert_eq!(a.size.width, px(95.));
    let ratio = m.ratio.borrow().unwrap();
    assert_eq!(ratio.size.width, px(200.));
    assert_eq!(ratio.size.height, px(100.));
    let center = m.center.borrow().unwrap();
    let frame = m.frame.borrow().unwrap();
    assert_eq!(center.center(), frame.center());
}
#[gpui::test]
fn application_navigation_is_owner_controlled_and_hidden_content_not_laid_out(
    cx: &mut TestAppContext,
) {
    let (view, cx) = mount(cx, Mode::Shell);
    let m = view.read_with(cx, |this, _| this.measure.clone());
    assert!(m.nav.get() > 0);
    cx.update(|window, cx| window.focus_next(cx));
    key(cx, "enter");
    assert!(!view.read_with(cx, |this, _| this.open));
    let count = m.nav.get();
    cx.update(|window, cx| window.draw(cx).clear(cx));
    assert_eq!(m.nav.get(), count);
    key(cx, "space");
    assert!(view.read_with(cx, |this, _| this.open));
    assert!(m.nav.get() > count);
}
#[gpui::test]
fn icon_only_native_button_activates_and_respects_disable(cx: &mut TestAppContext) {
    let (view, cx) = mount(cx, Mode::Media);
    cx.simulate_click(point(px(10.), px(10.)), Modifiers::default());
    assert_eq!(view.read_with(cx, |this, _| this.clicks), 1);
    view.update(cx, |this, cx| {
        this.disabled = true;
        cx.notify();
    });
    cx.update(|window, cx| window.draw(cx).clear(cx));
    cx.simulate_click(point(px(10.), px(10.)), Modifiers::default());
    key(cx, "enter");
    assert_eq!(view.read_with(cx, |this, _| this.clicks), 1);
}
#[gpui::test]
fn button_group_native_arrows_move_focus_between_actions(cx: &mut TestAppContext) {
    let (view, cx) = mount(cx, Mode::Group);
    cx.update(|window, cx| window.focus_next(cx));
    key(cx, "right");
    cx.update(|window, cx| assert!(view.read(cx).focus_b.is_focused(window)));
    key(cx, "enter");
    assert_eq!(view.read_with(cx, |this, _| this.clicks), 10);
}
