use crate::{Input, InputState, Theme};
use gpui::{
    App, Entity, InteractiveElement as _, IntoElement, ParentElement, RenderOnce, StyleRefinement,
    Styled, Window, px,
};
use gpui_base::StyledExt as _;
pub use gpui_base::{NumberInputEvent, NumberStep, StepAction};

/// Native numeric editing and bounded stepping over a caller-owned InputState.
/// Configure `step`, `min` and `max` on that state; subscribe to its InputEvent
/// updates or NumberInputEvent when using a caller-controlled stepping strategy.
#[derive(IntoElement)]
pub struct NumberInput {
    state: Entity<InputState>,
    theme: Theme,
    style: StyleRefinement,
}
impl NumberInput {
    pub fn new(state: &Entity<InputState>) -> Self {
        Self {
            state: state.clone(),
            theme: Theme::default(),
            style: StyleRefinement::default(),
        }
    }
    pub fn theme(mut self, theme: Theme) -> Self {
        self.theme = theme;
        self
    }
}
impl Styled for NumberInput {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}
impl RenderOnce for NumberInput {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let c = self.theme.colors;
        let disabled = !self.state.read(cx).is_editable();
        gpui_base::NumberInput::new(&self.state)
            .disabled(disabled)
            .h(px(36.))
            .rounded(px(10.))
            .border_1()
            .border_color(c.border_emphasized)
            .bg(c.surface)
            .text_color(c.text)
            .text_size(px(14.))
            .line_height(px(20.))
            .decrement_button(move |b| {
                b.w(px(32.))
                    .h_full()
                    .text_color(c.muted)
                    .hover(move |style| style.bg(c.hover))
                    .child("−")
            })
            .increment_button(move |b| {
                b.w(px(32.))
                    .h_full()
                    .text_color(c.muted)
                    .hover(move |style| style.bg(c.hover))
                    .child("+")
            })
            .input(
                Input::new(&self.state)
                    .theme(self.theme)
                    .border_0()
                    .rounded(px(0.))
                    .h_full(),
            )
            .refine_style(&self.style)
    }
}
