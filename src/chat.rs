use crate::{Button, ButtonVariant, InputEvent, Markdown, Spinner, TextArea, TextareaState, Theme};
use gpui::{
    App, ElementId, Entity, EntityId, EntityInputHandler as _, InteractiveElement as _,
    IntoElement, ParentElement, Refineable as _, RenderOnce, SharedString,
    StatefulInteractiveElement as _, StyleRefinement, Styled, Subscription, Window, div,
    prelude::FluentBuilder as _, px,
};
use std::{cell::RefCell, rc::Rc};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ChatRole {
    User,
    #[default]
    Assistant,
    System,
}
#[derive(Clone, Debug)]
pub struct ChatMessage {
    pub id: SharedString,
    pub role: ChatRole,
    pub content: SharedString,
    pub pending: bool,
}
impl ChatMessage {
    pub fn new(
        id: impl Into<SharedString>,
        role: ChatRole,
        content: impl Into<SharedString>,
    ) -> Self {
        Self {
            id: id.into(),
            role,
            content: content.into(),
            pending: false,
        }
    }
    pub fn pending(mut self, pending: bool) -> Self {
        self.pending = pending;
        self
    }
}
type Send = Rc<dyn Fn(SharedString, &mut Window, &mut App)>;
type Cancel = Rc<dyn Fn(&mut Window, &mut App)>;
struct Handlers {
    input: EntityId,
    send: Option<Send>,
    blocked: bool,
    enter: bool,
}
struct ComposerWatch {
    input: EntityId,
    policy: bool,
    handlers: Rc<RefCell<Handlers>>,
    _observe: Subscription,
    _submit: Subscription,
}

fn submit(
    input: &Entity<TextareaState>,
    handlers: &Rc<RefCell<Handlers>>,
    window: &mut Window,
    cx: &mut App,
) {
    let callback = {
        let handlers = handlers.borrow();
        if handlers.blocked || handlers.input != input.entity_id() {
            return;
        }
        handlers.send.clone()
    };
    if input.update(cx, |state, cx| {
        state.marked_text_range(window, cx).is_some()
    }) {
        return;
    }
    let state = input.read(cx);
    if state.presentation().is_disabled() || state.presentation().is_readonly() {
        return;
    }
    let text = state.value().to_string();
    if !text.trim().is_empty()
        && let Some(callback) = callback
    {
        callback(text.into(), window, cx);
    }
}

