use astryx_gpui::{
    Button, Chat, ChatMessage, ChatRole, ClickableCard, CodeBlock, Layer, SegmentedControl,
    SelectOption, SelectableCard, TextareaState, ToggleButton,
};
use gpui::{
    AppContext, Context, Entity, EntityInputHandler as _, IntoElement, KeyDownEvent, KeyUpEvent,
    Keystroke, Modifiers, ParentElement, Render, SharedString, Styled, TestAppContext,
    VisualTestContext, Window, div, point, px,
};
use gpui_base::ElementExt as _;
use std::{cell::Cell, rc::Rc};

#[derive(Clone, Copy)]
enum Kind {
    Chat,
    Copy,
    Card,
    Selectable,
    Toggle,
    Segments,
    Layer,
}
struct Harness {
    kind: Kind,
    input: Entity<TextareaState>,
    other_input: Entity<TextareaState>,
    swap: bool,
    sending: bool,
    sent: Vec<SharedString>,
    value: bool,
    disabled: bool,
    selected: SharedString,
    changes: usize,
    cancels: usize,
    layouts: Rc<Cell<usize>>,
}
impl Render for Harness {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let owner = cx.entity().downgrade();
        let cancel = cx.entity().downgrade();
        let layouts = self.layouts.clone();
        let child = match self.kind {
            Kind::Chat => Chat::new("chat")
                .messages(vec![ChatMessage::new(
                    "intro",
                    ChatRole::Assistant,
                    "Ready to help.",
                )])
                .input(if self.swap {
                    &self.other_input
                } else {
                    &self.input
                })
                .sending(self.sending)
                .on_send(move |text, _, cx| {
                    let _ = owner.update(cx, |this, cx| {
                        this.sent.push(text);
                        cx.notify();
                    });
                })
                .on_cancel(move |_, cx| {
                    let _ = cancel.update(cx, |this, cx| {
                        this.cancels += 1;
                        cx.notify();
                    });
                })
                .w(px(440.))
                .h(px(400.))
                .into_any_element(),
            Kind::Copy => CodeBlock::new("code", "const value = `literal`;\n```")
                .language("js")
                .w(px(440.))
                .into_any_element(),
            Kind::Card => ClickableCard::new("card", "Open task")
                .disabled(self.disabled)
                .child("Task")
                .on_click(cx.listener(|this, _, _, cx| {
                    this.changes += 1;
                    cx.notify();
                }))
                .w(px(180.))
                .into_any_element(),
            Kind::Selectable => SelectableCard::new("card", "Select task")
                .disabled(self.disabled)
                .selected(self.value)
                .child("Task")
                .on_change(move |value, _, cx| {
                    let _ = owner.update(cx, |this, cx| {
                        this.value = value;
                        this.changes += 1;
                        cx.notify();
                    });
                })
                .w(px(180.))
                .into_any_element(),
            Kind::Toggle => ToggleButton::new("toggle", "Pin")
                .disabled(self.disabled)
                .pressed(self.value)
                .on_change(move |value, _, cx| {
                    let _ = owner.update(cx, |this, cx| {
                        this.value = value;
                        this.changes += 1;
                        cx.notify();
                    });
                })
                .into_any_element(),
            Kind::Segments => SegmentedControl::new("segments")
                .selected(self.selected.clone())
                .items(vec![
                    SelectOption::new("a", "Alpha"),
                    SelectOption::new("b", "Beta").disabled(true),
                    SelectOption::new("c", "Gamma"),
                ])
                .on_change(move |selected, _, cx| {
                    let _ = owner.update(cx, |this, cx| {
                        this.selected = selected;
                        this.changes += 1;
                        cx.notify();
                    });
                })
                .into_any_element(),
            Kind::Layer => Layer::new("layer", Button::new("trigger", "Anchor"))
                .open(self.value)
                .child(
                    div()
                        .size(px(40.))
                        .on_prepaint(move |_, _, _| layouts.set(layouts.get() + 1)),
                )
                .into_any_element(),
        };
        div().w(px(600.)).h(px(500.)).child(child)
    }
}
fn mount(cx: &mut TestAppContext, kind: Kind) -> (Entity<Harness>, &mut VisualTestContext) {
    cx.update(astryx_gpui::init);
    let (view, cx) = cx.add_window_view(move |window, cx| Harness {
        kind,
        input: cx.new(|cx| TextareaState::new(window, cx)),
        other_input: cx.new(|cx| TextareaState::new(window, cx)),
        swap: false,
        sending: false,
        sent: vec![],
        value: false,
        disabled: false,
        selected: "a".into(),
        changes: 0,
        cancels: 0,
        layouts: Rc::new(Cell::new(0)),
    });
    cx.update(|window, cx| window.draw(cx).clear(cx));
    (view, cx)
}
fn key(cx: &mut VisualTestContext, key: &str) {
    let keystroke = Keystroke::parse(key).unwrap();
    cx.simulate_event(KeyDownEvent {
        keystroke: keystroke.clone(),
        is_held: false,
        prefer_character_input: false,
    });
    cx.simulate_event(KeyUpEvent { keystroke });
    cx.update(|window, cx| window.draw(cx).clear(cx));
}
fn focus_text(view: &Entity<Harness>, cx: &mut VisualTestContext, text: &str) {
    let input = view.read_with(cx, |this, _| {
        if this.swap {
            this.other_input.clone()
        } else {
            this.input.clone()
        }
    });
    cx.update(|window, cx| {
        input.update(cx, |state, cx| {
            state.set_value(text, window, cx);
            state.focus(window, cx);
        })
    });
    cx.update(|window, cx| window.draw(cx).clear(cx));
}

