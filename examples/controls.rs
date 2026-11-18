//! Real native forms and layers, with an optional offscreen Metal PNG capture.
use std::{path::PathBuf, sync::Arc};

use anyhow::{Context as _, Result, bail};
use astryx_gpui::{
    Banner, Button, ButtonVariant, Card, Dialog, EmptyState, Input, InputState, Kbd, Popover,
    ProgressBar, Select, SelectOption, Skeleton, StatusDot, Switch, Text, TextArea, TextKind,
    TextareaState, Theme, Tone, Tooltip,
};
use gpui::{
    App, AppContext, Bounds, Context, Entity, FocusHandle, IntoElement, KeyBinding, Render,
    SharedString, Window, WindowBounds, WindowOptions, actions, div, prelude::*, px, size,
};

actions!(controls, [FocusNext, FocusPrevious]);

struct Controls {
    theme: Theme,
    focus: FocusHandle,
    name: Entity<InputState>,
    description: Entity<TextareaState>,
    selected: SharedString,
    highlighted: usize,
    select_open: bool,
    popover_open: bool,
    dialog_open: bool,
    notifications: bool,
}

impl Controls {
    fn new(theme: Theme, dialog_open: bool, window: &mut Window, cx: &mut Context<Self>) -> Self {
        Self {
            theme,
            focus: cx.focus_handle(),
            name: cx.new(|cx| InputState::new(window, cx).placeholder("Session name")),
            description: cx
                .new(|cx| TextareaState::new(window, cx).placeholder("Describe the task…")),
            selected: "claude".into(),
            highlighted: 0,
            select_open: false,
            popover_open: false,
            dialog_open,
            notifications: true,
        }
    }
    fn heading(&self, label: &'static str) -> Text {
        Text::new(label).kind(TextKind::Heading).theme(self.theme)
    }
}

