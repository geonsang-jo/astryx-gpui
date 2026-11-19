use crate::{DialogChangeReason, Theme};
use gpui::{
    AnyElement, App, ClickEvent, ElementId, Entity, FocusHandle, FontWeight,
    InteractiveElement as _, IntoElement, ParentElement, RenderOnce, SharedString, StyleRefinement,
    Styled, Window, div, px, rgba,
};
use gpui_base::StyledExt as _;
use std::rc::Rc;

type Decision = Rc<dyn Fn(&ClickEvent, &mut Window, &mut App) -> bool>;
type Change = Rc<dyn Fn(bool, DialogChangeReason, &mut Window, &mut App)>;
struct Focus {
    root: FocusHandle,
    cancel: FocusHandle,
    confirm: FocusHandle,
    previous: Option<FocusHandle>,
    was_open: bool,
    was_busy: bool,
}
fn restore(state: &Entity<Focus>, window: &mut Window, cx: &mut App) {
    state.update(cx, |state, cx| {
        if let Some(previous) = state.previous.take()
            && state.root.contains_focused(window, cx)
        {
            previous.focus(window, cx);
        }
    });
}
fn request(
    state: &Entity<Focus>,
    callback: &Option<Change>,
    reason: DialogChangeReason,
    window: &mut Window,
    cx: &mut App,
) {
    if let Some(callback) = callback {
        callback(false, reason, window, cx);
    }
    restore(state, window, cx);
}

