use std::{rc::Rc, time::Duration};

use gpui::{
    AnyElement, App, Context, ElementId, InteractiveElement as _, IntoElement, ParentElement,
    RenderOnce, StyleRefinement, Styled, Window, div, px,
};
use gpui_base::StyledExt as _;

use crate::{Popover, Theme};

pub use gpui_base::HoverCardState;
type Content =
    Box<dyn FnOnce(&mut HoverCardState, &mut Window, &mut Context<HoverCardState>) -> AnyElement>;
type OpenHandler = Rc<dyn Fn(bool, &mut Window, &mut App)>;

/// A delayed, hover-preserving desktop preview using the native hover lifecycle.
/// Hover-open state and pending timers are transient presentation state; callbacks report changes.
#[derive(IntoElement)]
pub struct HoverCard {
    base: gpui_base::HoverCard,
    theme: Theme,
    id: ElementId,
    content: Option<Content>,
    style: StyleRefinement,
}
impl HoverCard {
    pub fn new(id: impl Into<ElementId>) -> Self {
        let id = id.into();
        Self {
            base: gpui_base::HoverCard::new(id.clone()),
            theme: Theme::default(),
            id,
            content: None,
            style: StyleRefinement::default(),
        }
    }
    pub fn theme(mut self, theme: Theme) -> Self {
        self.theme = theme;
        self
    }
    pub fn trigger(mut self, trigger: impl IntoElement) -> Self {
        self.base = self.base.trigger(trigger);
        self
    }
    pub fn open_delay(mut self, delay: Duration) -> Self {
        self.base = self.base.open_delay(delay);
        self
    }
    pub fn close_delay(mut self, delay: Duration) -> Self {
        self.base = self.base.close_delay(delay);
        self
    }
    pub fn on_open_change(
        mut self,
        callback: impl Fn(bool, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.base = self
            .base
            .on_open_change(move |open, window, cx| callback(*open, window, cx));
        self
    }
    pub fn content<E: IntoElement>(
        mut self,
        content: impl FnOnce(&mut HoverCardState, &mut Window, &mut Context<HoverCardState>) -> E
        + 'static,
    ) -> Self {
        self.content = Some(Box::new(move |state, window, cx| {
            content(state, window, cx).into_any_element()
        }));
        self
    }
}
impl Styled for HoverCard {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}
impl RenderOnce for HoverCard {
    fn render(self, _: &mut Window, _: &mut App) -> impl IntoElement {
        let c = self.theme.colors;
        self.base.content(move |state, window, cx| {
            div()
                .id((self.id, "hover-surface"))
                .flex()
                .flex_col()
                .min_w_0()
                .max_w(px(360.))
                .p(px(12.))
                .gap(px(8.))
                .rounded(px(12.))
                .border_1()
                .border_color(c.border_emphasized)
                .bg(c.surface)
                .text_color(c.text)
                .text_size(px(14.))
                .line_height(px(20.))
                .children(self.content.map(|content| content(state, window, cx)))
                .refine_style(&self.style)
        })
    }
}

/// A compact information button that opens explanatory content by pointer or keyboard.
#[derive(IntoElement)]
pub struct InfoTip {
    id: ElementId,
    label: gpui::SharedString,
    theme: Theme,
    open: Option<bool>,
    on_change: Option<OpenHandler>,
}
impl InfoTip {
    pub fn new(id: impl Into<ElementId>, label: impl Into<gpui::SharedString>) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            theme: Theme::default(),
            open: None,
            on_change: None,
        }
    }
    pub fn theme(mut self, theme: Theme) -> Self {
        self.theme = theme;
        self
    }
    pub fn open(mut self, open: bool) -> Self {
        self.open = Some(open);
        self
    }
    pub fn on_open_change(
        mut self,
        handler: impl Fn(bool, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_change = Some(Rc::new(handler));
        self
    }
}
impl RenderOnce for InfoTip {
    fn render(self, _: &mut Window, _: &mut App) -> impl IntoElement {
        let c = self.theme.colors;
        let mut popover = Popover::new(self.id.clone())
            .theme(self.theme)
            .trigger(
                gpui_base::Button::new((self.id, "info-trigger"))
                    .accessibility_label("More information")
                    .size(px(24.))
                    .rounded_full()
                    .text_color(c.muted)
                    .child("ⓘ"),
            )
            .content(move |_, _, _| self.label);
        if let Some(open) = self.open {
            popover = popover.open(open);
        }
        if let Some(handler) = self.on_change {
            popover = popover.on_open_change(move |open, window, cx| handler(open, window, cx));
        }
        popover
    }
}