#[gpui::test]
fn chat_enter_sends_once_shift_enter_edits_and_pending_blocks_submission(cx: &mut TestAppContext) {
    let (view, cx) = mount(cx, Kind::Chat);
    focus_text(&view, cx, "안녕하세요");
    key(cx, "enter");
    assert_eq!(
        view.read_with(cx, |this, _| this.sent.clone()),
        vec![SharedString::from("안녕하세요")]
    );
    key(cx, "shift-enter");
    assert_eq!(view.read_with(cx, |this, _| this.sent.len()), 1);
    view.update(cx, |this, cx| {
        this.sending = true;
        cx.notify();
    });
    cx.update(|window, cx| window.draw(cx).clear(cx));
    key(cx, "enter");
    assert_eq!(view.read_with(cx, |this, _| this.sent.len()), 1);
}
#[gpui::test]
fn chat_waits_for_composition_and_rebinds_native_input_on_swap(cx: &mut TestAppContext) {
    let (view, cx) = mount(cx, Kind::Chat);
    let input = view.read_with(cx, |this, _| this.input.clone());
    cx.update(|window, cx| {
        input.update(cx, |state, cx| {
            state.focus(window, cx);
            state.replace_and_mark_text_in_range(None, "메", Some(1..1), window, cx);
        })
    });
    cx.update(|window, cx| window.draw(cx).clear(cx));
    key(cx, "enter");
    assert!(view.read_with(cx, |this, _| this.sent.is_empty()));
    view.update(cx, |this, cx| {
        this.swap = true;
        cx.notify();
    });
    cx.update(|window, cx| window.draw(cx).clear(cx));
    focus_text(&view, cx, "new draft");
    key(cx, "enter");
    assert_eq!(
        view.read_with(cx, |this, _| this.sent.clone()),
        vec![SharedString::from("new draft")]
    );
}
#[gpui::test]
fn code_block_copy_preserves_original_code_not_rendered_markdown(cx: &mut TestAppContext) {
    let (_, cx) = mount(cx, Kind::Copy);
    cx.update(|window, cx| window.focus_next(cx));
    key(cx, "enter");
    cx.update(|_, cx| {
        assert_eq!(
            cx.read_from_clipboard().and_then(|item| item.text()),
            Some("const value = `literal`;\n```".into())
        )
    });
}
#[gpui::test]
fn clickable_card_uses_native_activation_and_disabled_guard(cx: &mut TestAppContext) {
    let (view, cx) = mount(cx, Kind::Card);
    cx.simulate_click(point(px(15.), px(15.)), Modifiers::default());
    key(cx, "enter");
    assert_eq!(view.read_with(cx, |this, _| this.changes), 2);
    view.update(cx, |this, cx| {
        this.disabled = true;
        cx.notify();
    });
    cx.update(|window, cx| window.draw(cx).clear(cx));
    key(cx, "space");
    assert_eq!(view.read_with(cx, |this, _| this.changes), 2);
}
#[gpui::test]
fn selectable_card_and_toggle_report_owner_values(cx: &mut TestAppContext) {
    let (view, cx) = mount(cx, Kind::Selectable);
    cx.simulate_click(point(px(15.), px(15.)), Modifiers::default());
    assert!(view.read_with(cx, |this, _| this.value));
    key(cx, "space");
    assert!(!view.read_with(cx, |this, _| this.value));
    view.update(cx, |this, cx| {
        this.kind = Kind::Toggle;
        cx.notify();
    });
    cx.update(|window, cx| window.draw(cx).clear(cx));
    cx.simulate_click(point(px(10.), px(10.)), Modifiers::default());
    key(cx, "enter");
    assert!(!view.read_with(cx, |this, _| this.value));
    assert_eq!(view.read_with(cx, |this, _| this.changes), 4);
}
#[gpui::test]
fn segmented_radios_skip_disabled_and_leave_modified_arrows_to_host(cx: &mut TestAppContext) {
    let (view, cx) = mount(cx, Kind::Segments);
    cx.update(|window, cx| window.focus_next(cx));
    key(cx, "right");
    assert_eq!(view.read_with(cx, |this, _| this.selected.to_string()), "c");
    key(cx, "home");
    assert_eq!(view.read_with(cx, |this, _| this.selected.to_string()), "a");
    key(cx, "ctrl-right");
    assert_eq!(view.read_with(cx, |this, _| this.changes), 2);
}
#[gpui::test]
fn context_layer_never_lays_out_hidden_popup_content(cx: &mut TestAppContext) {
    let (view, cx) = mount(cx, Kind::Layer);
    assert_eq!(view.read_with(cx, |this, _| this.layouts.get()), 0);
    view.update(cx, |this, cx| {
        this.value = true;
        cx.notify();
    });
    for _ in 0..2 {
        cx.update(|window, cx| window.draw(cx).clear(cx));
    }
    assert!(view.read_with(cx, |this, _| this.layouts.get()) > 0);
    view.update(cx, |this, cx| {
        this.value = false;
        cx.notify();
    });
    cx.update(|window, cx| window.draw(cx).clear(cx));
    let count = view.read_with(cx, |this, _| this.layouts.get());
    cx.update(|window, cx| window.draw(cx).clear(cx));
    assert_eq!(view.read_with(cx, |this, _| this.layouts.get()), count);
}
