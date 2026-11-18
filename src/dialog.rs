use std::rc::Rc;

use gpui::{
    AnyElement, App, ClickEvent, ElementId, FocusHandle, FontWeight, InteractiveElement,
    IntoElement, ParentElement, Refineable as _, RenderOnce, SharedString, StyleRefinement, Styled,
    Window, div, prelude::FluentBuilder as _, px, rgba,
};

use crate::Theme;

pub use gpui_base::DialogChangeReason;

type OpenChange = Rc<dyn Fn(bool, DialogChangeReason, &mut Window, &mut App)>;
type Decision = Rc<dyn Fn(&ClickEvent, &mut Window, &mut App) -> bool>;

struct DialogFocus {
    handle: FocusHandle,
    previous: Option<FocusHandle>,
    was_open: bool,
}

/// An owner-controlled modal with a Neutral surface and native focus trap.
///
/// Apply `on_open_change` values back to `open`. Use `on_cancel` to refuse a
/// dismissal (for example while saving). Enter confirms only when an explicit
/// `on_confirm` decision has been supplied; buttons and inputs keep their own
/// native Enter behavior.
#[derive(IntoElement)]
pub struct Dialog {
    id: ElementId,
    open: bool,
    title: Option<SharedString>,
    theme: Theme,
    dismissible: bool,
    on_open_change: Option<OpenChange>,
    on_cancel: Decision,
    on_confirm: Decision,
    children: Vec<AnyElement>,
    style: StyleRefinement,
}

impl Dialog {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            open: false,
            title: None,
            theme: Theme::default(),
            dismissible: true,
            on_open_change: None,
            on_cancel: Rc::new(|_, _, _| true),
            on_confirm: Rc::new(|_, _, _| false),
            children: Vec::new(),
            style: StyleRefinement::default(),
        }
    }

    pub fn open(mut self, open: bool) -> Self {
        self.open = open;
        self
    }

    pub fn title(mut self, title: impl Into<SharedString>) -> Self {
        self.title = Some(title.into());
        self
    }

    pub fn theme(mut self, theme: Theme) -> Self {
        self.theme = theme;
        self
    }

    pub fn dismissible(mut self, dismissible: bool) -> Self {
        self.dismissible = dismissible;
        self
    }

    pub fn on_open_change(
        mut self,
        handler: impl Fn(bool, DialogChangeReason, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_open_change = Some(Rc::new(handler));
        self
    }

    pub fn on_cancel(
        mut self,
        handler: impl Fn(&ClickEvent, &mut Window, &mut App) -> bool + 'static,
    ) -> Self {
        self.on_cancel = Rc::new(handler);
        self
    }

    pub fn on_confirm(
        mut self,
        handler: impl Fn(&ClickEvent, &mut Window, &mut App) -> bool + 'static,
    ) -> Self {
        self.on_confirm = Rc::new(handler);
        self
    }
}

impl Styled for Dialog {
    /// Refines the visible dialog surface, not its full-window backdrop.
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl ParentElement for Dialog {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for Dialog {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let state = window.use_keyed_state(self.id.clone(), cx, |_, cx| DialogFocus {
            handle: cx.focus_handle(),
            previous: None,
            was_open: false,
        });
        let focus = state.update(cx, |state, cx| {
            if self.open != state.was_open {
                if self.open {
                    state.previous = window.focused(cx);
                    state.handle.focus(window, cx);
                } else if let Some(previous) = state.previous.take()
                    && state.handle.contains_focused(window, cx)
                {
                    previous.focus(window, cx);
                }
                state.was_open = self.open;
            }
            state.handle.clone()
        });
        if !self.open {
            return div().into_any_element();
        }

        let colors = self.theme.colors;
        let mut surface = gpui_base::DialogPopup::new()
            .flex()
            .flex_col()
            .min_w_0()
            .w(px(480.))
            .max_w(window.viewport_size().width - px(32.))
            .max_h(window.viewport_size().height - px(32.))
            .p(px(16.))
            .gap(px(16.))
            .rounded(px(12.))
            .border_1()
            .border_color(colors.border)
            .bg(colors.surface)
            .text_color(colors.text)
            .text_size(px(14.))
            .line_height(px(20.));
        surface.style().refine(&self.style);
        surface = surface
            .when_some(self.title, |surface, title| {
                surface.child(
                    gpui_base::DialogTitle::new()
                        .text_size(px(17.))
                        .line_height(px(24.))
                        .font_weight(FontWeight::SEMIBOLD)
                        .child(title),
                )
            })
            .children(self.children);
        let callback = self.on_open_change;
        let change_state = state.clone();
        let cancel = self.on_cancel;
        let confirm = self.on_confirm;
        let confirm_focus = focus.clone();
        div()
            .id(self.id)
            .child(
                gpui_base::Dialog::new(cx)
                    .focus_handle(focus)
                    .open(true)
                    .close_on_escape(self.dismissible)
                    .close_on_backdrop_press(self.dismissible)
                    .on_cancel(move |event, window, cx| cancel(event, window, cx))
                    .on_ok(move |event, window, cx| {
                        if !confirm_focus.is_focused(window) {
                            cx.propagate();
                            return false;
                        }
                        confirm(event, window, cx)
                    })
                    .on_open_change(move |open, reason, window, cx| {
                        if let Some(callback) = callback.as_ref() {
                            callback(open, reason, window, cx);
                        }
                        if !open {
                            change_state.update(cx, |state, cx| {
                                if let Some(previous) = state.previous.take()
                                    && state.handle.contains_focused(window, cx)
                                {
                                    previous.focus(window, cx);
                                }
                            });
                        }
                    })
                    .backdrop(div().absolute().inset_0().bg(rgba(0x00000052)))
                    .popup(surface),
            )
            .into_any_element()
    }
}
