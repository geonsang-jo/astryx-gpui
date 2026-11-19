use astryx_gpui::{
    Button, Carousel, OverflowList, Resizable, ResizablePane, ResizableState, ScrollableArea,
    SnapPaging, Swipe, SwipeDirection, Theme,
};
use gpui::{
    AnyElement, AppContext as _, Context, Entity, FocusHandle, IntoElement, KeyDownEvent,
    KeyUpEvent, Keystroke, Modifiers, MouseButton, ParentElement, Render, ScrollHandle, Styled,
    TestAppContext, VisualTestContext, Window, div, point, px,
};

#[derive(Clone, Copy)]
enum Kind {
    Carousel,
    Swipe,
    Snap,
    Scroll,
    Resize,
    Overflow,
}
struct Scene {
    kind: Kind,
    index: usize,
    changes: usize,
    disabled: bool,
    dark: bool,
    root_focus: FocusHandle,
    child_focus: FocusHandle,
    outside_focus: FocusHandle,
    scroll: ScrollHandle,
    resize: Entity<ResizableState>,
    swipes: Vec<SwipeDirection>,
    overflow: bool,
    visible_count: usize,
    child_count: usize,
}
impl Render for Scene {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = if self.dark {
            Theme::dark()
        } else {
            Theme::light()
        };
        let weak = cx.entity().downgrade();
        let changed = move |index, _: &mut Window, cx: &mut gpui::App| {
            let _ = weak.update(cx, |this, cx| {
                this.index = index;
                this.changes += 1;
                cx.notify();
            });
        };
        let control: AnyElement = match self.kind {
            Kind::Carousel => Carousel::new("carousel")
                .theme(theme)
                .index(self.index)
                .disabled(self.disabled)
                .focus_handle(&self.root_focus)
                .on_change(changed)
                .children((0usize..3).map(|i| {
                    div().h(px(48.)).child(
                        Button::new(("page-action", i), format!("Page {i}"))
                            .theme(theme)
                            .focus_handle(&self.child_focus),
                    )
                }))
                .into_any_element(),
            Kind::Swipe => {
                let weak = cx.entity().downgrade();
                Swipe::new("swipe")
                    .w(px(240.))
                    .h(px(140.))
                    .disabled(self.disabled)
                    .on_swipe(move |direction, _, cx| {
                        let _ = weak.update(cx, |this, cx| {
                            this.swipes.push(direction);
                            cx.notify();
                        });
                    })
                    .child(div().size_full().bg(theme.colors.canvas))
                    .into_any_element()
            }
            Kind::Snap => SnapPaging::new("snap")
                .index(self.index)
                .page_width(240.)
                .scroll_handle(&self.scroll)
                .focus_handle(&self.root_focus)
                .on_change(changed)
                .children((0usize..3).map(|i| div().h(px(80.)).child(format!("Page {i}"))))
                .into_any_element(),
            Kind::Scroll => ScrollableArea::new("scroll")
                .theme(theme)
                .scroll_handle(&self.scroll)
                .w(px(240.))
                .h(px(120.))
                .children((0..20).map(|i| div().flex_none().h(px(30.)).child(format!("Row {i}"))))
                .into_any_element(),
            Kind::Resize => {
                let weak = cx.entity().downgrade();
                Resizable::new("resize", &self.resize)
                    .theme(theme)
                    .w(px(340.))
                    .h(px(120.))
                    .panel(
                        ResizablePane::new()
                            .size(170.)
                            .size_range(100., 240.)
                            .child("Left"),
                    )
                    .panel(
                        ResizablePane::new()
                            .size(170.)
                            .size_range(100., 240.)
                            .child("Right"),
                    )
                    .on_resize(move |_, _, cx| {
                        let _ = weak.update(cx, |this, cx| {
                            this.changes += 1;
                            cx.notify();
                        });
                    })
                    .into_any_element()
            }
            Kind::Overflow => {
                let weak = cx.entity().downgrade();
                OverflowList::new("overflow")
                    .theme(theme)
                    .focus_handle(&self.root_focus)
                    .visible_count(self.visible_count)
                    .open(self.overflow)
                    .on_open_change(move |open, _, cx| {
                        let _ = weak.update(cx, |this, cx| {
                            this.overflow = open;
                            cx.notify();
                        });
                    })
                    .children(
                        ["One", "Two", "Three", "Four"]
                            .into_iter()
                            .take(self.child_count)
                            .enumerate()
                            .map(|(i, title)| {
                                let button = Button::new(("item", i), title).theme(theme);
                                if i == 2 {
                                    button.focus_handle(&self.child_focus)
                                } else {
                                    button
                                }
                            }),
                    )
                    .into_any_element()
            }
        };
        let root = div()
            .flex()
            .flex_col()
            .items_start()
            .w(px(400.))
            .h(px(280.))
            .child(control);
        if matches!(self.kind, Kind::Overflow) {
            root.child(
                Button::new("after-overflow", "After list").focus_handle(&self.outside_focus),
            )
        } else {
            root
        }
    }
}
fn mount(cx: &mut TestAppContext, kind: Kind) -> (Entity<Scene>, &mut VisualTestContext) {
    cx.update(astryx_gpui::init);
    let (scene, cx) = cx.add_window_view(move |_, cx| Scene {
        kind,
        index: 0,
        changes: 0,
        disabled: false,
        dark: false,
        root_focus: cx.focus_handle(),
        child_focus: cx.focus_handle(),
        outside_focus: cx.focus_handle(),
        scroll: ScrollHandle::new(),
        resize: cx.new(|_| ResizableState::default()),
        swipes: vec![],
        overflow: false,
        visible_count: 2,
        child_count: 4,
    });
    draw(cx);
    (scene, cx)
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
fn carousel_controls_move_pages_once_and_leave_child_and_modified_keys_alone(
    cx: &mut TestAppContext,
) {
    let (scene, cx) = mount(cx, Kind::Carousel);
    cx.update(|window, cx| {
        let focus = scene.read(cx).root_focus.clone();
        focus.focus(window, cx);
    });
    key(cx, "right");
    assert_eq!(scene.read_with(cx, |s, _| (s.index, s.changes)), (1, 1));
    cx.update(|window, cx| {
        let focus = scene.read(cx).child_focus.clone();
        focus.focus(window, cx);
    });
    key(cx, "right");
    assert_eq!(
        scene.read_with(cx, |s, _| s.index),
        1,
        "nested controls retain their own arrow keys"
    );
    cx.update(|window, cx| {
        let focus = scene.read(cx).root_focus.clone();
        focus.focus(window, cx);
    });
    key(cx, "alt-left");
    assert_eq!(scene.read_with(cx, |s, _| s.index), 1);
    scene.update(cx, |s, cx| {
        s.disabled = true;
        cx.notify();
    });
    draw(cx);
    key(cx, "left");
    assert_eq!(scene.read_with(cx, |s, _| s.index), 1);
}

#[gpui::test]
fn swipe_distinguishes_clicks_intentional_drags_and_outside_release(cx: &mut TestAppContext) {
    let (scene, cx) = mount(cx, Kind::Swipe);
    cx.simulate_click(point(px(80.), px(50.)), Modifiers::default());
    assert!(scene.read_with(cx, |s, _| s.swipes.is_empty()));
    cx.simulate_mouse_down(
        point(px(180.), px(70.)),
        MouseButton::Left,
        Modifiers::default(),
    );
    cx.simulate_mouse_move(
        point(px(30.), px(70.)),
        MouseButton::Left,
        Modifiers::default(),
    );
    cx.simulate_mouse_up(
        point(px(30.), px(70.)),
        MouseButton::Left,
        Modifiers::default(),
    );
    draw(cx);
    assert_eq!(
        scene.read_with(cx, |s, _| s.swipes.clone()),
        [SwipeDirection::Left]
    );
    cx.simulate_mouse_down(
        point(px(100.), px(70.)),
        MouseButton::Left,
        Modifiers::default(),
    );
    cx.simulate_mouse_up(
        point(px(300.), px(70.)),
        MouseButton::Left,
        Modifiers::default(),
    );
    cx.simulate_mouse_up(
        point(px(30.), px(70.)),
        MouseButton::Left,
        Modifiers::default(),
    );
    assert_eq!(
        scene.read_with(cx, |s, _| s.swipes.len()),
        1,
        "a release outside cancels and clears its press"
    );
    scene.update(cx, |s, cx| {
        s.disabled = true;
        cx.notify();
    });
    draw(cx);
    cx.simulate_mouse_down(
        point(px(180.), px(70.)),
        MouseButton::Left,
        Modifiers::default(),
    );
    cx.simulate_mouse_up(
        point(px(30.), px(70.)),
        MouseButton::Left,
        Modifiers::default(),
    );
    assert_eq!(scene.read_with(cx, |s, _| s.swipes.len()), 1);
}

#[gpui::test]
fn snap_paging_moves_the_actual_native_viewport_and_clamps_at_the_last_page(
    cx: &mut TestAppContext,
) {
    let (scene, cx) = mount(cx, Kind::Snap);
    cx.update(|window, cx| {
        let focus = scene.read(cx).root_focus.clone();
        focus.focus(window, cx);
    });
    key(cx, "right");
    assert_eq!(scene.read_with(cx, |s, _| s.index), 1);
    assert_eq!(scene.read_with(cx, |s, _| s.scroll.offset().x), px(-240.));
    key(cx, "right");
    key(cx, "right");
    assert_eq!(scene.read_with(cx, |s, _| s.index), 2);
    assert_eq!(scene.read_with(cx, |s, _| s.scroll.offset().x), px(-480.));
}

#[gpui::test]
fn scrollable_area_keeps_the_native_offset_when_the_theme_changes(cx: &mut TestAppContext) {
    let (scene, cx) = mount(cx, Kind::Scroll);
    scene.read_with(cx, |s, _| s.scroll.set_offset(point(px(0.), px(-100.))));
    draw(cx);
    assert_eq!(scene.read_with(cx, |s, _| s.scroll.offset().y), px(-100.));
    scene.update(cx, |s, cx| {
        s.dark = true;
        cx.notify();
    });
    draw(cx);
    assert_eq!(scene.read_with(cx, |s, _| s.scroll.offset().y), px(-100.));
}

#[gpui::test]
fn resizable_uses_native_constraints_and_reports_new_sizes(cx: &mut TestAppContext) {
    let (scene, cx) = mount(cx, Kind::Resize);
    let resize = scene.read_with(cx, |s, _| s.resize.clone());
    let boundary = resize.read_with(cx, |s, _| s.sizes()[0]);
    cx.simulate_mouse_down(
        point(boundary - px(2.), px(50.)),
        MouseButton::Left,
        Modifiers::default(),
    );
    cx.simulate_mouse_move(
        point(px(300.), px(50.)),
        MouseButton::Left,
        Modifiers::default(),
    );
    cx.simulate_mouse_up(
        point(px(300.), px(50.)),
        MouseButton::Left,
        Modifiers::default(),
    );
    draw(cx);
    let sizes = resize.read_with(cx, |s, _| s.sizes().clone());
    assert_eq!(sizes.len(), 2);
    assert!(
        sizes.iter().all(|s| *s >= px(100.) && *s <= px(240.)),
        "actual resized dimensions honor the host's limits: {sizes:?}"
    );
    assert!(scene.read_with(cx, |s, _| s.changes > 0));
}

#[gpui::test]
fn overflow_items_open_in_a_real_popup_and_escape_returns_to_the_list(cx: &mut TestAppContext) {
    let (scene, cx) = mount(cx, Kind::Overflow);
    cx.simulate_click(point(px(130.), px(14.)), Modifiers::default());
    draw(cx);
    draw(cx);
    assert!(scene.read_with(cx, |s, _| s.overflow));
    key(cx, "escape");
    assert!(!scene.read_with(cx, |s, _| s.overflow));
}

#[gpui::test]
fn expanding_overflow_capacity_closes_the_popup_without_stranding_focus(cx: &mut TestAppContext) {
    let (scene, cx) = mount(cx, Kind::Overflow);
    cx.simulate_click(point(px(130.), px(14.)), Modifiers::default());
    draw(cx);
    draw(cx);
    let popup_focus = cx.update(|window, cx| window.focused(cx).expect("open popup focus"));
    scene.update(cx, |s, cx| {
        s.visible_count = 4;
        cx.notify();
    });
    draw(cx);
    draw(cx);
    assert!(
        scene.read_with(cx, |s, _| s.overflow),
        "the owner retains its controlled flag"
    );
    assert!(!cx.update(|window, _| popup_focus.is_focused(window)));
    assert!(cx.update(|window, cx| scene.read(cx).root_focus.is_focused(window)));
    assert!(!cx.update(|_, cx| gpui_base::GlobalState::is_in_deferred_context(cx)));
}

#[gpui::test]
fn shrinking_overflow_children_closes_the_popup_without_stranding_focus(cx: &mut TestAppContext) {
    let (scene, cx) = mount(cx, Kind::Overflow);
    cx.simulate_click(point(px(130.), px(14.)), Modifiers::default());
    draw(cx);
    draw(cx);
    cx.update(|window, cx| scene.read(cx).child_focus.clone().focus(window, cx));
    draw(cx);
    let popup_child_focus = cx.update(|window, cx| window.focused(cx).expect("popup child focus"));
    scene.update(cx, |s, cx| {
        s.child_count = 2;
        cx.notify();
    });
    draw(cx);
    draw(cx);
    assert!(
        scene.read_with(cx, |s, _| s.overflow),
        "the owner retains its controlled flag"
    );
    assert!(!cx.update(|window, _| popup_child_focus.is_focused(window)));
    assert!(cx.update(|window, cx| scene.read(cx).root_focus.is_focused(window)));
    assert!(!cx.update(|_, cx| gpui_base::GlobalState::is_in_deferred_context(cx)));
}

#[gpui::test]
fn removing_overflow_does_not_steal_focus_from_another_control(cx: &mut TestAppContext) {
    let (scene, cx) = mount(cx, Kind::Overflow);
    cx.simulate_click(point(px(130.), px(14.)), Modifiers::default());
    draw(cx);
    draw(cx);
    cx.update(|window, cx| scene.read(cx).outside_focus.clone().focus(window, cx));
    scene.update(cx, |s, cx| {
        s.visible_count = 4;
        cx.notify();
    });
    draw(cx);
    draw(cx);
    assert!(scene.read_with(cx, |s, _| s.overflow));
    assert!(cx.update(|window, cx| scene.read(cx).outside_focus.is_focused(window)));
    assert!(!cx.update(|_, cx| gpui_base::GlobalState::is_in_deferred_context(cx)));
}