impl Render for Controls {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = self.theme;
        let c = theme.colors;
        let on_value = cx.entity().downgrade();
        let on_open = cx.entity().downgrade();
        let on_highlight = cx.entity().downgrade();
        let on_popover = cx.entity().downgrade();
        let on_dialog = cx.entity().downgrade();
        let on_notifications = cx.entity().downgrade();
        div().id("controls-gallery").key_context("AstryxControls").track_focus(&self.focus)
            .tab_group().tab_stop(false).size_full().bg(c.canvas).text_color(c.text).font_family("Helvetica")
            .on_action(|_: &FocusNext, window, cx| window.focus_next(cx))
            .on_action(|_: &FocusPrevious, window, cx| window.focus_prev(cx))
            .child(div().flex().flex_col().p(px(28.)).gap(px(20.)).size_full()
                .child(div().flex().items_center().justify_between()
                    .child(Text::new("Astryx / Native controls").kind(TextKind::Heading).theme(theme).text_size(px(28.)).line_height(px(32.)))
                    .child(Button::new("theme", "Switch theme").theme(theme).variant(ButtonVariant::Outline)
                        .on_click(cx.listener(|this,_,_,cx| {this.theme = if this.theme == Theme::light() {Theme::dark()}else{Theme::light()};cx.notify();}))))
                .child(Banner::new("Native state, native focus").tone(Tone::Info).theme(theme)
                    .child("Editing, IME, selection, disabled behavior and popup dismissal use native GPUI primitives."))
                .child(div().flex().gap(px(16.)).min_h_0().flex_1()
                    .child(Card::new().theme(theme).flex_1()
                        .child(self.heading("Create a session"))
                        .child(Text::new("Session name").kind(TextKind::Supporting).muted().theme(theme))
                        .child(Input::new(&self.name).theme(theme).label("Session name"))
                        .child(Text::new("Description").kind(TextKind::Supporting).muted().theme(theme))
                        .child(TextArea::new(&self.description).theme(theme).label("Description"))
                        .child(Text::new("Program").kind(TextKind::Supporting).muted().theme(theme))
                        .child(Select::new("program").theme(theme).label("Program")
                            .items(vec![SelectOption::new("claude","Claude Code"),SelectOption::new("codex","Codex"),SelectOption::new("terminal","Terminal"),SelectOption::new("offline","Unavailable").disabled(true)])
                            .selected(self.selected.clone()).highlighted(self.highlighted).open(self.select_open)
                            .on_select(move |value,_,cx|{let _=on_value.update(cx,|this,cx|{this.selected=value;cx.notify();});})
                            .on_open_change(move |value,_,cx|{let _=on_open.update(cx,|this,cx|{this.select_open=value;cx.notify();});})
                            .on_highlight(move |value,_,cx|{let _=on_highlight.update(cx,|this,cx|{this.highlighted=value;cx.notify();});}))
                        .child(Switch::new("notifications","Notify when complete").theme(theme).checked(self.notifications)
                            .on_change(move |value,_,cx|{let _=on_notifications.update(cx,|this,cx|{this.notifications=value;cx.notify();});}))
                        .child(div().flex().items_center().justify_between()
                            .child(Kbd::new("⌘ Enter").theme(theme))
                            .child(Button::new("create","Create session").theme(theme).on_click(cx.listener(|this,_,_,cx|{this.dialog_open=true;cx.notify();})))))
                    .child(Card::new().theme(theme).flex_1()
                        .child(self.heading("Feedback and layers"))
                        .child(StatusDot::new("Agent is ready").theme(theme).tone(Tone::Success))
                        .child(ProgressBar::new("usage").theme(theme).label("Plan usage").value(47.))
                        .child(Text::new("47% used · 53% remaining").kind(TextKind::Supporting).theme(theme).muted())
                        .child(Skeleton::new().theme(theme).w(px(240.)))
                        .child(Skeleton::new().theme(theme).w(px(180.)))
                        .child(Popover::new("details").theme(theme).open(self.popover_open)
                            .on_open_change(move |value,_,cx|{let _=on_popover.update(cx,|this,cx|{this.popover_open=value;cx.notify();});})
                            .trigger(Button::new("details-trigger","Open details").theme(theme).variant(ButtonVariant::Outline))
                            .content(move |_,_,_| Text::new("Anchored native popover. Escape returns focus.").theme(theme)))
                        .child(Button::new("hint","Hover for a tooltip").theme(theme).variant(ButtonVariant::Ghost)
                            .leading(div().child("ⓘ")).tooltip(Tooltip::with_theme("A native accessible tooltip",theme)))
                        .child(EmptyState::new("No reviews waiting").description("New review requests will appear here.").theme(theme)))))
            .child(Dialog::new("confirmation").open(self.dialog_open).theme(theme).title("Ready to start")
                .on_open_change(move |value,_,_,cx|{let _=on_dialog.update(cx,|this,cx|{this.dialog_open=value;cx.notify();});})
                .child(Text::new("This is a synthetic gallery. It does not launch an agent or access a repository.").theme(theme))
                .child(Button::new("close-dialog","Close").theme(theme).on_click(cx.listener(|this,_,_,cx|{this.dialog_open=false;cx.notify();}))))
    }
}

fn init(cx: &mut App) {
    astryx_gpui::init(cx);
    cx.bind_keys([
        KeyBinding::new("tab", FocusNext, Some("AstryxControls")),
        KeyBinding::new("shift-tab", FocusPrevious, Some("AstryxControls")),
    ]);
}
fn build(window: &mut Window, cx: &mut App, theme: Theme, dialog: bool) -> Entity<Controls> {
    let view = cx.new(|cx| Controls::new(theme, dialog, window, cx));
    let focus = view.read(cx).focus.clone();
    focus.focus(window, cx);
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
                ))
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
        let window = cx.open_window(size(px(1120.), px(860.)), |window, cx| {
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
                    size(px(1120.), px(860.)),
                    cx,
                ))),
                ..Default::default()
            },
            move |window, cx| build(window, cx, theme, dialog),
        )
        .expect("open controls gallery");
        cx.activate(true);
    });
    Ok(())
}
