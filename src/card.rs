use gpui::{
    AnyElement, App, ElementId, InteractiveElement, IntoElement, ParentElement, Refineable as _,
    RenderOnce, StyleRefinement, Styled, Window, div, px,
};

use crate::theme::{BODY_LINE_HEIGHT, BODY_SIZE, CARD_PADDING, CARD_RADIUS, Theme};

/// A flat Neutral card. The owner composes its header, content and footer.
#[derive(IntoElement)]
pub struct Card {
    id: Option<ElementId>,
    children: Vec<AnyElement>,
    padding: f32,
    theme: Theme,
    style: StyleRefinement,
}

impl Card {
    pub fn new() -> Self {
        Self {
            id: None,
            children: Vec::new(),
            padding: CARD_PADDING,
            theme: Theme::default(),
            style: StyleRefinement::default(),
        }
    }

    pub fn padding(mut self, padding: f32) -> Self {
        self.padding = padding.max(0.);
        self
    }

    /// Sets the actual native frame identity when a host needs to retain it.
    pub fn id(mut self, id: impl Into<ElementId>) -> Self {
        self.id = Some(id.into());
        self
    }

    pub fn theme(mut self, theme: Theme) -> Self {
        self.theme = theme;
        self
    }
}

impl Default for Card {
    fn default() -> Self {
        Self::new()
    }
}

impl ParentElement for Card {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl Styled for Card {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for Card {
    fn render(self, _: &mut Window, _: &mut App) -> impl IntoElement {
        let mut element = div()
            .flex()
            .flex_col()
            .min_w_0()
            .gap(px(CARD_PADDING))
            .p(px(self.padding))
            .rounded(px(CARD_RADIUS))
            .border_1()
            .border_color(self.theme.colors.border)
            .bg(self.theme.colors.card)
            .text_color(self.theme.colors.text)
            .text_size(px(BODY_SIZE))
            .line_height(px(BODY_LINE_HEIGHT));
        element.style().refine(&self.style);
        if let Some(id) = self.id {
            element.id(id).children(self.children).into_any_element()
        } else {
            element.children(self.children).into_any_element()
        }
    }
}
