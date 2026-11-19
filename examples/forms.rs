//! Synthetic, interactive native forms and navigation gallery.
//! Run with `--snapshot PATH [--dark] [--navigation]` for an offscreen Metal PNG.
use std::{path::PathBuf, sync::Arc};

use anyhow::{Context as _, Result, bail};
use astryx_gpui::{
    Breadcrumb, Breadcrumbs, Button, ButtonVariant, Calendar, CalendarState, Card, CheckboxList,
    ComplexSelector, Date, DateInput, DateRangeInput, DateTimeInput, Field, FileInput, FormLayout,
    InlineToken, Input, InputContent, InputGroup, InputState, Link, MobileNav, MultiSelector,
    NavItem, NumberInput, Pagination, RadioList, Select, SelectGroup, SelectOption, SideNav,
    Slider, SliderState, Step, Stepper, Text, TextArea, TextKind, TextareaState, Theme,
    TimeFieldState, TimeInput, Token, Tokenizer, Toolbar, TopNav, Typeahead,
};
use chrono::{NaiveDate, NaiveTime};
use gpui::{
    App, AppContext, Bounds, Context, Entity, FocusHandle, IntoElement, KeyBinding, Render,
    SharedString, Subscription, Window, WindowBounds, WindowOptions, actions, div, prelude::*, px,
    size,
};

actions!(forms_gallery, [FocusNext, FocusPrevious]);

struct Forms {
    theme: Theme,
    focus: FocusHandle,
    navigation: bool,
    name: Entity<InputState>,
    description: Entity<TextareaState>,
    number: Entity<InputState>,
    query: Entity<InputState>,
    grouped_query: Entity<InputState>,
    tokenizer: Entity<InputState>,
    calendar: Entity<CalendarState>,
    range: Entity<CalendarState>,
    time: Entity<TimeFieldState>,
    slider: Entity<SliderState>,
    opened: Option<&'static str>,
    selected: SharedString,
    grouped: SharedString,
    radio: SharedString,
    checked: Vec<SharedString>,
    multiple: Vec<SharedString>,
    highlight: usize,
    files: Vec<PathBuf>,
    token_visible: bool,
    route: SharedString,
    page: usize,
    step: usize,
    notice: SharedString,
    _observations: Vec<Subscription>,
}

