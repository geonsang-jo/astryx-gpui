use crate::Theme;
use gpui::{
    AnyElement, App, ElementId, InteractiveElement, IntoElement, ParentElement, Refineable as _,
    RenderOnce, SharedString, StatefulInteractiveElement as _, StyleRefinement, Styled, Window,
    div, px,
};

/// A native quotation surface with a source/author caption.
#[derive(IntoElement)]
pub struct Blockquote {
    children: Vec<AnyElement>,
    cite: Option<SharedString>,
    theme: Theme,
    style: StyleRefinement,
}
impl Blockquote {
    pub fn new() -> Self {
        Self {
            children: vec![],
            cite: None,
            theme: Theme::default(),
            style: StyleRefinement::default(),
        }
    }
    pub fn cite(mut self, label: impl Into<SharedString>) -> Self {
        self.cite = Some(label.into());
        self
    }
    pub fn theme(mut self, theme: Theme) -> Self {
        self.theme = theme;
        self
    }
}
impl Default for Blockquote {
    fn default() -> Self {
        Self::new()
    }
}
impl Styled for Blockquote {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}
impl ParentElement for Blockquote {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements)
    }
}
impl RenderOnce for Blockquote {
    fn render(self, _: &mut Window, _: &mut App) -> impl IntoElement {
        let c = self.theme.colors;
        let mut el = div()
            .flex()
            .flex_col()
            .gap(px(8.))
            .pl(px(12.))
            .border_l(px(3.))
            .border_color(c.border_emphasized)
            .text_color(c.muted)
            .text_size(px(14.))
            .line_height(px(20.));
        el.style().refine(&self.style);
        el.children(self.children).children(
            self.cite
                .map(|cite| div().text_size(px(12.)).child(format!("— {cite}"))),
        )
    }
}

/// Label/value rows for compact metadata. Values may be arbitrary native content.
#[derive(IntoElement)]
pub struct MetadataList {
    rows: Vec<(SharedString, AnyElement)>,
    theme: Theme,
    style: StyleRefinement,
}
impl MetadataList {
    pub fn new() -> Self {
        Self {
            rows: vec![],
            theme: Theme::default(),
            style: StyleRefinement::default(),
        }
    }
    pub fn row(mut self, label: impl Into<SharedString>, value: impl IntoElement) -> Self {
        self.rows.push((label.into(), value.into_any_element()));
        self
    }
    pub fn theme(mut self, theme: Theme) -> Self {
        self.theme = theme;
        self
    }
}
impl Default for MetadataList {
    fn default() -> Self {
        Self::new()
    }
}
impl Styled for MetadataList {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}
impl RenderOnce for MetadataList {
    fn render(self, _: &mut Window, _: &mut App) -> impl IntoElement {
        let c = self.theme.colors;
        let mut el = div()
            .flex()
            .flex_col()
            .gap(px(8.))
            .min_w_0()
            .text_color(c.text)
            .text_size(px(14.))
            .line_height(px(20.));
        el.style().refine(&self.style);
        el.children(self.rows.into_iter().map(|(label, value)| {
            div()
                .flex()
                .gap(px(12.))
                .min_w_0()
                .child(
                    div()
                        .w(px(100.))
                        .flex_none()
                        .text_color(c.muted)
                        .child(label),
                )
                .child(div().flex_1().min_w_0().child(value))
        }))
    }
}

/// Screen-reader text with a one-pixel off-flow native accessibility node.
/// It deliberately has no painted text or focus stop.
#[derive(IntoElement)]
pub struct VisuallyHidden {
    id: ElementId,
    text: SharedString,
}
impl VisuallyHidden {
    pub fn new(id: impl Into<ElementId>, text: impl Into<SharedString>) -> Self {
        Self {
            id: id.into(),
            text: text.into(),
        }
    }
}
impl RenderOnce for VisuallyHidden {
    fn render(self, _: &mut Window, _: &mut App) -> impl IntoElement {
        div()
            .id(self.id)
            .absolute()
            .size(px(1.))
            .overflow_hidden()
            .role(gpui::Role::Label)
            .aria_label(self.text)
    }
}

/// A controlled visual outline around native content. The owner supplies focus state.
#[derive(IntoElement)]
pub struct Outline {
    active: bool,
    children: Vec<AnyElement>,
    theme: Theme,
    style: StyleRefinement,
}
impl Outline {
    pub fn new() -> Self {
        Self {
            active: true,
            children: vec![],
            theme: Theme::default(),
            style: StyleRefinement::default(),
        }
    }
    pub fn active(mut self, active: bool) -> Self {
        self.active = active;
        self
    }
    pub fn theme(mut self, theme: Theme) -> Self {
        self.theme = theme;
        self
    }
}
impl Default for Outline {
    fn default() -> Self {
        Self::new()
    }
}
impl Styled for Outline {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}
impl ParentElement for Outline {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements)
    }
}
impl RenderOnce for Outline {
    fn render(self, _: &mut Window, _: &mut App) -> impl IntoElement {
        let mut el = div()
            .p(px(2.))
            .rounded(px(12.))
            .border_2()
            .border_color(if self.active {
                self.theme.colors.focus
            } else {
                gpui::transparent_black()
            });
        el.style().refine(&self.style);
        el.children(self.children)
    }
}
