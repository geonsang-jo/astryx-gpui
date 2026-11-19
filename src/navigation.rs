use crate::Theme;
use gpui::{
    AnyElement, App, ClickEvent, ElementId, FocusHandle, InteractiveElement as _, IntoElement,
    ParentElement, RenderOnce, Role, SharedString, StatefulInteractiveElement as _,
    StyleRefinement, Styled, Window, div, prelude::FluentBuilder as _, px,
};
use gpui_base::StyledExt as _;
use std::{collections::HashMap, rc::Rc};
type Select = Rc<dyn Fn(SharedString, &mut Window, &mut App)>;
type Index = Rc<dyn Fn(usize, &mut Window, &mut App)>;

/// Native link semantics with host-injected navigation. No URL is opened during
/// render; `open_with` decides between internal routing and external navigation.
#[derive(IntoElement)]
pub struct Link {
    base: gpui_base::Link,
    label: SharedString,
    theme: Theme,
    style: StyleRefinement,
}
impl Link {
    pub fn new(id: impl Into<ElementId>, label: impl Into<SharedString>) -> Self {
        let label = label.into();
        Self {
            base: gpui_base::Link::new(id).accessibility_label(label.clone()),
            label,
            theme: Theme::default(),
            style: StyleRefinement::default(),
        }
    }
    pub fn href(mut self, href: impl Into<SharedString>) -> Self {
        self.base = self.base.href(href);
        self
    }
    pub fn open_with(
        mut self,
        f: impl Fn(&str, &ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.base = self.base.open_with(f);
        self
    }
    pub fn on_activate(mut self, f: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static) -> Self {
        self.base = self.base.on_activate(f);
        self
    }
    pub fn disabled(mut self, v: bool) -> Self {
        self.base = self.base.disabled(v);
        self
    }
    pub fn theme(mut self, v: Theme) -> Self {
        self.theme = v;
        self
    }
}
impl Styled for Link {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}
impl RenderOnce for Link {
    fn render(self, _: &mut Window, _: &mut App) -> impl IntoElement {
        let c = self.theme.colors;
        self.base
            .text_size(px(14.))
            .line_height(px(20.))
            .text_color(c.info)
            .underline()
            .rounded(px(4.))
            .focus_visible(move |s| s.bg(c.neutral))
            .styles(|styles| styles.disabled(|style| style.opacity(0.5)))
            .child(self.label)
            .refine_style(&self.style)
    }
}

#[derive(Clone, Debug)]
pub struct Breadcrumb {
    pub key: SharedString,
    pub label: SharedString,
    pub disabled: bool,
}
impl Breadcrumb {
    pub fn new(key: impl Into<SharedString>, label: impl Into<SharedString>) -> Self {
        Self {
            key: key.into(),
            label: label.into(),
            disabled: false,
        }
    }
    pub fn disabled(mut self, v: bool) -> Self {
        self.disabled = v;
        self
    }
}
/// A navigation trail. The final item is the current location and is not a link.
#[derive(IntoElement)]
pub struct Breadcrumbs {
    id: ElementId,
    items: Vec<Breadcrumb>,
    theme: Theme,
    change: Option<Select>,
    style: StyleRefinement,
}
impl Breadcrumbs {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            items: vec![],
            theme: Theme::default(),
            change: None,
            style: StyleRefinement::default(),
        }
    }
    pub fn items(mut self, v: Vec<Breadcrumb>) -> Self {
        self.items = v;
        self
    }
    pub fn theme(mut self, v: Theme) -> Self {
        self.theme = v;
        self
    }
    pub fn on_select(mut self, f: impl Fn(SharedString, &mut Window, &mut App) + 'static) -> Self {
        self.change = Some(Rc::new(f));
        self
    }
}
impl Styled for Breadcrumbs {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}
impl RenderOnce for Breadcrumbs {
    fn render(self, _: &mut Window, _: &mut App) -> impl IntoElement {
        let c = self.theme.colors;
        let count = self.items.len();
        let mut row = div()
            .id(self.id.clone())
            .role(Role::Navigation)
            .aria_label("Breadcrumb")
            .flex()
            .items_center()
            .gap(px(8.))
            .text_size(px(14.))
            .line_height(px(20.));
        for (index, item) in self.items.into_iter().enumerate() {
            if index > 0 {
                row = row.child(div().text_color(c.faint).child("/"));
            }
            if index + 1 == count {
                row = row.child(div().text_color(c.text).child(item.label));
            } else {
                let callback = self.change.clone();
                row = row.child(
                    Link::new((self.id.clone(), item.key.clone()), item.label)
                        .href(item.key)
                        .theme(self.theme)
                        .disabled(item.disabled)
                        .open_with(move |href, _, w, cx| {
                            if let Some(f) = &callback {
                                f(href.to_string().into(), w, cx);
                            }
                        }),
                );
            }
        }
        row.refine_style(&self.style)
    }
}