impl Forms {
    fn new(theme: Theme, navigation: bool, window: &mut Window, cx: &mut Context<Self>) -> Self {
        let name = cx.new(|cx| {
            InputState::new(window, cx)
                .default_value("Improve onboarding")
                .placeholder("Task name")
        });
        let description = cx.new(|cx| {
            TextareaState::new(window, cx).default_value(
                "Make the first session feel simple.\nKeep the important choices visible.",
            )
        });
        let number = cx.new(|cx| {
            InputState::new(window, cx)
                .default_value("3")
                .step(1.)
                .min(0.)
                .max(10.)
        });
        let query = cx.new(|cx| InputState::new(window, cx).placeholder("Find a teammate…"));
        let grouped_query =
            cx.new(|cx| InputState::new(window, cx).placeholder("Filter projects…"));
        let tokenizer = cx.new(|cx| InputState::new(window, cx).placeholder("Add a person…"));
        tokenizer.update(cx, |state, cx| {
            state.set_value(
                InputContent::new("@Alex ")
                    .with_token(0..5, InlineToken::new("alex", "@Alex"))
                    .expect("valid synthetic token"),
                window,
                cx,
            );
        });
        let calendar = cx.new(|cx| CalendarState::new(window, cx));
        calendar.update(cx, |state, cx| {
            state.set_date(
                Date::Single(NaiveDate::from_ymd_opt(2026, 11, 18)),
                window,
                cx,
            );
        });
        let range = cx.new(|cx| CalendarState::new(window, cx));
        range.update(cx, |state, cx| {
            state.set_date(
                Date::Range(
                    NaiveDate::from_ymd_opt(2026, 11, 18),
                    NaiveDate::from_ymd_opt(2026, 11, 24),
                ),
                window,
                cx,
            );
        });
        let time = cx.new(|cx| TimeFieldState::new(window, cx));
        time.update(cx, |state, cx| {
            state.set_time(
                NaiveTime::from_hms_opt(9, 30, 0).expect("valid synthetic time"),
                window,
                cx,
            );
        });
        let slider = cx.new(|_| {
            SliderState::new()
                .min(0.)
                .max(100.)
                .default_value((25., 75.))
        });
        let observations = vec![
            cx.observe(&name, |_, _, cx| cx.notify()),
            cx.observe(&description, |_, _, cx| cx.notify()),
            cx.observe(&number, |_, _, cx| cx.notify()),
            cx.observe(&query, |_, _, cx| cx.notify()),
            cx.observe(&grouped_query, |_, _, cx| cx.notify()),
            cx.observe(&tokenizer, |_, _, cx| cx.notify()),
            cx.observe(&calendar, |_, _, cx| cx.notify()),
            cx.observe(&range, |_, _, cx| cx.notify()),
            cx.observe(&time, |_, _, cx| cx.notify()),
            cx.observe(&slider, |_, _, cx| cx.notify()),
        ];
        Self {
            theme,
            focus: cx.focus_handle(),
            navigation,
            name,
            description,
            number,
            query,
            grouped_query,
            tokenizer,
            calendar,
            range,
            time,
            slider,
            opened: None,
            selected: "claude".into(),
            grouped: "studio".into(),
            radio: "balanced".into(),
            checked: vec!["notifications".into()],
            multiple: vec!["design".into(), "engineering".into()],
            highlight: 0,
            files: vec![PathBuf::from("example-spec.md")],
            token_visible: true,
            route: "overview".into(),
            page: 2,
            step: 1,
            notice: "All values are synthetic. Changes stay in this gallery.".into(),
            _observations: observations,
        }
    }

    fn heading(&self, text: &'static str) -> Text {
        Text::new(text).kind(TextKind::Heading).theme(self.theme)
    }

    fn label(&self, text: &'static str) -> Text {
        Text::new(text)
            .kind(TextKind::Supporting)
            .muted()
            .theme(self.theme)
    }

