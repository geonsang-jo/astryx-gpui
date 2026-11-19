use std::rc::Rc;

use gpui::{
    AnyElement, App, ElementId, IntoElement, ParentElement, Refineable as _, RenderOnce,
    SharedString, StyleRefinement, Styled, Window, div, prelude::FluentBuilder as _, px,
};

use crate::{Button, ButtonVariant, Theme};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Direction {
    Horizontal,
    #[default]
    Vertical,
}

/// A native flex stack with caller-refinable alignment, wrapping, size and spacing.
#[derive(IntoElement)]
pub struct Stack {
    children: Vec<AnyElement>,
    direction: Direction,
    gap: f32,
    theme: Theme,
    style: StyleRefinement,
}
impl Stack {
    pub fn new() -> Self {
        Self {
            children: vec![],
            direction: Direction::Vertical,
            gap: 8.,
            theme: Theme::default(),
            style: StyleRefinement::default(),
        }
    }
    pub fn horizontal() -> Self {
        Self::new().direction(Direction::Horizontal)
    }
    pub fn vertical() -> Self {
        Self::new()
    }
    pub fn direction(mut self, value: Direction) -> Self {
        self.direction = value;
        self
    }
    pub fn spacing(mut self, pixels: f32) -> Self {
        self.gap = pixels.max(0.);
        self
    }
    pub fn theme(mut self, theme: Theme) -> Self {
        self.theme = theme;
        self
    }
}
impl Default for Stack {
    fn default() -> Self {
        Self::new()
    }
}
impl Styled for Stack {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}
impl ParentElement for Stack {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements)
    }
}
impl RenderOnce for Stack {
    fn render(self, _: &mut Window, _: &mut App) -> impl IntoElement {
        let mut element = div()
            .flex()
            .min_w_0()
            .gap(px(self.gap))
            .text_color(self.theme.colors.text)
            .when(self.direction == Direction::Vertical, |el| el.flex_col());
        element.style().refine(&self.style);
        element.children(self.children)
    }
}

/// A two-axis centering container rather than a styled placeholder.
#[derive(IntoElement)]
pub struct Center {
    children: Vec<AnyElement>,
    style: StyleRefinement,
}
impl Center {
    pub fn new() -> Self {
        Self {
            children: vec![],
            style: StyleRefinement::default(),
        }
    }
}
impl Default for Center {
    fn default() -> Self {
        Self::new()
    }
}
impl Styled for Center {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}
impl ParentElement for Center {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements)
    }
}
impl RenderOnce for Center {
    fn render(self, _: &mut Window, _: &mut App) -> impl IntoElement {
        let mut el = div().flex().items_center().justify_center();
        el.style().refine(&self.style);
        el.children(self.children)
    }
}

/// Native aspect-ratio sizing. Invalid ratios fall back to a square.
#[derive(IntoElement)]
pub struct AspectRatio {
    ratio: f32,
    children: Vec<AnyElement>,
    style: StyleRefinement,
}
impl AspectRatio {
    pub fn new(ratio: f32) -> Self {
        Self {
            ratio: if ratio.is_finite() && ratio > 0. {
                ratio
            } else {
                1.
            },
            children: vec![],
            style: StyleRefinement::default(),
        }
    }
}
impl Styled for AspectRatio {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}
impl ParentElement for AspectRatio {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements)
    }
}
impl RenderOnce for AspectRatio {
    fn render(self, _: &mut Window, _: &mut App) -> impl IntoElement {
        let mut el = div().relative().w_full().aspect_ratio(self.ratio);
        el.style().refine(&self.style);
        el.children(self.children)
    }
}

/// A real CSS-grid-equivalent native layout with at least one column.
#[derive(IntoElement)]
pub struct Grid {
    columns: u16,
    gap: f32,
    children: Vec<AnyElement>,
    style: StyleRefinement,
}
impl Grid {
    pub fn new(columns: u16) -> Self {
        Self {
            columns: columns.max(1),
            gap: 12.,
            children: vec![],
            style: StyleRefinement::default(),
        }
    }
    pub fn spacing(mut self, pixels: f32) -> Self {
        self.gap = pixels.max(0.);
        self
    }
}
impl Styled for Grid {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}
impl ParentElement for Grid {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements)
    }
}
impl RenderOnce for Grid {
    fn render(self, _: &mut Window, _: &mut App) -> impl IntoElement {
        let mut el = div()
            .grid()
            .grid_cols(self.columns)
            .gap(px(self.gap))
            .min_w_0();
        el.style().refine(&self.style);
        el.children(self.children)
    }
}

