//! Native geometry gallery; captures use the real GPUI Metal renderer.
use anyhow::{Context as _, Result, bail};
use astryx_gpui::{
    Button, ButtonVariant, Card, Carousel, OverflowList, Resizable, ResizablePane, ResizableState,
    ScrollableArea, SnapPaging, Swipe, Text, TextKind, Theme,
};
use gpui::{
    App, AppContext as _, Bounds, Context, Entity, FocusHandle, IntoElement, KeyBinding, Render,
    Subscription, Window, WindowBounds, WindowOptions, actions, div, prelude::*, px, size,
};
use std::{path::PathBuf, sync::Arc};

actions!(geometry_gallery, [FocusNext, FocusPrevious]);
struct Geometry {
    theme: Theme,
    focus: FocusHandle,
    carousel: usize,
    page: usize,
    more: bool,
    gesture: String,
    resize: Entity<ResizableState>,
    _resize_watch: Subscription,
}
impl Geometry {
    fn new(theme: Theme, cx: &mut Context<Self>) -> Self {
        let resize = cx.new(|_| ResizableState::default());
        let watch = cx.observe(&resize, |_, _, cx| cx.notify());
        Self {
            theme,
            focus: cx.focus_handle(),
            carousel: 0,
            page: 0,
            more: false,
            gesture: "Waiting for a gesture".into(),
            resize,
            _resize_watch: watch,
        }
    }
    fn card(&self, title: &'static str) -> Card {
        Card::new()
            .theme(self.theme)
            .flex_1()
            .min_w_0()
            .gap(px(12.))
            .child(Text::new(title).kind(TextKind::Heading).theme(self.theme))
    }
}
impl Render for Geometry {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let t = self.theme;
        let c = t.colors;
        let carousel_owner = cx.entity().downgrade();
        let page_owner = cx.entity().downgrade();
        let swipe_owner = cx.entity().downgrade();
        let more_owner = cx.entity().downgrade();
        let carousel = self.card("Carousel").child(
            Carousel::new("demo-carousel")
                .theme(t)
                .index(self.carousel)
                .on_change(move |i, _, cx| {
                    let _ = carousel_owner.update(cx, |s, cx| {
                        s.carousel = i;
                        cx.notify();
                    });
                })
                .children((0..3).map(|i| {
                    div()
                        .h(px(70.))
                        .p(px(12.))
                        .rounded(px(8.))
                        .bg(c.neutral)
                        .child(format!("Slide {}", i + 1))
                })),
        );
        let paging = self
            .card("SnapPaging")
            .child(
                SnapPaging::new("demo-pages")
                    .page_width(460.)
                    .index(self.page)
                    .on_change(move |i, _, cx| {
                        let _ = page_owner.update(cx, |s, cx| {
                            s.page = i;
                            cx.notify();
                        });
                    })
                    .children((0..3).map(|i| {
                        div()
                            .h(px(70.))
                            .p(px(12.))
                            .rounded(px(8.))
                            .bg(c.neutral)
                            .child(format!("Page {}", i + 1))
                    })),
            )
            .child(
                Text::new("Horizontal wheel, swipe or focused Left / Right.")
                    .theme(t)
                    .muted(),
            );
        let swipe = self
            .card("Swipe")
            .child(
                Swipe::new("demo-swipe")
                    .h(px(108.))
                    .w_full()
                    .on_swipe(move |direction, _, cx| {
                        let _ = swipe_owner.update(cx, |s, cx| {
                            s.gesture = format!("Last gesture: {direction:?}");
                            cx.notify();
                        });
                    })
                    .child(
                        div()
                            .size_full()
                            .p(px(12.))
                            .rounded(px(8.))
                            .bg(c.neutral)
                            .child("Drag horizontally across this area"),
                    ),
            )
            .child(Text::new(self.gesture.clone()).theme(t).muted());
        let scroll = self
            .card("ScrollableArea")
            .child(
                ScrollableArea::new("demo-scroll")
                    .theme(t)
                    .h(px(108.))
                    .w_full()
                    .children((0..16).map(|i| {
                        div()
                            .flex_none()
                            .h(px(28.))
                            .child(format!("Scrollable row {}", i + 1))
                    })),
            )
            .child(
                Text::new("Native scroll offsets survive rebuilds.")
                    .theme(t)
                    .muted(),
            );
        let resize = self
            .card("Resizable")
            .child(
                Resizable::new("demo-resize", &self.resize)
                    .theme(t)
                    .h(px(120.))
                    .panel(
                        ResizablePane::new()
                            .size(240.)
                            .size_range(100., 400.)
                            .child(div().p(px(12.)).child("Drag the divider")),
                    )
                    .panel(
                        ResizablePane::new()
                            .size(240.)
                            .size_range(100., 400.)
                            .child(div().p(px(12.)).child("Constrained native panels")),
                    ),
            )
            .child(
                Text::new("Drag within the retained min/max bounds.")
                    .theme(t)
                    .muted(),
            );
        let overflow = self
            .card("OverflowList")
            .child(
                OverflowList::new("demo-overflow")
                    .theme(t)
                    .visible_count(2)
                    .open(self.more)
                    .on_open_change(move |open, _, cx| {
                        let _ = more_owner.update(cx, |s, cx| {
                            s.more = open;
                            cx.notify();
                        });
                    })
                    .children(
                        ["Alpha", "Beta", "Gamma", "Delta", "Epsilon"]
                            .into_iter()
                            .enumerate()
                            .map(|(i, title)| {
                                Button::new(("demo-item", i), title)
                                    .theme(t)
                                    .variant(ButtonVariant::Outline)
                            }),
                    ),
            )
            .child(
                Text::new("Remaining items use an anchored popup.")
                    .theme(t)
                    .muted(),
            );
        div()
            .key_context("AstryxGeometry")
            .track_focus(&self.focus)
            .tab_group()
            .on_action(|_: &FocusNext, w, cx| w.focus_next(cx))
            .on_action(|_: &FocusPrevious, w, cx| w.focus_prev(cx))
            .size_full()
            .p(px(24.))
            .flex()
            .flex_col()
            .gap(px(16.))
            .bg(c.canvas)
            .text_color(c.text)
            .font_family("Helvetica")
            .text_size(px(14.))
            .line_height(px(20.))
            .child(
                div()
                    .flex()
                    .justify_between()
                    .items_center()
                    .child(
                        Text::new("Astryx / Native geometry")
                            .kind(TextKind::Heading)
                            .theme(t)
                            .text_size(px(28.))
                            .line_height(px(32.)),
                    )
                    .child(
                        Button::new("theme", "Switch theme")
                            .theme(t)
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
                Text::new(
                    "Paging, pointer gestures, native scrolling and constrained resize handles.",
                )
                .theme(t)
                .muted(),
            )
            .child(
                div()
                    .flex()
                    .gap(px(16.))
                    .items_stretch()
                    .child(carousel)
                    .child(paging),
            )
            .child(
                div()
                    .flex()
                    .gap(px(16.))
                    .items_stretch()
                    .child(swipe)
                    .child(scroll),
            )
            .child(
                div()
                    .flex()
                    .gap(px(16.))
                    .items_stretch()
                    .child(resize)
                    .child(overflow),
            )
    }
}
fn init(cx: &mut App) {
    astryx_gpui::init(cx);
    cx.bind_keys([
        KeyBinding::new("tab", FocusNext, Some("AstryxGeometry")),
        KeyBinding::new("shift-tab", FocusPrevious, Some("AstryxGeometry")),
    ]);
}
fn build(window: &mut Window, cx: &mut App, theme: Theme) -> Entity<Geometry> {
    let view = cx.new(|cx| Geometry::new(theme, cx));
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
                    args.next().context("--snapshot needs a PNG path")?,
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
        let window = cx.open_window(size(px(1120.), px(900.)), |w, cx| build(w, cx, theme))?;
        cx.update_window(window.into(), |_, w, cx| w.draw(cx).clear(cx))?;
        if let Some(parent) = path.parent().filter(|p| !p.as_os_str().is_empty()) {
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
            move |w, cx| build(w, cx, theme),
        )
        .expect("open geometry gallery");
        cx.activate(true);
    });
    Ok(())
}