    fn open_change(
        cx: &Context<Self>,
        key: &'static str,
    ) -> impl Fn(bool, &mut Window, &mut App) + 'static {
        let owner = cx.entity().downgrade();
        move |open, _, cx| {
            let _ = owner.update(cx, |this, cx| {
                this.opened = open.then_some(key);
                cx.notify();
            });
        }
    }

    fn choose(
        cx: &Context<Self>,
        key: &'static str,
    ) -> impl Fn(SharedString, &mut Window, &mut App) + 'static {
        let owner = cx.entity().downgrade();
        move |value, _, cx| {
            let _ = owner.update(cx, |this, cx| {
                match key {
                    "program" => this.selected = value,
                    "grouped" => this.grouped = value,
                    "radio" => this.radio = value,
                    "route" => this.route = value,
                    _ => this.notice = format!("Selected {value}").into(),
                }
                cx.notify();
            });
        }
    }

    fn forms(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = self.theme;
        let multiple_owner = cx.entity().downgrade();
        let checked_owner = cx.entity().downgrade();
        let highlight_owner = cx.entity().downgrade();
        let type_highlight = cx.entity().downgrade();
        let file_owner = cx.entity().downgrade();
        let file_error = cx.entity().downgrade();
        let token_owner = cx.entity().downgrade();
        let commit_owner = cx.entity().downgrade();
        let options = vec![
            SelectOption::new("design", "Design"),
            SelectOption::new("engineering", "Engineering"),
            SelectOption::new("operations", "Operations"),
            SelectOption::new("archived", "Archived").disabled(true),
        ];
        div()
            .flex()
            .gap(px(16.))
            .items_start()
            .child(
                Card::new()
                    .theme(theme)
                    .flex_1()
                    .min_w_0()
                    .child(self.heading("Text and scheduling"))
                    .child(
                        FormLayout::new()
                            .child(
                                Field::new("name-field", "Task name")
                                    .theme(theme)
                                    .required(true)
                                    .child(Input::new(&self.name).theme(theme).label("Task name")),
                            )
                            .child(
                                Field::new("description-field", "Description")
                                    .theme(theme)
                                    .child(
                                        TextArea::new(&self.description)
                                            .theme(theme)
                                            .label("Description")
                                            .h(px(94.)),
                                    ),
                            )
                            .child(
                                Field::new("program-field", "Program").theme(theme).child(
                                    Select::new("program")
                                        .theme(theme)
                                        .label("Program")
                                        .selected(self.selected.clone())
                                        .open(self.opened == Some("program"))
                                        .highlighted(self.highlight)
                                        .items(vec![
                                            SelectOption::new("claude", "Claude Code"),
                                            SelectOption::new("codex", "Codex"),
                                            SelectOption::new("terminal", "Terminal"),
                                        ])
                                        .on_select(Self::choose(cx, "program"))
                                        .on_open_change(Self::open_change(cx, "program"))
                                        .on_highlight(move |value, _, cx| {
                                            let _ = highlight_owner.update(cx, |s, cx| {
                                                s.highlight = value;
                                                cx.notify();
                                            });
                                        }),
                                ),
                            )
                            .child(
                                Field::new("number-field", "Parallel tasks")
                                    .theme(theme)
                                    .child(
                                        InputGroup::new().theme(theme).child(
                                            NumberInput::new(&self.number)
                                                .theme(theme)
                                                .flex_1()
                                                .min_w_0()
                                                .border_0(),
                                        ),
                                    ),
                            )
                            .child(
                                Field::new("time-field", "Start time")
                                    .theme(theme)
                                    .child(TimeInput::new("time", &self.time).theme(theme)),
                            )
                            .child(
                                Field::new("date-field", "Due date").theme(theme).child(
                                    DateInput::new("date", &self.calendar)
                                        .theme(theme)
                                        .open(self.opened == Some("date"))
                                        .on_open_change(Self::open_change(cx, "date")),
                                ),
                            )
                            .child(
                                Field::new("range-field", "Date range").theme(theme).child(
                                    DateRangeInput::new("range", &self.range)
                                        .theme(theme)
                                        .open(self.opened == Some("range"))
                                        .on_open_change(Self::open_change(cx, "range")),
                                ),
                            )
                            .child(
                                Field::new("datetime-field", "Scheduled start")
                                    .theme(theme)
                                    .child(
                                        DateTimeInput::new("datetime", &self.calendar, &self.time)
                                            .theme(theme)
                                            .open(self.opened == Some("datetime"))
                                            .on_open_change(Self::open_change(cx, "datetime")),
                                    ),
                            ),
                    ),
            )
            .child(
                Card::new()
                    .theme(theme)
                    .flex_1()
                    .min_w_0()
                    .child(self.heading("Dates and choices"))
                    .child(Calendar::new("calendar", &self.calendar).theme(theme))
                    .child(self.label("Review window · range slider"))
                    .child(
                        Slider::new(&self.slider)
                            .theme(theme)
                            .label("Review window"),
                    )
                    .child(
                        Text::new(format!(
                            "{:.0}–{:.0}%",
                            self.slider.read(cx).value().start(),
                            self.slider.read(cx).value().end()
                        ))
                        .theme(theme)
                        .muted(),
                    )
                    .child(self.label("Review depth"))
                    .child(
                        RadioList::new("depth")
                            .theme(theme)
                            .selected(self.radio.clone())
                            .items(vec![
                                SelectOption::new("quick", "Quick pass"),
                                SelectOption::new("balanced", "Balanced"),
                                SelectOption::new("deep", "Deep review"),
                            ])
                            .on_change(Self::choose(cx, "radio")),
                    )
                    .child(self.label("Preferences"))
                    .child(
                        CheckboxList::new("preferences")
                            .theme(theme)
                            .selected(self.checked.clone())
                            .items(vec![
                                SelectOption::new("notifications", "Notify when ready"),
                                SelectOption::new("context", "Include project context"),
                                SelectOption::new("locked", "Managed by workspace").disabled(true),
                            ])
                            .on_change(move |value, _, cx| {
                                let _ = checked_owner.update(cx, |s, cx| {
                                    s.checked = value;
                                    cx.notify();
                                });
                            }),
                    ),
            )
            .child(
                Card::new()
                    .theme(theme)
                    .flex_1()
                    .min_w_0()
                    .child(self.heading("People and resources"))
                    .child(
                        Field::new("teams-field", "Teams").theme(theme).child(
                            MultiSelector::new("teams")
                                .theme(theme)
                                .items(options)
                                .selected(self.multiple.clone())
                                .open(self.opened == Some("teams"))
                                .on_open_change(Self::open_change(cx, "teams"))
                                .on_change(move |value, _, cx| {
                                    let _ = multiple_owner.update(cx, |s, cx| {
                                        s.multiple = value;
                                        cx.notify();
                                    });
                                }),
                        ),
                    )
                    .child(
                        Field::new("project-field", "Project").theme(theme).child(
                            ComplexSelector::new("grouped")
                                .theme(theme)
                                .query(&self.grouped_query)
                                .groups(vec![
                                    SelectGroup::new(
                                        "Workspace",
                                        vec![
                                            SelectOption::new("studio", "Product studio"),
                                            SelectOption::new("console", "Console"),
                                        ],
                                    ),
                                    SelectGroup::new(
                                        "Personal",
                                        vec![SelectOption::new("sandbox", "Sandbox")],
                                    ),
                                ])
                                .selected(self.grouped.clone())
                                .open(self.opened == Some("grouped"))
                                .on_open_change(Self::open_change(cx, "grouped"))
                                .on_select(Self::choose(cx, "grouped")),
                        ),
                    )
                    .child(
                        Field::new("teammate-field", "Teammate")
                            .theme(theme)
                            .description("Type, then use ↑ ↓ and Enter.")
                            .child(
                                Typeahead::new("teammate", &self.query)
                                    .theme(theme)
                                    .open(self.opened == Some("typeahead"))
                                    .highlighted(self.highlight)
                                    .items(vec![
                                        SelectOption::new("alex", "Alex Morgan"),
                                        SelectOption::new("jamie", "Jamie Kim"),
                                        SelectOption::new("sam", "Sam Rivera"),
                                    ])
                                    .on_open_change(Self::open_change(cx, "typeahead"))
                                    .on_select(Self::choose(cx, "typeahead"))
                                    .on_highlight(move |value, _, cx| {
                                        let _ = type_highlight.update(cx, |s, cx| {
                                            s.highlight = value;
                                            cx.notify();
                                        });
                                    }),
                            ),
                    )
                    .child(self.label("Selected token"))
                    .child(div().flex().when(self.token_visible, |el| {
                        el.child(Token::new("token", "Design review").theme(theme).on_remove(
                            move |_, cx| {
                                let _ = token_owner.update(cx, |s, cx| {
                                    s.token_visible = false;
                                    cx.notify();
                                });
                            },
                        ))
                    }))
                    .child(
                        Field::new("tokenizer-field", "Participants")
                            .theme(theme)
                            .description("Tokens keep native selection and undo.")
                            .child(
                                Tokenizer::new(&self.tokenizer)
                                    .theme(theme)
                                    .label("Participants")
                                    .on_commit(move |_, _, cx| {
                                        let _ = commit_owner.update(cx, |s, cx| {
                                            s.notice = "Participants committed locally.".into();
                                            cx.notify();
                                        });
                                    }),
                            ),
                    )
                    .child(
                        Field::new("files-field", "Attachments")
                            .theme(theme)
                            .description("Browse opens the native chooser only when clicked.")
                            .child(
                                FileInput::new("files")
                                    .theme(theme)
                                    .multiple(true)
                                    .files(self.files.clone())
                                    .on_change(move |value, _, cx| {
                                        let _ = file_owner.update(cx, |s, cx| {
                                            s.files = value;
                                            cx.notify();
                                        });
                                    })
                                    .on_error(move |value, _, cx| {
                                        let _ = file_error.update(cx, |s, cx| {
                                            s.notice = value;
                                            cx.notify();
                                        });
                                    }),
                            ),
                    ),
            )
    }

    fn navigation(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = self.theme;
        let page_owner = cx.entity().downgrade();
        let step_owner = cx.entity().downgrade();
        let breadcrumb_owner = cx.entity().downgrade();
        let link_owner = cx.entity().downgrade();
        let items = vec![
            NavItem::new("overview", "Overview").badge("3"),
            NavItem::new("reviews", "Reviews").badge("8"),
            NavItem::new("archive", "Archive").disabled(true),
            NavItem::new("settings", "Settings"),
        ];
        div()
            .flex()
            .flex_col()
            .gap(px(16.))
            .child(
                Card::new()
                    .theme(theme)
                    .child(self.heading("Navigation and location"))
                    .child(
                        Breadcrumbs::new("breadcrumbs")
                            .theme(theme)
                            .items(vec![
                                Breadcrumb::new("home", "Workspace"),
                                Breadcrumb::new("projects", "Projects"),
                                Breadcrumb::new("current", "Product studio"),
                            ])
                            .on_select(move |value, _, cx| {
                                let _ = breadcrumb_owner.update(cx, |s, cx| {
                                    s.notice = format!("Breadcrumb selected: {value}").into();
                                    cx.notify();
                                });
                            }),
                    )
                    .child(
                        TopNav::new("top-nav")
                            .theme(theme)
                            .items(items.clone())
                            .selected(self.route.clone())
                            .on_select(Self::choose(cx, "route")),
                    ),
            )
            .child(
                div()
                    .flex()
                    .gap(px(16.))
                    .child(
                        Card::new()
                            .theme(theme)
                            .flex_1()
                            .child(self.heading("Side navigation"))
                            .child(
                                SideNav::new("side-nav")
                                    .theme(theme)
                                    .items(items.clone())
                                    .selected(self.route.clone())
                                    .on_select(Self::choose(cx, "route")),
                            ),
                    )
                    .child(
                        Card::new()
                            .theme(theme)
                            .flex_1()
                            .child(self.heading("Compact navigation"))
                            .child(
                                MobileNav::new("mobile-nav")
                                    .theme(theme)
                                    .items(items)
                                    .selected(self.route.clone())
                                    .on_select(Self::choose(cx, "route")),
                            )
                            .child(self.label("Arrow keys move focus; Enter selects.")),
                    ),
            )
            .child(
                Card::new()
                    .theme(theme)
                    .child(self.heading("Progress and paging"))
                    .child(
                        Stepper::new("steps")
                            .theme(theme)
                            .current(self.step)
                            .steps(vec![
                                Step::new("Connect"),
                                Step::new("Configure"),
                                Step::new("Review"),
                                Step::new("Launch"),
                            ])
                            .on_select(move |value, _, cx| {
                                let _ = step_owner.update(cx, |s, cx| {
                                    s.step = value;
                                    cx.notify();
                                });
                            }),
                    )
                    .child(
                        Pagination::new("pages", self.page, 12)
                            .theme(theme)
                            .visible_pages(7)
                            .on_change(move |value, _, cx| {
                                let _ = page_owner.update(cx, |s, cx| {
                                    s.page = value;
                                    cx.notify();
                                });
                            }),
                    ),
            )
            .child(
                Card::new()
                    .theme(theme)
                    .child(self.heading("Actions and links"))
                    .child(
                        Toolbar::new("toolbar")
                            .theme(theme)
                            .child(
                                Button::new("previous-action", "Previous")
                                    .theme(theme)
                                    .variant(ButtonVariant::Outline)
                                    .on_click(cx.listener(|s, _, _, cx| {
                                        s.notice = "Previous local item".into();
                                        cx.notify();
                                    })),
                            )
                            .child(
                                Button::new("disabled-action", "Unavailable")
                                    .theme(theme)
                                    .disabled(true),
                            )
                            .child(
                                Button::new("next-action", "Next")
                                    .theme(theme)
                                    .variant(ButtonVariant::Outline)
                                    .on_click(cx.listener(|s, _, _, cx| {
                                        s.notice = "Next local item".into();
                                        cx.notify();
                                    })),
                            ),
                    )
                    .child(
                        Link::new("local-link", "View local documentation")
                            .theme(theme)
                            .href("gallery:documentation")
                            .open_with(move |url, _, _, cx| {
                                let _ = link_owner.update(cx, |s, cx| {
                                    s.notice = format!("Handled locally: {url}").into();
                                    cx.notify();
                                });
                            }),
                    ),
            )
    }
}

