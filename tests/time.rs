use astryx_gpui::{Button, Timer, TimerState};
use gpui::{
    AppContext, Context, Entity, IntoElement, KeyDownEvent, KeyUpEvent, Keystroke, ParentElement,
    Render, Subscription, TestAppContext, Window, div,
};
use std::time::Duration;

struct ClockHarness {
    timer: Entity<TimerState>,
    notifications: usize,
    _watch: Subscription,
}
impl Render for ClockHarness {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        let timer = self.timer.clone();
        div().child(Timer::new("elapsed", &self.timer)).child(
            Button::new("toggle", "Start or pause").on_click(move |_, _, cx| {
                timer.update(cx, |state, cx| {
                    if state.is_running() {
                        state.pause(cx);
                    } else {
                        state.start(cx);
                    }
                });
            }),
        )
    }
}

#[gpui::test]
fn timer_starts_from_native_action_ticks_and_cancels_on_pause(cx: &mut TestAppContext) {
    cx.update(astryx_gpui::init);
    let (view, cx) = cx.add_window_view(|_, cx| {
        let timer = cx.new(|_| TimerState::new());
        let watch = cx.observe(&timer, |this: &mut ClockHarness, _, cx| {
            this.notifications += 1;
            cx.notify();
        });
        ClockHarness {
            timer,
            notifications: 0,
            _watch: watch,
        }
    });
    cx.update(|window, cx| {
        window.draw(cx).clear(cx);
        window.focus_next(cx);
    });
    let enter = Keystroke::parse("enter").unwrap();
    cx.simulate_event(KeyDownEvent {
        keystroke: enter.clone(),
        is_held: false,
        prefer_character_input: false,
    });
    cx.simulate_event(KeyUpEvent { keystroke: enter });
    cx.run_until_parked();
    let timer = view.read_with(cx, |this, _| this.timer.clone());
    assert!(timer.read_with(cx, |state, _| state.is_running()));
    let after_start = view.read_with(cx, |this, _| this.notifications);
    cx.executor().advance_clock(Duration::from_millis(1100));
    cx.run_until_parked();
    assert!(view.read_with(cx, |this, _| this.notifications) > after_start);

    timer.update(cx, |state, cx| state.pause(cx));
    cx.run_until_parked();
    let after_pause = view.read_with(cx, |this, _| this.notifications);
    let paused_elapsed = timer.read_with(cx, |state, _| state.elapsed());
    cx.executor().advance_clock(Duration::from_secs(3));
    cx.run_until_parked();
    assert!(!timer.read_with(cx, |state, _| state.is_running()));
    assert_eq!(
        timer.read_with(cx, |state, _| state.elapsed()),
        paused_elapsed
    );
    assert_eq!(
        view.read_with(cx, |this, _| this.notifications),
        after_pause
    );
    timer.update(cx, |state, cx| state.reset(cx));
    assert_eq!(
        timer.read_with(cx, |state, _| state.elapsed()),
        Duration::ZERO
    );
}
