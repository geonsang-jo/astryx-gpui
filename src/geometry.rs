//! Native paging, pointer gestures, scrolling and resizable content.
use std::rc::Rc;

use gpui::{
    AnyElement, App, Axis, ElementId, Entity, FocusHandle, InteractiveElement as _, IntoElement,
    MouseButton, ParentElement, Pixels, Point, RenderOnce, ScrollHandle,
    StatefulInteractiveElement as _, StyleRefinement, Styled, Window, div, point,
    prelude::FluentBuilder as _, px,
};
use gpui_base::StyledExt as _;

use crate::{Button, ButtonVariant, ControlSize, Popover, Theme};
pub use gpui_base::{ResizablePanelEvent, ResizableState};

type PageChange = Rc<dyn Fn(usize, &mut Window, &mut App)>;
type OpenChange = Rc<dyn Fn(bool, &mut Window, &mut App)>;
type ResizeChange = Rc<dyn Fn(Vec<Pixels>, &mut Window, &mut App)>;

fn page_target(current: usize, count: usize, forward: bool, wrap: bool) -> Option<usize> {
    if count < 2 {
        return None;
    }
    let current = current.min(count - 1);
    if forward {
        (current + 1 < count)
            .then_some(current + 1)
            .or(wrap.then_some(0))
    } else {
        current.checked_sub(1).or(wrap.then_some(count - 1))
    }
}

fn own_focus(window: &mut Window, cx: &mut App, id: ElementId) -> FocusHandle {
    let state = window.use_keyed_state((id, "geometry-focus"), cx, |_, cx| cx.focus_handle());
    state.read(cx).clone()
}

macro_rules! styled {
    ($name:ident) => {
        impl Styled for $name {
            fn style(&mut self) -> &mut StyleRefinement {
                &mut self.style
            }
        }
    };
}
macro_rules! parent {
    ($name:ident) => {
        impl ParentElement for $name {
            fn extend(&mut self, children: impl IntoIterator<Item = AnyElement>) {
                self.children.extend(children);
            }
        }
    };
}

