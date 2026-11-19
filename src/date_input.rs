use crate::{Calendar, CalendarState, Date, Popover, Theme, TimeFieldState, TimeInput};
use gpui::{
    App, ElementId, Entity, IntoElement, ParentElement, RenderOnce, StyleRefinement, Styled,
    Window, div, px,
};
use gpui_base::StyledExt as _;
use std::rc::Rc;
type OpenChange = Rc<dyn Fn(bool, &mut Window, &mut App)>;

/// An owner-controlled date popup. CalendarState owns its selected date; the
/// `open` value is controlled with on_open_change. A complete choice requests
/// dismissal; Escape/outside cancellation does not change the selected date.
#[derive(IntoElement)]
pub struct DateInput {
    id: ElementId,
    state: Entity<CalendarState>,
    theme: Theme,
    open: bool,
    disabled: bool,
    range: bool,
    change: Option<OpenChange>,
    style: StyleRefinement,
}
impl DateInput {
    pub fn new(id: impl Into<ElementId>, state: &Entity<CalendarState>) -> Self {
        Self {
            id: id.into(),
            state: state.clone(),
            theme: Theme::default(),
            open: false,
            disabled: false,
            range: false,
            change: None,
            style: StyleRefinement::default(),
        }
    }
    pub fn theme(mut self, theme: Theme) -> Self {
        self.theme = theme;
        self
    }
    pub fn open(mut self, open: bool) -> Self {
        self.open = open;
        self
    }
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
    pub fn on_open_change(mut self, f: impl Fn(bool, &mut Window, &mut App) + 'static) -> Self {
        self.change = Some(Rc::new(f));
        self
    }
}
impl Styled for DateInput {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}
impl RenderOnce for DateInput {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let c = self.theme.colors;
        let date = self.state.read(cx).date();
        assert_eq!(
            matches!(date, Date::Range(..)),
            self.range,
            "initialize CalendarState with the date input's single/range mode"
        );
        let label = date
            .format("%Y-%m-%d")
            .map(|v| v.to_string())
            .unwrap_or_else(|| {
                if self.range {
                    "Start date → End date".into()
                } else {
                    "Choose date".into()
                }
            });
        let trigger = gpui_base::Button::new((self.id.clone(), "trigger"))
            .disabled(self.disabled)
            .h(px(36.))
            .w_full()
            .px(px(12.))
            .rounded(px(10.))
            .border_1()
            .border_color(c.border_emphasized)
            .bg(c.surface)
            .text_color(c.text)
            .text_size(px(14.))
            .line_height(px(20.))
            .accessibility_label(label.clone())
            .child(label);
        let id = self.id.clone();
        let theme = self.theme;
        let state = self.state;
        let close = self.change.clone();
        let changed = self.change;
        Popover::new(self.id)
            .theme(theme)
            .open(self.open)
            .disabled(self.disabled)
            .trigger(trigger)
            .on_open_change(move |open, window, cx| {
                if let Some(f) = &changed {
                    f(open, window, cx);
                }
            })
            .content(move |_, _, _| {
                Calendar::new((id, "calendar"), &state)
                    .theme(theme)
                    .p(px(0.))
                    .on_change(move |date, window, cx| {
                        if date.is_complete()
                            && let Some(f) = &close
                        {
                            f(false, window, cx);
                        }
                    })
            })
            .refine_style(&self.style)
            .into_any_element()
    }
}

/// A range picker. Initialize the supplied state with Date::Range(None,None).
#[derive(IntoElement)]
pub struct DateRangeInput {
    inner: DateInput,
}
impl DateRangeInput {
    pub fn new(id: impl Into<ElementId>, state: &Entity<CalendarState>) -> Self {
        let mut inner = DateInput::new(id, state);
        inner.range = true;
        Self { inner }
    }
    pub fn theme(mut self, theme: Theme) -> Self {
        self.inner = self.inner.theme(theme);
        self
    }
    pub fn open(mut self, open: bool) -> Self {
        self.inner = self.inner.open(open);
        self
    }
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.inner = self.inner.disabled(disabled);
        self
    }
    pub fn on_open_change(mut self, f: impl Fn(bool, &mut Window, &mut App) + 'static) -> Self {
        self.inner = self.inner.on_open_change(f);
        self
    }
}
impl Styled for DateRangeInput {
    fn style(&mut self) -> &mut StyleRefinement {
        self.inner.style()
    }
}
impl RenderOnce for DateRangeInput {
    fn render(self, _: &mut Window, _: &mut App) -> impl IntoElement {
        self.inner
    }
}

/// Date and time controls sharing the same visual rhythm; their native states
/// remain separate so the host owns timezone and combined-value policy.
#[derive(IntoElement)]
pub struct DateTimeInput {
    id: ElementId,
    date: Entity<CalendarState>,
    time: Entity<TimeFieldState>,
    theme: Theme,
    open: bool,
    disabled: bool,
    change: Option<OpenChange>,
    style: StyleRefinement,
}
impl DateTimeInput {
    pub fn new(
        id: impl Into<ElementId>,
        date: &Entity<CalendarState>,
        time: &Entity<TimeFieldState>,
    ) -> Self {
        Self {
            id: id.into(),
            date: date.clone(),
            time: time.clone(),
            theme: Theme::default(),
            open: false,
            disabled: false,
            change: None,
            style: StyleRefinement::default(),
        }
    }
    pub fn theme(mut self, theme: Theme) -> Self {
        self.theme = theme;
        self
    }
    pub fn open(mut self, open: bool) -> Self {
        self.open = open;
        self
    }
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
    pub fn on_open_change(mut self, f: impl Fn(bool, &mut Window, &mut App) + 'static) -> Self {
        self.change = Some(Rc::new(f));
        self
    }
}
impl Styled for DateTimeInput {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}
impl RenderOnce for DateTimeInput {
    fn render(self, _: &mut Window, _: &mut App) -> impl IntoElement {
        let callback = self.change;
        div()
            .flex()
            .gap(px(8.))
            .child(
                DateInput::new((self.id.clone(), "date"), &self.date)
                    .theme(self.theme)
                    .open(self.open)
                    .disabled(self.disabled)
                    .on_open_change(move |open, w, cx| {
                        if let Some(f) = &callback {
                            f(open, w, cx);
                        }
                    })
                    .flex_1(),
            )
            .child(
                TimeInput::new((self.id, "time"), &self.time)
                    .theme(self.theme)
                    .disabled(self.disabled),
            )
            .refine_style(&self.style)
    }
}
