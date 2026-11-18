use crate::Theme;
use gpui::{
    Anchor, AnyElement, App, Context, ElementId, FocusHandle, IntoElement, ParentElement,
    RenderOnce, StyleRefinement, Styled, Window, px,
};

pub use gpui_base::PopoverState;
type Content =
    Box<dyn FnOnce(&mut PopoverState, &mut Window, &mut Context<PopoverState>) -> AnyElement>;

/// A controlled native popover. GPUI owns anchored placement, Escape/outside
/// dismissal, keyed focus identity, and returning focus to the trigger.
#[derive(IntoElement)]
pub struct Popover {
    base: gpui_base::Popover,
    theme: Theme,
    content: Option<Content>,
}

impl Popover {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            base: gpui_base::Popover::new(id).offset(px(6.)),
            theme: Theme::default(),
            content: None,
        }
    }

    pub fn theme(mut self, theme: Theme) -> Self {
        self.theme = theme;
        self
    }

    pub fn open(mut self, open: bool) -> Self {
        self.base = self.base.open(open);
        self
    }

    pub fn anchor(mut self, anchor: Anchor) -> Self {
        self.base = self.base.anchor(anchor);
        self
    }

    pub fn track_focus(mut self, focus: &FocusHandle) -> Self {
        self.base = self.base.track_focus(focus);
        self
    }

    pub fn overlay_closable(mut self, closable: bool) -> Self {
        self.base = self.base.overlay_closable(closable);
        self
    }

    pub fn on_open_change(
        mut self,
        handler: impl Fn(bool, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.base = self
            .base
            .on_open_change(move |open, window, cx| handler(*open, window, cx));
        self
    }

    pub fn trigger(mut self, trigger: impl IntoElement + 'static) -> Self {
        self.base = self
            .base
            .trigger_with(move |_, _, _| trigger.into_any_element());
        self
    }

    pub fn content<E: IntoElement>(
        mut self,
        content: impl FnOnce(&mut PopoverState, &mut Window, &mut Context<PopoverState>) -> E + 'static,
    ) -> Self {
        self.content = Some(Box::new(move |state, window, cx| {
            content(state, window, cx).into_any_element()
        }));
        self
    }
}

impl Styled for Popover {
    /// Styles the trigger's layout container. Style content inside `content`.
    fn style(&mut self) -> &mut StyleRefinement {
        self.base.style()
    }
}

impl RenderOnce for Popover {
    fn render(self, _: &mut Window, _: &mut App) -> impl IntoElement {
        let colors = self.theme.colors;
        self.base.content(move |state, window, cx| {
            gpui::div()
                .flex()
                .flex_col()
                .min_w_0()
                .p(px(12.))
                .gap(px(8.))
                .rounded(px(12.))
                .border_1()
                .border_color(colors.border_emphasized)
                .bg(colors.surface)
                .text_color(colors.text)
                .text_size(px(14.))
                .line_height(px(20.))
                .children(self.content.map(|content| content(state, window, cx)))
        })
    }
}
