//! Run an interactive native gallery, or capture genuine GPU-rendered PNGs.
use std::{path::PathBuf, sync::Arc};

use anyhow::{Context as _, Result, bail};
use astryx_gpui::{
    Appearance, Badge, Button, ButtonVariant, Card, Checkbox, Collapsible, ControlSize, Divider,
    Tab, Tabs, Text, TextKind, Theme,
};
use gpui::{
    App, AppContext, Bounds, Context, Entity, FocusHandle, IntoElement, KeyBinding, Keystroke,
    Render, SharedString, Window, WindowBounds, WindowOptions, actions, div, prelude::*, px, size,
};

actions!(gallery, [FocusNext, FocusPrevious]);

struct Gallery {
    theme: Theme,
    focus: FocusHandle,
    clicks: usize,
    checked: bool,
    tab: SharedString,
    open: bool,
}

impl Gallery {
    fn new(theme: Theme, cx: &mut Context<Self>) -> Self {
        Self {
            theme,
            focus: cx.focus_handle(),
            clicks: 0,
            checked: true,
            tab: "overview".into(),
            open: true,
        }
    }

    fn section(&self, title: &'static str) -> Text {
        Text::new(title).kind(TextKind::Heading).theme(self.theme)
    }

    fn note(&self, text: impl Into<SharedString>) -> Text {
        Text::new(text)
            .kind(TextKind::Supporting)
            .muted()
            .theme(self.theme)
    }
}

impl Render for Gallery {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = self.theme;
        let color = theme.colors;
        let change_checked = cx.listener(|this, next: &bool, _, cx| {
            this.checked = *next;
            cx.notify();
        });
        let change_tab = cx.listener(|this, next: &SharedString, _, cx| {
            this.tab = next.clone();
            cx.notify();
        });
        let change_open = cx.listener(|this, next: &bool, _, cx| {
            this.open = *next;
            cx.notify();
        });

        div()
            .id("astryx-gallery")
            .key_context("AstryxGallery")
            .track_focus(&self.focus)
            .tab_stop(false)
            .on_action(|_: &FocusNext, window, cx| window.focus_next(cx))
            .on_action(|_: &FocusPrevious, window, cx| window.focus_prev(cx))
            .size_full()
            .overflow_y_scroll()
            .bg(color.canvas)
            .text_color(color.text)
            .font_family("Helvetica")
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap(px(20.))
                    .p(px(28.))
                    .w_full()
                    .max_w(px(1120.))
                    .mx_auto()
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .justify_between()
                            .gap(px(16.))
                            .child(
                                div()
                                    .flex()
                                    .flex_col()
                                    .gap(px(8.))
                                    .child(
                                        Text::new("Astryx / GPUI")
                                            .kind(TextKind::Heading)
                                            .theme(theme)
                                            .text_size(px(28.))
                                            .line_height(px(32.)),
                                    )
                                    .child(self.note("Unofficial native components · controlled state · no web runtime")),
                            )
                            .child(
                                Button::new("switch-theme", "Switch theme")
                                    .theme(theme)
                                    .variant(ButtonVariant::Outline)
                                    .on_click(cx.listener(|this, _, _, cx| {
                                        this.theme = if this.theme.appearance == Appearance::Light {
                                            Theme::dark()
                                        } else {
                                            Theme::light()
                                        };
                                        cx.notify();
                                    })),
                            ),
                    )
                    .child(
                        div()
                            .flex()
                            .items_start()
                            .gap(px(20.))
                            .child(
                                Card::new()
                                    .theme(theme)
                                    .flex_1()
                                    .min_w_0()
                                    .child(self.section("Buttons"))
                                    .child(self.note("Native pointer, Enter and Space activation. Disabled controls remain inert."))
                                    .child(
                                        div()
                                            .flex()
                                            .flex_wrap()
                                            .gap(px(8.))
                                            .child(
                                                Button::new("primary", "Primary")
                                                    .theme(theme)
                                                    .on_click(cx.listener(|this, _, _, cx| {
                                                        this.clicks += 1;
                                                        cx.notify();
                                                    })),
                                            )
                                            .child(Button::new("secondary", "Secondary").theme(theme).variant(ButtonVariant::Secondary))
                                            .child(Button::new("outline", "Outline").theme(theme).variant(ButtonVariant::Outline))
                                            .child(Button::new("ghost", "Ghost").theme(theme).variant(ButtonVariant::Ghost)),
                                    )
                                    .child(
                                        div()
                                            .flex()
                                            .flex_wrap()
                                            .items_center()
                                            .gap(px(8.))
                                            .child(Button::new("small", "Small").theme(theme).size(ControlSize::Small))
                                            .child(Button::new("medium", "Medium").theme(theme))
                                            .child(Button::new("large", "Large").theme(theme).size(ControlSize::Large))
                                            .child(Button::new("disabled", "Disabled").theme(theme).disabled(true)),
                                    )
                                    .child(Text::new(format!("Primary activated {} times", self.clicks)).theme(theme))
                                    .child(Divider::new().theme(theme))
                                    .child(self.section("Checkbox"))
                                    .child(
                                        Checkbox::new("notifications", "Notify me about new activity")
                                            .theme(theme)
                                            .checked(self.checked)
                                            .on_change(move |value, window, cx| change_checked(&value, window, cx)),
                                    )
                                    .child(Checkbox::new("disabled-checkbox", "Disabled option").theme(theme).disabled(true))
                                    .child(self.note(if self.checked { "Notifications enabled" } else { "Notifications disabled" })),
                            )
                            .child(
                                Card::new()
                                    .theme(theme)
                                    .flex_1()
                                    .min_w_0()
                                    .child(self.section("Typography & badges"))
                                    .child(Text::new("A readable native body at 14 / 20 px.").theme(theme))
                                    .child(self.note("Supporting metadata uses 12 / 20 px."))
                                    .child(
                                        div()
                                            .flex()
                                            .flex_wrap()
                                            .gap(px(8.))
                                            .child(Badge::new("Neutral").theme(theme))
                                            .child(Badge::new("Success").theme(theme).tone(astryx_gpui::Tone::Success))
                                            .child(Badge::new("Warning").theme(theme).tone(astryx_gpui::Tone::Warning))
                                            .child(Badge::new("Danger").theme(theme).tone(astryx_gpui::Tone::Danger))
                                            .child(Badge::new("Info").theme(theme).tone(astryx_gpui::Tone::Info)),
                                    )
                                    .child(Divider::new().theme(theme))
                                    .child(self.section("Controlled disclosure"))
                                    .child(
                                        Collapsible::new("details", "Implementation details")
                                            .theme(theme)
                                            .open(self.open)
                                            .on_open_change(move |value, window, cx| change_open(&value, window, cx))
                                            .child(Text::new("The owning view keeps this open state. A redraw never resets your choice.").theme(theme))
                                            .child(self.note("Theme tokens and component behavior stay separate.")),
                                    ),
                            ),
                    )
                    .child(
                        Card::new()
                            .theme(theme)
                            .child(self.section("Tabs"))
                            .child(
                                Tabs::new("example-tabs")
                                    .theme(theme)
                                    .items(vec![
                                        Tab::new("overview", "Overview"),
                                        Tab::new("activity", "Activity"),
                                        Tab::new("settings", "Settings"),
                                        Tab::new("unavailable", "Unavailable").disabled(true),
                                    ])
                                    .selected(self.tab.clone())
                                    .on_select(move |value, window, cx| change_tab(&value, window, cx)),
                            )
                            .child(Text::new(format!("Selected: {}", self.tab)).theme(theme))
                            .child(self.note("Arrow keys and Home / End move between enabled tabs.")),
                    )
                    .child(self.note("Tab / Shift+Tab move focus. Enter / Space activate. This gallery owns its own focus-navigation keybindings.")),
            )
    }
}

