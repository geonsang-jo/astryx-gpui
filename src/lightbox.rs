use crate::{Button, ButtonVariant, Dialog, Theme};
use gpui::{
    App, ElementId, ImageSource, InteractiveElement as _, IntoElement, ParentElement, RenderOnce,
    Role, SharedString, StatefulInteractiveElement as _, StyleRefinement, Styled, StyledImage as _,
    Window, div, img, prelude::FluentBuilder as _, px, relative,
};
use gpui_base::StyledExt as _;
use std::rc::Rc;

type IndexChange = Rc<dyn Fn(usize, &mut Window, &mut App)>;
type OpenChange = Rc<dyn Fn(bool, &mut Window, &mut App)>;
#[derive(Clone)]
pub struct LightboxItem {
    pub source: ImageSource,
    pub label: SharedString,
}
impl LightboxItem {
    pub fn new(source: impl Into<ImageSource>, label: impl Into<SharedString>) -> Self {
        Self {
            source: source.into(),
            label: label.into(),
        }
    }
}
/// A controlled native image gallery with modal Escape/focus return, wraparound
/// previous/next controls and plain Left/Right keyboard navigation.
#[derive(IntoElement)]
pub struct Lightbox {
    id: ElementId,
    items: Vec<LightboxItem>,
    active: usize,
    open: bool,
    theme: Theme,
    on_index: Option<IndexChange>,
    on_open: Option<OpenChange>,
    style: StyleRefinement,
}
impl Lightbox {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            items: vec![],
            active: 0,
            open: false,
            theme: Theme::default(),
            on_index: None,
            on_open: None,
            style: StyleRefinement::default(),
        }
    }
    pub fn items(mut self, items: Vec<LightboxItem>) -> Self {
        self.items = items;
        self
    }
    pub fn active(mut self, index: usize) -> Self {
        self.active = index;
        self
    }
    pub fn open(mut self, open: bool) -> Self {
        self.open = open;
        self
    }
    pub fn theme(mut self, theme: Theme) -> Self {
        self.theme = theme;
        self
    }
    pub fn on_active_change(
        mut self,
        handler: impl Fn(usize, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_index = Some(Rc::new(handler));
        self
    }
    pub fn on_open_change(
        mut self,
        handler: impl Fn(bool, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_open = Some(Rc::new(handler));
        self
    }
}
impl Styled for Lightbox {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}
impl RenderOnce for Lightbox {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let focus = window
            .use_keyed_state((self.id.clone(), "lightbox-focus"), cx, |_, cx| {
                cx.focus_handle()
            })
            .read(cx)
            .clone();
        let count = self.items.len();
        let active = self.active.min(count.saturating_sub(1));
        let c = self.theme.colors;
        let keyboard = self.on_index.clone();
        let previous = self.on_index.clone();
        let next = self.on_index;
        let close = self.on_open.clone();
        let content = div()
            .id((self.id.clone(), "gallery"))
            .size_full()
            .flex()
            .flex_col()
            .min_h_0()
            .gap(px(16.))
            .track_focus(&focus)
            .tab_stop(false)
            .on_key_down(move |event, window, cx| {
                if count < 2 || event.keystroke.modifiers != gpui::Modifiers::default() {
                    return;
                }
                let index = match event.keystroke.key.as_str() {
                    "left" => Some((active + count - 1) % count),
                    "right" => Some((active + 1) % count),
                    _ => None,
                };
                if let Some(index) = index {
                    cx.stop_propagation();
                    if let Some(callback) = &keyboard {
                        callback(index, window, cx);
                    }
                }
            })
            .child(
                div()
                    .flex()
                    .items_center()
                    .justify_between()
                    .gap(px(12.))
                    .child(
                        div()
                            .text_color(c.muted)
                            .text_size(px(12.))
                            .child(if count == 0 {
                                "0 / 0".into()
                            } else {
                                format!("{} / {}", active + 1, count)
                            }),
                    )
                    .child(
                        Button::new((self.id.clone(), "close"), "Close")
                            .theme(self.theme)
                            .variant(ButtonVariant::Ghost)
                            .on_click(move |_, window, cx| {
                                if let Some(close) = &close {
                                    close(false, window, cx);
                                }
                            }),
                    ),
            )
            .child(
                div()
                    .flex_1()
                    .min_h_0()
                    .flex()
                    .items_center()
                    .justify_center()
                    .when_some(self.items.get(active).cloned(), |el, item| {
                        el.child(
                            div()
                                .id("lightbox-image")
                                .role(Role::Image)
                                .aria_label(item.label.clone())
                                .size_full()
                                .child(
                                    img(item.source)
                                        .object_fit(gpui::ObjectFit::Contain)
                                        .size_full(),
                                ),
                        )
                    })
                    .when(count == 0, |el| el.child("No images")),
            )
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap(px(12.))
                    .child(
                        Button::new((self.id.clone(), "previous"), "Previous")
                            .theme(self.theme)
                            .variant(ButtonVariant::Outline)
                            .disabled(count < 2)
                            .on_click(move |_, window, cx| {
                                if count > 1
                                    && let Some(callback) = &previous
                                {
                                    callback((active + count - 1) % count, window, cx);
                                }
                            }),
                    )
                    .child(
                        div()
                            .flex_1()
                            .min_w_0()
                            .text_color(c.muted)
                            .text_size(px(12.))
                            .line_height(px(20.))
                            .children(self.items.get(active).map(|item| item.label.clone())),
                    )
                    .child(
                        Button::new((self.id.clone(), "next"), "Next")
                            .theme(self.theme)
                            .variant(ButtonVariant::Outline)
                            .disabled(count < 2)
                            .on_click(move |_, window, cx| {
                                if count > 1
                                    && let Some(callback) = &next
                                {
                                    callback((active + 1) % count, window, cx);
                                }
                            }),
                    ),
            );
        Dialog::new(self.id)
            .open(self.open)
            .theme(self.theme)
            .initial_focus(&focus)
            .on_open_change(move |open, _, window, cx| {
                if let Some(callback) = &self.on_open {
                    callback(open, window, cx);
                }
            })
            .w_full()
            .h_full()
            .max_w(relative(1.))
            .max_h(relative(1.))
            .border_0()
            .rounded(px(0.))
            .p(px(24.))
            .child(content)
            .refine_style(&self.style)
    }
}
