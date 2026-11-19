use astryx_gpui::{
    Calendar, CalendarState, CheckboxList, Date, DateInput, DateRangeInput, InputState,
    NumberInput, RadioList, SelectOption, Slider, SliderState, TimeFieldState, TimeInput,
};
use chrono::{NaiveDate, NaiveTime};
use gpui::{
    AppContext, Context, Entity, Focusable as _, InteractiveElement as _, IntoElement,
    KeyDownEvent, KeyUpEvent, Keystroke, Modifiers, ParentElement, Render, Styled, TestAppContext,
    VisualTestContext, Window, div, point, px,
};

#[derive(Clone, Copy)]
enum Kind {
    Calendar,
    Number,
    Time,
    Radio,
    Checkboxes,
    Slider,
    Date,
    Range,
}
struct Harness {
    kind: Kind,
    calendar: Entity<CalendarState>,
    input: Entity<InputState>,
    time: Entity<TimeFieldState>,
    slider: Entity<SliderState>,
    selected: gpui::SharedString,
    many: Vec<gpui::SharedString>,
    changes: usize,
    disabled: bool,
    open: bool,
}
impl Render for Harness {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let value = cx.entity().downgrade();
        let owner = cx.entity().downgrade();
        let options = vec![
            SelectOption::new("a", "Alpha"),
            SelectOption::new("b", "Beta").disabled(true),
            SelectOption::new("c", "Gamma"),
        ];
        let control = match self.kind {
            Kind::Calendar => Calendar::new("cal", &self.calendar).into_any_element(),
            Kind::Number => NumberInput::new(&self.input).w(px(280.)).into_any_element(),
            Kind::Time => TimeInput::new("time", &self.time)
                .disabled(self.disabled)
                .into_any_element(),
            Kind::Radio => RadioList::new("radios")
                .items(options)
                .selected(self.selected.clone())
                .on_change(move |selected, _, cx| {
                    let _ = value.update(cx, |s, cx| {
                        s.selected = selected;
                        s.changes += 1;
                        cx.notify();
                    });
                })
                .into_any_element(),
            Kind::Checkboxes => CheckboxList::new("checks")
                .items(options)
                .selected(self.many.clone())
                .on_change(move |selected, _, cx| {
                    let _ = value.update(cx, |s, cx| {
                        s.many = selected;
                        s.changes += 1;
                        cx.notify();
                    });
                })
                .into_any_element(),
            Kind::Slider => Slider::new(&self.slider)
                .disabled(self.disabled)
                .into_any_element(),
            Kind::Date => DateInput::new("date", &self.calendar)
                .open(self.open)
                .disabled(self.disabled)
                .on_open_change(move |open, _, cx| {
                    let _ = owner.update(cx, |s, cx| {
                        s.open = open;
                        cx.notify();
                    });
                })
                .into_any_element(),
            Kind::Range => DateRangeInput::new("range", &self.calendar)
                .open(self.open)
                .on_open_change(move |open, _, cx| {
                    let _ = owner.update(cx, |s, cx| {
                        s.open = open;
                        cx.notify();
                    });
                })
                .into_any_element(),
        };
        div()
            .id("harness")
            .p(px(10.))
            .w(px(340.))
            .h(px(400.))
            .child(control)
    }
}
fn mount(
    cx: &mut TestAppContext,
    kind: Kind,
    disabled: bool,
) -> (Entity<Harness>, &mut VisualTestContext) {
    cx.update(astryx_gpui::init);
    let (view, cx) = cx.add_window_view(move |window, cx| {
        let calendar = cx.new(|cx| CalendarState::new(window, cx));
        calendar.update(cx, |state, cx| {
            state.set_date(
                if matches!(kind, Kind::Range) {
                    Date::Range(None, None)
                } else {
                    Date::Single(Some(NaiveDate::from_ymd_opt(2026, 11, 18).unwrap()))
                },
                window,
                cx,
            );
        });
        let time = cx.new(|cx| TimeFieldState::new(window, cx));
        time.update(cx, |state, cx| {
            state.set_time(NaiveTime::from_hms_opt(9, 30, 0).unwrap(), window, cx)
        });
        Harness {
            kind,
            calendar,
            input: cx.new(|cx| {
                InputState::new(window, cx)
                    .default_value("1")
                    .step(0.5)
                    .min(0.)
                    .max(2.)
            }),
            time,
            slider: cx.new(|_| SliderState::new().min(0.).max(10.).default_value(5.)),
            selected: "a".into(),
            many: vec![],
            changes: 0,
            disabled,
            open: false,
        }
    });
    cx.update(|w, cx| w.draw(cx).clear(cx));
    (view, cx)
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
fn calendar_pointer_and_arrow_enter_select_real_dates(cx: &mut TestAppContext) {
    let (v, cx) = mount(cx, Kind::Calendar, false);
    cx.simulate_click(point(px(35.), px(100.)), Modifiers::default());
    cx.update(|w, cx| w.draw(cx).clear(cx));
    let date = v.read_with(cx, |s, cx| s.calendar.read(cx).date());
    assert_eq!(date.start(), NaiveDate::from_ymd_opt(2026, 11, 1));
    key(cx, "right");
    key(cx, "enter");
    assert_eq!(
        v.read_with(cx, |s, cx| s.calendar.read(cx).date().start()),
        NaiveDate::from_ymd_opt(2026, 11, 2)
    );
}
#[gpui::test]
fn calendar_disabled_matcher_rejects_click(cx: &mut TestAppContext) {
    let (v, cx) = mount(cx, Kind::Calendar, false);
    let state = v.read_with(cx, |s, _| s.calendar.clone());
    cx.update(|w, cx| {
        state.update(cx, |s, cx| {
            s.set_disabled_matcher(
                |date: &NaiveDate| *date == NaiveDate::from_ymd_opt(2026, 11, 1).unwrap(),
                w,
                cx,
            )
        })
    });
    v.update(cx, |_, cx| cx.notify());
    cx.update(|w, cx| w.draw(cx).clear(cx));
    cx.simulate_click(point(px(35.), px(100.)), Modifiers::default());
    assert_eq!(
        state.read_with(cx, |s, _| s.date().start()),
        NaiveDate::from_ymd_opt(2026, 11, 18)
    );
}

#[gpui::test]
fn calendar_vertical_navigation_counts_grid_days_not_enabled_items(cx: &mut TestAppContext) {
    let (v, cx) = mount(cx, Kind::Calendar, false);
    let state = v.read_with(cx, |s, _| s.calendar.clone());
    cx.update(|w, cx| {
        state.update(cx, |s, cx| {
            s.set_disabled_matcher(
                |date: &NaiveDate| *date == NaiveDate::from_ymd_opt(2026, 11, 11).unwrap(),
                w,
                cx,
            )
        })
    });
    v.update(cx, |_, cx| cx.notify());
    cx.update(|w, cx| w.draw(cx).clear(cx));
    cx.simulate_click(point(px(70.), px(135.)), Modifiers::default());
    assert_eq!(
        state.read_with(cx, |s, _| s.date().start()),
        NaiveDate::from_ymd_opt(2026, 11, 9)
    );
    key(cx, "down");
    key(cx, "enter");
    assert_eq!(
        state.read_with(cx, |s, _| s.date().start()),
        NaiveDate::from_ymd_opt(2026, 11, 16)
    );
}
#[gpui::test]
fn number_buttons_step_and_respect_bounds(cx: &mut TestAppContext) {
    let (v, cx) = mount(cx, Kind::Number, false);
    for _ in 0..4 {
        cx.simulate_click(point(px(275.), px(25.)), Modifiers::default());
        cx.update(|w, cx| w.draw(cx).clear(cx));
    }
    assert_eq!(
        v.read_with(cx, |s, cx| s.input.read(cx).value().parse::<f64>().unwrap()),
        2.
    );
}
#[gpui::test]
fn radio_arrow_skips_disabled_and_controls_owner(cx: &mut TestAppContext) {
    let (v, cx) = mount(cx, Kind::Radio, false);
    cx.simulate_click(point(px(20.), px(20.)), Modifiers::default());
    key(cx, "down");
    assert_eq!(
        v.read_with(cx, |s, _| (s.selected.to_string(), s.changes)),
        ("c".into(), 1)
    );
}
#[gpui::test]
fn checkbox_list_changes_only_the_enabled_clicked_value(cx: &mut TestAppContext) {
    let (v, cx) = mount(cx, Kind::Checkboxes, false);
    cx.simulate_click(point(px(20.), px(20.)), Modifiers::default());
    cx.update(|w, cx| w.draw(cx).clear(cx));
    assert_eq!(
        v.read_with(cx, |s, _| s.many.clone()),
        vec![gpui::SharedString::from("a")]
    );
    cx.simulate_click(point(px(20.), px(50.)), Modifiers::default());
    assert_eq!(v.read_with(cx, |s, _| s.changes), 1);
}
#[gpui::test]
fn time_keyboard_edits_and_disabled_blocks_editing(cx: &mut TestAppContext) {
    let (v, cx) = mount(cx, Kind::Time, false);
    let state = v.read_with(cx, |s, _| s.time.clone());
    cx.update(|w, cx| {
        let handle = state.read(cx).focus_handle(cx);
        handle.focus(w, cx);
    });
    key(cx, "up");
    assert_eq!(
        state.read_with(cx, |s, _| s.time()),
        NaiveTime::from_hms_opt(10, 30, 0).unwrap()
    );
    v.update(cx, |s, cx| {
        s.disabled = true;
        cx.notify();
    });
    cx.update(|w, cx| w.draw(cx).clear(cx));
    key(cx, "up");
    assert_eq!(
        state.read_with(cx, |s, _| s.time()),
        NaiveTime::from_hms_opt(10, 30, 0).unwrap()
    );
}
#[gpui::test]
fn slider_pointer_maps_the_full_track_and_disabled_blocks_changes(cx: &mut TestAppContext) {
    let (v, cx) = mount(cx, Kind::Slider, false);
    cx.simulate_click(point(px(235.), px(25.)), Modifiers::default());
    let value = v.read_with(cx, |s, cx| s.slider.read(cx).value().end());
    assert!((value - 7.).abs() <= 1., "actualvalue{value}");
    v.update(cx, |s, cx| {
        s.disabled = true;
        cx.notify();
    });
    cx.update(|w, cx| w.draw(cx).clear(cx));
    cx.simulate_click(point(px(50.), px(25.)), Modifiers::default());
    assert_eq!(
        v.read_with(cx, |s, cx| s.slider.read(cx).value().end()),
        value
    );
}

#[gpui::test]
fn range_slider_exposes_two_independently_adjustable_keyboard_thumbs(cx: &mut TestAppContext) {
    let (v, cx) = mount(cx, Kind::Slider, false);
    let state = v.read_with(cx, |s, _| s.slider.clone());
    cx.update(|w, cx| state.update(cx, |s, cx| s.set_value((2., 8.), w, cx)));
    v.update(cx, |_, cx| cx.notify());
    cx.update(|w, cx| w.draw(cx).clear(cx));
    cx.update(|w, cx| w.focus_next(cx));
    key(cx, "right");
    assert_eq!(
        state.read_with(cx, |s, _| s.value()),
        astryx_gpui::SliderValue::Range(3., 8.)
    );
    cx.update(|w, cx| w.focus_next(cx));
    key(cx, "left");
    assert_eq!(
        state.read_with(cx, |s, _| s.value()),
        astryx_gpui::SliderValue::Range(3., 7.)
    );
}
#[gpui::test]
fn date_popup_requests_open_escape_close_without_mutating_date(cx: &mut TestAppContext) {
    let (v, cx) = mount(cx, Kind::Date, false);
    cx.simulate_click(point(px(40.), px(25.)), Modifiers::default());
    assert!(v.read_with(cx, |s, _| s.open));
    key(cx, "escape");
    assert!(!v.read_with(cx, |s, _| s.open));
    assert_eq!(
        v.read_with(cx, |s, cx| s.calendar.read(cx).date().start()),
        NaiveDate::from_ymd_opt(2026, 11, 18)
    );
}
#[gpui::test]
fn range_popup_retains_native_two_step_range_mode(cx: &mut TestAppContext) {
    let (v, cx) = mount(cx, Kind::Range, false);
    cx.simulate_click(point(px(40.), px(25.)), Modifiers::default());
    assert!(v.read_with(cx, |s, _| s.open));
    assert!(v.read_with(cx, |s, cx| matches!(
        s.calendar.read(cx).date(),
        Date::Range(..)
    )));
    key(cx, "escape");
    assert!(!v.read_with(cx, |s, _| s.open));
}

#[gpui::test]
fn date_popup_programmatic_disable_releases_its_content_focus(cx: &mut TestAppContext) {
    let (v, cx) = mount(cx, Kind::Date, false);
    cx.simulate_click(point(px(40.), px(25.)), Modifiers::default());
    cx.update(|w, cx| w.draw(cx).clear(cx));
    let focus = cx.update(|w, cx| w.focused(cx).unwrap());
    v.update(cx, |s, cx| {
        s.disabled = true;
        cx.notify();
    });
    cx.update(|w, cx| w.draw(cx).clear(cx));
    assert!(
        !cx.update(|w, _| focus.is_focused(w)),
        "disabledclosedpopupmustnotretainfocus"
    );
}
