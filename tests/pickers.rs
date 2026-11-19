use astryx_gpui::{
    ComplexSelector, FileInput, InlineToken, InputContent, InputState, MultiSelector, SelectGroup,
    SelectOption, Token, Tokenizer, Typeahead,
};
use gpui::{
    AppContext, Context, Entity, EntityInputHandler as _, InteractiveElement as _, IntoElement,
    KeyDownEvent, KeyUpEvent, Keystroke, Modifiers, ParentElement, Render, Styled, TestAppContext,
    VisualTestContext, Window, div, point, px,
};
use std::path::PathBuf;
#[derive(Clone, Copy)]
enum Kind {
    Typeahead,
    Multi,
    Grouped,
    File,
    Token,
    Tokenizer,
}
struct Picker {
    kind: Kind,
    input: Entity<InputState>,
    open: bool,
    highlight: usize,
    selected: gpui::SharedString,
    many: Vec<gpui::SharedString>,
    files: Vec<PathBuf>,
    changes: usize,
    disabled: bool,
}
impl Render for Picker {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let owner = cx.entity().downgrade();
        let open_owner = owner.clone();
        let highlight_owner = owner.clone();
        let options = vec![
            SelectOption::new("a", "Alpha"),
            SelectOption::new("b", "Alpine").disabled(true),
            SelectOption::new("c", "Alchemy"),
        ];
        let content = match self.kind {
            Kind::Typeahead => Typeahead::new("typeahead", &self.input)
                .items(options)
                .open(self.open)
                .highlighted(self.highlight)
                .on_open_change(move |open, _, cx| {
                    let _ = open_owner.update(cx, |s, cx| {
                        s.open = open;
                        cx.notify();
                    });
                })
                .on_highlight(move |index, _, cx| {
                    let _ = highlight_owner.update(cx, |s, cx| {
                        s.highlight = index;
                        cx.notify();
                    });
                })
                .on_select(move |value, _, cx| {
                    let _ = owner.update(cx, |s, cx| {
                        s.selected = value;
                        s.changes += 1;
                        cx.notify();
                    });
                })
                .into_any_element(),
            Kind::Multi => MultiSelector::new("multi")
                .items(options)
                .selected(self.many.clone())
                .open(self.open)
                .disabled(self.disabled)
                .on_open_change(move |open, _, cx| {
                    let _ = open_owner.update(cx, |s, cx| {
                        s.open = open;
                        cx.notify();
                    });
                })
                .on_change(move |value, _, cx| {
                    let _ = owner.update(cx, |s, cx| {
                        s.many = value;
                        s.changes += 1;
                        cx.notify();
                    });
                })
                .into_any_element(),
            Kind::Grouped => ComplexSelector::new("groups")
                .groups(vec![
                    SelectGroup::new("First", vec![SelectOption::new("a", "Alpha")]),
                    SelectGroup::new("Second", vec![SelectOption::new("c", "Gamma")]),
                ])
                .query(&self.input)
                .open(self.open)
                .on_open_change(move |open, _, cx| {
                    let _ = open_owner.update(cx, |s, cx| {
                        s.open = open;
                        cx.notify();
                    });
                })
                .on_select(move |value, _, cx| {
                    let _ = owner.update(cx, |s, cx| {
                        s.selected = value;
                        s.changes += 1;
                        cx.notify();
                    });
                })
                .into_any_element(),
            Kind::File => FileInput::new("files")
                .files(self.files.clone())
                .multiple(true)
                .disabled(self.disabled)
                .on_change(move |files, _, cx| {
                    let _ = owner.update(cx, |s, cx| {
                        s.files = files;
                        s.changes += 1;
                        cx.notify();
                    });
                })
                .into_any_element(),
            Kind::Token => Token::new("token", "Example")
                .disabled(self.disabled)
                .on_remove(move |_, cx| {
                    let _ = owner.update(cx, |s, cx| {
                        s.changes += 1;
                        cx.notify();
                    });
                })
                .into_any_element(),
            Kind::Tokenizer => Tokenizer::new(&self.input)
                .on_commit(move |_, _, cx| {
                    let _ = owner.update(cx, |s, cx| {
                        s.changes += 1;
                        cx.notify();
                    });
                })
                .into_any_element(),
        };
        div()
            .id("picker-owner")
            .p(px(10.))
            .w(px(350.))
            .h(px(400.))
            .child(content)
    }
}
fn mount(cx: &mut TestAppContext, kind: Kind) -> (Entity<Picker>, &mut VisualTestContext) {
    cx.update(astryx_gpui::init);
    let (v, cx) = cx.add_window_view(move |w, cx| Picker {
        kind,
        input: cx.new(|cx| InputState::new(w, cx).placeholder("Query")),
        open: false,
        highlight: 0,
        selected: "".into(),
        many: vec![],
        files: vec![],
        changes: 0,
        disabled: false,
    });
    cx.update(|w, cx| w.draw(cx).clear(cx));
    (v, cx)
}
fn key(cx: &mut VisualTestContext, key: &str) {
    let k = Keystroke::parse(key).unwrap();
    cx.simulate_event(KeyDownEvent {
        keystroke: k.clone(),
        is_held: false,
        prefer_character_input: false,
    });
    cx.simulate_event(KeyUpEvent { keystroke: k });
    cx.update(|w, cx| w.draw(cx).clear(cx));
}
#[gpui::test]
fn typeahead_filters_skips_disabled_commits_once_and_keeps_native_input(cx: &mut TestAppContext) {
    let (v, cx) = mount(cx, Kind::Typeahead);
    let input = v.read_with(cx, |s, _| s.input.clone());
    cx.update(|w, cx| {
        input.update(cx, |s, cx| {
            s.set_value("al", w, cx);
            s.focus(w, cx);
        })
    });
    v.update(cx, |s, cx| {
        s.open = true;
        cx.notify();
    });
    cx.update(|w, cx| w.draw(cx).clear(cx));
    key(cx, "down");
    assert_eq!(v.read_with(cx, |s, _| s.highlight), 2);
    key(cx, "enter");
    assert_eq!(
        v.read_with(cx, |s, _| (s.selected.to_string(), s.open, s.changes)),
        ("c".into(), false, 1)
    );
    assert_eq!(input.read_with(cx, |s, _| s.value().to_string()), "Alchemy");
    cx.simulate_input("!");
    assert_eq!(
        input.read_with(cx, |s, _| s.value().to_string()),
        "Alchemy!"
    );
}
#[gpui::test]
fn multiselector_keeps_popup_open_for_multiple_checks(cx: &mut TestAppContext) {
    let (v, cx) = mount(cx, Kind::Multi);
    cx.simulate_click(point(px(35.), px(25.)), Modifiers::default());
    cx.update(|w, cx| w.draw(cx).clear(cx));
    assert!(v.read_with(cx, |s, _| s.open));
    cx.simulate_click(point(px(35.), px(72.)), Modifiers::default());
    cx.update(|w, cx| w.draw(cx).clear(cx));
    assert_eq!(
        v.read_with(cx, |s, _| s.many.clone()),
        vec![gpui::SharedString::from("a")]
    );
    assert!(v.read_with(cx, |s, _| s.open));
    key(cx, "escape");
    assert!(!v.read_with(cx, |s, _| s.open));
}
#[gpui::test]
fn complex_selector_filters_groups_and_commits_the_matching_option(cx: &mut TestAppContext) {
    let (v, cx) = mount(cx, Kind::Grouped);
    let input = v.read_with(cx, |s, _| s.input.clone());
    cx.update(|w, cx| input.update(cx, |s, cx| s.set_value("gam", w, cx)));
    cx.simulate_click(point(px(35.), px(25.)), Modifiers::default());
    cx.update(|w, cx| w.draw(cx).clear(cx));
    cx.simulate_click(point(px(45.), px(146.)), Modifiers::default());
    assert_eq!(
        v.read_with(cx, |s, _| (s.selected.to_string(), s.open, s.changes)),
        ("c".into(), false, 1)
    );
}
#[gpui::test]
fn file_input_uses_native_prompt_and_cancellation_preserves_selection(cx: &mut TestAppContext) {
    let (v, cx) = mount(cx, Kind::File);
    cx.simulate_click(point(px(35.), px(25.)), Modifiers::default());
    assert!(cx.did_prompt_for_paths());
    cx.simulate_path_prompt_response(|options| {
        assert!(options.files && options.multiple && !options.directories);
        Some(vec![PathBuf::from("/synthetic/report.txt")])
    });
    cx.run_until_parked();
    assert_eq!(
        v.read_with(cx, |s, _| s.files.clone()),
        vec![PathBuf::from("/synthetic/report.txt")]
    );
    cx.simulate_click(point(px(35.), px(25.)), Modifiers::default());
    cx.simulate_path_prompt_response(|_| None);
    cx.run_until_parked();
    assert_eq!(v.read_with(cx, |s, _| s.changes), 1);
}
#[gpui::test]
fn disabled_file_input_never_opens_a_system_prompt(cx: &mut TestAppContext) {
    let (v, cx) = mount(cx, Kind::File);
    v.update(cx, |s, cx| {
        s.disabled = true;
        cx.notify();
    });
    cx.update(|w, cx| w.draw(cx).clear(cx));
    cx.simulate_click(point(px(35.), px(25.)), Modifiers::default());
    assert!(!cx.did_prompt_for_paths());
}
#[gpui::test]
fn token_removal_is_controlled_and_disabled_suppresses_it(cx: &mut TestAppContext) {
    let (v, cx) = mount(cx, Kind::Token);
    cx.update(|w, cx| w.focus_next(cx));
    key(cx, "enter");
    assert_eq!(v.read_with(cx, |s, _| s.changes), 1);
    v.update(cx, |s, cx| {
        s.disabled = true;
        cx.notify();
    });
    cx.update(|w, cx| w.draw(cx).clear(cx));
    key(cx, "enter");
    assert_eq!(v.read_with(cx, |s, _| s.changes), 1);
}
#[gpui::test]
fn tokenizer_preserves_atomic_tokens_during_native_backspace(cx: &mut TestAppContext) {
    let (v, cx) = mount(cx, Kind::Tokenizer);
    let input = v.read_with(cx, |s, _| s.input.clone());
    let content = InputContent::new("@Geon")
        .with_token(0..5, InlineToken::new("geon", "@Geon"))
        .unwrap();
    cx.update(|w, cx| {
        input.update(cx, |s, cx| {
            s.set_value(content, w, cx);
            s.focus(w, cx);
        })
    });
    cx.update(|w, cx| w.draw(cx).clear(cx));
    key(cx, "backspace");
    assert_eq!(input.read_with(cx, |s, _| s.value().to_string()), "");
    assert_eq!(input.read_with(cx, |s, _| s.tokens().len()), 0);
}

