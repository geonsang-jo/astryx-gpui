use std::time::Duration;

use crate::{Button, ButtonVariant, ControlSize, Theme};
use gpui::{
    Animation, AnimationExt as _, App, ClickEvent, ElementId, ImageSource, InteractiveElement,
    IntoElement, ParentElement, Refineable as _, RenderOnce, SharedString,
    StatefulInteractiveElement as _, StyleRefinement, Styled, StyledImage as _, Window, div, img,
    prelude::FluentBuilder as _, px,
};

/// A caller-supplied native SVG. No icon font or third-party icon set is bundled.
#[derive(IntoElement)]
pub struct Icon {
    id: ElementId,
    path: Option<SharedString>,
    data: Option<SharedString>,
    label: Option<SharedString>,
    theme: Theme,
    style: StyleRefinement,
}
impl Icon {
    pub fn new(id: impl Into<ElementId>, path: impl Into<SharedString>) -> Self {
        Self {
            id: id.into(),
            path: Some(path.into()),
            data: None,
            label: None,
            theme: Theme::default(),
            style: StyleRefinement::default(),
        }
    }
    pub fn svg(id: impl Into<ElementId>, data: impl Into<SharedString>) -> Self {
        Self {
            id: id.into(),
            path: None,
            data: Some(data.into()),
            label: None,
            theme: Theme::default(),
            style: StyleRefinement::default(),
        }
    }
    pub fn label(mut self, label: impl Into<SharedString>) -> Self {
        self.label = Some(label.into());
        self
    }
    pub fn theme(mut self, theme: Theme) -> Self {
        self.theme = theme;
        self
    }
}
impl Styled for Icon {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}
impl RenderOnce for Icon {
    fn render(self, _: &mut Window, _: &mut App) -> impl IntoElement {
        let color = self.style.text.color.unwrap_or(self.theme.colors.text);
        let mut svg = gpui::svg().size_full().text_color(color);
        if let Some(path) = self.path {
            svg = svg.path(path);
        }
        if let Some(data) = self.data {
            svg = svg.data(data.as_bytes());
        }
        let mut root = div()
            .id(self.id)
            .size(px(16.))
            .flex_none()
            .text_color(self.theme.colors.text)
            .when_some(self.label, |el, label| {
                el.role(gpui::Role::Image).aria_label(label)
            });
        root.style().refine(&self.style);
        root.child(svg)
    }
}

/// An icon-only native button requiring an accessible text name.
#[derive(IntoElement)]
pub struct IconButton {
    base: Button,
    icon: Icon,
    size: ControlSize,
    variant: ButtonVariant,
    theme: Theme,
    style: StyleRefinement,
}
impl IconButton {
    pub fn new(id: impl Into<ElementId>, label: impl Into<SharedString>, icon: Icon) -> Self {
        Self {
            base: Button::new(id, "")
                .accessibility_label(label)
                .variant(ButtonVariant::Ghost),
            icon,
            size: ControlSize::Medium,
            variant: ButtonVariant::Ghost,
            theme: Theme::default(),
            style: StyleRefinement::default(),
        }
    }
    pub fn theme(mut self, theme: Theme) -> Self {
        self.theme = theme;
        self.base = self.base.theme(theme);
        self.icon = self.icon.theme(theme);
        self
    }
    pub fn size(mut self, size: ControlSize) -> Self {
        self.size = size;
        self.base = self.base.size(size);
        self
    }
    pub fn variant(mut self, variant: ButtonVariant) -> Self {
        self.variant = variant;
        self.base = self.base.variant(variant);
        self
    }
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.base = self.base.disabled(disabled);
        self
    }
    pub fn on_click(
        mut self,
        handler: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.base = self.base.on_click(handler);
        self
    }
}
impl Styled for IconButton {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}
impl RenderOnce for IconButton {
    fn render(self, _: &mut Window, _: &mut App) -> impl IntoElement {
        let color = self
            .style
            .text
            .color
            .unwrap_or(if self.variant == ButtonVariant::Primary {
                self.theme.colors.on_accent
            } else {
                self.theme.colors.text
            });
        let mut base = self
            .base
            .leading(self.icon.text_color(color))
            .w(px(self.size.height()))
            .px(px(0.));
        base.style().refine(&self.style);
        base
    }
}