/// A controlled one-page carousel with buttons and root-scoped arrow navigation.
/// The host applies the requested index; child editors retain their arrow keys.
#[derive(IntoElement)]
pub struct Carousel {
    id: ElementId,
    children: Vec<AnyElement>,
    index: usize,
    wrap: bool,
    disabled: bool,
    theme: Theme,
    on_change: Option<PageChange>,
    focus: Option<FocusHandle>,
    style: StyleRefinement,
}
impl Carousel {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            children: vec![],
            index: 0,
            wrap: false,
            disabled: false,
            theme: Theme::default(),
            on_change: None,
            focus: None,
            style: StyleRefinement::default(),
        }
    }
    pub fn focus_handle(mut self, focus: &FocusHandle) -> Self {
        self.focus = Some(focus.clone());
        self
    }
    pub fn index(mut self, index: usize) -> Self {
        self.index = index;
        self
    }
    pub fn wrap(mut self, wrap: bool) -> Self {
        self.wrap = wrap;
        self
    }
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
    pub fn theme(mut self, theme: Theme) -> Self {
        self.theme = theme;
        self
    }
    pub fn on_change(mut self, callback: impl Fn(usize, &mut Window, &mut App) + 'static) -> Self {
        self.on_change = Some(Rc::new(callback));
        self
    }
}
styled!(Carousel);
parent!(Carousel);
impl RenderOnce for Carousel {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let count = self.children.len();
        let index = self.index.min(count.saturating_sub(1));
        let focus = self
            .focus
            .unwrap_or_else(|| own_focus(window, cx, self.id.clone()));
        let key_focus = focus.clone();
        let change = self.on_change.clone();
        let disabled = self.disabled;
        let wrap = self.wrap;
        let previous = page_target(index, count, false, wrap);
        let next = page_target(index, count, true, wrap);
        let prev_change = self.on_change.clone();
        let next_change = self.on_change;
        let mut view = div()
            .id(self.id.clone())
            .flex()
            .flex_col()
            .min_w_0()
            .gap(px(8.))
            .text_color(self.theme.colors.text)
            .when(!disabled, |el| el.track_focus(&focus.tab_stop(true)))
            .aria_label("Carousel")
            .aria_value(format!("{} of {}", usize::from(count > 0) + index, count))
            .on_key_down(move |event, window, cx| {
                let modifiers = &event.keystroke.modifiers;
                if disabled
                    || !key_focus.is_focused(window)
                    || modifiers.alt
                    || modifiers.control
                    || modifiers.platform
                    || modifiers.shift
                {
                    return;
                }
                let key = event.keystroke.key.as_str();
                let target = match key {
                    "left" => previous,
                    "right" => next,
                    "home" if count > 0 => Some(0),
                    "end" if count > 0 => Some(count - 1),
                    _ => None,
                };
                if let Some(target) = target {
                    cx.stop_propagation();
                    if let Some(callback) = &change {
                        let callback = callback.clone();
                        window.defer(cx, move |window, cx| callback(target, window, cx));
                    }
                }
            });
        if let Some(page) = self.children.into_iter().nth(index) {
            view = view.child(
                div()
                    .id((self.id.clone(), "page"))
                    .w_full()
                    .min_w_0()
                    .child(page),
            );
        }
        view.child(
            div()
                .flex()
                .items_center()
                .gap(px(8.))
                .child(
                    Button::new((self.id.clone(), "previous"), "‹")
                        .theme(self.theme)
                        .variant(ButtonVariant::Ghost)
                        .size(ControlSize::Small)
                        .accessibility_label("Previous slide")
                        .disabled(disabled || previous.is_none())
                        .on_click(move |_, window, cx| {
                            if let (Some(target), Some(callback)) = (previous, &prev_change) {
                                let callback = callback.clone();
                                window.defer(cx, move |window, cx| callback(target, window, cx));
                            }
                        }),
                )
                .child(
                    div()
                        .text_size(px(12.))
                        .line_height(px(20.))
                        .text_color(self.theme.colors.muted)
                        .child(format!("{} / {}", usize::from(count > 0) + index, count)),
                )
                .child(
                    Button::new((self.id, "next"), "›")
                        .theme(self.theme)
                        .variant(ButtonVariant::Ghost)
                        .size(ControlSize::Small)
                        .accessibility_label("Next slide")
                        .disabled(disabled || next.is_none())
                        .on_click(move |_, window, cx| {
                            if let (Some(target), Some(callback)) = (next, &next_change) {
                                let callback = callback.clone();
                                window.defer(cx, move |window, cx| callback(target, window, cx));
                            }
                        }),
                ),
        )
        .refine_style(&self.style)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SwipeDirection {
    Left,
    Right,
    Up,
    Down,
}
type SwipeChange = Rc<dyn Fn(SwipeDirection, &mut Window, &mut App)>;

fn swipe_direction(
    start: Point<Pixels>,
    end: Point<Pixels>,
    axis: Axis,
    threshold: f32,
) -> Option<SwipeDirection> {
    let x = f32::from(end.x - start.x);
    let y = f32::from(end.y - start.y);
    let (along, across) = if axis == Axis::Horizontal {
        (x, y)
    } else {
        (y, x)
    };
    if !along.is_finite() || along.abs() < threshold || along.abs() < across.abs() * 1.2 {
        return None;
    }
    Some(match (axis, along.is_sign_positive()) {
        (Axis::Horizontal, false) => SwipeDirection::Left,
        (Axis::Horizontal, true) => SwipeDirection::Right,
        (_, false) => SwipeDirection::Up,
        (_, true) => SwipeDirection::Down,
    })
}

/// Desktop pointer-swipe recognition without a drag preview or editing state.
/// A regular click never requests a swipe. Outside release cancels the gesture.
#[derive(IntoElement)]
pub struct Swipe {
    id: ElementId,
    children: Vec<AnyElement>,
    axis: Axis,
    threshold: f32,
    disabled: bool,
    on_swipe: Option<SwipeChange>,
    style: StyleRefinement,
}
impl Swipe {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            children: vec![],
            axis: Axis::Horizontal,
            threshold: 40.,
            disabled: false,
            on_swipe: None,
            style: StyleRefinement::default(),
        }
    }
    pub fn axis(mut self, axis: Axis) -> Self {
        self.axis = axis;
        self
    }
    pub fn threshold(mut self, pixels: f32) -> Self {
        if pixels.is_finite() && pixels > 0. {
            self.threshold = pixels;
        }
        self
    }
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
    pub fn on_swipe(
        mut self,
        callback: impl Fn(SwipeDirection, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_swipe = Some(Rc::new(callback));
        self
    }
}
styled!(Swipe);
parent!(Swipe);
impl RenderOnce for Swipe {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let press = window.use_keyed_state((self.id.clone(), "swipe-press"), cx, |_, _| {
            None::<Point<Pixels>>
        });
        let down = press.clone();
        let up = press.clone();
        let outside = press;
        let axis = self.axis;
        let threshold = self.threshold;
        let disabled = self.disabled;
        div()
            .id(self.id)
            .min_w_0()
            .children(self.children)
            .on_mouse_down(MouseButton::Left, move |event, _, cx| {
                down.update(cx, |start, _| {
                    *start = (!disabled
                        && !event.modifiers.alt
                        && !event.modifiers.control
                        && !event.modifiers.platform)
                        .then_some(event.position);
                });
            })
            .on_mouse_up(MouseButton::Left, move |event, window, cx| {
                let start = up.update(cx, |start, _| start.take());
                if !disabled
                    && let Some(direction) = start
                        .and_then(|start| swipe_direction(start, event.position, axis, threshold))
                    && let Some(callback) = &self.on_swipe
                {
                    callback(direction, window, cx);
                }
            })
            .on_mouse_up_out(MouseButton::Left, move |_, _, cx| {
                outside.update(cx, |start, _| *start = None);
            })
            .refine_style(&self.style)
    }
}

