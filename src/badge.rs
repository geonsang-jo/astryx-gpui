use gpui::{
    App, FontWeight, IntoElement, ParentElement, Refineable as _, RenderOnce, SharedString,
    StyleRefinement, Styled, Window, div, px,
};

use crate::theme::{INNER_RADIUS, SUPPORT_LINE_HEIGHT, SUPPORT_SIZE, Theme};

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum Tone {
    #[default]
    Neutral,
    Success,
    Warning,
    Danger,
    Info,
}

/// A noninteractive status label using Neutral's named color variants.
#[derive(IntoElement)]
pub struct Badge {
    label: SharedString,
    tone: Tone,
    theme: Theme,
    style: StyleRefinement,
}

impl Badge {
    pub fn new(label: impl Into<SharedString>) -> Self {
        Self {
            label: label.into(),
            tone: Tone::Neutral,
            theme: Theme::default(),
            style: StyleRefinement::default(),
        }
    }

    pub fn tone(mut self, tone: Tone) -> Self {
        self.tone = tone;
        self
    }

    pub fn theme(mut self, theme: Theme) -> Self {
        self.theme = theme;
        self
    }
}

impl Styled for Badge {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for Badge {
    fn render(self, _: &mut Window, _: &mut App) -> impl IntoElement {
        let c = self.theme.colors;
        let (background, foreground) = match self.tone {
            Tone::Neutral => (c.neutral_background, c.neutral_foreground),
            Tone::Success => (c.success_background, c.success_foreground),
            Tone::Warning => (c.warning_background, c.warning_foreground),
            Tone::Danger => (c.danger_background, c.danger_foreground),
            Tone::Info => (c.info_background, c.info_foreground),
        };
        let mut element = div()
            .flex()
            .flex_none()
            .items_center()
            .h(px(24.))
            .px(px(6.))
            .rounded(px(INNER_RADIUS))
            .bg(background)
            .text_color(foreground)
            .text_size(px(SUPPORT_SIZE))
            .line_height(px(SUPPORT_LINE_HEIGHT))
            .font_weight(FontWeight::MEDIUM);
        element.style().refine(&self.style);
        element.child(self.label)
    }
}
