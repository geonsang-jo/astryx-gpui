use gpui::{
    AnyElement, App, ElementId, IntoElement, ParentElement, Refineable as _, RenderOnce,
    SharedString, StyleRefinement, Styled, Window, div, px, relative,
};
use gpui_base::StyledExt as _;

use crate::{Theme, Tone};

fn colors(theme: Theme, tone: Tone) -> (gpui::Hsla, gpui::Hsla) {
    let c = theme.colors;
    match tone {
        Tone::Neutral => (c.neutral_background, c.neutral_foreground),
        Tone::Success => (c.success_background, c.success_foreground),
        Tone::Warning => (c.warning_background, c.warning_foreground),
        Tone::Danger => (c.danger_background, c.danger_foreground),
        Tone::Info => (c.info_background, c.info_foreground),
    }
}

/// Inline status content; dismissal/actions are composed by the owner as children.
#[derive(IntoElement)]
pub struct Banner {
    title: SharedString,
    children: Vec<AnyElement>,
    tone: Tone,
    theme: Theme,
    style: StyleRefinement,
}

impl Banner {
    pub fn new(title: impl Into<SharedString>) -> Self {
        Self {
            title: title.into(),
            children: vec![],
            tone: Tone::Info,
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
impl Styled for Banner {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}
impl ParentElement for Banner {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}
impl RenderOnce for Banner {
    fn render(self, _: &mut Window, _: &mut App) -> impl IntoElement {
        let (background, foreground) = colors(self.theme, self.tone);
        let mut element = div()
            .flex()
            .flex_col()
            .min_w_0()
            .p(px(12.))
            .gap(px(8.))
            .rounded(px(12.))
            .bg(background)
            .text_color(foreground)
            .text_size(px(14.))
            .line_height(px(20.));
        element.style().refine(&self.style);
        element
            .child(
                div()
                    .font_weight(gpui::FontWeight::MEDIUM)
                    .child(self.title),
            )
            .children(self.children)
    }
}

/// Determinate subscription/task progress, with native numeric accessibility values.
#[derive(IntoElement)]
pub struct ProgressBar {
    base: gpui_base::Progress,
    value: f32,
    theme: Theme,
    style: StyleRefinement,
}
impl ProgressBar {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            base: gpui_base::Progress::new(id),
            value: 0.,
            theme: Theme::default(),
            style: StyleRefinement::default(),
        }
    }
    /// Non-finite values render as zero; finite values are clamped to 0–100.
    pub fn value(mut self, value: f32) -> Self {
        self.value = if value.is_finite() {
            value.clamp(0., 100.)
        } else {
            0.
        };
        self.base = self.base.value(self.value);
        self
    }
    pub fn label(mut self, label: impl Into<SharedString>) -> Self {
        self.base = self.base.accessibility_label(label);
        self
    }
    pub fn theme(mut self, theme: Theme) -> Self {
        self.theme = theme;
        self
    }
}
impl Styled for ProgressBar {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}
impl RenderOnce for ProgressBar {
    fn render(self, _: &mut Window, _: &mut App) -> impl IntoElement {
        let c = self.theme.colors;
        self.base
            .w_full()
            .h(px(4.))
            .rounded_full()
            .bg(c.neutral)
            .overflow_hidden()
            .child(
                div()
                    .h_full()
                    .w(relative(self.value / 100.))
                    .rounded_full()
                    .bg(c.accent),
            )
            .refine_style(&self.style)
    }
}

/// A labelled status marker. Color is accompanied by the caller's visible label.
#[derive(IntoElement)]
pub struct StatusDot {
    label: SharedString,
    tone: Tone,
    theme: Theme,
    style: StyleRefinement,
}
impl StatusDot {
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
impl Styled for StatusDot {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}
impl RenderOnce for StatusDot {
    fn render(self, _: &mut Window, _: &mut App) -> impl IntoElement {
        let (_, tint) = colors(self.theme, self.tone);
        let mut element = div()
            .flex()
            .items_center()
            .gap(px(6.))
            .text_size(px(12.))
            .line_height(px(20.))
            .text_color(self.theme.colors.muted);
        element.style().refine(&self.style);
        element
            .child(div().flex_none().size(px(8.)).rounded_full().bg(tint))
            .child(self.label)
    }
}

/// A static loading placeholder; the host may animate it using its motion policy.
#[derive(IntoElement)]
pub struct Skeleton {
    theme: Theme,
    style: StyleRefinement,
}
impl Skeleton {
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
impl Default for Skeleton {
    fn default() -> Self {
        Self::new()
    }
}
impl Styled for Skeleton {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}
impl RenderOnce for Skeleton {
    fn render(self, _: &mut Window, _: &mut App) -> impl IntoElement {
        let mut element = div()
            .h(px(16.))
            .w_full()
            .rounded(px(6.))
            .bg(self.theme.colors.neutral);
        element.style().refine(&self.style);
        element
    }
}

/// A compact keyboard shortcut label; it never handles or installs keybindings.
#[derive(IntoElement)]
pub struct Kbd {
    label: SharedString,
    theme: Theme,
    style: StyleRefinement,
}
impl Kbd {
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
}
impl Styled for Kbd {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}
impl RenderOnce for Kbd {
    fn render(self, _: &mut Window, _: &mut App) -> impl IntoElement {
        let c = self.theme.colors;
        let mut element = div()
            .flex_none()
            .px(px(6.))
            .py(px(2.))
            .rounded(px(6.))
            .border_1()
            .border_color(c.border_emphasized)
            .bg(c.canvas)
            .text_color(c.muted)
            .text_size(px(12.))
            .line_height(px(16.))
            .font_family("monospace");
        element.style().refine(&self.style);
        element.child(self.label)
    }
}

/// An empty result state with a title, explanation and owner-composed actions.
#[derive(IntoElement)]
pub struct EmptyState {
    title: SharedString,
    description: Option<SharedString>,
    children: Vec<AnyElement>,
    theme: Theme,
    style: StyleRefinement,
}
impl EmptyState {
    pub fn new(title: impl Into<SharedString>) -> Self {
        Self {
            title: title.into(),
            description: None,
            children: vec![],
            theme: Theme::default(),
            style: StyleRefinement::default(),
        }
    }
    pub fn description(mut self, value: impl Into<SharedString>) -> Self {
        self.description = Some(value.into());
        self
    }
    pub fn theme(mut self, theme: Theme) -> Self {
        self.theme = theme;
        self
    }
}
impl Styled for EmptyState {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}
impl ParentElement for EmptyState {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements)
    }
}
impl RenderOnce for EmptyState {
    fn render(self, _: &mut Window, _: &mut App) -> impl IntoElement {
        let c = self.theme.colors;
        let mut element = div()
            .flex()
            .flex_col()
            .items_center()
            .justify_center()
            .min_w_0()
            .gap(px(8.))
            .p(px(24.))
            .text_color(c.text)
            .text_align(gpui::TextAlign::Center)
            .text_size(px(14.))
            .line_height(px(20.));
        element.style().refine(&self.style);
        element
            .child(
                div()
                    .font_weight(gpui::FontWeight::MEDIUM)
                    .child(self.title),
            )
            .children(
                self.description
                    .map(|description| div().text_color(c.muted).child(description)),
            )
            .children(self.children)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn progress_rejects_non_finite_and_clamps_values() {
        assert_eq!(ProgressBar::new("p").value(f32::NAN).value, 0.);
        assert_eq!(ProgressBar::new("p").value(f32::INFINITY).value, 0.);
        assert_eq!(ProgressBar::new("p").value(-1.).value, 0.);
        assert_eq!(ProgressBar::new("p").value(140.).value, 100.);
    }
}
