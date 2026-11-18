use gpui::{
    App, IntoElement, Refineable as _, RenderOnce, StyleRefinement, Styled, Window, div, px,
};

use crate::Theme;

/// A horizontal, one-pixel separator that consumes no focus or input.
#[derive(IntoElement)]
pub struct Divider {
    theme: Theme,
    style: StyleRefinement,
}

impl Divider {
    pub fn new() -> Self {
        Self {
            theme: Theme::default(),
            style: StyleRefinement::default(),
        }
    }

    pub fn theme(mut self, theme: Theme) -> Self {
        self.theme = theme;
        self
    }
}

impl Default for Divider {
    fn default() -> Self {
        Self::new()
    }
}

impl Styled for Divider {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for Divider {
    fn render(self, _: &mut Window, _: &mut App) -> impl IntoElement {
        let mut element = div()
            .flex_none()
            .w_full()
            .h(px(1.))
            .bg(self.theme.colors.border);
        element.style().refine(&self.style);
        element
    }
}
