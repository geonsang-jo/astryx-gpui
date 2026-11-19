use astryx_gpui::{
    AlertDialog, CommandItem, CommandPalette, InputState, Lightbox, LightboxItem, PowerSearch,
    SearchSection,
};
use gpui::{
    AppContext as _, Bounds, Context, Entity, EntityInputHandler as _, FocusHandle, ImageSource,
    InteractiveElement as _, IntoElement, KeyDownEvent, KeyUpEvent, Keystroke, Modifiers,
    ParentElement, Pixels, Render, Styled, TestAppContext, VisualTestContext, Window, canvas, div,
    point, px,
};
use std::{cell::Cell, rc::Rc, sync::Arc};
#[derive(Clone, Copy)]
enum Kind {
    Alert,
    Lightbox,
    Commands,
    Search,
}
struct Harness {
    kind: Kind,
    open: bool,
    busy: bool,
    loading: bool,
    trigger: FocusHandle,
    query: Entity<InputState>,
    active: usize,
    highlighted: usize,
    confirms: usize,
    values: Vec<String>,
    measure: Rc<Cell<Bounds<Pixels>>>,
}
impl Render for Harness {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let open_owner = cx.entity().downgrade();
        let value_owner = open_owner.clone();
        let highlight_owner = open_owner.clone();
        let items = vec![
            CommandItem::new("a", "Alpha"),
            CommandItem::new("b", "Beta").disabled(true),
            CommandItem::new("c", "Gamma"),
        ];
        let control = match self.kind {
            Kind::Alert => {
                let measure = self.measure.clone();
                AlertDialog::new("alert", "Continue?")
                    .open(self.open)
                    .busy(self.busy)
                    .on_confirm(move |_, _, cx| {
                        value_owner
                            .update(cx, |this, _| {
                                this.confirms += 1;
                                true
                            })
                            .unwrap()
                    })
                    .on_open_change(move |open, _, _, cx| {
                        open_owner
                            .update(cx, |this, cx| {
                                this.open = open;
                                cx.notify();
                            })
                            .unwrap();
                    })
                    .child(
                        canvas(move |bounds, _, _| measure.set(bounds), |_, _, _, _| {})
                            .w_full()
                            .h(px(1.)),
                    )
                    .into_any_element()
            }
            Kind::Lightbox => Lightbox::new("lightbox")
                .open(self.open)
                .active(self.active)
                .items(vec![
                    LightboxItem::new(ImageSource::Custom(Arc::new(|_, _| None)), "First image"),
                    LightboxItem::new(ImageSource::Custom(Arc::new(|_, _| None)), "Second image"),
                ])
                .on_active_change(move |index, _, cx| {
                    value_owner
                        .update(cx, |this, cx| {
                            this.active = index;
                            cx.notify();
                        })
                        .unwrap();
                })
                .on_open_change(move |open, _, cx| {
                    open_owner
                        .update(cx, |this, cx| {
                            this.open = open;
                            cx.notify();
                        })
                        .unwrap();
                })
                .into_any_element(),
            Kind::Commands => CommandPalette::new("commands", &self.query)
                .open(self.open)
                .highlighted(self.highlighted)
                .items(items)
                .on_highlight(move |index, _, cx| {
                    highlight_owner
                        .update(cx, |this, cx| {
                            this.highlighted = index;
                            cx.notify();
                        })
                        .unwrap();
                })
                .on_select(move |value, _, cx| {
                    value_owner
                        .update(cx, |this, cx| {
                            this.values.push(value.to_string());
                            cx.notify();
                        })
                        .unwrap();
                })
                .on_open_change(move |open, _, cx| {
                    open_owner
                        .update(cx, |this, cx| {
                            this.open = open;
                            cx.notify();
                        })
                        .unwrap();
                })
                .into_any_element(),
            Kind::Search => PowerSearch::new("search", &self.query)
                .open(self.open)
                .loading(self.loading)
                .sections(if self.loading {
                    vec![SearchSection::new("Results", items)]
                } else {
                    vec![]
                })
                .on_query_submit(move |value, _, cx| {
                    value_owner
                        .update(cx, |this, cx| {
                            this.values.push(value.to_string());
                            cx.notify();
                        })
                        .unwrap();
                })
                .on_open_change(move |open, _, cx| {
                    open_owner
                        .update(cx, |this, cx| {
                            this.open = open;
                            cx.notify();
                        })
                        .unwrap();
                })
                .into_any_element(),
        };
        div()
            .id("advanced-owner")
            .size_full()
            .child(
                gpui_base::Button::new("open")
                    .track_focus(&self.trigger)
                    .w(px(120.))
                    .h(px(30.))
                    .child("Open")
                    .on_click(cx.listener(|this, _, _, cx| {
                        this.open = true;
                        cx.notify();
                    })),
            )
            .child(control)
    }
}
fn mount(cx: &mut TestAppContext, kind: Kind) -> (Entity<Harness>, &mut VisualTestContext) {
    cx.update(astryx_gpui::init);
    let (view, cx) = cx.add_window_view(move |window, cx| Harness {
        kind,
        open: false,
        busy: false,
        loading: false,
        trigger: cx.focus_handle(),
        query: cx.new(|cx| InputState::new(window, cx).placeholder("Search")),
        active: 0,
        highlighted: 0,
        confirms: 0,
        values: vec![],
        measure: Rc::new(Cell::new(Bounds::default())),
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
fn show(cx: &mut VisualTestContext) {
    cx.simulate_click(point(px(10.), px(10.)), Modifiers::default());
    draw(cx);
}

#[gpui::test]
fn alert_initial_enter_cancels_and_confirm_button_runs_once(cx: &mut TestAppContext) {
    let (view, cx) = mount(cx, Kind::Alert);
    show(cx);
    key(cx, "enter");
    assert_eq!(
        view.read_with(cx, |v, _| (v.open, v.confirms)),
        (false, 0),
        "initialfocusisCancel"
    );
    show(cx);
    let bounds = view.read_with(cx, |v, _| v.measure.get());
    cx.simulate_click(
        point(bounds.right() - px(30.), bounds.bottom() + px(32.)),
        Modifiers::default(),
    );
    draw(cx);
    assert_eq!(view.read_with(cx, |v, _| (v.open, v.confirms)), (false, 1));
    assert!(cx.update(|window, cx| view.read(cx).trigger.is_focused(window)));
}
#[gpui::test]
fn alert_busy_blocks_escape_and_confirmation(cx: &mut TestAppContext) {
    let (view, cx) = mount(cx, Kind::Alert);
    view.update(cx, |v, cx| {
        v.busy = true;
        cx.notify();
    });
    show(cx);
    key(cx, "escape");
    key(cx, "enter");
    assert_eq!(view.read_with(cx, |v, _| (v.open, v.confirms)), (true, 0));
}

#[gpui::test]
fn alert_becoming_busy_keeps_valid_modal_focus_and_returns_to_cancel(cx: &mut TestAppContext) {
    let (view, cx) = mount(cx, Kind::Alert);
    show(cx);
    view.update(cx, |v, cx| {
        v.busy = true;
        cx.notify();
    });
    draw(cx);
    assert!(
        cx.update(|window, cx| {
            let focused = window.focused(cx);
            let trap = gpui_base::active_focus_trap(window, cx);
            focused.is_some() && focused == trap
        }),
        "busybuttonsremovefocusnodes,somovetothemodalroot"
    );
    key(cx, "escape");
    assert!(view.read_with(cx, |v, _| v.open));
    view.update(cx, |v, cx| {
        v.busy = false;
        cx.notify();
    });
    draw(cx);
    key(cx, "enter");
    assert_eq!(view.read_with(cx, |v, _| (v.open, v.confirms)), (false, 0));
}
#[gpui::test]
fn lightbox_keyboard_wraps_and_escape_returns_to_trigger(cx: &mut TestAppContext) {
    let (view, cx) = mount(cx, Kind::Lightbox);
    show(cx);
    key(cx, "left");
    assert_eq!(view.read_with(cx, |v, _| v.active), 1);
    key(cx, "right");
    assert_eq!(view.read_with(cx, |v, _| v.active), 0);
    key(cx, "escape");
    assert!(!view.read_with(cx, |v, _| v.open));
    assert!(cx.update(|window, cx| view.read(cx).trigger.is_focused(window)));
}
#[gpui::test]
fn commands_filter_live_native_query_and_select_matching_item(cx: &mut TestAppContext) {
    let (view, cx) = mount(cx, Kind::Commands);
    show(cx);
    cx.simulate_input("ga");
    draw(cx);
    key(cx, "enter");
    assert_eq!(view.read_with(cx, |v, _| v.values.clone()), vec!["c"]);
    assert!(!view.read_with(cx, |v, _| v.open));
}
#[gpui::test]
fn commands_arrow_navigation_skips_disabled_items(cx: &mut TestAppContext) {
    let (view, cx) = mount(cx, Kind::Commands);
    show(cx);
    key(cx, "down");
    assert_eq!(view.read_with(cx, |v, _| v.highlighted), 2);
    key(cx, "enter");
    assert_eq!(view.read_with(cx, |v, _| v.values.clone()), vec!["c"]);
}
#[gpui::test]
fn composing_query_does_not_run_a_command_and_escape_cancels_ime_first(cx: &mut TestAppContext) {
    let (view, cx) = mount(cx, Kind::Commands);
    show(cx);
    let query = view.read_with(cx, |v, _| v.query.clone());
    cx.update(|window, cx| {
        query.update(cx, |state, cx| {
            state.replace_and_mark_text_in_range(None, "ga", Some(2..2), window, cx)
        })
    });
    key(cx, "enter");
    assert!(view.read_with(cx, |v, _| v.values.is_empty()));
    assert!(view.read_with(cx, |v, _| v.open));
    cx.update(|window, cx| {
        query.update(cx, |state, cx| {
            state.replace_and_mark_text_in_range(None, "ga", Some(2..2), window, cx)
        })
    });
    key(cx, "escape");
    assert!(view.read_with(cx, |v, _| v.open));
    key(cx, "escape");
    assert!(!view.read_with(cx, |v, _| v.open));
}
#[gpui::test]
fn provider_search_submits_query_but_loading_never_executes(cx: &mut TestAppContext) {
    let (view, cx) = mount(cx, Kind::Search);
    show(cx);
    cx.simulate_input("find me");
    draw(cx);
    key(cx, "enter");
    assert_eq!(view.read_with(cx, |v, _| v.values.clone()), vec!["find me"]);
    view.update(cx, |v, cx| {
        v.loading = true;
        cx.notify();
    });
    show(cx);
    key(cx, "enter");
    assert!(view.read_with(cx, |v, _| v.open));
    assert_eq!(view.read_with(cx, |v, _| v.values.len()), 1);
}
