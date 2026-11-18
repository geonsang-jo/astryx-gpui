use gpui::{
    AnyView, App, AppContext as _, Context, IntoElement, ParentElement, Refineable as _, Render,
    SharedString, StyleRefinement, Styled, Window, px,
};

use crate::Theme;

/// Astryx presentation for GPUI's native tooltip lifecycle.
///
/// Pass `Tooltip::text("Save")` or `Tooltip::with_theme("Save", theme)` to a
/// native element's `.tooltip(...)`. The host owns hover delay and dismissal.
pub struct Tooltip {
    label: SharedString,
    theme: Theme,
    style: StyleRefinement,
}

impl Tooltip {
    pub fn new(label: impl Into<SharedString>) -> Self {
        Self {
            label: label.into(),
            theme: Theme::default(),
            style: StyleRefinement::default(),
        }
    }

    pub fn theme(mut self, theme: Theme) -> Self {
        self.theme = theme;
        self
    }

    pub fn text(label: impl Into<SharedString>) -> impl Fn(&mut Window, &mut App) -> AnyView {
        Self::with_theme(label, Theme::default())
    }

    pub fn with_theme(
        label: impl Into<SharedString>,
        theme: Theme,
    ) -> impl Fn(&mut Window, &mut App) -> AnyView {
        let label = label.into();
        move |_, cx| cx.new(|_| Self::new(label.clone()).theme(theme)).into()
    }
}

impl Styled for Tooltip {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl Render for Tooltip {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        let colors = self.theme.colors;
        let mut surface = gpui_base::Tooltip::new("astryx-tooltip")
            .max_w(px(320.))
            .px(px(10.))
            .py(px(6.))
            .rounded(px(6.))
            .border_1()
            .border_color(colors.border_emphasized)
            .bg(colors.surface)
            .text_color(colors.text)
            .text_size(px(12.))
            .line_height(px(20.));
        surface.style().refine(&self.style);
        surface.child(self.label.clone())
    }
}