/// A native alert modal with explicit confirm/cancel actions. Backdrop presses
/// never dismiss it. The least-destructive Cancel action receives initial focus.
#[derive(IntoElement)]
pub struct AlertDialog {
    id: ElementId,
    title: SharedString,
    description: Option<SharedString>,
    open: bool,
    busy: bool,
    theme: Theme,
    confirm_label: SharedString,
    cancel_label: SharedString,
    on_confirm: Decision,
    on_cancel: Decision,
    on_change: Option<Change>,
    children: Vec<AnyElement>,
    style: StyleRefinement,
}
impl AlertDialog {
    pub fn new(id: impl Into<ElementId>, title: impl Into<SharedString>) -> Self {
        Self {
            id: id.into(),
            title: title.into(),
            description: None,
            open: false,
            busy: false,
            theme: Theme::default(),
            confirm_label: "Continue".into(),
            cancel_label: "Cancel".into(),
            on_confirm: Rc::new(|_, _, _| true),
            on_cancel: Rc::new(|_, _, _| true),
            on_change: None,
            children: vec![],
            style: StyleRefinement::default(),
        }
    }
    pub fn open(mut self, value: bool) -> Self {
        self.open = value;
        self
    }
    pub fn busy(mut self, value: bool) -> Self {
        self.busy = value;
        self
    }
    pub fn theme(mut self, value: Theme) -> Self {
        self.theme = value;
        self
    }
    pub fn description(mut self, value: impl Into<SharedString>) -> Self {
        self.description = Some(value.into());
        self
    }
    pub fn confirm_label(mut self, value: impl Into<SharedString>) -> Self {
        self.confirm_label = value.into();
        self
    }
    pub fn cancel_label(mut self, value: impl Into<SharedString>) -> Self {
        self.cancel_label = value.into();
        self
    }
    pub fn on_confirm(
        mut self,
        handler: impl Fn(&ClickEvent, &mut Window, &mut App) -> bool + 'static,
    ) -> Self {
        self.on_confirm = Rc::new(handler);
        self
    }
    pub fn on_cancel(
        mut self,
        handler: impl Fn(&ClickEvent, &mut Window, &mut App) -> bool + 'static,
    ) -> Self {
        self.on_cancel = Rc::new(handler);
        self
    }
    pub fn on_open_change(
        mut self,
        handler: impl Fn(bool, DialogChangeReason, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_change = Some(Rc::new(handler));
        self
    }
}
impl Styled for AlertDialog {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}
impl ParentElement for AlertDialog {
    fn extend(&mut self, children: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(children)
    }
}
impl RenderOnce for AlertDialog {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let state = window.use_keyed_state((self.id.clone(), "alert-focus"), cx, |_, cx| Focus {
            root: cx.focus_handle(),
            cancel: cx.focus_handle(),
            confirm: cx.focus_handle(),
            previous: None,
            was_open: false,
            was_busy: false,
        });
        state.update(cx, |state, cx| {
            if self.open != state.was_open {
                if self.open {
                    state.previous = window.focused(cx);
                    if self.busy {
                        state.root.focus(window, cx);
                    } else {
                        state.cancel.focus(window, cx);
                    }
                } else if let Some(previous) = state.previous.take()
                    && state.root.contains_focused(window, cx)
                {
                    previous.focus(window, cx);
                }
                state.was_open = self.open;
            }
            if self.open && self.busy && !state.was_busy && state.root.contains_focused(window, cx)
            {
                state.root.focus(window, cx);
            } else if self.open && !self.busy && state.was_busy && state.root.is_focused(window) {
                state.cancel.focus(window, cx);
            }
            state.was_busy = self.busy;
        });
        if !self.open {
            return div().into_any_element();
        }
        let focus = state.read(cx).root.clone();
        let cancel_focus = state.read(cx).cancel.clone();
        let confirm_focus = state.read(cx).confirm.clone();
        let c = self.theme.colors;
        let cancel = self.on_cancel.clone();
        let cancel_state = state.clone();
        let cancel_change = self.on_change.clone();
        let confirm = self.on_confirm.clone();
        let confirm_state = state.clone();
        let confirm_change = self.on_change.clone();
        let actions = div()
            .flex()
            .justify_end()
            .items_center()
            .gap(px(8.))
            .child(
                gpui_base::Button::new((self.id.clone(), "cancel"))
                    .track_focus(&cancel_focus)
                    .accessibility_label(self.cancel_label.clone())
                    .disabled(self.busy)
                    .h(px(32.))
                    .px(px(12.))
                    .rounded(px(10.))
                    .border_1()
                    .border_color(c.border_emphasized)
                    .bg(c.surface)
                    .text_color(c.text)
                    .focus_visible(move |s| s.border_color(c.focus))
                    .on_click(move |event, window, cx| {
                        cx.stop_propagation();
                        if cancel(event, window, cx) {
                            request(
                                &cancel_state,
                                &cancel_change,
                                DialogChangeReason::Cancel,
                                window,
                                cx,
                            );
                        }
                    })
                    .child(self.cancel_label),
            )
            .child(
                gpui_base::Button::new((self.id.clone(), "confirm"))
                    .track_focus(&confirm_focus)
                    .accessibility_label(self.confirm_label.clone())
                    .disabled(self.busy)
                    .h(px(32.))
                    .px(px(12.))
                    .rounded(px(10.))
                    .border_1()
                    .border_color(gpui::transparent_black())
                    .bg(c.accent)
                    .text_color(c.on_accent)
                    .focus_visible(move |s| s.border_color(c.on_accent))
                    .on_click(move |event, window, cx| {
                        cx.stop_propagation();
                        if confirm(event, window, cx) {
                            request(
                                &confirm_state,
                                &confirm_change,
                                DialogChangeReason::Confirm,
                                window,
                                cx,
                            );
                        }
                    })
                    .child(self.confirm_label),
            );
        let surface = gpui_base::AlertDialogPopup::new()
            .flex()
            .flex_col()
            .w(px(440.))
            .max_w(window.viewport_size().width - px(32.))
            .p(px(16.))
            .gap(px(16.))
            .rounded(px(12.))
            .border_1()
            .border_color(c.border)
            .bg(c.surface)
            .text_color(c.text)
            .text_size(px(14.))
            .line_height(px(20.))
            .child(
                gpui_base::AlertDialogTitle::new()
                    .text_size(px(17.))
                    .line_height(px(24.))
                    .font_weight(FontWeight::SEMIBOLD)
                    .child(self.title),
            )
            .children(self.description.map(|text| {
                gpui_base::AlertDialogDescription::new()
                    .text_color(c.muted)
                    .child(text)
            }))
            .children(self.children)
            .child(actions)
            .refine_style(&self.style);
        let keyboard_focus = focus.clone();
        let busy = self.busy;
        let callback = self.on_change;
        let close_state = state;
        div()
            .id(self.id)
            .child(
                gpui_base::AlertDialog::new(cx)
                    .open(true)
                    .focus_handle(focus)
                    .close_on_escape(!busy)
                    .on_cancel(move |event, window, cx| {
                        !busy && (self.on_cancel)(event, window, cx)
                    })
                    .on_ok(move |event, window, cx| {
                        if !keyboard_focus.is_focused(window) {
                            cx.propagate();
                            return false;
                        }
                        !busy && (self.on_confirm)(event, window, cx)
                    })
                    .on_open_change(move |open, reason, window, cx| {
                        if let Some(callback) = callback.as_ref() {
                            callback(open, reason, window, cx);
                        }
                        if !open {
                            restore(&close_state, window, cx);
                        }
                    })
                    .backdrop(div().absolute().inset_0().bg(rgba(0x00000052)))
                    .popup(surface),
            )
            .into_any_element()
    }
}