/// Labelled content group with a flat section divider.
#[derive(IntoElement)]
pub struct Section {
    title: SharedString,
    description: Option<SharedString>,
    children: Vec<AnyElement>,
    theme: Theme,
    style: StyleRefinement,
}
impl Section {
    pub fn new(title: impl Into<SharedString>) -> Self {
        Self {
            title: title.into(),
            description: None,
            children: vec![],
            theme: Theme::default(),
            style: StyleRefinement::default(),
        }
    }
    pub fn description(mut self, text: impl Into<SharedString>) -> Self {
        self.description = Some(text.into());
        self
    }
    pub fn theme(mut self, theme: Theme) -> Self {
        self.theme = theme;
        self
    }
}
impl Styled for Section {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}
impl ParentElement for Section {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements)
    }
}
impl RenderOnce for Section {
    fn render(self, _: &mut Window, _: &mut App) -> impl IntoElement {
        let c = self.theme.colors;
        let mut el = div()
            .flex()
            .flex_col()
            .min_w_0()
            .gap(px(12.))
            .py(px(16.))
            .border_b_1()
            .border_color(c.border)
            .text_color(c.text)
            .text_size(px(14.))
            .line_height(px(20.));
        el.style().refine(&self.style);
        el.child(
            div()
                .text_size(px(17.))
                .line_height(px(24.))
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

/// Header/panel/main/footer composition. Side panels retain their own scroll policy.
#[derive(IntoElement)]
pub struct Layout {
    header: Option<AnyElement>,
    side: Option<AnyElement>,
    aside: Option<AnyElement>,
    footer: Option<AnyElement>,
    content: Vec<AnyElement>,
    side_width: f32,
    aside_width: f32,
    theme: Theme,
    style: StyleRefinement,
}
impl Layout {
    pub fn new() -> Self {
        Self {
            header: None,
            side: None,
            aside: None,
            footer: None,
            content: vec![],
            side_width: 240.,
            aside_width: 280.,
            theme: Theme::default(),
            style: StyleRefinement::default(),
        }
    }
    pub fn header(mut self, element: impl IntoElement) -> Self {
        self.header = Some(element.into_any_element());
        self
    }
    pub fn side_nav(mut self, element: impl IntoElement) -> Self {
        self.side = Some(element.into_any_element());
        self
    }
    pub fn aside(mut self, element: impl IntoElement) -> Self {
        self.aside = Some(element.into_any_element());
        self
    }
    pub fn footer(mut self, element: impl IntoElement) -> Self {
        self.footer = Some(element.into_any_element());
        self
    }
    pub fn side_width(mut self, pixels: f32) -> Self {
        self.side_width = pixels.max(0.);
        self
    }
    pub fn aside_width(mut self, pixels: f32) -> Self {
        self.aside_width = pixels.max(0.);
        self
    }
    pub fn theme(mut self, theme: Theme) -> Self {
        self.theme = theme;
        self
    }
}
impl Default for Layout {
    fn default() -> Self {
        Self::new()
    }
}
impl Styled for Layout {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}
impl ParentElement for Layout {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.content.extend(elements)
    }
}
impl RenderOnce for Layout {
    fn render(self, _: &mut Window, _: &mut App) -> impl IntoElement {
        let c = self.theme.colors;
        let mut el = div()
            .flex()
            .flex_col()
            .size_full()
            .min_w_0()
            .min_h_0()
            .bg(c.canvas)
            .text_color(c.text);
        el.style().refine(&self.style);
        el.children(self.header.map(|element| {
            div()
                .flex_none()
                .w_full()
                .border_b_1()
                .border_color(c.border)
                .child(element)
        }))
        .child(
            div()
                .flex()
                .flex_1()
                .min_h_0()
                .min_w_0()
                .children(self.side.map(|element| {
                    div()
                        .w(px(self.side_width))
                        .h_full()
                        .flex_none()
                        .border_r_1()
                        .border_color(c.border)
                        .child(element)
                }))
                .child(
                    div()
                        .flex()
                        .flex_col()
                        .flex_1()
                        .min_w_0()
                        .min_h_0()
                        .children(self.content),
                )
                .children(self.aside.map(|element| {
                    div()
                        .w(px(self.aside_width))
                        .h_full()
                        .flex_none()
                        .border_l_1()
                        .border_color(c.border)
                        .child(element)
                })),
        )
        .children(self.footer.map(|element| {
            div()
                .flex_none()
                .w_full()
                .border_t_1()
                .border_color(c.border)
                .child(element)
        }))
    }
}

type NavigationChange = Rc<dyn Fn(bool, &mut Window, &mut App)>;
/// A desktop application shell with controlled navigation visibility.
#[derive(IntoElement)]
pub struct AppShell {
    id: ElementId,
    title: SharedString,
    nav: Option<AnyElement>,
    open: bool,
    on_change: Option<NavigationChange>,
    content: Vec<AnyElement>,
    theme: Theme,
    style: StyleRefinement,
}
impl AppShell {
    pub fn new(id: impl Into<ElementId>, title: impl Into<SharedString>) -> Self {
        Self {
            id: id.into(),
            title: title.into(),
            nav: None,
            open: true,
            on_change: None,
            content: vec![],
            theme: Theme::default(),
            style: StyleRefinement::default(),
        }
    }
    pub fn navigation(mut self, nav: impl IntoElement) -> Self {
        self.nav = Some(nav.into_any_element());
        self
    }
    pub fn navigation_open(mut self, open: bool) -> Self {
        self.open = open;
        self
    }
    pub fn on_navigation_change(
        mut self,
        handler: impl Fn(bool, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_change = Some(Rc::new(handler));
        self
    }
    pub fn theme(mut self, theme: Theme) -> Self {
        self.theme = theme;
        self
    }
}
impl Styled for AppShell {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}
impl ParentElement for AppShell {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.content.extend(elements)
    }
}
impl RenderOnce for AppShell {
    fn render(self, _: &mut Window, _: &mut App) -> impl IntoElement {
        let theme = self.theme;
        let c = theme.colors;
        let open = self.open;
        let callback = self.on_change;
        let toggle = Button::new(
            (self.id, "navigation"),
            if open {
                "Hide navigation"
            } else {
                "Show navigation"
            },
        )
        .theme(theme)
        .variant(ButtonVariant::Ghost)
        .on_click(move |_, window, cx| {
            if let Some(handler) = &callback {
                handler(!open, window, cx);
            }
        });
        let mut layout = Layout::new().theme(theme).header(
            div()
                .flex()
                .items_center()
                .justify_between()
                .h(px(52.))
                .px(px(16.))
                .bg(c.surface)
                .child(
                    div()
                        .font_weight(gpui::FontWeight::MEDIUM)
                        .child(self.title),
                )
                .child(toggle),
        );
        if open && let Some(nav) = self.nav {
            layout = layout.side_nav(nav);
        }
        layout.style().refine(&self.style);
        layout.children(self.content)
    }
}

/// Shared-action grouping with native roving keyboard focus.
#[derive(IntoElement)]
pub struct ButtonGroup {
    base: gpui_base::Toolbar,
    theme: Theme,
    style: StyleRefinement,
    children: Vec<AnyElement>,
}
impl ButtonGroup {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            base: gpui_base::Toolbar::new(id),
            theme: Theme::default(),
            style: StyleRefinement::default(),
            children: vec![],
        }
    }
    pub fn theme(mut self, theme: Theme) -> Self {
        self.theme = theme;
        self
    }
}
impl Styled for ButtonGroup {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}
impl ParentElement for ButtonGroup {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements)
    }
}
impl RenderOnce for ButtonGroup {
    fn render(self, _: &mut Window, _: &mut App) -> impl IntoElement {
        use gpui_base::StyledExt as _;
        self.base
            .flex()
            .items_center()
            .gap(px(4.))
            .p(px(4.))
            .rounded(px(10.))
            .bg(self.theme.colors.neutral)
            .children(self.children)
            .refine_style(&self.style)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn layout_inputs_are_bounded() {
        assert_eq!(Grid::new(0).columns, 1);
        assert_eq!(AspectRatio::new(f32::NAN).ratio, 1.);
        assert_eq!(AspectRatio::new(-1.).ratio, 1.);
        assert_eq!(Layout::new().side_width(-1.).side_width, 0.);
    }
}
