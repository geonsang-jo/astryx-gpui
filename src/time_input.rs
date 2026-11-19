use crate::Theme;
use gpui::{
    App, ElementId, Entity, IntoElement, RenderOnce, StyleRefinement, Styled, Window,
    prelude::FluentBuilder as _, px,
};
use gpui_base::StyledExt as _;
pub use gpui_base::{HourCycle, TimeFieldEvent, TimeFieldState, TimePrecision};

/// Native segmented time entry with Arrow, digit, Tab and AM/PM editing.
#[derive(IntoElement)]
pub struct TimeInput {
    id: ElementId,
    state: Entity<TimeFieldState>,
    theme: Theme,
    disabled: bool,
    style: StyleRefinement,
}
impl TimeInput {
    pub fn new(id: impl Into<ElementId>, state: &Entity<TimeFieldState>) -> Self {
        Self {
            id: id.into(),
            state: state.clone(),
            theme: Theme::default(),
            disabled: false,
            style: StyleRefinement::default(),
        }
    }
    pub fn theme(mut self, theme: Theme) -> Self {
        self.theme = theme;
        self
    }
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}
impl Styled for TimeInput {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}
impl RenderOnce for TimeInput {
    fn render(self, _: &mut Window, _: &mut App) -> impl IntoElement {
        let c = self.theme.colors;
        gpui_base::TimeField::new(self.id, &self.state)
            .disabled(self.disabled)
            .flex()
            .items_center()
            .h(px(36.))
            .px(px(10.))
            .rounded(px(10.))
            .border_1()
            .border_color(c.border_emphasized)
            .bg(c.surface)
            .text_color(c.text)
            .text_size(px(14.))
            .line_height(px(20.))
            .when(self.disabled, |el| el.opacity(0.5))
            .render_segment(move |segment, state, _, _| {
                segment
                    .px(px(3.))
                    .py(px(2.))
                    .rounded(px(4.))
                    .bg(if state.is_selected() {
                        c.neutral
                    } else {
                        gpui::transparent_black()
                    })
                    .into_any_element()
            })
            .refine_style(&self.style)
    }
}
