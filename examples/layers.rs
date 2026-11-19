//! Native layers and collections with synthetic data and offscreen Metal capture.
use std::{path::PathBuf, sync::Arc};

use anyhow::{Context as _, Result, bail};
use astryx_gpui::{
    AlertDialog, Badge, Banner, Button, ButtonVariant, Card, CommandItem, CommandPalette,
    ContextMenu, InfoTip, InputState, Item, Kbd, List, ListEntry, Menu, MenuItem, Popover,
    SortDirection, StatusDot, Table, TableColumn, TableRow, Text, TextKind, Theme, Toast, Tone,
    TreeItem, TreeList, TreeState,
};
use gpui::{
    App, AppContext, Bounds, Context, Entity, FocusHandle, IntoElement, KeyBinding, Render,
    SharedString, Window, WindowBounds, WindowOptions, actions, div, prelude::*, px, size,
};

actions!(layers, [FocusNext, FocusPrevious]);

struct Layers {
    theme: Theme,
    focus: FocusHandle,
    query: Entity<InputState>,
    tree: Entity<TreeState>,
    menu_open: bool,
    menu_highlight: usize,
    context_open: bool,
    context_highlight: usize,
    popover_open: bool,
    info_open: bool,
    command_open: bool,
    command_highlight: usize,
    dialog_open: bool,
    toast_visible: bool,
    selected_list: SharedString,
    selected_row: SharedString,
    list_order: Vec<SharedString>,
    sort: SortDirection,
    last_action: SharedString,
}

impl Layers {
    fn new(theme: Theme, dialog_open: bool, window: &mut Window, cx: &mut Context<Self>) -> Self {
        Self {
            theme,
            focus: cx.focus_handle(),
            query: cx.new(|cx| InputState::new(window, cx).placeholder("Search commands…")),
            tree: cx.new(|cx| {
                TreeState::new(cx).items(vec![
                    TreeItem::new("workspace", "Workspace")
                        .expanded(true)
                        .child(
                            TreeItem::new("components", "Components")
                                .expanded(true)
                                .child(TreeItem::new("navigation", "Navigation"))
                                .child(TreeItem::new("feedback", "Feedback")),
                        )
                        .child(TreeItem::new("guides", "Guides"))
                        .child(TreeItem::new("archived", "Archived").disabled(true)),
                ])
            }),
            menu_open: false,
            menu_highlight: 0,
            context_open: false,
            context_highlight: 0,
            popover_open: false,
            info_open: false,
            command_open: false,
            command_highlight: 0,
            dialog_open,
            toast_visible: true,
            selected_list: "navigation".into(),
            selected_row: "navigation".into(),
            list_order: vec!["navigation".into(), "feedback".into(), "inputs".into()],
            sort: SortDirection::Ascending,
            last_action: "Ready for an action".into(),
        }
    }

    fn heading(&self, title: &'static str) -> Text {
        Text::new(title).kind(TextKind::Heading).theme(self.theme)
    }

    fn supporting(&self, text: &'static str) -> Text {
        Text::new(text)
            .kind(TextKind::Supporting)
            .muted()
            .theme(self.theme)
    }
}

fn list_label(value: &str) -> &'static str {
    match value {
        "navigation" => "Navigation patterns",
        "feedback" => "Feedback states",
        _ => "Input and focus",
    }
}

impl Render for Layers {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = self.theme;
        let c = theme.colors;
        let menu_open = cx.entity().downgrade();
        let menu_highlight = menu_open.clone();
        let menu_select = menu_open.clone();
        let context_open = menu_open.clone();
        let context_highlight = menu_open.clone();
        let context_select = menu_open.clone();
        let popover_open = menu_open.clone();
        let info_open = menu_open.clone();
        let list_select = menu_open.clone();
        let list_move = menu_open.clone();
        let row_select = menu_open.clone();
        let table_sort = menu_open.clone();
        let tree_activate = menu_open.clone();
        let command_open = menu_open.clone();
        let command_highlight = menu_open.clone();
        let command_select = menu_open.clone();
        let dialog_open = menu_open.clone();
        let dialog_confirm = menu_open.clone();
        let toast_dismiss = menu_open.clone();