impl Render for Forms {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = self.theme;
        div()
            .id("forms-gallery")
            .key_context("AstryxForms")
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
                                Text::new(if self.navigation {
                                    "Astryx / Navigation"
                                } else {
                                    "Astryx / Forms"
                                })
                                .kind(TextKind::Heading)
                                .theme(theme)
                                .text_size(px(28.))
                                .line_height(px(32.)),
                            )
                            .child(
                                div()
                                    .flex()
                                    .gap(px(8.))
                                    .child(
                                        Button::new(
                                            "page",
                                            if self.navigation {
                                                "Forms"
                                            } else {
                                                "Navigation"
                                            },
                                        )
                                        .theme(theme)
                                        .variant(ButtonVariant::Outline)
                                        .on_click(
                                            cx.listener(|s, _, _, cx| {
                                                s.navigation = !s.navigation;
                                                s.opened = None;
                                                cx.notify();
                                            }),
                                        ),
                                    )
                                    .child(
                                        Button::new("theme", "Switch theme")
                                            .theme(theme)
                                            .variant(ButtonVariant::Outline)
                                            .on_click(cx.listener(|s, _, _, cx| {
                                                s.theme = if s.theme == Theme::light() {
                                                    Theme::dark()
                                                } else {
                                                    Theme::light()
                                                };
                                                cx.notify();
                                            })),
                                    ),
                            ),
                    )
                    .child(Text::new(self.notice.clone()).theme(theme).muted())
                    .child(div().flex_1().min_h_0().child(if self.navigation {
                        self.navigation(cx).into_any_element()
                    } else {
                        self.forms(cx).into_any_element()
                    })),
            )
    }
}