#[gpui::test]
fn typeahead_does_not_commit_ime_and_escape_cancels_composition_first(cx: &mut TestAppContext) {
    let (v, cx) = mount(cx, Kind::Typeahead);
    let input = v.read_with(cx, |s, _| s.input.clone());
    v.update(cx, |s, cx| {
        s.open = true;
        cx.notify();
    });
    cx.update(|w, cx| {
        input.update(cx, |s, cx| {
            s.focus(w, cx);
            s.replace_and_mark_text_in_range(None, "al", Some(2..2), w, cx);
        })
    });
    cx.update(|w, cx| w.draw(cx).clear(cx));
    key(cx, "enter");
    assert_eq!(v.read_with(cx, |s, _| s.changes), 0);
    assert!(v.read_with(cx, |s, _| s.open));
    cx.update(|w, cx| {
        input.update(cx, |s, cx| {
            s.replace_and_mark_text_in_range(None, "al", Some(2..2), w, cx)
        })
    });
    key(cx, "escape");
    assert!(v.read_with(cx, |s, _| s.open));
    key(cx, "escape");
    assert!(!v.read_with(cx, |s, _| s.open));
}

#[gpui::test]
fn tokenizer_commit_waits_for_composition_and_respects_readonly(cx: &mut TestAppContext) {
    let (v, cx) = mount(cx, Kind::Tokenizer);
    let input = v.read_with(cx, |s, _| s.input.clone());
    cx.update(|w, cx| {
        input.update(cx, |s, cx| {
            s.focus(w, cx);
            s.replace_and_mark_text_in_range(None, "ga", Some(2..2), w, cx);
        })
    });
    key(cx, "enter");
    assert_eq!(v.read_with(cx, |s, _| s.changes), 0);
    cx.update(|w, cx| input.update(cx, |s, cx| s.unmark_text(w, cx)));
    key(cx, "enter");
    assert_eq!(v.read_with(cx, |s, _| s.changes), 1);
    cx.update(|_, cx| input.update(cx, |s, cx| s.set_readonly(true, cx)));
    v.update(cx, |_, cx| cx.notify());
    cx.update(|w, cx| w.draw(cx).clear(cx));
    key(cx, "enter");
    assert_eq!(v.read_with(cx, |s, _| s.changes), 1);
}