/// Discrete native horizontal paging. Geometry is explicit in logical pixels;
/// pointer swipes, horizontal wheel gestures and root arrow keys request pages.
#[derive(IntoElement)]
pub struct SnapPaging {
    id: ElementId,
    children: Vec<AnyElement>,
    index: usize,
    width: f32,
    handle: Option<ScrollHandle>,
    on_change: Option<PageChange>,
    focus: Option<FocusHandle>,
    style: StyleRefinement,
}
impl SnapPaging {
    pub fn scroll_handle(mut self, handle: &ScrollHandle) -> Self {
        self.handle = Some(handle.clone());
        self
    }
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            children: vec![],
            index: 0,
            width: 320.,
            handle: None,
            on_change: None,
            focus: None,
            style: StyleRefinement::default(),
        }
    }
    pub fn focus_handle(mut self, focus: &FocusHandle) -> Self {
        self.focus = Some(focus.clone());
        self
    }
    pub fn index(mut self, index: usize) -> Self {
        self.index = index;
        self
    }
    pub fn page_width(mut self, width: f32) -> Self {
        if width.is_finite() && width > 0. {
            self.width = width;
        }
        self
    }
    pub fn on_change(mut self, callback: impl Fn(usize, &mut Window, &mut App) + 'static) -> Self {
        self.on_change = Some(Rc::new(callback));
        self
    }
}
styled!(SnapPaging);
parent!(SnapPaging);
impl RenderOnce for SnapPaging {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let count = self.children.len();
        let index = self.index.min(count.saturating_sub(1));
        let handle = self.handle.unwrap_or_else(|| {
            window
                .use_keyed_state((self.id.clone(), "snap-scroll"), cx, |_, _| {
                    ScrollHandle::new()
                })
                .read(cx)
                .clone()
        });
        handle.set_offset(point(px(-(index as f32) * self.width), px(0.)));
        let focus = self
            .focus
            .unwrap_or_else(|| own_focus(window, cx, self.id.clone()));
        let key_focus = focus.clone();
        let key_change = self.on_change.clone();
        let wheel_change = self.on_change.clone();
        let swipe_change = self.on_change;
        let pages = div()
            .id((self.id.clone(), "viewport"))
            .w(px(self.width))
            .overflow_x_scroll()
            .track_scroll(&handle)
            .child(
                div().flex().w(px(self.width * count as f32)).children(
                    self.children
                        .into_iter()
                        .map(|page| div().w(px(self.width)).flex_none().child(page)),
                ),
            );
        Swipe::new((self.id.clone(), "swipe"))
            .on_swipe(move |direction, window, cx| {
                let forward = direction == SwipeDirection::Left;
                if let (Some(target), Some(callback)) =
                    (page_target(index, count, forward, false), &swipe_change)
                {
                    let callback = callback.clone();
                    window.defer(cx, move |window, cx| callback(target, window, cx));
                }
            })
            .child(
                div()
                    .id(self.id)
                    .w(px(self.width))
                    .min_w_0()
                    .track_focus(&focus.tab_stop(true))
                    .aria_label("Paged content")
                    .aria_value(format!("{} of {}", usize::from(count > 0) + index, count))
                    .on_key_down(move |event, window, cx| {
                        let m = &event.keystroke.modifiers;
                        if !key_focus.is_focused(window)
                            || m.alt
                            || m.control
                            || m.platform
                            || m.shift
                        {
                            return;
                        }
                        let target = match event.keystroke.key.as_str() {
                            "left" => page_target(index, count, false, false),
                            "right" => page_target(index, count, true, false),
                            _ => None,
                        };
                        if let (Some(target), Some(callback)) = (target, &key_change) {
                            cx.stop_propagation();
                            let callback = callback.clone();
                            window.defer(cx, move |window, cx| callback(target, window, cx));
                        }
                    })
                    .on_scroll_wheel(move |event, window, cx| {
                        let delta = event.delta.pixel_delta(px(24.));
                        if delta.x.abs() >= px(20.)
                            && delta.x.abs() > delta.y.abs()
                            && let (Some(target), Some(callback)) = (
                                page_target(index, count, delta.x < px(0.), false),
                                &wheel_change,
                            )
                        {
                            cx.stop_propagation();
                            let callback = callback.clone();
                            window.defer(cx, move |window, cx| callback(target, window, cx));
                        }
                    })
                    .child(pages),
            )
            .refine_style(&self.style)
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum ScrollAxis {
    #[default]
    Vertical,
    Horizontal,
    Both,
}

/// A native scroll viewport with retained offset and themed overlay scrollbars.
#[derive(IntoElement)]
pub struct ScrollableArea {
    id: ElementId,
    children: Vec<AnyElement>,
    axis: ScrollAxis,
    handle: Option<ScrollHandle>,
    theme: Theme,
    style: StyleRefinement,
}
impl ScrollableArea {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            children: vec![],
            axis: ScrollAxis::Vertical,
            handle: None,
            theme: Theme::default(),
            style: StyleRefinement::default(),
        }
    }
    pub fn axis(mut self, axis: ScrollAxis) -> Self {
        self.axis = axis;
        self
    }
    pub fn scroll_handle(mut self, handle: &ScrollHandle) -> Self {
        self.handle = Some(handle.clone());
        self
    }
    pub fn theme(mut self, theme: Theme) -> Self {
        self.theme = theme;
        self
    }
}
styled!(ScrollableArea);
parent!(ScrollableArea);
impl RenderOnce for ScrollableArea {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let handle = self.handle.unwrap_or_else(|| {
            window
                .use_keyed_state((self.id.clone(), "scroll"), cx, |_, _| ScrollHandle::new())
                .read(cx)
                .clone()
        });
        let c = self.theme.colors;
        let viewport = div()
            .id((self.id.clone(), "viewport"))
            .size_full()
            .min_w_0()
            .min_h_0()
            .track_scroll(&handle)
            .flex()
            .when(self.axis == ScrollAxis::Horizontal, |el| el.flex_row())
            .when(self.axis != ScrollAxis::Horizontal, |el| el.flex_col())
            .when(self.axis != ScrollAxis::Horizontal, |el| {
                el.overflow_y_scroll()
            })
            .when(self.axis != ScrollAxis::Vertical, |el| {
                el.overflow_x_scroll()
            })
            .children(self.children);
        div()
            .id(self.id)
            .relative()
            .min_w_0()
            .min_h_0()
            .child(viewport)
            .when(self.axis != ScrollAxis::Horizontal, |el| {
                let bar = gpui_base::Scrollbar::vertical(&handle)
                    .styles(|s| s.thumb(|t| t.bg(c.faint).width(px(5.)).radius(px(3.))));
                el.child(div().absolute().inset_0().child(bar))
            })
            .when(self.axis != ScrollAxis::Vertical, |el| {
                let bar = gpui_base::Scrollbar::horizontal(&handle)
                    .styles(|s| s.thumb(|t| t.bg(c.faint).width(px(5.)).radius(px(3.))));
                el.child(div().absolute().inset_0().child(bar))
            })
            .refine_style(&self.style)
    }
}