fn init(cx: &mut App) {
    astryx_gpui::init(cx);
    cx.bind_keys([
        KeyBinding::new("tab", FocusNext, Some("AstryxForms")),
        KeyBinding::new("shift-tab", FocusPrevious, Some("AstryxForms")),
    ]);
}
fn build(window: &mut Window, cx: &mut App, theme: Theme, navigation: bool) -> Entity<Forms> {
    let view = cx.new(|cx| Forms::new(theme, navigation, window, cx));
    view.read(cx).focus.clone().focus(window, cx);
    view
}
fn main() -> Result<()> {
    let mut theme = Theme::light();
    let mut output = None;
    let mut navigation = false;
    let mut args = std::env::args().skip(1);
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--dark" => theme = Theme::dark(),
            "--navigation" => navigation = true,
            "--snapshot" => {
                output = Some(PathBuf::from(
                    args.next().context("--snapshot requires a PNG path")?,
                ))
            }
            _ => bail!("Unknown argument: {arg}"),
        }
    }
    let dimensions = size(px(1120.), px(900.));
    if let Some(path) = output {
        let platform = gpui_platform::current_platform(true);
        let mut cx = gpui::HeadlessAppContext::with_platform(
            platform.text_system(),
            Arc::new(()),
            gpui_platform::current_headless_renderer,
        );
        cx.update(init);
        let window = cx.open_window(dimensions, |window, cx| {
            build(window, cx, theme, navigation)
        })?;
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
            move |window, cx| build(window, cx, theme, navigation),
        )
        .expect("open forms gallery");
        cx.activate(true);
    });
    Ok(())
}
