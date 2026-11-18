use astryx_gpui::{Input, InputState, Select, SelectOption, TextArea, TextareaState, Theme};
use gpui::{
    AppContext, Context, Entity, InteractiveElement as _, IntoElement, KeyDownEvent, KeyUpEvent,
    Keystroke, Modifiers, ParentElement, Render, Styled, TestAppContext, VisualTestContext, Window,
    div, point, px,
};

struct Form {
    input: Entity<InputState>,
    area: Entity<TextareaState>,
    selected: gpui::SharedString,
    open: bool,
    highlighted: usize,
    disabled: bool,
    commits: usize,
    dark: bool,
    focus_input_on_select: bool,
    disabled_override: Option<bool>,
}

impl Render for Form {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = if self.dark {
            Theme::dark()
        } else {
            Theme::light()
        };
        let value_owner = cx.entity().downgrade();
        let open_owner = cx.entity().downgrade();
        let highlight_owner = cx.entity().downgrade();
        div()
            .id("form")
            .flex()
            .flex_col()
            .gap(px(12.))
            .p(px(10.))
            .w(px(400.))
            .h(px(400.))
            .child({
                let input = Input::new(&self.input).theme(theme).label("Project");
                if let Some(disabled) = self.disabled_override {
                    input.disabled(disabled)
                } else {
                    input
                }
            })
            .child({
                let area = TextArea::new(&self.area).theme(theme).label("Description");
                if let Some(disabled) = self.disabled_override {
                    area.disabled(disabled)
                } else {
                    area
                }
            })
            .child(
                Select::new("program")
                    .theme(theme)
                    .label("Program")
                    .items(vec![
                        SelectOption::new("a", "Alpha"),
                        SelectOption::new("b", "Beta").disabled(true),
                        SelectOption::new("c", "Gamma"),
                    ])
                    .selected(self.selected.clone())
                    .highlighted(self.highlighted)
                    .open(self.open)
                    .disabled(self.disabled)
                    .on_select(move |value, window, cx| {
                        let _ = value_owner.update(cx, |this, cx| {
                            this.selected = value;
                            this.commits += 1;
                            if this.focus_input_on_select {
                                this.input.update(cx, |state, cx| state.focus(window, cx));
                            }
                            cx.notify();
                        });
                    })
                    .on_open_change(move |open, _, cx| {
                        let _ = open_owner.update(cx, |this, cx| {
                            this.open = open;
                            cx.notify();
                        });
                    })
                    .on_highlight(move |index, _, cx| {
                        let _ = highlight_owner.update(cx, |this, cx| {
                            this.highlighted = index;
                            cx.notify();
                        });
                    }),
            )
    }
}

