use gpui::{
    App, FontWeight, IntoElement, ParentElement, Refineable as _, RenderOnce, SharedString,
    StyleRefinement, Styled, Window, div, px,
};

use crate::theme::{
    BODY_LINE_HEIGHT, BODY_SIZE, HEADING_LINE_HEIGHT, HEADING_SIZE, SUPPORT_LINE_HEIGHT,
    SUPPORT_SIZE, Theme,
};

/// The initial port's three desktop text roles.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum TextKind {
    #[default]
    Body,
    Supporting,
    Heading,
}

#[derive(IntoElement)]
pub struct Text {
    label: SharedString,
    kind: TextKind,
    muted: bool,
    theme: Theme,
    style: StyleRefinement,
}

impl Text {
    pub fn new(label: impl Into<SharedString>) -> Self {
        Self {
            label: label.into(),
            kind: TextKind::Body,
            muted: false,
            theme: Theme::default(),
            style: StyleRefinement::default(),
        }
    }

    pub fn kind(mut self, kind: TextKind) -> Self {
        self.kind = kind;
        self
    }

    pub fn muted(mut self) -> Self {
        self.muted = true;
        self
    }

    pub fn theme(mut self, theme: Theme) -> Self {
        self.theme = theme;
        self
    }
}

impl Styled for Text {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for Text {
    fn render(self, _: &mut Window, _: &mut App) -> impl IntoElement {
        let (font_size, line_height, weight) = match self.kind {
            TextKind::Body => (BODY_SIZE, BODY_LINE_HEIGHT, FontWeight::NORMAL),
            TextKind::Supporting => (SUPPORT_SIZE, SUPPORT_LINE_HEIGHT, FontWeight::NORMAL),
            TextKind::Heading => (HEADING_SIZE, HEADING_LINE_HEIGHT, FontWeight::SEMIBOLD),
        };
        let mut element = div()
            .text_size(px(font_size))
            .line_height(px(line_height))
            .font_weight(weight)
            .text_color(if self.muted {
                self.theme.colors.muted
            } else {
                self.theme.colors.text
            });
        element.style().refine(&self.style);
        element.child(self.label)
    }
}
