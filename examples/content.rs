//! Synthetic native content, chat and media. No provider or network is contacted.
//! `--snapshot PATH [--dark]` captures the real offscreen Metal rendering.
use std::{path::PathBuf, sync::Arc};

use anyhow::{Context as _, Result, bail};
use astryx_gpui::{
    Avatar, AvatarGroup, Blockquote, Button, ButtonVariant, Card, Chat, ChatMessage, ChatRole,
    ClickableCard, CodeBlock, Icon, Markdown, MetadataList, SegmentedControl, SelectOption,
    SelectableCard, Spinner, Text, TextKind, TextareaState, Theme, Timer, TimerState, Timestamp,
    ToggleButton,
};
use chrono::{DateTime, Utc};
use gpui::{
    App, AppContext, Bounds, Context, Entity, FocusHandle, IntoElement, KeyBinding, Render,
    SharedString, Subscription, Window, WindowBounds, WindowOptions, actions, div, prelude::*, px,
    size,
};

actions!(content_gallery, [FocusNext, FocusPrevious]);
const CHECK: &str = "<svg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 24 24'><path fill='none' stroke='currentColor' stroke-width='2' stroke-linecap='round' stroke-linejoin='round' d='m5 12 4 4L19 6'/></svg>";

struct Content {
    theme: Theme,
    focus: FocusHandle,
    draft: Entity<TextareaState>,
    timer: Entity<TimerState>,
    messages: Vec<ChatMessage>,
    sending: bool,
    pinned: bool,
    selected: bool,
    segment: SharedString,
    notice: SharedString,
    sequence: usize,
    _observations: Vec<Subscription>,
}

impl Content {
    fn new(theme: Theme, window: &mut Window, cx: &mut Context<Self>) -> Self {
        let draft = cx.new(|cx| {
            TextareaState::new(window, cx)
                .default_value("변경된 부분을 짧게 정리해 주세요.")
                .placeholder("메시지를 입력하세요…")
        });
        let timer = cx.new(|_| TimerState::new());
        let observations = vec![
            cx.observe(&draft, |_, _, cx| cx.notify()),
            cx.observe(&timer, |_, _, cx| cx.notify()),
        ];
        Self {
            theme,
            focus: cx.focus_handle(),
            draft,
            timer,
            messages: vec![
                ChatMessage::new(
                    "intro",
                    ChatRole::Assistant,
                    "### A calmer review workspace\nThe new layout keeps **context**, feedback and follow-up questions together.\n\n- Keep the current draft when changing views.\n- Make each action clear and reversible.",
                ),
                ChatMessage::new(
                    "question",
                    ChatRole::User,
                    "좋아요. 한국어 입력과 후속 질문도 여기서 이어갈 수 있나요?",
                ),
                ChatMessage::new(
                    "answer",
                    ChatRole::Assistant,
                    "네. 입력 상태를 유지하므로 **한국어 IME**, 선택과 실행 취소를 그대로 사용할 수 있습니다. 이 갤러리는 로컬 데모입니다.",
                ),
            ],
            sending: false,
            pinned: true,
            selected: true,
            segment: "overview".into(),
            notice: "Synthetic content · Local actions · No AI provider connected".into(),
            sequence: 0,
            _observations: observations,
        }
    }

    fn heading(&self, text: &'static str) -> Text {
        Text::new(text).kind(TextKind::Heading).theme(self.theme)
    }

    fn chat(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = self.theme;
        let send = cx.entity().downgrade();
        let cancel = cx.entity().downgrade();
        Card::new().theme(theme).flex_1().min_h_0()
            .child(div().flex().items_center().justify_between()
                .child(self.heading("Conversation"))
                .child(Button::new("finish", "Finish sample reply").theme(theme)
                    .variant(ButtonVariant::Outline).disabled(!self.sending)
                    .on_click(cx.listener(|this, _, _, cx| {
                        this.sending = false;
                        if let Some(message) = this.messages.last_mut().filter(|message| message.pending) {
                            message.pending = false;
                            message.content = "샘플 응답이 준비됐어요. 변경 사항은 입력 유지, 명확한 작업 상태, 일관된 컴포넌트입니다.".into();
                        }
                        this.notice = "Local sample reply completed.".into();
                        cx.notify();
                    }))))
            .child(Chat::new("conversation").theme(theme)
                .messages(self.messages.clone()).input(&self.draft).sending(self.sending)
                .on_send(move |text, window, cx| {
                    let _ = send.update(cx, |this, cx| {
                        this.sequence += 1;
                        this.messages.push(ChatMessage::new(format!("user-{}",this.sequence),ChatRole::User,text));
                        this.messages.push(ChatMessage::new(format!("pending-{}",this.sequence),ChatRole::Assistant,"Preparing a local sample reply…").pending(true));
                        this.sending = true;
                        this.draft.update(cx, |state,cx|state.set_value("",window,cx));
                        this.notice = "Local reply pending. Use Stop or Finish sample reply.".into();
                        cx.notify();
                    });
                })
                .on_cancel(move |_,cx| {
                    let _ = cancel.update(cx, |this,cx| {
                        this.sending = false;
                        if this.messages.last().is_some_and(|message|message.pending) {this.messages.pop();}
                        this.notice = "Local reply stopped. Existing messages were kept.".into();
                        cx.notify();
                    });
                }).flex_1().min_h_0())
    }

