use crate::Theme;
use gpui::{
    AnyElement, App, ElementId, InteractiveElement as _, IntoElement, ParentElement, RenderOnce,
    SharedString, StyleRefinement, Styled, Window, div, prelude::FluentBuilder as _, px,
};
use gpui_base::StyledExt as _;

/// Label, control and supporting/error copy. Name the contained control with
/// the same label through its own accessibility API.
#[derive(IntoElement)]
pub struct Field {
    id: ElementId,
    label: SharedString,
    description: Option<SharedString>,
    error: Option<SharedString>,
    required: bool,
    theme: Theme,
    style: StyleRefinement,
    children: Vec<AnyElement>,
}
impl Field {
    pub fn new(id: impl Into<ElementId>, label: impl Into<SharedString>) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            description: None,
            error: None,
            required: false,
            theme: Theme::default(),
            style: StyleRefinement::default(),
            children: vec![],
        }
    }
    pub fn theme(mut self, theme: Theme) -> Self {
        self.theme = theme;
        self
    }
    pub fn description(mut self, text: impl Into<SharedString>) -> Self {
        self.description = Some(text.into());
        self
    }
    pub fn error(mut self, text: impl Into<SharedString>) -> Self {
        self.error = Some(text.into());
        self
    }
    pub fn required(mut self, required: bool) -> Self {
        self.required = required;
        self
    }
}
impl Styled for Field {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}
impl ParentElement for Field {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}
impl RenderOnce for Field {
    fn render(self, _: &mut Window, _: &mut App) -> impl IntoElement {
        let c = self.theme.colors;
        let invalid = self.error.is_some();
        div()
            .id(self.id)
            .flex()
            .flex_col()
            .min_w(px(0.))
            .gap(px(6.))
            .text_color(c.text)
            .text_size(px(14.))
            .line_height(px(20.))
            .child(div().child(if self.required {
                format!("{} *", self.label)
            } else {
                self.label.to_string()
            }))
            .children(self.children)
            .when_some(self.error.or(self.description), |el, text| {
                el.child(
                    div()
                        .text_size(px(12.))
                        .line_height(px(20.))
                        .text_color(if invalid { c.danger } else { c.muted })
                        .child(text),
                )
            })
            .refine_style(&self.style)
    }
}

/// A vertical form layout with a consistent field rhythm.
#[derive(IntoElement, Default)]
pub struct FormLayout {
    children: Vec<AnyElement>,
    style: StyleRefinement,
}
impl FormLayout {
    pub fn new() -> Self {
        Self::default()
    }
}
impl Styled for FormLayout {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}
impl ParentElement for FormLayout {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}
impl RenderOnce for FormLayout {
    fn render(self, _: &mut Window, _: &mut App) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .min_w(px(0.))
            .gap(px(16.))
            .children(self.children)
            .refine_style(&self.style)
    }
}

/// A shared horizontal frame for prefixes, input controls and suffix actions.
#[derive(IntoElement, Default)]
pub struct InputGroup {
    theme: Theme,
    children: Vec<AnyElement>,
    style: StyleRefinement,
}
impl InputGroup {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn theme(mut self, theme: Theme) -> Self {
        self.theme = theme;
        self
    }
}
impl Styled for InputGroup {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}
impl ParentElement for InputGroup {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}
impl RenderOnce for InputGroup {
    fn render(self, _: &mut Window, _: &mut App) -> impl IntoElement {
        let c = self.theme.colors;
        div()
            .flex()
            .items_center()
            .min_w(px(0.))
            .gap(px(8.))
            .px(px(10.))
            .rounded(px(10.))
            .border_1()
            .border_color(c.border_emphasized)
            .bg(c.surface)
            .text_color(c.muted)
            .text_size(px(14.))
            .line_height(px(20.))
            .children(self.children)
            .refine_style(&self.style)
    }
}