        let menus = Card::new()
            .theme(theme)
            .h(px(318.))
            .child(self.heading("Menus & help"))
            .child(self.supporting("Anchored layers with native focus return."))
            .child(
                Menu::new("actions")
                    .theme(theme)
                    .open(self.menu_open)
                    .highlighted(self.menu_highlight)
                    .items(vec![
                        MenuItem::new("review", "Start review").shortcut("↵"),
                        MenuItem::new("copy", "Copy reference"),
                        MenuItem::new("archive", "Archive").disabled(true),
                    ])
                    .trigger(
                        Button::new("actions-trigger", "Review actions  ▾")
                            .theme(theme)
                            .variant(ButtonVariant::Outline),
                    )
                    .on_open_change(move |open, _, cx| {
                        let _ = menu_open.update(cx, |this, cx| {
                            this.menu_open = open;
                            cx.notify();
                        });
                    })
                    .on_highlight(move |index, _, cx| {
                        let _ = menu_highlight.update(cx, |this, cx| {
                            this.menu_highlight = index;
                            cx.notify();
                        });
                    })
                    .on_select(move |value, _, cx| {
                        let _ = menu_select.update(cx, |this, cx| {
                            this.last_action = format!("Menu: {value}").into();
                            this.toast_visible = true;
                            cx.notify();
                        });
                    }),
            )
            .child(
                Popover::new("review-details")
                    .theme(theme)
                    .open(self.popover_open)
                    .trigger(
                        Button::new("details-trigger", "View details")
                            .theme(theme)
                            .variant(ButtonVariant::Outline),
                    )
                    .on_open_change(move |open, _, cx| {
                        let _ = popover_open.update(cx, |this, cx| {
                            this.popover_open = open;
                            cx.notify();
                        });
                    })
                    .content(move |_, _, _| {
                        div()
                            .flex()
                            .flex_col()
                            .gap(px(8.))
                            .w(px(230.))
                            .child(Text::new("A little more context").theme(theme))
                            .child(
                                Text::new("Native popup positioning. Escape returns focus to the trigger.")
                                    .theme(theme)
                                    .muted(),
                            )
                    }),
            )
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap(px(8.))
                    .child(self.supporting("Why is Archive unavailable?"))
                    .child(
                        InfoTip::new("archive-help", "Only completed reviews can be archived.")
                            .theme(theme)
                            .open(self.info_open)
                            .on_open_change(move |open, _, cx| {
                                let _ = info_open.update(cx, |this, cx| {
                                    this.info_open = open;
                                    cx.notify();
                                });
                            }),
                    ),
            )
            .child(
                ContextMenu::new("context-actions")
                    .theme(theme)
                    .open(self.context_open)
                    .highlighted(self.context_highlight)
                    .items(vec![
                        MenuItem::new("pin", "Pin to workspace"),
                        MenuItem::new("duplicate", "Duplicate view"),
                    ])
                    .trigger(
                        Button::new("context-trigger", "Right-click for actions")
                            .theme(theme)
                            .variant(ButtonVariant::Ghost),
                    )
                    .on_open_change(move |open, _, cx| {
                        let _ = context_open.update(cx, |this, cx| {
                            this.context_open = open;
                            cx.notify();
                        });
                    })
                    .on_highlight(move |index, _, cx| {
                        let _ = context_highlight.update(cx, |this, cx| {
                            this.context_highlight = index;
                            cx.notify();
                        });
                    })
                    .on_select(move |value, _, cx| {
                        let _ = context_select.update(cx, |this, cx| {
                            this.last_action = format!("Context: {value}").into();
                            cx.notify();
                        });
                    }),
            );

        let queue = Card::new()
            .theme(theme)
            .flex_1()
            .child(self.heading("Review queue"))
            .child(self.supporting("Select with arrows. Drag to reorder."))
            .child(
                List::new("review-queue")
                    .theme(theme)
                    .entries(
                        self.list_order
                            .iter()
                            .map(|value| {
                                ListEntry::new(value.clone(), list_label(value))
                                    .description("Workspace / Component library")
                            })
                            .collect(),
                    )
                    .selected(self.selected_list.clone())
                    .on_select(move |value, _, cx| {
                        let _ = list_select.update(cx, |this, cx| {
                            this.selected_list = value;
                            cx.notify();
                        });
                    })
                    .on_move(move |value, before, _, cx| {
                        let _ = list_move.update(cx, |this, cx| {
                            this.list_order.retain(|id| id != &value);
                            let index = before
                                .as_ref()
                                .and_then(|id| this.list_order.iter().position(|row| row == id))
                                .unwrap_or(this.list_order.len());
                            this.list_order.insert(index, value);
                            this.last_action = "Queue reordered".into();
                            cx.notify();
                        });
                    }),
            );

        let mut review_rows = vec![
            ("feedback", "Feedback states", "Recheck", Tone::Warning),
            ("inputs", "Input and focus", "Approved", Tone::Success),
            ("navigation", "Navigation", "Review", Tone::Info),
        ];
        review_rows.sort_by_key(|row| row.1);
        if self.sort == SortDirection::Descending {
            review_rows.reverse();
        }
        let table = Card::new()
            .theme(theme)
            .h(px(318.))
            .child(self.heading("Status at a glance"))
            .child(self.supporting("Sort the title column; select a row."))
            .child(
                Table::new("reviews")
                    .theme(theme)
                    .columns(vec![
                        TableColumn::new("title", "Title").sortable(true),
                        TableColumn::new("state", "State").width(90.),
                    ])
                    .rows(
                        review_rows
                            .into_iter()
                            .map(|(key, title, state, tone)| {
                                TableRow::new(key)
                                    .cell(title)
                                    .cell(Badge::new(state).theme(theme).tone(tone))
                            })
                            .collect(),
                    )
                    .selected(self.selected_row.clone())
                    .sorted_by("title", self.sort)
                    .on_sort(move |_, direction, _, cx| {
                        let _ = table_sort.update(cx, |this, cx| {
                            this.sort = direction;
                            this.last_action = "Table ordering updated".into();
                            cx.notify();
                        });
                    })
                    .on_row_select(move |value, _, cx| {
                        let _ = row_select.update(cx, |this, cx| {
                            this.selected_row = value;
                            cx.notify();
                        });
                    }),
            );
        let feedback = Card::new()
            .theme(theme)
            .flex_1()
            .child(self.heading("Feedback & confirmation"))
            .child(
                StatusDot::new("All changes saved")
                    .theme(theme)
                    .tone(Tone::Success),
            )
            .child(self.supporting("Clear feedback, explicit decisions."))
            .child(
                Button::new("show-toast", "Show notification")
                    .theme(theme)
                    .variant(ButtonVariant::Outline)
                    .on_click(cx.listener(|this, _, _, cx| {
                        this.toast_visible = true;
                        cx.notify();
                    })),
            )
            .child(
                Button::new("show-alert", "Archive completed reviews")
                    .theme(theme)
                    .variant(ButtonVariant::Outline)
                    .on_click(cx.listener(|this, _, _, cx| {
                        this.dialog_open = true;
                        cx.notify();
                    })),
            )
            .when(self.toast_visible, |card| {
                card.child(
                    Toast::new("saved-notification", "Workspace updated")
                        .theme(theme)
                        .tone(Tone::Success)
                        .w_full()
                        .description("Your review queue is up to date.")
                        .on_dismiss(move |_, cx| {
                            let _ = toast_dismiss.update(cx, |this, cx| {
                                this.toast_visible = false;
                                this.last_action = "Notification dismissed".into();
                                cx.notify();
                            });
                        }),
                )
            });

        let tree = Card::new()
            .theme(theme)
            .h(px(318.))
            .child(self.heading("Workspace tree"))
            .child(self.supporting("Virtual rows. Expand with arrow keys."))
            .child(
                TreeList::new(&self.tree)
                    .theme(theme)
                    .h(px(210.))
                    .on_activate(move |value, _, cx| {
                        let _ = tree_activate.update(cx, |this, cx| {
                            this.last_action = format!("Opened: {value}").into();
                            cx.notify();
                        });
                    }),
            );
        let search = Card::new()
            .theme(theme)
            .flex_1()
            .child(self.heading("Find an action"))
            .child(self.supporting("Local filtering with IME-safe native input."))
            .child(
                Button::new("show-commands", "Search commands…")
                    .theme(theme)
                    .variant(ButtonVariant::Outline)
                    .w_full()
                    .on_click(cx.listener(|this, _, _, cx| {
                        this.command_open = true;
                        cx.notify();
                    })),
            )
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap(px(8.))
                    .child(Kbd::new("↑ ↓").theme(theme))
                    .child(self.supporting("Navigate"))
                    .child(Kbd::new("↵").theme(theme))
                    .child(self.supporting("Select")),
            )
            .child(
                Item::new("palette-example", "Jump to review queue")
                    .theme(theme)
                    .description("An example command")
                    .on_click(cx.listener(|this, _, _, cx| {
                        this.command_open = true;
                        cx.notify();
                    })),
            );

        div()
            .id("layers-gallery")
            .key_context("AstryxLayers")
            .track_focus(&self.focus)
            .tab_group()
            .tab_stop(false)
            .size_full()
            .bg(c.canvas)
            .text_color(c.text)
            .font_family("Helvetica")
            .on_action(|_: &FocusNext, window, cx| window.focus_next(cx))
            .on_action(|_: &FocusPrevious, window, cx| window.focus_prev(cx))
            .child(
                div()
                    .flex()
                    .flex_col()
                    .p(px(28.))
                    .gap(px(18.))
                    .size_full()
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .justify_between()
                            .child(
                                div()
                                    .flex()
                                    .flex_col()
                                    .gap(px(6.))
                                    .child(
                                        Text::new("Astryx / Layers & collections")
                                            .kind(TextKind::Heading)
                                            .theme(theme)
                                            .text_size(px(26.))
                                            .line_height(px(32.)),
                                    )
                                    .child(self.supporting("Native presentation. Owner-controlled behavior.")),
                            )
                            .child(
                                Button::new("switch-theme", "Switch theme")
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
                    .child(
                        Banner::new("A synthetic workspace")
                            .theme(theme)
                            .tone(Tone::Info)
                            .child("No repositories, providers or system actions are accessed. Every control updates local state."),
                    )
                    .child(
                        div()
                            .flex()
                            .flex_1()
                            .min_h_0()
                            .gap(px(16.))
                            .child(div().flex().flex_col().flex_1().min_w_0().gap(px(16.)).child(menus).child(queue))
                            .child(div().flex().flex_col().flex_1().min_w_0().gap(px(16.)).child(table).child(feedback))
                            .child(div().flex().flex_col().flex_1().min_w_0().gap(px(16.)).child(tree).child(search)),
                    )
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .justify_between()
                            .child(Text::new(self.last_action.clone()).theme(theme).muted())
                            .child(self.supporting("Tab to explore · Escape to dismiss")),
                    ),
            )
            .child(
                CommandPalette::new("commands", &self.query)
                    .theme(theme)
                    .title("Search workspace commands")
                    .open(self.command_open)
                    .highlighted(self.command_highlight)
                    .items(vec![
                        CommandItem::new("queue", "Jump to review queue").description("Workspace navigation"),
                        CommandItem::new("new", "Create a new review").description("Start from a clean view"),
                        CommandItem::new("theme", "Change appearance").keywords("light dark theme"),
                        CommandItem::new("export", "Export workspace").disabled(true),
                    ])
                    .on_open_change(move |open, _, cx| {
                        let _ = command_open.update(cx, |this, cx| {
                            this.command_open = open;
                            cx.notify();
                        });
                    })
                    .on_highlight(move |index, _, cx| {
                        let _ = command_highlight.update(cx, |this, cx| {
                            this.command_highlight = index;
                            cx.notify();
                        });
                    })
                    .on_select(move |value, _, cx| {
                        let _ = command_select.update(cx, |this, cx| {
                            this.last_action = format!("Command: {value}").into();
                            cx.notify();
                        });
                    }),
            )
            .child(
                AlertDialog::new("archive-confirmation", "Archive completed reviews?")
                    .theme(theme)
                    .open(self.dialog_open)
                    .description("This sample only updates local gallery state. Your workspace and repositories remain untouched.")
                    .confirm_label("Archive reviews")
                    .cancel_label("Keep reviews")
                    .on_confirm(move |_, _, cx| {
                        let _ = dialog_confirm.update(cx, |this, cx| {
                            this.last_action = "Completed reviews archived (sample)".into();
                            cx.notify();
                        });
                        true
                    })
                    .on_open_change(move |open, _, _, cx| {
                        let _ = dialog_open.update(cx, |this, cx| {
                            this.dialog_open = open;
                            cx.notify();
                        });
                    }),
            )
    }
}