    fn choices(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = self.theme;
        let select = cx.entity().downgrade();
        let pin = cx.entity().downgrade();
        let segment = cx.entity().downgrade();
        div()
            .flex()
            .flex_col()
            .gap(px(12.))
            .child(
                div()
                    .flex()
                    .gap(px(12.))
                    .child(
                        ClickableCard::new("open-card", "Open sample brief")
                            .theme(theme)
                            .flex_1()
                            .child(self.heading("Review brief"))
                            .child(Text::new("Open the local context").theme(theme).muted())
                            .on_click(cx.listener(|this, _, _, cx| {
                                this.notice = "Review brief selected locally.".into();
                                cx.notify();
                            })),
                    )
                    .child(
                        SelectableCard::new("select-card", "Include design context")
                            .theme(theme)
                            .flex_1()
                            .selected(self.selected)
                            .child(self.heading("Design context"))
                            .child(Text::new("Include in this review").theme(theme).muted())
                            .on_change(move |value, _, cx| {
                                let _ = select.update(cx, |this, cx| {
                                    this.selected = value;
                                    cx.notify();
                                });
                            }),
                    ),
            )
            .child(
                Card::new().theme(theme).child(
                    div()
                        .flex()
                        .items_center()
                        .justify_between()
                        .gap(px(12.))
                        .child(
                            SegmentedControl::new("sections")
                                .theme(theme)
                                .selected(self.segment.clone())
                                .items(vec![
                                    SelectOption::new("overview", "Overview"),
                                    SelectOption::new("activity", "Activity"),
                                    SelectOption::new("notes", "Notes"),
                                ])
                                .on_change(move |value, _, cx| {
                                    let _ = segment.update(cx, |this, cx| {
                                        this.segment = value;
                                        cx.notify();
                                    });
                                }),
                        )
                        .child(
                            ToggleButton::new("pin", "Pin context")
                                .theme(theme)
                                .pressed(self.pinned)
                                .on_change(move |value, _, cx| {
                                    let _ = pin.update(cx, |this, cx| {
                                        this.pinned = value;
                                        cx.notify();
                                    });
                                }),
                        ),
                ),
            )
    }

    fn rich_content(&self) -> impl IntoElement {
        let theme = self.theme;
        Card::new().theme(theme)
            .child(self.heading("Rich content"))
            .child(Markdown::new("markdown",
                "### Keep the important details close\nUse **clear hierarchy**, selectable text and `native controls`.\n\n1. Read the context.\n2. Ask a focused follow-up.").theme(theme))
            .child(CodeBlock::new("code", "let context = ReviewContext {\n    keep_draft: true,\n    use_native_focus: true,\n};").language("rust").theme(theme))
            .child(Blockquote::new().theme(theme).cite("Design principles")
                .child("The next action should feel obvious, while the current work stays safe."))
    }

    fn metadata(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = self.theme;
        let timestamp: DateTime<Utc> = "2026-11-18T00:30:00Z".parse().expect("synthetic timestamp");
        Card::new()
            .theme(theme)
            .child(
                div()
                    .flex()
                    .items_center()
                    .justify_between()
                    .child(self.heading("People and activity"))
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap(px(8.))
                            .child(Icon::svg("ready-icon", CHECK).theme(theme).label("Ready"))
                            .child(
                                Spinner::new("sample-spinner")
                                    .theme(theme)
                                    .label("Animated preview"),
                            ),
                    ),
            )
            .child(
                AvatarGroup::new(
                    "team",
                    vec![
                        Avatar::new("alex", "Alex Morgan"),
                        Avatar::new("jamie", "Jamie Kim"),
                        Avatar::new("sam", "Sam Rivera"),
                        Avatar::new("taylor", "Taylor Lee"),
                    ],
                )
                .theme(theme)
                .visible(3),
            )
            .child(
                MetadataList::new()
                    .theme(theme)
                    .row("Project", Text::new("Product studio").theme(theme))
                    .row(
                        "Status",
                        Text::new(if self.sending {
                            "Local reply pending"
                        } else {
                            "Ready for review"
                        })
                        .theme(theme),
                    )
                    .row("Updated", Timestamp::new("updated", timestamp).theme(theme))
                    .row("Active time", Timer::new("timer", &self.timer).theme(theme)),
            )
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap(px(8.))
                    .child(
                        Button::new(
                            "timer-toggle",
                            if self.timer.read(cx).is_running() {
                                "Pause timer"
                            } else {
                                "Start timer"
                            },
                        )
                        .theme(theme)
                        .variant(ButtonVariant::Outline)
                        .on_click(cx.listener(|this, _, _, cx| {
                            this.timer.update(cx, |state, cx| {
                                if state.is_running() {
                                    state.pause(cx)
                                } else {
                                    state.start(cx)
                                }
                            });
                        })),
                    )
                    .child(
                        Button::new("timer-reset", "Reset")
                            .theme(theme)
                            .variant(ButtonVariant::Ghost)
                            .on_click(cx.listener(|this, _, _, cx| {
                                this.timer.update(cx, |state, cx| state.reset(cx))
                            })),
                    ),
            )
    }
}

