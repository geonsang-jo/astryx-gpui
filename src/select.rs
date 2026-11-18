use std::rc::Rc;

use gpui::{
    App, ElementId, FocusHandle, InteractiveElement as _, IntoElement, ParentElement, RenderOnce,
    Role, SharedString, StatefulInteractiveElement as _, StyleRefinement, Styled, Window, div,
    prelude::FluentBuilder as _, px,
};
use gpui_base::StyledExt as _;

use crate::Theme;

type ValueHandler = Rc<dyn Fn(SharedString, &mut Window, &mut App)>;
type OpenHandler = Rc<dyn Fn(bool, &mut Window, &mut App)>;
type HighlightHandler = Rc<dyn Fn(usize, &mut Window, &mut App)>;

#[derive(Clone, Debug)]
pub struct SelectOption {
    pub value: SharedString,
    pub label: SharedString,
    pub disabled: bool,
}

impl SelectOption {
    pub fn new(value: impl Into<SharedString>, label: impl Into<SharedString>) -> Self {
        Self {
            value: value.into(),
            label: label.into(),
            disabled: false,
        }
    }
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

struct SelectFocus {
    trigger: FocusHandle,
    content: FocusHandle,
}

/// A controlled single-choice selector with a native anchored popup.
///
/// The host applies `on_open_change`, `on_highlight` and `on_select` requests to
/// its own state. Focus identity is keyed by `id`; only focus handles are stored
/// internally. Up/Down wrap enabled options, Home/End jump, Enter confirms and
/// Escape/outside click dismiss without committing the highlighted value.
#[derive(IntoElement)]
pub struct Select {
    id: ElementId,
    items: Vec<SelectOption>,
    selected: Option<SharedString>,
    open: bool,
    highlighted: Option<usize>,
    disabled: bool,
    placeholder: SharedString,
    label: Option<SharedString>,
    theme: Theme,
    style: StyleRefinement,
    on_select: Option<ValueHandler>,
    on_open_change: Option<OpenHandler>,
    on_highlight: Option<HighlightHandler>,
}

impl Select {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            items: Vec::new(),
            selected: None,
            open: false,
            highlighted: None,
            disabled: false,
            placeholder: "Select…".into(),
            label: None,
            theme: Theme::light(),
            style: StyleRefinement::default(),
            on_select: None,
            on_open_change: None,
            on_highlight: None,
        }
    }
    pub fn items(mut self, items: Vec<SelectOption>) -> Self {
        self.items = items;
        self
    }
    pub fn selected(mut self, value: impl Into<SharedString>) -> Self {
        self.selected = Some(value.into());
        self
    }
    pub fn open(mut self, open: bool) -> Self {
        self.open = open;
        self
    }
    pub fn highlighted(mut self, index: usize) -> Self {
        self.highlighted = Some(index);
        self
    }
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
    pub fn placeholder(mut self, text: impl Into<SharedString>) -> Self {
        self.placeholder = text.into();
        self
    }
    pub fn label(mut self, text: impl Into<SharedString>) -> Self {
        self.label = Some(text.into());
        self
    }
    pub fn theme(mut self, theme: Theme) -> Self {
        self.theme = theme;
        self
    }
    pub fn on_select(
        mut self,
        handler: impl Fn(SharedString, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_select = Some(Rc::new(handler));
        self
    }
    pub fn on_open_change(
        mut self,
        handler: impl Fn(bool, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_open_change = Some(Rc::new(handler));
        self
    }
    pub fn on_highlight(
        mut self,
        handler: impl Fn(usize, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_highlight = Some(Rc::new(handler));
        self
    }
}

impl Styled for Select {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

fn destination(items: &[SelectOption], current: Option<usize>, key: &str) -> Option<usize> {
    let enabled: Vec<usize> = items
        .iter()
        .enumerate()
        .filter_map(|(i, item)| (!item.disabled).then_some(i))
        .collect();
    let position = enabled.iter().position(|&i| Some(i) == current);
    match key {
        "up" => enabled
            .get(position.map_or(0, |i| (i + enabled.len() - 1) % enabled.len()))
            .copied(),
        "down" => enabled
            .get(position.map_or(0, |i| (i + 1) % enabled.len()))
            .copied(),
        "home" => enabled.first().copied(),
        "end" => enabled.last().copied(),
        _ => None,
    }
}

impl RenderOnce for Select {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let c = self.theme.colors;
        let focus =
            window.use_keyed_state((self.id.clone(), "select-focus"), cx, |_, cx| SelectFocus {
                trigger: cx.focus_handle(),
                content: cx.focus_handle(),
            });
        let trigger_focus = focus.read(cx).trigger.clone();
        let content_focus = focus.read(cx).content.clone();
        let selected = self
            .items
            .iter()
            .find(|item| Some(&item.value) == self.selected.as_ref());
        let has_selection = selected.is_some();
        let title = selected
            .map(|item| item.label.clone())
            .unwrap_or(self.placeholder);
        let active = self
            .highlighted
            .filter(|&index| self.items.get(index).is_some_and(|item| !item.disabled))
            .or_else(|| {
                self.items
                    .iter()
                    .position(|item| !item.disabled && Some(&item.value) == self.selected.as_ref())
            })
            .or_else(|| self.items.iter().position(|item| !item.disabled));
        let items = Rc::new(self.items);
        let disabled = self.disabled;
        let open = self.open && !disabled;
        let opened = self.on_open_change.clone();
        let click_content_focus = content_focus.clone();
        let click_trigger_focus = trigger_focus.clone();
        let trigger = div()
            .id((self.id.clone(), "trigger"))
            .flex()
            .items_center()
            .justify_between()
            .w_full()
            .h(px(36.))
            .px(px(12.))
            .gap(px(8.))
            .rounded(px(10.))
            .border_1()
            .border_color(if open || trigger_focus.is_focused(window) {
                c.focus
            } else {
                c.border_emphasized
            })
            .bg(c.surface)
            .text_color(if has_selection { c.text } else { c.faint })
            .text_size(px(14.))
            .line_height(px(20.))
            .when(disabled, |el| el.opacity(0.5))
            .when(!disabled, |el| {
                el.cursor_pointer()
                    .hover(move |style| style.bg(c.hover))
                    .on_click(move |_, window, cx| {
                        cx.stop_propagation();
                        if let Some(handler) = &opened {
                            handler(!open, window, cx);
                        }
                        if open {
                            click_trigger_focus.focus(window, cx);
                        } else {
                            click_content_focus.focus(window, cx);
                        }
                    })
            })
            .child(
                div()
                    .flex_1()
                    .min_w(px(0.))
                    .text_ellipsis()
                    .child(title.clone()),
            )
            .child(div().text_color(c.muted).child("⌄"));
        let mut popup = gpui_base::Popup::new((self.id.clone(), "popup"), trigger)
            .w_full()
            .offset(px(4.));
        if open {
            let highlight = self.on_highlight.clone();
            let key_items = items.clone();
            let close = self.on_open_change.clone();
            let outside_focus = trigger_focus.clone();
            let outside_content_focus = content_focus.clone();
            let mut list = div()
                .id((self.id.clone(), "list"))
                .role(Role::ListBox)
                .track_focus(&content_focus)
                .key_context("Select")
                .w(px(260.))
                .max_h(px(260.))
                .overflow_y_scroll()
                .p(px(4.))
                .rounded(px(12.))
                .border_1()
                .border_color(c.border)
                .bg(c.surface)
                .text_color(c.text)
                .on_key_down(move |event, window, cx| {
                    if event.keystroke.modifiers != gpui::Modifiers::default() {
                        return;
                    }
                    if let Some(next) = destination(&key_items, active, &event.keystroke.key) {
                        cx.stop_propagation();
                        if let Some(handler) = &highlight {
                            handler(next, window, cx);
                        }
                    }
                })
                .on_mouse_down_out(move |_, window, cx| {
                    if let Some(handler) = &close {
                        handler(false, window, cx);
                    }
                    if outside_content_focus.is_focused(window) {
                        outside_focus.focus(window, cx);
                    }
                });
            for (index, item) in items.iter().enumerate() {
                let callback = self.on_select.clone();
                let close = self.on_open_change.clone();
                let trigger = trigger_focus.clone();
                let value = item.value.clone();
                list = list.child(
                    gpui_base::Button::new((self.id.clone(), item.value.clone()))
                        .role(Role::ListBoxOption)
                        .disabled(item.disabled)
                        .focusable(false)
                        .tab_stop(false)
                        .justify_start()
                        .aria_selected(Some(&item.value) == self.selected.as_ref())
                        .when(active == Some(index), |el| el.aria_active_descendant())
                        .h(px(32.))
                        .px(px(10.))
                        .rounded(px(8.))
                        .flex()
                        .items_center()
                        .text_size(px(14.))
                        .line_height(px(20.))
                        .bg(if active == Some(index) {
                            c.neutral
                        } else {
                            gpui::transparent_black()
                        })
                        .when(item.disabled, |el| {
                            el.opacity(0.5)
                                .accessibility_label(format!("{} (unavailable)", item.label))
                        })
                        .when(!item.disabled, |el| {
                            el.cursor_pointer()
                                .hover(move |style| style.bg(c.hover))
                                .on_click(move |_, window, cx| {
                                    cx.stop_propagation();
                                    if let Some(handler) = &close {
                                        handler(false, window, cx);
                                    }
                                    trigger.focus(window, cx);
                                    if let Some(handler) = &callback {
                                        handler(value.clone(), window, cx);
                                    }
                                })
                        })
                        .child(item.label.clone()),
                );
            }
            popup = popup.content(list);
        }
        let confirm_items = items;
        let confirm = self.on_select;
        let close = self.on_open_change.clone();
        let confirm_focus = trigger_focus.clone();
        let confirm_content_focus = content_focus.clone();
        gpui_base::Select::new(self.id)
            .open(open)
            .disabled(disabled)
            .focus_handle(&trigger_focus)
            .content_focus_handle(&content_focus)
            .accessibility_value(title)
            .when_some(self.label, |el, label| el.accessibility_label(label))
            .on_open_change(move |open, window, cx| {
                if let Some(handler) = &self.on_open_change {
                    handler(open, window, cx);
                }
            })
            .on_confirm(move |window, cx| {
                if let Some(handler) = &close {
                    handler(false, window, cx);
                }
                confirm_focus.focus(window, cx);
                if let Some(item) = active.and_then(|i| confirm_items.get(i))
                    && let Some(handler) = &confirm
                {
                    handler(item.value.clone(), window, cx);
                }
                // gpui-base 0.7.1 refocuses popup content after its confirmation
                // callback. Restore the consumer's requested focus only when
                // that exact post-callback override is still current.
                if let Some(desired) = window.focused(cx) {
                    let content = confirm_content_focus.clone();
                    window.defer(cx, move |window, cx| {
                        if window.focused(cx).as_ref() == Some(&content) {
                            desired.focus(window, cx);
                        }
                    });
                }
            })
            .w_full()
            .min_w(px(0.))
            .child(popup)
            .refine_style(&self.style)
    }
}