fn mount(cx: &mut TestAppContext, disabled: bool) -> (Entity<Form>, &mut VisualTestContext) {
    cx.update(astryx_gpui::init);
    let (view, cx) = cx.add_window_view(move |window, cx| Form {
        input: cx.new(|cx| InputState::new(window, cx).placeholder("Project name")),
        area: cx.new(|cx| TextareaState::new(window, cx).placeholder("Description")),
        selected: "a".into(),
        open: false,
        highlighted: 0,
        disabled,
        commits: 0,
        dark: false,
        focus_input_on_select: false,
        disabled_override: None,
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

#[gpui::test]
fn native_input_retains_text_when_theme_and_wrapper_rebuild(cx: &mut TestAppContext) {
    let (view, cx) = mount(cx, false);
    cx.simulate_click(point(px(20.), px(25.)), Modifiers::default());
    cx.simulate_input("프로젝트");
    assert_eq!(
        view.read_with(cx, |form, cx| form.input.read(cx).value().to_string()),
        "프로젝트"
    );
    view.update(cx, |form, cx| {
        form.dark = true;
        cx.notify();
    });
    cx.update(|window, cx| window.draw(cx).clear(cx));
    cx.simulate_input(" 2");
    assert_eq!(
        view.read_with(cx, |form, cx| form.input.read(cx).value().to_string()),
        "프로젝트 2"
    );
}

#[gpui::test]
fn native_disabled_and_readonly_input_reject_edits(cx: &mut TestAppContext) {
    let (view, cx) = mount(cx, false);
    let input = view.read_with(cx, |form, _| form.input.clone());
    cx.update(|window, cx| {
        input.update(cx, |state, cx| {
            state.set_value("original", window, cx);
            state.set_readonly(true, cx);
            state.focus(window, cx);
        })
    });
    cx.simulate_input(" forbidden");
    assert_eq!(
        input.read_with(cx, |state, _| state.value().to_string()),
        "original"
    );
    cx.update(|_, cx| {
        input.update(cx, |state, cx| {
            state.set_readonly(false, cx);
            state.set_disabled(true, cx);
        })
    });
    cx.simulate_input(" forbidden");
    assert_eq!(
        input.read_with(cx, |state, _| state.value().to_string()),
        "original"
    );
}

#[gpui::test]
fn textarea_keeps_native_multiline_editing(cx: &mut TestAppContext) {
    let (view, cx) = mount(cx, false);
    let area = view.read_with(cx, |form, _| form.area.clone());
    cx.update(|window, cx| area.update(cx, |state, cx| state.focus(window, cx)));
    cx.simulate_input("first");
    key(cx, "enter");
    cx.simulate_input("둘째");
    assert_eq!(
        area.read_with(cx, |state, _| state.value().to_string()),
        "first\n둘째"
    );
}

#[gpui::test]
fn select_skips_disabled_options_confirms_once_and_restores_trigger_focus(cx: &mut TestAppContext) {
    let (view, cx) = mount(cx, false);
    cx.simulate_click(point(px(30.), px(185.)), Modifiers::default());
    assert!(view.read_with(cx, |form, _| form.open));
    key(cx, "down");
    assert_eq!(view.read_with(cx, |form, _| form.highlighted), 2);
    key(cx, "enter");
    assert_eq!(
        view.read_with(cx, |form, _| (
            form.selected.to_string(),
            form.open,
            form.commits
        )),
        ("c".into(), false, 1)
    );
    key(cx, "enter");
    assert!(
        view.read_with(cx, |form, _| form.open),
        "Enter reopens after focus returns to the trigger"
    );
    key(cx, "home");
    key(cx, "escape");
    assert_eq!(
        view.read_with(cx, |form, _| (
            form.selected.to_string(),
            form.open,
            form.commits
        )),
        ("c".into(), false, 1)
    );
}

#[gpui::test]
fn disabled_select_does_not_open(cx: &mut TestAppContext) {
    let (view, cx) = mount(cx, true);
    cx.simulate_click(point(px(30.), px(185.)), Modifiers::default());
    key(cx, "enter");
    key(cx, "down");
    assert_eq!(
        view.read_with(cx, |form, _| (form.open, form.commits)),
        (false, 0)
    );
}

#[gpui::test]
fn select_confirmation_preserves_consumer_requested_input_focus(cx: &mut TestAppContext) {
    let (view, cx) = mount(cx, false);
    view.update(cx, |this, cx| {
        this.focus_input_on_select = true;
        cx.notify();
    });
    cx.simulate_click(point(px(30.), px(185.)), Modifiers::default());
    key(cx, "down");
    key(cx, "enter");
    cx.simulate_input("focused");
    assert_eq!(
        view.read_with(cx, |this, cx| this.input.read(cx).value().to_string()),
        "focused"
    );
    assert_eq!(view.read_with(cx, |this, _| this.commits), 1);
}

#[gpui::test]
fn select_external_close_restores_trigger_and_disable_clears_popup_focus(cx: &mut TestAppContext) {
    let (view, cx) = mount(cx, false);
    cx.simulate_click(point(px(30.), px(185.)), Modifiers::default());
    let popup_focus = cx.update(|w, cx| w.focused(cx).expect("openpopupfocus"));
    view.update(cx, |s, cx| {
        s.open = false;
        cx.notify();
    });
    cx.update(|w, cx| w.draw(cx).clear(cx));
    key(cx, "enter");
    assert!(view.read_with(cx, |s, _| s.open));
    key(cx, "escape");
    let input = view.read_with(cx, |s, _| s.input.clone());
    cx.update(|w, cx| input.update(cx, |s, cx| s.focus(w, cx)));
    cx.simulate_click(point(px(30.), px(185.)), Modifiers::default());
    view.update(cx, |s, cx| {
        s.disabled = true;
        cx.notify();
    });
    cx.update(|w, cx| w.draw(cx).clear(cx));
    assert!(
        !cx.update(|w, _| popup_focus.is_focused(w)),
        "disabledpopupmustnotownfocus"
    );
}

#[gpui::test]
fn select_uses_host_supplied_trigger_and_content_focus_identities(cx: &mut TestAppContext) {
    struct Host {
        trigger: gpui::FocusHandle,
        content: gpui::FocusHandle,
        open: bool,
    }
    impl Render for Host {
        fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
            let owner = cx.entity().downgrade();
            Select::new("host-select")
                .focus_handle(&self.trigger)
                .content_focus_handle(&self.content)
                .items(vec![SelectOption::new("a", "Alpha")])
                .selected("a")
                .open(self.open)
                .on_open_change(move |open, _, cx| {
                    let _ = owner.update(cx, |s, cx| {
                        s.open = open;
                        cx.notify();
                    });
                })
        }
    }
    cx.update(astryx_gpui::init);
    let (view, cx) = cx.add_window_view(|_, cx| Host {
        trigger: cx.focus_handle(),
        content: cx.focus_handle(),
        open: false,
    });
    cx.update(|w, cx| w.draw(cx).clear(cx));
    let (trigger, content) = view.read_with(cx, |s, _| (s.trigger.clone(), s.content.clone()));
    cx.update(|w, cx| trigger.focus(w, cx));
    key(cx, "enter");
    assert!(cx.update(|w, _| content.is_focused(w)));
    key(cx, "escape");
    assert!(cx.update(|w, _| trigger.is_focused(w)));
}

#[gpui::test]
fn view_disabled_override_preserves_values_and_omitted_override_keeps_owner_policy(
    cx: &mut TestAppContext,
) {
    let (view, cx) = mount(cx, false);
    let (input, area) = view.read_with(cx, |s, _| (s.input.clone(), s.area.clone()));
    cx.update(|w, cx| {
        input.update(cx, |s, cx| {
            s.set_value("input", w, cx);
            s.set_disabled(true, cx);
        });
        area.update(cx, |s, cx| {
            s.set_value("area", w, cx);
            s.set_disabled(true, cx);
        });
        w.draw(cx).clear(cx);
    });
    assert!(input.read_with(cx, |s, _| s.presentation().is_disabled()));
    assert!(area.read_with(cx, |s, _| s.presentation().is_disabled()));
    view.update(cx, |s, cx| {
        s.disabled_override = Some(false);
        cx.notify();
    });
    cx.update(|w, cx| w.draw(cx).clear(cx));
    assert!(!input.read_with(cx, |s, _| s.presentation().is_disabled()));
    assert!(!area.read_with(cx, |s, _| s.presentation().is_disabled()));
    cx.update(|w, cx| input.update(cx, |s, cx| s.focus(w, cx)));
    cx.simulate_input("+");
    view.update(cx, |s, cx| {
        s.disabled_override = Some(true);
        cx.notify();
    });
    cx.update(|w, cx| w.draw(cx).clear(cx));
    cx.simulate_input("blocked");
    assert_eq!(input.read_with(cx, |s, _| s.value().to_string()), "input+");
    assert_eq!(area.read_with(cx, |s, _| s.value().to_string()), "area");
    assert!(area.read_with(cx, |s, _| s.presentation().is_disabled()));
    // Rendering the same controlled policy repeatedly must remain stable.
    for _ in 0..3 {
        cx.update(|w, cx| w.draw(cx).clear(cx));
    }
}