/// A navigation glyph frame with optional unread count.
#[derive(IntoElement)]
pub struct NavIcon {
    icon: Icon,
    selected: bool,
    count: Option<usize>,
    theme: Theme,
    style: StyleRefinement,
}
impl NavIcon {
    pub fn new(icon: Icon) -> Self {
        Self {
            icon,
            selected: false,
            count: None,
            theme: Theme::default(),
            style: StyleRefinement::default(),
        }
    }
    pub fn selected(mut self, selected: bool) -> Self {
        self.selected = selected;
        self
    }
    pub fn count(mut self, count: usize) -> Self {
        self.count = (count > 0).then_some(count);
        self
    }
    pub fn theme(mut self, theme: Theme) -> Self {
        self.theme = theme;
        self.icon = self.icon.theme(theme);
        self
    }
}
impl Styled for NavIcon {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}
impl RenderOnce for NavIcon {
    fn render(self, _: &mut Window, _: &mut App) -> impl IntoElement {
        let c = self.theme.colors;
        let mut el = div()
            .relative()
            .flex()
            .items_center()
            .justify_center()
            .size(px(32.))
            .rounded(px(8.))
            .when(self.selected, |el| el.bg(c.neutral));
        el.style().refine(&self.style);
        el.child(self.icon).children(self.count.map(|count| {
            div()
                .absolute()
                .top(px(-3.))
                .right(px(-5.))
                .min_w(px(14.))
                .h(px(14.))
                .px(px(3.))
                .rounded_full()
                .bg(c.danger)
                .text_color(c.on_accent)
                .text_size(px(9.))
                .line_height(px(14.))
                .child(if count > 99 {
                    "99+".into()
                } else {
                    count.to_string()
                })
        }))
    }
}

fn initials(name: &str) -> SharedString {
    name.split_whitespace()
        .take(2)
        .filter_map(|word| word.chars().next())
        .collect::<String>()
        .into()
}
fn fallback(label: SharedString, theme: Theme, round: bool) -> gpui::AnyElement {
    div()
        .flex()
        .items_center()
        .justify_center()
        .size_full()
        .bg(theme.colors.neutral)
        .when(round, |el| el.rounded_full())
        .text_color(theme.colors.text)
        .child(label)
        .into_any_element()
}

/// A native image-backed avatar, with initials while loading and on failure.
#[derive(IntoElement)]
pub struct Avatar {
    id: ElementId,
    name: SharedString,
    image: Option<ImageSource>,
    fallback_text: Option<SharedString>,
    size: f32,
    theme: Theme,
    style: StyleRefinement,
}
impl Avatar {
    pub fn new(id: impl Into<ElementId>, name: impl Into<SharedString>) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            image: None,
            fallback_text: None,
            size: 32.,
            theme: Theme::default(),
            style: StyleRefinement::default(),
        }
    }
    pub fn image(mut self, source: impl Into<ImageSource>) -> Self {
        self.image = Some(source.into());
        self
    }
    /// Override the displayed fallback without changing the accessible name.
    pub fn fallback_text(mut self, label: impl Into<SharedString>) -> Self {
        self.fallback_text = Some(label.into());
        self
    }
    pub fn size(mut self, pixels: f32) -> Self {
        self.size = pixels.max(16.);
        self
    }
    pub fn theme(mut self, theme: Theme) -> Self {
        self.theme = theme;
        self
    }
}
impl Styled for Avatar {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}
impl RenderOnce for Avatar {
    fn render(self, _: &mut Window, _: &mut App) -> impl IntoElement {
        let initial = self.fallback_text.unwrap_or_else(|| initials(&self.name));
        let loading = initial.clone();
        let error = initial.clone();
        let theme = self.theme;
        let mut root = div()
            .id(self.id)
            .flex_none()
            .size(px(self.size))
            .rounded_full()
            .overflow_hidden()
            .role(gpui::Role::Image)
            .aria_label(self.name)
            .text_size(px(12.))
            .line_height(px(20.));
        root.style().refine(&self.style);
        root.child(if let Some(source) = self.image {
            img(source)
                .size_full()
                .rounded_full()
                .object_fit(gpui::ObjectFit::Cover)
                .with_loading(move || fallback(loading.clone(), theme, true))
                .with_fallback(move || fallback(error.clone(), theme, true))
                .into_any_element()
        } else {
            fallback(initial, theme, true)
        })
    }
}

/// Overlapping avatar composition with an explicit overflow count.
#[derive(IntoElement)]
pub struct AvatarGroup {
    id: ElementId,
    items: Vec<Avatar>,
    limit: usize,
    theme: Theme,
    style: StyleRefinement,
}
impl AvatarGroup {
    pub fn new(id: impl Into<ElementId>, items: Vec<Avatar>) -> Self {
        Self {
            id: id.into(),
            items,
            limit: 5,
            theme: Theme::default(),
            style: StyleRefinement::default(),
        }
    }
    pub fn visible(mut self, count: usize) -> Self {
        self.limit = count.max(1);
        self
    }
    pub fn theme(mut self, theme: Theme) -> Self {
        self.theme = theme;
        self
    }
}
impl Styled for AvatarGroup {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}
impl RenderOnce for AvatarGroup {
    fn render(self, _: &mut Window, _: &mut App) -> impl IntoElement {
        let extra = self.items.len().saturating_sub(self.limit);
        let theme = self.theme;
        let mut root = div().id(self.id).flex().items_center();
        root.style().refine(&self.style);
        root.children(
            self.items
                .into_iter()
                .take(self.limit)
                .enumerate()
                .map(|(i, avatar)| {
                    avatar
                        .theme(theme)
                        .ml(px(if i == 0 { 0. } else { -8. }))
                        .border_2()
                        .border_color(theme.colors.surface)
                }),
        )
        .when(extra > 0, |el| {
            el.child(
                Avatar::new("avatar-overflow", format!("{extra} more people"))
                    .fallback_text(format!("+{extra}"))
                    .theme(theme)
                    .ml(px(-8.))
                    .border_2()
                    .border_color(theme.colors.surface),
            )
        })
    }
}