impl Render for Content {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = self.theme;
        div()
            .id("content-gallery")
            .key_context("AstryxContent")
            .track_focus(&self.focus)
            .tab_group()
            .tab_stop(false)
            .size_full()
            .bg(theme.colors.canvas)
            .text_color(theme.colors.text)
            .font_family("Helvetica")
            .text_size(px(14.))
            .line_height(px(20.))
            .on_action(|_: &FocusNext, window, cx| window.focus_next(cx))
            .on_action(|_: &FocusPrevious, window, cx| window.focus_prev(cx))
            .child(
                div()
                    .flex()
                    .flex_col()
                    .p(px(24.))
                    .gap(px(16.))
                    .size_full()
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .justify_between()
                            .child(
                                Text::new("Astryx / Content")
                                    .kind(TextKind::Heading)
                                    .theme(theme)
                                    .text_size(px(28.))
                                    .line_height(px(32.)),
                            )
                            .child(
                                Button::new("theme", "Switch theme")
                                    .theme(theme)
                                    .variant(ButtonVariant::Outline)
                                    .on_click(cx.listener(|this, _, _, cx| {
                                        this.theme = if this.theme == Theme::light() {
                                            Theme::dark()
                                        } else {
                                            Theme::light()
                                        };
                                        cx.notify();
                                    })),
                            ),
                    )
                    .child(Text::new(self.notice.clone()).theme(theme).muted())
                    .child(
                        div()
                            .flex()
                            .gap(px(16.))
                            .flex_1()
                            .min_h_0()
                            .child(
                                div()
                                    .flex()
                                    .flex_col()
                                    .gap(px(12.))
                                    .flex_1()
                                    .min_w_0()
                                    .min_h_0()
                                    .child(self.chat(cx))
                                    .child(self.choices(cx)),
                            )
                            .child(
                                div()
                                    .flex()
                                    .flex_col()
                                    .gap(px(12.))
                                    .w(px(412.))
                                    .flex_none()
                                    .child(self.rich_content())
                                    .child(self.metadata(cx)),
                            ),
                    ),
            )
    }
}

fn init(cx: &mut App) {
    astryx_gpui::init(cx);
    cx.bind_keys([
        KeyBinding::new("tab", FocusNext, Some("AstryxContent")),
        KeyBinding::new("shift-tab", FocusPrevious, Some("AstryxContent")),
    ]);
}
fn build(window: &mut Window, cx: &mut App, theme: Theme) -> Entity<Content> {
    let view = cx.new(|cx| Content::new(theme, window, cx));
    view.read(cx).focus.clone().focus(window, cx);
    view
}
fn main() -> Result<()> {
    let mut theme = Theme::light();
    let mut output = None;
    let mut args = std::env::args().skip(1);
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--dark" => theme = Theme::dark(),
            "--snapshot" => {
                output = Some(PathBuf::from(
                    args.next().context("--snapshot requires a PNG path")?,
                ))
            }
            _ => bail!("Unknown argument: {arg}"),
        }
    }
    let dimensions = size(px(1120.), px(920.));
    if let Some(path) = output {
        let platform = gpui_platform::current_platform(true);
        let mut cx = gpui::HeadlessAppContext::with_platform(
            platform.text_system(),
            Arc::new(()),
            gpui_platform::current_headless_renderer,
        );
        cx.update(init);
        let window = cx.open_window(dimensions, |window, cx| build(window, cx, theme))?;
        cx.update_window(window.into(), |_, window, cx| window.draw(cx).clear(cx))?;
        cx.run_until_parked();
        cx.update_window(window.into(), |_, window, cx| window.draw(cx).clear(cx))?;
        if let Some(parent) = path
            .parent()
            .filter(|parent| !parent.as_os_str().is_empty())
        {
            std::fs::create_dir_all(parent)?;
        }
        cx.capture_screenshot(window.into())?.save(&path)?;
        println!("{}", path.display());
        return Ok(());
    }
    gpui_platform::application().run(move |cx| {
        init(cx);
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(Bounds::centered(
                    None, dimensions, cx,
                ))),
                ..Default::default()
            },
            move |window, cx| build(window, cx, theme),
        )
        .expect("open content gallery");
        cx.activate(true);
    });
    Ok(())
}
