use std::rc::Rc;

use crate::Theme;
use gpui::{
    Anchor, AnyElement, App, AppContext as _, Context, ElementId, Entity, FocusHandle,
    Focusable as _, InteractiveElement as _, IntoElement, MouseButton, ParentElement, RenderOnce,
    Role, StatefulInteractiveElement as _, StyleRefinement, Styled, Subscription, Window, div,
    prelude::FluentBuilder as _, px,
};
use gpui_base::StyledExt as _;

pub use gpui_base::PopoverState;
type Content =
    Box<dyn FnOnce(&mut PopoverState, &mut Window, &mut Context<PopoverState>) -> AnyElement>;
type OpenChange = Rc<dyn Fn(bool, &mut Window, &mut App)>;

struct Lifecycle {
    native: Entity<PopoverState>,
    on_change: Option<OpenChange>,
    last_open: bool,
    rendered_open: bool,
    _observation: Subscription,
}

/// An anchored native popover. Open state can be owner-controlled; GPUI's native
/// state handles Escape/outside dismissal and focus capture/return. Trigger
/// keyboard handling is scoped to the trigger so Enter in a content button
/// activates that button instead of toggling the entire popover.
#[derive(IntoElement)]
pub struct Popover {
    id: ElementId,
    theme: Theme,
    open: Option<bool>,
    disabled: bool,
    anchor: Anchor,
    tracked_focus: Option<FocusHandle>,
    overlay_closable: bool,
    trigger: Option<AnyElement>,
    on_open_change: Option<OpenChange>,
    content: Option<Content>,
    style: StyleRefinement,
}

impl Popover {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            theme: Theme::default(),
            open: None,
            disabled: false,
            anchor: Anchor::TopLeft,
            tracked_focus: None,
            overlay_closable: true,
            trigger: None,
            on_open_change: None,
            content: None,
            style: StyleRefinement::default(),
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
    /// Suppresses opening and closes a displayed popup, preserving focus return.
    /// Disable the supplied interactive trigger as well when its own action is disabled.
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
    pub fn anchor(mut self, anchor: Anchor) -> Self {
        self.anchor = anchor;
        self
    }
    pub fn track_focus(mut self, focus: &FocusHandle) -> Self {
        self.tracked_focus = Some(focus.clone());
        self
    }
    pub fn overlay_closable(mut self, closable: bool) -> Self {
        self.overlay_closable = closable;
        self
    }
    pub fn on_open_change(
        mut self,
        handler: impl Fn(bool, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_open_change = Some(Rc::new(handler));
        self
    }
    pub fn trigger(mut self, trigger: impl IntoElement + 'static) -> Self {
        self.trigger = Some(trigger.into_any_element());
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
        &mut self.style
    }
}
impl RenderOnce for Popover {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let lifecycle = window.use_keyed_state((self.id.clone(), "lifecycle"), cx, |window, cx| {
            let native = cx.new(|cx| PopoverState::new(false, cx));
            let observation = cx.observe_in(
                &native,
                window,
                |this: &mut Lifecycle, native, window, cx| {
                    let open = native.read(cx).is_open();
                    if this.last_open != open {
                        this.last_open = open;
                        if let Some(callback) = this.on_change.clone() {
                            callback(open, window, cx);
                        }
                        window.refresh();
                    }
                },
            );
            Lifecycle {
                native,
                on_change: None,
                last_open: false,
                rendered_open: false,
                _observation: observation,
            }
        });
        let native = lifecycle.update(cx, |state, cx| {
            state.on_change = self.on_open_change;
            if let Some(open) = if self.disabled {
                Some(false)
            } else {
                self.open
            } {
                state.last_open = open;
                state
                    .native
                    .update(cx, |native, cx| native.sync_open(open, window, cx));
            }
            let open = state.native.read(cx).is_open();
            if open
                && !state.rendered_open
                && let Some(focus) = self.tracked_focus.as_ref()
            {
                focus.focus(window, cx);
            }
            state.rendered_open = open;
            state.native.clone()
        });
        let open = native.read(cx).is_open();
        let focus = native.read(cx).focus_handle(cx);
        let Some(trigger) = self.trigger else {
            return div().into_any_element();
        };
        let mouse_state = native.clone();
        let key_state = native.clone();
        let trigger = div()
            .id((self.id.clone(), "trigger"))
            .when(!self.disabled, |trigger| {
                trigger
                    .on_mouse_down(MouseButton::Left, move |_, window, cx| {
                        cx.stop_propagation();
                        mouse_state.update(cx, |state, cx| state.toggle_open(window, cx));
                    })
                    .capture_key_down(move |event, window, cx| {
                        if event.keystroke.modifiers == gpui::Modifiers::default()
                            && matches!(event.keystroke.key.as_str(), "enter" | "space")
                        {
                            cx.stop_propagation();
                            key_state.update(cx, |state, cx| state.toggle_open(window, cx));
                        }
                    })
            })
            .child(trigger);
        let popup = gpui_base::Popup::new(self.id.clone(), trigger)
            .anchor(self.anchor)
            .offset(px(6.))
            .refine_style(&self.style);
        if !open {
            return popup.into_any_element();
        }
        let colors = self.theme.colors;
        let content = native.update(cx, |state, cx| {
            self.content.map(|content| content(state, window, cx))
        });
        let outside = native.clone();
        let surface = div()
            .id((self.id, "content"))
            .role(Role::Dialog)
            .occlude()
            .tab_group()
            .track_focus(&focus)
            .key_context("Popover")
            .on_action(window.listener_for(&native, PopoverState::on_action_cancel))
            .when(self.overlay_closable, |el| {
                el.on_mouse_down_out(move |_, window, cx| {
                    outside.update(cx, |state, cx| state.dismiss(window, cx));
                })
            })
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
            .children(content);
        popup.content(surface).into_any_element()
    }
}