/// Controlled native message thread and composer. Providers, persistence,
/// clearing accepted drafts and scroll-follow policy remain explicit host work.
/// Enter submission defaults on, with Shift+Enter preserving native newlines.
#[derive(IntoElement)]
pub struct Chat {
    id: ElementId,
    messages: Vec<ChatMessage>,
    input: Option<Entity<TextareaState>>,
    send: Option<Send>,
    cancel: Option<Cancel>,
    sending: bool,
    send_disabled: bool,
    send_on_enter: bool,
    theme: Theme,
    scroll: Option<gpui::ScrollHandle>,
    style: StyleRefinement,
}
impl Chat {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            messages: vec![],
            input: None,
            send: None,
            cancel: None,
            sending: false,
            send_disabled: false,
            send_on_enter: true,
            theme: Theme::default(),
            scroll: None,
            style: StyleRefinement::default(),
        }
    }
    pub fn messages(mut self, messages: Vec<ChatMessage>) -> Self {
        self.messages = messages;
        self
    }
    pub fn input(mut self, input: &Entity<TextareaState>) -> Self {
        self.input = Some(input.clone());
        self
    }
    pub fn sending(mut self, sending: bool) -> Self {
        self.sending = sending;
        self
    }
    pub fn send_disabled(mut self, disabled: bool) -> Self {
        self.send_disabled = disabled;
        self
    }
    pub fn send_on_enter(mut self, value: bool) -> Self {
        self.send_on_enter = value;
        self
    }
    pub fn on_send(
        mut self,
        handler: impl Fn(SharedString, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.send = Some(Rc::new(handler));
        self
    }
    pub fn on_cancel(mut self, handler: impl Fn(&mut Window, &mut App) + 'static) -> Self {
        self.cancel = Some(Rc::new(handler));
        self
    }
    pub fn scroll_handle(mut self, handle: gpui::ScrollHandle) -> Self {
        self.scroll = Some(handle);
        self
    }
    pub fn theme(mut self, theme: Theme) -> Self {
        self.theme = theme;
        self
    }
}
impl Styled for Chat {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}
impl RenderOnce for Chat {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = self.theme;
        let c = theme.colors;
        let scroll = self.scroll.unwrap_or_else(|| {
            window
                .use_keyed_state((self.id.clone(), "scroll"), cx, |_, _| {
                    gpui::ScrollHandle::new()
                })
                .read(cx)
                .clone()
        });
        let mut root = div()
            .flex()
            .flex_col()
            .min_w_0()
            .min_h_0()
            .gap(px(12.))
            .text_color(c.text)
            .text_size(px(14.))
            .line_height(px(20.));
        root.style().refine(&self.style);
        root = root.child(
            div()
                .id((self.id.clone(), "messages"))
                .flex()
                .flex_col()
                .flex_1()
                .min_h_0()
                .overflow_y_scroll()
                .track_scroll(&scroll)
                .gap(px(12.))
                .children(self.messages.into_iter().map(|message| {
                    let user = message.role == ChatRole::User;
                    div().flex().when(user, |el| el.justify_end()).child(
                        div()
                            .flex()
                            .flex_col()
                            .min_w_0()
                            .max_w(gpui::relative(0.92))
                            .gap(px(6.))
                            .p(px(12.))
                            .rounded(px(12.))
                            .bg(if user { c.neutral } else { c.surface })
                            .child(
                                div()
                                    .text_size(px(12.))
                                    .line_height(px(20.))
                                    .text_color(c.muted)
                                    .child(match message.role {
                                        ChatRole::User => "You",
                                        ChatRole::Assistant => "Assistant",
                                        ChatRole::System => "System",
                                    }),
                            )
                            .child(
                                Markdown::new(
                                    (self.id.clone(), message.id.clone()),
                                    message.content,
                                )
                                .theme(theme),
                            )
                            .when(message.pending, |el| {
                                el.child(Spinner::new((self.id.clone(), message.id)).theme(theme))
                            }),
                    )
                })),
        );
        if let Some(input) = self.input {
            let id = input.entity_id();
            let send = self.send;
            let blocked = self.sending || self.send_disabled || send.is_none();
            let enter = self.send_on_enter;
            let watch =
                window.use_keyed_state((self.id.clone(), "composer-watch"), cx, |window, cx| {
                    input.update(cx, |state, cx| state.set_submit_on_enter(enter, cx));
                    let handlers = Rc::new(RefCell::new(Handlers {
                        input: id,
                        send: send.clone(),
                        blocked,
                        enter,
                    }));
                    let submit_handlers = handlers.clone();
                    ComposerWatch {
                        input: id,
                        policy: enter,
                        handlers,
                        _observe: window.observe(&input, cx, |_, window, _| window.refresh()),
                        _submit: window.subscribe(&input, cx, move |entity, event, window, cx| {
                            let enter = submit_handlers.borrow().enter;
                            if let InputEvent::PressEnter { shift: false, .. } = event
                                && enter
                            {
                                submit(&entity, &submit_handlers, window, cx);
                            }
                        }),
                    }
                });
            watch.update(cx, |watch, cx| {
                if watch.input != id {
                    watch.input = id;
                    watch._observe = window.observe(&input, cx, |_, window, _| window.refresh());
                    let handlers = watch.handlers.clone();
                    watch._submit =
                        window.subscribe(&input, cx, move |entity, event, window, cx| {
                            let enter = handlers.borrow().enter;
                            if let InputEvent::PressEnter { shift: false, .. } = event
                                && enter
                            {
                                submit(&entity, &handlers, window, cx);
                            }
                        });
                    watch.policy = enter;
                    input.update(cx, |state, cx| state.set_submit_on_enter(enter, cx));
                }
                if watch.policy != enter {
                    watch.policy = enter;
                    input.update(cx, |state, cx| state.set_submit_on_enter(enter, cx));
                }
                *watch.handlers.borrow_mut() = Handlers {
                    input: id,
                    send,
                    blocked,
                    enter,
                };
            });
            let handlers = watch.read(cx).handlers.clone();
            let value = input.read(cx).value().to_string();
            let editing = input.read(cx).presentation();
            let disabled = blocked
                || value.trim().is_empty()
                || editing.is_disabled()
                || editing.is_readonly();
            let submit_input = input.clone();
            root =
                root.child(
                    div()
                        .flex()
                        .flex_col()
                        .gap(px(8.))
                        .p(px(12.))
                        .rounded(px(16.))
                        .border_1()
                        .border_color(c.border_emphasized)
                        .bg(c.surface)
                        .child(
                            TextArea::new(&input)
                                .theme(theme)
                                .label("Message")
                                .h(px(84.)),
                        )
                        .child(
                            div()
                                .flex()
                                .items_center()
                                .justify_between()
                                .gap(px(8.))
                                .child(div().text_size(px(12.)).text_color(c.muted).child(
                                    if enter {
                                        "Enter to send · Shift+Enter for a new line"
                                    } else {
                                        "Write a message, then select Send"
                                    },
                                ))
                                .when(self.sending, |el| {
                                    el.children(self.cancel.map(|cancel| {
                                        Button::new((self.id.clone(), "cancel"), "Stop")
                                            .theme(theme)
                                            .variant(ButtonVariant::Outline)
                                            .on_click(move |_, window, cx| cancel(window, cx))
                                    }))
                                })
                                .when(!self.sending, |el| {
                                    el.child(
                                        Button::new((self.id, "send"), "Send")
                                            .theme(theme)
                                            .disabled(disabled)
                                            .on_click(move |_, window, cx| {
                                                submit(&submit_input, &handlers, window, cx)
                                            }),
                                    )
                                }),
                        ),
                );
        }
        root
    }
}