fn build(window: &mut Window, cx: &mut App, theme: Theme) -> Entity<Gallery> {
    let view = cx.new(|cx| Gallery::new(theme, cx));
    let focus = view.read(cx).focus.clone();
    window.focus(&focus, cx);
    view
}

fn init(cx: &mut App) {
    astryx_gpui::init(cx);
    cx.bind_keys([
        KeyBinding::new("tab", FocusNext, Some("AstryxGallery")),
        KeyBinding::new("shift-tab", FocusPrevious, Some("AstryxGallery")),
    ]);
}

fn snapshot(path: PathBuf, theme: Theme, focus_primary: bool) -> Result<()> {
    let platform = gpui_platform::current_platform(true);
    let mut cx = gpui::HeadlessAppContext::with_platform(
        platform.text_system(),
        Arc::new(()),
        gpui_platform::current_headless_renderer,
    );
    cx.update(init);
    let window = cx.open_window(size(px(1120.), px(860.)), |window, cx| {
        build(window, cx, theme)
    })?;
    cx.update_window(window.into(), |_, window, cx| {
        window.draw(cx).clear(cx);
        if focus_primary {
            // The first stop is the theme switch; the second is the primary button.
            for _ in 0..2 {
                window.dispatch_keystroke(Keystroke::parse("tab").expect("valid key"), cx);
                window.draw(cx).clear(cx);
            }
        }
    })?;
    if let Some(parent) = path
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
    {
        std::fs::create_dir_all(parent)?;
    }
    cx.capture_screenshot(window.into())?.save(&path)?;
    println!("{}", path.display());
    Ok(())
}

fn main() -> Result<()> {
    let mut args = std::env::args().skip(1);
    let mut output = None;
    let mut focus_primary = false;
    let mut theme = Theme::light();
    while let Some(argument) = args.next() {
        match argument.as_str() {
            "--dark" => theme = Theme::dark(),
            "--focus-primary" => focus_primary = true,
            "--snapshot" => {
                output = Some(PathBuf::from(
                    args.next().context("--snapshot requires a PNG path")?,
                ))
            }
            _ => bail!(
                "Unknown argument: {argument}. Use --dark, --snapshot PATH.png or --focus-primary"
            ),
        }
    }
    if let Some(path) = output {
        return snapshot(path, theme, focus_primary);
    }
    if focus_primary {
        bail!("--focus-primary requires --snapshot PATH.png");
    }
    gpui_platform::application().run(move |cx: &mut App| {
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
            |window, cx| build(window, cx, theme),
        )
        .expect("open gallery window");
        cx.activate(true);
    });
    Ok(())
}