#[derive(Clone, Debug)]
pub struct NavItem {
    pub key: SharedString,
    pub label: SharedString,
    pub disabled: bool,
    pub icon: Option<SharedString>,
    pub badge: Option<SharedString>,
}
impl NavItem {
    pub fn new(key: impl Into<SharedString>, label: impl Into<SharedString>) -> Self {
        Self {
            key: key.into(),
            label: label.into(),
            disabled: false,
            icon: None,
            badge: None,
        }
    }
    pub fn disabled(mut self, v: bool) -> Self {
        self.disabled = v;
        self
    }
    pub fn icon(mut self, path: impl Into<SharedString>) -> Self {
        self.icon = Some(path.into());
        self
    }
    pub fn badge(mut self, label: impl Into<SharedString>) -> Self {
        self.badge = Some(label.into());
        self
    }
}
#[derive(Clone, Copy)]
enum NavLayout {
    Top,
    Side,
    Mobile,
}
#[derive(Default)]
struct NavFocus(HashMap<SharedString, FocusHandle>);
struct Nav {
    id: ElementId,
    items: Vec<NavItem>,
    selected: Option<SharedString>,
    theme: Theme,
    layout: NavLayout,
    change: Option<Select>,
    style: StyleRefinement,
}
impl Nav {
    fn new(id: impl Into<ElementId>, layout: NavLayout) -> Self {
        Self {
            id: id.into(),
            items: vec![],
            selected: None,
            theme: Theme::default(),
            layout,
            change: None,
            style: StyleRefinement::default(),
        }
    }
    fn render(self, window: &mut Window, cx: &mut App) -> AnyElement {
        let c = self.theme.colors;
        let focus = window.use_keyed_state((self.id.clone(), "nav-focus"), cx, |_, _| {
            NavFocus::default()
        });
        let handles = focus.update(cx, |focus, cx| {
            focus
                .0
                .retain(|key, _| self.items.iter().any(|item| &item.key == key));
            self.items
                .iter()
                .map(|item| {
                    focus
                        .0
                        .entry(item.key.clone())
                        .or_insert_with(|| cx.focus_handle())
                        .clone()
                })
                .collect::<Vec<_>>()
        });
        let handles = Rc::new(handles);
        let items = Rc::new(self.items);
        let active = handles
            .iter()
            .position(|handle| handle.is_focused(window))
            .filter(|&index| !items[index].disabled)
            .or_else(|| {
                items
                    .iter()
                    .position(|item| !item.disabled && Some(&item.key) == self.selected.as_ref())
            })
            .or_else(|| items.iter().position(|item| !item.disabled));
        let side = matches!(self.layout, NavLayout::Side);
        let compact = matches!(self.layout, NavLayout::Mobile);
        let mut root = div()
            .id(self.id.clone())
            .role(Role::Navigation)
            .aria_label(if side {
                "Side navigation"
            } else if compact {
                "Mobile navigation"
            } else {
                "Top navigation"
            })
            .flex()
            .gap(px(4.))
            .when(side, |el| el.flex_col().w_full())
            .when(!side, |el| el.items_center())
            .when(compact, |el| {
                el.justify_around().border_t_1().border_color(c.border)
            });
        for (index, item) in items.iter().enumerate() {
            let change = self.change.clone();
            let value = item.key.clone();
            let keyboard = items.clone();
            let keyboard_handles = handles.clone();
            let selected = Some(&item.key) == self.selected.as_ref();
            let mut button = gpui_base::Button::new((self.id.clone(), item.key.clone()))
                .disabled(item.disabled)
                .track_focus(&handles[index])
                .tab_stop(active == Some(index))
                .accessibility_label(if selected {
                    format!("{}, current page", item.label)
                } else {
                    item.label.to_string()
                })
                .flex()
                .gap(px(8.))
                .px(px(12.))
                .h(px(if compact { 56. } else { 40. }))
                .rounded(px(8.))
                .text_size(px(if compact { 12. } else { 14. }))
                .line_height(px(20.))
                .text_color(if selected { c.text } else { c.muted })
                .bg(if selected {
                    c.neutral
                } else {
                    gpui::transparent_black()
                })
                .when(side, |el| el.w_full().justify_start())
                .when(compact, |el| el.flex_col().gap(px(2.)))
                .when(item.disabled, |el| el.opacity(0.5))
                .when(!item.disabled, |el| el.hover(move |s| s.bg(c.hover)))
                .focus_visible(move |s| s.bg(c.neutral))
                .on_click(move |_, w, cx| {
                    if let Some(f) = &change {
                        f(value.clone(), w, cx);
                    }
                })
                .on_key_down(move |event, w, cx| {
                    if event.keystroke.modifiers != gpui::Modifiers::default() {
                        return;
                    }
                    let enabled: Vec<usize> = keyboard
                        .iter()
                        .enumerate()
                        .filter_map(|(i, item)| (!item.disabled).then_some(i))
                        .collect();
                    let Some(at) = enabled.iter().position(|&i| i == index) else {
                        return;
                    };
                    let key = event.keystroke.key.as_str();
                    let backward = if side { "up" } else { "left" };
                    let forward = if side { "down" } else { "right" };
                    let next = if key == backward {
                        Some(enabled[(at + enabled.len() - 1) % enabled.len()])
                    } else if key == forward {
                        Some(enabled[(at + 1) % enabled.len()])
                    } else if key == "home" {
                        enabled.first().copied()
                    } else if key == "end" {
                        enabled.last().copied()
                    } else {
                        None
                    };
                    if let Some(next) = next {
                        cx.stop_propagation();
                        keyboard_handles[next].focus(w, cx);
                    }
                });
            if let Some(icon) = &item.icon {
                button = button.child(
                    gpui::svg()
                        .path(icon.clone())
                        .size(px(16.))
                        .text_color(if selected { c.text } else { c.muted }),
                );
            }
            button = button.child(item.label.clone());
            if let Some(badge) = &item.badge {
                button = button.child(
                    div()
                        .px(px(5.))
                        .rounded(px(4.))
                        .bg(c.neutral)
                        .text_size(px(12.))
                        .child(badge.clone()),
                );
            }
            root = root.child(button);
        }
        root.refine_style(&self.style).into_any_element()
    }
}
macro_rules! nav {
    ($name:ident,$layout:expr,$doc:literal) => {
        #[doc=$doc]
        #[derive(IntoElement)]
        pub struct $name {
            inner: Nav,
        }
        impl $name {
            pub fn new(id: impl Into<ElementId>) -> Self {
                Self {
                    inner: Nav::new(id, $layout),
                }
            }
            pub fn items(mut self, v: Vec<NavItem>) -> Self {
                self.inner.items = v;
                self
            }
            pub fn selected(mut self, v: impl Into<SharedString>) -> Self {
                self.inner.selected = Some(v.into());
                self
            }
            pub fn theme(mut self, v: Theme) -> Self {
                self.inner.theme = v;
                self
            }
            pub fn on_select(
                mut self,
                f: impl Fn(SharedString, &mut Window, &mut App) + 'static,
            ) -> Self {
                self.inner.change = Some(Rc::new(f));
                self
            }
        }
        impl Styled for $name {
            fn style(&mut self) -> &mut StyleRefinement {
                &mut self.inner.style
            }
        }
        impl RenderOnce for $name {
            fn render(self, w: &mut Window, cx: &mut App) -> impl IntoElement {
                self.inner.render(w, cx)
            }
        }
    };
}
nav!(
    TopNav,
    NavLayout::Top,
    "A horizontal controlled page navigation bar with roving focus."
);
nav!(
    SideNav,
    NavLayout::Side,
    "A vertical controlled navigation list with icons and badges."
);
nav!(
    MobileNav,
    NavLayout::Mobile,
    "A compact icon-and-label navigation bar. Hosts choose responsive visibility."
);