fn init(cx: &mut App) {
    astryx_gpui::init(cx);
    cx.bind_keys([
        KeyBinding::new("tab", FocusNext, Some("AstryxLayers")),
        KeyBinding::new("shift-tab", FocusPrevious, Some("AstryxLayers")),
    ]);
}

fn build(window: &mut Window, cx: &mut App, theme: Theme, dialog: bool) -> Entity<Layers> {
    let view = cx.new(|cx| Layers::new(theme, dialog, window, cx));
    view.read(cx).focus.clone().focus(window, cx);
    view
}

fn main() -> Result<()> {
    let mut theme = Theme::light();
    let mut output = None;
    let mut dialog = false;
    let mut args = std::env::args().skip(1);
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--dark" => theme = Theme::dark(),
            "--dialog" => dialog = true,
            "--snapshot" => {
                output = Some(PathBuf::from(
                    args.next().context("--snapshot requires a PNG path")?,
                ));
            }
            _ => bail!("Unknown argument: {arg}"),
        }
    }
    if let Some(path) = output {
        let platform = gpui_platform::current_platform(true);
        let mut cx = gpui::HeadlessAppContext::with_platform(
            platform.text_system(),
            Arc::new(()),
            gpui_platform::current_headless_renderer,
        );
        cx.update(init);
        let window = cx.open_window(size(px(1120.), px(900.)), |window, cx| {
            build(window, cx, theme, dialog)
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
                    None,
                    size(px(1120.), px(900.)),
                    cx,
                ))),
                ..Default::default()
            },
            move |window, cx| build(window, cx, theme, dialog),
        )
        .expect("open layers gallery");
        cx.activate(true);
    });
    Ok(())
}