/// Native raster/SVG media with labelled failure and loading presentation.
#[derive(IntoElement)]
pub struct Thumbnail {
    id: ElementId,
    source: ImageSource,
    label: SharedString,
    theme: Theme,
    style: StyleRefinement,
}
impl Thumbnail {
    pub fn new(
        id: impl Into<ElementId>,
        source: impl Into<ImageSource>,
        label: impl Into<SharedString>,
    ) -> Self {
        Self {
            id: id.into(),
            source: source.into(),
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
impl Styled for Thumbnail {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}
impl RenderOnce for Thumbnail {
    fn render(self, _: &mut Window, _: &mut App) -> impl IntoElement {
        let theme = self.theme;
        let mut root = div()
            .id(self.id)
            .w(px(96.))
            .h(px(72.))
            .rounded(px(8.))
            .overflow_hidden()
            .role(gpui::Role::Image)
            .aria_label(self.label);
        root.style().refine(&self.style);
        root.child(
            img(self.source)
                .size_full()
                .object_fit(gpui::ObjectFit::Cover)
                .with_loading(move || fallback("Loading image…".into(), theme, false))
                .with_fallback(move || fallback("Image unavailable".into(), theme, false)),
        )
    }
}

/// Content with a corner badge. The count remains visible text, not color alone.
#[derive(IntoElement)]
pub struct Indicator {
    children: Vec<gpui::AnyElement>,
    count: usize,
    theme: Theme,
    style: StyleRefinement,
}
impl Indicator {
    pub fn new(count: usize) -> Self {
        Self {
            children: vec![],
            count,
            theme: Theme::default(),
            style: StyleRefinement::default(),
        }
    }
    pub fn theme(mut self, theme: Theme) -> Self {
        self.theme = theme;
        self
    }
}
impl Styled for Indicator {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}
impl ParentElement for Indicator {
    fn extend(&mut self, elements: impl IntoIterator<Item = gpui::AnyElement>) {
        self.children.extend(elements)
    }
}
impl RenderOnce for Indicator {
    fn render(self, _: &mut Window, _: &mut App) -> impl IntoElement {
        let c = self.theme.colors;
        let mut root = div().relative();
        root.style().refine(&self.style);
        root.children(self.children).when(self.count > 0, |el| {
            el.child(
                div()
                    .absolute()
                    .top(px(-5.))
                    .right(px(-5.))
                    .min_w(px(16.))
                    .h(px(16.))
                    .px(px(4.))
                    .rounded_full()
                    .bg(c.danger)
                    .text_color(c.on_accent)
                    .text_size(px(10.))
                    .line_height(px(16.))
                    .child(if self.count > 99 {
                        "99+".into()
                    } else {
                        self.count.to_string()
                    }),
            )
        })
    }
}

const SPINNER_SVG: &str = "<svg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 24 24'><path fill='none' stroke='currentColor' stroke-width='2' stroke-linecap='round' d='M12 3a9 9 0 1 1-9 9'/></svg>";
/// A genuine native animated SVG spinner. Hosts may disable motion explicitly.
#[derive(IntoElement)]
pub struct Spinner {
    id: ElementId,
    label: SharedString,
    animated: bool,
    theme: Theme,
    style: StyleRefinement,
}
impl Spinner {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            label: "Loading".into(),
            animated: true,
            theme: Theme::default(),
            style: StyleRefinement::default(),
        }
    }
    pub fn label(mut self, label: impl Into<SharedString>) -> Self {
        self.label = label.into();
        self
    }
    pub fn animated(mut self, animated: bool) -> Self {
        self.animated = animated;
        self
    }
    pub fn theme(mut self, theme: Theme) -> Self {
        self.theme = theme;
        self
    }
}
impl Styled for Spinner {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}
impl RenderOnce for Spinner {
    fn render(self, _: &mut Window, _: &mut App) -> impl IntoElement {
        let color = self.style.text.color.unwrap_or(self.theme.colors.text);
        let icon = gpui::svg()
            .data(SPINNER_SVG.as_bytes())
            .size_full()
            .text_color(color);
        let mut root = div()
            .id(self.id.clone())
            .size(px(16.))
            .text_color(self.theme.colors.text)
            .role(gpui::Role::ProgressIndicator)
            .aria_label(self.label);
        root.style().refine(&self.style);
        let content = if self.animated {
            icon.with_animation(
                self.id,
                Animation::new(Duration::from_millis(900)).repeat(),
                |icon, delta| {
                    icon.with_transformation(gpui::Transformation::rotate(gpui::radians(
                        delta * std::f32::consts::TAU,
                    )))
                },
            )
            .into_any_element()
        } else {
            icon.into_any_element()
        };
        root.child(content)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn names_produce_unicode_initials() {
        assert_eq!(initials("Ada Lovelace").as_ref(), "AL");
        assert_eq!(initials("홍 길동").as_ref(), "홍길");
        assert_eq!(initials("").as_ref(), "");
    }
}