/// A panel configuration for the native resizable group.
pub struct ResizablePane {
    children: Vec<AnyElement>,
    size: Option<f32>,
    minimum: f32,
    maximum: f32,
}
impl ResizablePane {
    pub fn new() -> Self {
        Self {
            children: vec![],
            size: None,
            minimum: 48.,
            maximum: 100_000.,
        }
    }
    pub fn size(mut self, size: f32) -> Self {
        if size.is_finite() && size > 0. {
            self.size = Some(size);
        }
        self
    }
    pub fn size_range(mut self, minimum: f32, maximum: f32) -> Self {
        if minimum.is_finite() && maximum.is_finite() && minimum > 0. && maximum >= minimum {
            self.minimum = minimum;
            self.maximum = maximum;
        }
        self
    }
    fn native(self) -> gpui_base::ResizablePanel {
        gpui_base::resizable_panel()
            .size_range(px(self.minimum)..px(self.maximum))
            .when_some(self.size, |panel, size| panel.size(px(size)))
            .children(self.children)
    }
}
impl Default for ResizablePane {
    fn default() -> Self {
        Self::new()
    }
}
parent!(ResizablePane);

/// Astryx divider appearance around native constrained resize behavior.
/// The owner retains `ResizableState` and can persist sizes from `on_resize`.
#[derive(IntoElement)]
pub struct Resizable {
    id: ElementId,
    state: Entity<ResizableState>,
    panels: Vec<ResizablePane>,
    axis: Axis,
    theme: Theme,
    on_resize: Option<ResizeChange>,
    style: StyleRefinement,
}
impl Resizable {
    pub fn new(id: impl Into<ElementId>, state: &Entity<ResizableState>) -> Self {
        Self {
            id: id.into(),
            state: state.clone(),
            panels: vec![],
            axis: Axis::Horizontal,
            theme: Theme::default(),
            on_resize: None,
            style: StyleRefinement::default(),
        }
    }
    pub fn panel(mut self, panel: ResizablePane) -> Self {
        self.panels.push(panel);
        self
    }
    pub fn axis(mut self, axis: Axis) -> Self {
        self.axis = axis;
        self
    }
    pub fn theme(mut self, theme: Theme) -> Self {
        self.theme = theme;
        self
    }
    pub fn on_resize(
        mut self,
        callback: impl Fn(Vec<Pixels>, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_resize = Some(Rc::new(callback));
        self
    }
}
styled!(Resizable);
impl RenderOnce for Resizable {
    fn render(self, _: &mut Window, _: &mut App) -> impl IntoElement {
        let c = self.theme.colors;
        let group = gpui_base::ResizablePanelGroup::new(self.id.clone())
            .with_state(&self.state)
            .axis(self.axis)
            .children(self.panels.into_iter().map(ResizablePane::native))
            .with_handle_appearance(Rc::new(move |handle, _, _| {
                Some(
                    div()
                        .bg(c.border_emphasized)
                        .when(handle.axis() == Axis::Horizontal, |el| {
                            el.w(px(1.)).h_full()
                        })
                        .when(handle.axis() == Axis::Vertical, |el| el.h(px(1.)).w_full())
                        .into_any_element(),
                )
            }))
            .on_resize(move |state, window, cx| {
                if let Some(callback) = &self.on_resize {
                    callback(state.read(cx).sizes().clone(), window, cx);
                }
            });
        div()
            .id(self.id)
            .size_full()
            .min_w_0()
            .min_h_0()
            .child(group)
            .refine_style(&self.style)
    }
}

/// An explicitly capacity-controlled overflow list with a real anchored popup.
/// A host can update `visible_count` from its own layout measurements.
#[derive(IntoElement)]
pub struct OverflowList {
    id: ElementId,
    children: Vec<AnyElement>,
    visible_count: usize,
    open: bool,
    theme: Theme,
    on_open_change: Option<OpenChange>,
    focus: Option<FocusHandle>,
    style: StyleRefinement,
}
impl OverflowList {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            children: vec![],
            visible_count: 3,
            open: false,
            theme: Theme::default(),
            on_open_change: None,
            focus: None,
            style: StyleRefinement::default(),
        }
    }
    pub fn visible_count(mut self, count: usize) -> Self {
        self.visible_count = count;
        self
    }
    pub fn open(mut self, open: bool) -> Self {
        self.open = open;
        self
    }
    pub fn theme(mut self, theme: Theme) -> Self {
        self.theme = theme;
        self
    }
    /// Retained fallback focus when capacity removes the overflow trigger.
    /// The list root is programmatically focusable and is not an extra Tab stop.
    pub fn focus_handle(mut self, focus: &FocusHandle) -> Self {
        self.focus = Some(focus.clone());
        self
    }
    pub fn on_open_change(
        mut self,
        callback: impl Fn(bool, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_open_change = Some(Rc::new(callback));
        self
    }
}
styled!(OverflowList);
parent!(OverflowList);
impl RenderOnce for OverflowList {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let visible = self.visible_count.min(self.children.len());
        let hidden_count = self.children.len() - visible;
        let mut items = self.children.into_iter();
        let shown: Vec<_> = items.by_ref().take(visible).collect();
        let hidden: Vec<_> = items.collect();
        let c = self.theme.colors;
        let content_id = (self.id.clone(), "overflow-items");
        let root_focus = self
            .focus
            .unwrap_or_else(|| own_focus(window, cx, self.id.clone()));
        let native_focus =
            window.use_keyed_state((self.id.clone(), "overflow-focus"), cx, |_, cx| {
                (cx.focus_handle(), cx.focus_handle())
            });
        let (trigger_focus, content_focus) = native_focus.read(cx).clone();
        let available = hidden_count > 0;
        if !available
            && (content_focus.contains_focused(window, cx) || trigger_focus.is_focused(window))
        {
            // The old trigger is about to disappear. Move only focus owned by
            // this overflow UI to a root which remains in the native focus tree.
            root_focus.focus(window, cx);
        }
        let trigger = if available {
            Button::new((self.id.clone(), "more"), format!("+{hidden_count}"))
                .theme(self.theme)
                .variant(ButtonVariant::Secondary)
                .size(ControlSize::Small)
                .focus_handle(&trigger_focus)
                .accessibility_label(format!("Show {hidden_count} more items"))
                .into_any_element()
        } else {
            div().into_any_element()
        };
        // Keep the native popup lifecycle mounted even when no overflow remains.
        // Its effective state closes without rewriting the owner's open flag.
        let popup = Popover::new((self.id.clone(), "overflow"))
            .theme(self.theme)
            .open(self.open && available)
            .disabled(!available)
            .track_focus(&content_focus)
            .on_open_change(move |open, window, cx| {
                if let Some(callback) = &self.on_open_change {
                    callback(open, window, cx);
                }
            })
            .trigger(trigger)
            .content(move |_, _, _| {
                div()
                    .id(content_id)
                    .track_focus(&content_focus.tab_stop(false))
                    .flex()
                    .flex_col()
                    .overflow_y_scroll()
                    .min_w(px(160.))
                    .max_h(px(320.))
                    .gap(px(8.))
                    .p(px(12.))
                    .bg(c.surface)
                    .text_color(c.text)
                    .children(hidden)
            })
            .when(!available, |popup| popup.absolute().size(px(0.)));
        div()
            .id(self.id.clone())
            .role(gpui::Role::Group)
            .aria_label("Overflow list")
            .track_focus(&root_focus.tab_stop(false))
            .flex()
            .items_center()
            .min_w_0()
            .gap(px(8.))
            .children(shown)
            .child(popup)
            .refine_style(&self.style)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn page_bounds_and_direction_do_not_invent_targets() {
        assert_eq!(page_target(0, 0, true, true), None);
        assert_eq!(page_target(0, 1, true, true), None);
        assert_eq!(page_target(0, 3, false, false), None);
        assert_eq!(page_target(2, 3, true, false), None);
        assert_eq!(page_target(2, 3, true, true), Some(0));
        assert_eq!(page_target(0, 3, false, true), Some(2));
        assert_eq!(page_target(99, 3, false, false), Some(1));
    }
    #[test]
    fn swipe_requires_distance_and_axis_intent() {
        let start = point(px(100.), px(100.));
        assert_eq!(
            swipe_direction(start, point(px(99.), px(100.)), Axis::Horizontal, 40.),
            None
        );
        assert_eq!(
            swipe_direction(start, point(px(20.), px(170.)), Axis::Horizontal, 40.),
            None
        );
        assert_eq!(
            swipe_direction(start, point(px(20.), px(102.)), Axis::Horizontal, 40.),
            Some(SwipeDirection::Left)
        );
        assert_eq!(
            swipe_direction(start, point(px(102.), px(180.)), Axis::Vertical, 40.),
            Some(SwipeDirection::Down)
        );
    }
}