/// Controlled page navigation with bounded page items and native range guards.
#[derive(IntoElement)]
pub struct Pagination {
    id: ElementId,
    current: usize,
    total: usize,
    visible: usize,
    disabled: bool,
    theme: Theme,
    change: Option<Index>,
    style: StyleRefinement,
}
impl Pagination {
    pub fn new(id: impl Into<ElementId>, current: usize, total: usize) -> Self {
        Self {
            id: id.into(),
            current,
            total,
            visible: 5,
            disabled: false,
            theme: Theme::default(),
            change: None,
            style: StyleRefinement::default(),
        }
    }
    pub fn visible_pages(mut self, v: usize) -> Self {
        self.visible = v;
        self
    }
    pub fn disabled(mut self, v: bool) -> Self {
        self.disabled = v;
        self
    }
    pub fn theme(mut self, v: Theme) -> Self {
        self.theme = v;
        self
    }
    pub fn on_change(mut self, f: impl Fn(usize, &mut Window, &mut App) + 'static) -> Self {
        self.change = Some(Rc::new(f));
        self
    }
}
impl Styled for Pagination {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}
fn page_button(
    id: impl Into<ElementId>,
    label: impl Into<SharedString>,
    theme: Theme,
) -> gpui_base::Button {
    let c = theme.colors;
    gpui_base::Button::new(id)
        .h(px(32.))
        .min_w(px(32.))
        .px(px(8.))
        .rounded(px(8.))
        .border_1()
        .border_color(c.border)
        .text_size(px(14.))
        .line_height(px(20.))
        .text_color(c.text)
        .hover(move |s| s.bg(c.hover))
        .focus_visible(move |s| s.bg(c.neutral))
        .styles(|s| s.disabled(|s| s.opacity(0.5)))
        .child(label.into())
}
impl RenderOnce for Pagination {
    fn render(self, _: &mut Window, _: &mut App) -> impl IntoElement {
        let c = self.theme.colors;
        let changed = self.change;
        let model = gpui_base::PaginationState::new(self.current, self.total)
            .visible_pages(self.visible)
            .disabled(self.disabled)
            .on_change(move |page, w, cx| {
                if let Some(f) = &changed {
                    f(page, w, cx);
                }
            });
        let previous = model.previous_page();
        let next = model.next_page();
        let previous_state = model.clone();
        let next_state = model.clone();
        let mut root = gpui_base::Pagination::new(self.id.clone(), model.clone())
            .flex()
            .items_center()
            .gap(px(4.))
            .child(
                page_button((self.id.clone(), "previous"), "‹", self.theme)
                    .accessibility_label("Previous page")
                    .disabled(previous.is_none())
                    .on_click(move |_, w, cx| {
                        if let Some(page) = previous {
                            previous_state.request_page(page, w, cx);
                        }
                    }),
            );
        for (index, item) in model.items().into_iter().enumerate() {
            root = root.child(match item {
                gpui_base::PaginationItem::Ellipsis(_) => div()
                    .px(px(6.))
                    .text_color(c.muted)
                    .child("…")
                    .into_any_element(),
                gpui_base::PaginationItem::Page(page) => {
                    let state = model.clone();
                    page_button(
                        (self.id.clone(), format!("page-{index}")),
                        page.to_string(),
                        self.theme,
                    )
                    .disabled(self.disabled)
                    .accessibility_label(format!(
                        "Page {page}{}",
                        if page == model.current_page() {
                            ", current page"
                        } else {
                            ""
                        }
                    ))
                    .bg(if page == model.current_page() {
                        c.neutral
                    } else {
                        gpui::transparent_black()
                    })
                    .on_click(move |_, w, cx| state.request_page(page, w, cx))
                    .into_any_element()
                }
            });
        }
        root.child(
            page_button((self.id, "next"), "›", self.theme)
                .accessibility_label("Next page")
                .disabled(next.is_none())
                .on_click(move |_, w, cx| {
                    if let Some(page) = next {
                        next_state.request_page(page, w, cx);
                    }
                }),
        )
        .refine_style(&self.style)
    }
}

