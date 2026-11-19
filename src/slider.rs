use crate::Theme;
use gpui::{
    App, Entity, FocusHandle, InteractiveElement as _, IntoElement, MouseButton, ParentElement,
    RenderOnce, Role, SharedString, StatefulInteractiveElement as _, StyleRefinement, Styled,
    Window, div, prelude::FluentBuilder as _, px, relative,
};
use gpui_base::StyledExt as _;
pub use gpui_base::slider::{SliderEvent, SliderScale, SliderState, SliderValue};

/// Native pointer/touch dragging and independently focusable single/range thumbs.
/// The host owns and observes SliderState; every thumb supports arrows, Home/End,
/// PageUp/PageDown and accessible increment/decrement within its own bounds.
#[derive(IntoElement)]
pub struct Slider {
    state: Entity<SliderState>,
    theme: Theme,
    disabled: bool,
    label: SharedString,
    style: StyleRefinement,
}
impl Slider {
    pub fn new(state: &Entity<SliderState>) -> Self {
        Self {
            state: state.clone(),
            theme: Theme::default(),
            disabled: false,
            label: "Value".into(),
            style: StyleRefinement::default(),
        }
    }
    pub fn theme(mut self, value: Theme) -> Self {
        self.theme = value;
        self
    }
    pub fn disabled(mut self, value: bool) -> Self {
        self.disabled = value;
        self
    }
    pub fn label(mut self, value: impl Into<SharedString>) -> Self {
        self.label = value.into();
        self
    }
}
impl Styled for Slider {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

fn adjust(
    state: &Entity<SliderState>,
    start: bool,
    key: &str,
    window: &mut Window,
    cx: &mut App,
) -> bool {
    if ![
        "left", "right", "up", "down", "home", "end", "pageup", "pagedown",
    ]
    .contains(&key)
    {
        return false;
    }
    state.update(cx, |state, cx| {
        let current = state.value();
        let value = if start {
            current.start()
        } else {
            current.end()
        };
        let minimum = if current.is_range() && !start {
            current.start()
        } else {
            state.min_value()
        };
        let maximum = if current.is_range() && start {
            current.end()
        } else {
            state.max_value()
        };
        let next = match key {
            "left" | "down" => value - state.step_value(),
            "right" | "up" => value + state.step_value(),
            "pageup" => value + 10. * state.step_value(),
            "pagedown" => value - 10. * state.step_value(),
            "home" => minimum,
            _ => maximum,
        }
        .clamp(minimum, maximum);
        let next = match current {
            SliderValue::Range(a, b) => {
                if start {
                    SliderValue::Range(next, b)
                } else {
                    SliderValue::Range(a, next)
                }
            }
            _ => SliderValue::Single(next),
        };
        state.set_value(next, window, cx);
    });
    true
}

fn thumb(
    state: &Entity<SliderState>,
    start: bool,
    handle: &FocusHandle,
    label: SharedString,
    disabled: bool,
    theme: Theme,
    cx: &App,
) -> gpui_base::SliderThumb {
    let c = theme.colors;
    let model = state.read(cx);
    let value = model.value();
    let percentage = model.percentage();
    let current = if start { value.start() } else { value.end() };
    let min = if value.is_range() && !start {
        value.start()
    } else {
        model.min_value()
    };
    let max = if value.is_range() && start {
        value.end()
    } else {
        model.max_value()
    };
    let mut result = gpui_base::SliderThumb::new(state)
        .start(start)
        .disabled(disabled)
        .absolute()
        .top(px(-6.))
        .left(relative(if start {
            percentage.start
        } else {
            percentage.end
        }))
        .ml(px(-8.))
        .size(px(16.))
        .rounded_full()
        .bg(c.surface)
        .border_1()
        .border_color(c.accent)
        .role(Role::Slider)
        .aria_label(if disabled {
            format!("{label} (unavailable)")
        } else {
            label.to_string()
        })
        .aria_numeric_value(current as f64)
        .aria_min_numeric_value(min as f64)
        .aria_max_numeric_value(max as f64)
        .aria_numeric_value_step(model.step_value() as f64);
    if !disabled {
        let key_state = state.clone();
        let increment = state.clone();
        let decrement = state.clone();
        let clicked = handle.clone();
        result = result
            .track_focus(&handle.clone().tab_stop(true))
            .focus_visible(move |style| style.border_2().border_color(c.focus))
            .on_mouse_down(MouseButton::Left, move |_, window, cx| {
                clicked.focus(window, cx)
            })
            .on_key_down(move |event, window, cx| {
                if event.keystroke.modifiers == gpui::Modifiers::default()
                    && adjust(&key_state, start, &event.keystroke.key, window, cx)
                {
                    cx.stop_propagation();
                }
            })
            .on_a11y_action(gpui::AccessibleAction::Increment, move |_, w, cx| {
                adjust(&increment, start, "right", w, cx);
            })
            .on_a11y_action(gpui::AccessibleAction::Decrement, move |_, w, cx| {
                adjust(&decrement, start, "left", w, cx);
            });
    }
    result
}

impl RenderOnce for Slider {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let c = self.theme.colors;
        let id = self.state.entity_id();
        let focus = window.use_keyed_state(("slider-focus", id), cx, |_, cx| {
            (cx.focus_handle(), cx.focus_handle())
        });
        let (start_focus, end_focus) = focus.read(cx).clone();
        let model = self.state.read(cx);
        let range = model.value().is_range();
        let percentage = model.percentage();
        let mut track = gpui_base::SliderTrack::new(&self.state)
            .disabled(self.disabled)
            .relative()
            .h(px(4.))
            .w_full()
            .rounded_full()
            .bg(c.border_emphasized)
            .child(
                gpui_base::SliderIndicator::new(&self.state)
                    .h_full()
                    .w_full()
                    .relative()
                    .child(
                        div()
                            .absolute()
                            .h_full()
                            .left(relative(percentage.start))
                            .w(relative(percentage.end - percentage.start))
                            .rounded_full()
                            .bg(c.accent),
                    ),
            );
        if range {
            track = track.child(thumb(
                &self.state,
                true,
                &start_focus,
                format!("{} minimum", self.label).into(),
                self.disabled,
                self.theme,
                cx,
            ));
        }
        track = track.child(thumb(
            &self.state,
            false,
            &end_focus,
            if range {
                format!("{} maximum", self.label).into()
            } else {
                self.label
            },
            self.disabled,
            self.theme,
            cx,
        ));
        let state = self.state.clone();
        let released = self.state.clone();
        let clicked_state = self.state.clone();
        div()
            .id(("astryx-slider", id))
            .role(Role::Group)
            .h(px(32.))
            .w_full()
            .flex()
            .items_center()
            .px(px(8.))
            .when(self.disabled, |el| el.opacity(0.5))
            .when(!self.disabled, |el| {
                el.on_mouse_down(MouseButton::Left, move |event, window, cx| {
                    let state = clicked_state.read(cx);
                    let percentage = state.percentage();
                    let bounds = state.bounds();
                    let position = (event.position.x - bounds.left()) / bounds.size.width;
                    if range
                        && (position - percentage.start).abs() < (position - percentage.end).abs()
                    {
                        start_focus.focus(window, cx);
                    } else {
                        end_focus.focus(window, cx);
                    }
                })
                .on_mouse_up(MouseButton::Left, move |_, _, cx| {
                    state.update(cx, |s, cx| s.handle_release(cx))
                })
                .on_mouse_up_out(MouseButton::Left, move |_, _, cx| {
                    released.update(cx, |s, cx| s.handle_release(cx))
                })
            })
            .child(track)
            .refine_style(&self.style)
    }
}