#[derive(Clone, Debug)]
pub struct Step {
    pub label: SharedString,
    pub disabled: bool,
}
impl Step {
    pub fn new(label: impl Into<SharedString>) -> Self {
        Self {
            label: label.into(),
            disabled: false,
        }
    }
    pub fn disabled(mut self, v: bool) -> Self {
        self.disabled = v;
        self
    }
}
/// A controlled workflow progress indicator with optional step-navigation actions.
#[derive(IntoElement)]
pub struct Stepper {
    id: ElementId,
    steps: Vec<Step>,
    current: usize,
    vertical: bool,
    theme: Theme,
    change: Option<Index>,
    style: StyleRefinement,
}
impl Stepper {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            steps: vec![],
            current: 0,
            vertical: false,
            theme: Theme::default(),
            change: None,
            style: StyleRefinement::default(),
        }
    }
    pub fn steps(mut self, v: Vec<Step>) -> Self {
        self.steps = v;
        self
    }
    pub fn current(mut self, v: usize) -> Self {
        self.current = v;
        self
    }
    pub fn vertical(mut self, v: bool) -> Self {
        self.vertical = v;
        self
    }
    pub fn theme(mut self, v: Theme) -> Self {
        self.theme = v;
        self
    }
    pub fn on_select(mut self, f: impl Fn(usize, &mut Window, &mut App) + 'static) -> Self {
        self.change = Some(Rc::new(f));
        self
    }
}
impl Styled for Stepper {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}
impl RenderOnce for Stepper {
    fn render(self, _: &mut Window, _: &mut App) -> impl IntoElement {
        let c = self.theme.colors;
        let count = self.steps.len();
        let current = self.current.min(count.saturating_sub(1));
        let mut root = div()
            .id(self.id.clone())
            .role(Role::Group)
            .aria_label("Progress steps")
            .flex()
            .gap(px(12.))
            .when(self.vertical, |el| el.flex_col());
        for (index, step) in self.steps.into_iter().enumerate() {
            let active = index == current;
            let complete = index < current;
            let content = div()
                .flex()
                .items_center()
                .gap(px(8.))
                .text_size(px(14.))
                .line_height(px(20.))
                .text_color(c.text)
                .when(step.disabled, |el| el.opacity(0.5))
                .child(
                    div()
                        .size(px(24.))
                        .rounded_full()
                        .flex()
                        .items_center()
                        .justify_center()
                        .text_size(px(12.))
                        .bg(if active {
                            c.accent
                        } else if complete {
                            c.success_background
                        } else {
                            c.neutral
                        })
                        .text_color(if active {
                            c.on_accent
                        } else if complete {
                            c.success_foreground
                        } else {
                            c.muted
                        })
                        .child(if complete {
                            "✓".to_string()
                        } else {
                            (index + 1).to_string()
                        }),
                )
                .child(step.label.clone());
            let name = format!(
                "Step {} of {count}: {}, {}",
                index + 1,
                step.label,
                if active {
                    "current"
                } else if complete {
                    "complete"
                } else {
                    "upcoming"
                }
            );
            root = if let Some(callback) = self.change.clone() {
                root.child(
                    gpui_base::Button::new((self.id.clone(), index.to_string()))
                        .disabled(step.disabled)
                        .accessibility_label(name)
                        .on_click(move |_, w, cx| callback(index, w, cx))
                        .child(content),
                )
            } else {
                root.child(content)
            };
        }
        root.refine_style(&self.style)
    }
}

/// Native toolbar roving focus. Disabling navigation does not mutate child controls.
#[derive(IntoElement)]
pub struct Toolbar {
    base: gpui_base::Toolbar,
    theme: Theme,
    children: Vec<AnyElement>,
    style: StyleRefinement,
}
impl Toolbar {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            base: gpui_base::Toolbar::new(id),
            theme: Theme::default(),
            children: vec![],
            style: StyleRefinement::default(),
        }
    }
    pub fn theme(mut self, v: Theme) -> Self {
        self.theme = v;
        self
    }
    pub fn disabled(mut self, v: bool) -> Self {
        self.base = self.base.disabled(v);
        self
    }
}
impl Styled for Toolbar {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}
impl ParentElement for Toolbar {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}
impl RenderOnce for Toolbar {
    fn render(self, _: &mut Window, _: &mut App) -> impl IntoElement {
        let c = self.theme.colors;
        self.base
            .flex()
            .items_center()
            .gap(px(4.))
            .p(px(4.))
            .rounded(px(10.))
            .border_1()
            .border_color(c.border)
            .bg(c.surface)
            .text_color(c.text)
            .children(self.children)
            .refine_style(&self.style)
    }
}
