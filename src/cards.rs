use crate::{SelectOption, Theme};
use gpui::{
    AnyElement, App, ClickEvent, ElementId, FocusHandle, InteractiveElement, IntoElement,
    ParentElement, RenderOnce, SharedString, StyleRefinement, Styled, Window,
    prelude::FluentBuilder as _, px,
};
use gpui_base::StyledExt as _;
use std::{collections::HashMap, rc::Rc};

/// A whole-card native button. Child content is composed by the owner.
#[derive(IntoElement)]
pub struct ClickableCard {
    base: gpui_base::Button,
    disabled: bool,
    children: Vec<AnyElement>,
    theme: Theme,
    style: StyleRefinement,
}
impl ClickableCard {
    pub fn new(id: impl Into<ElementId>, label: impl Into<SharedString>) -> Self {
        Self {
            base: gpui_base::Button::new(id).accessibility_label(label),
            disabled: false,
            children: vec![],
            theme: Theme::default(),
            style: StyleRefinement::default(),
        }
    }
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self.base = self.base.disabled(disabled);
        self
    }
    pub fn theme(mut self, theme: Theme) -> Self {
        self.theme = theme;
        self
    }
    pub fn on_click(
        mut self,
        handler: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.base = self.base.on_click(handler);
        self
    }
}
impl Styled for ClickableCard {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}
impl ParentElement for ClickableCard {
    fn extend(&mut self, children: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(children)
    }
}
impl RenderOnce for ClickableCard {
    fn render(self, _: &mut Window, _: &mut App) -> impl IntoElement {
        let c = self.theme.colors;
        self.base
            .flex()
            .flex_col()
            .items_start()
            .justify_start()
            .gap(px(8.))
            .p(px(12.))
            .rounded(px(12.))
            .border_1()
            .border_color(c.border)
            .bg(c.card)
            .text_color(c.text)
            .text_size(px(14.))
            .line_height(px(20.))
            .when(!self.disabled, |el| {
                el.cursor_pointer().hover(move |style| style.bg(c.hover))
            })
            .when(self.disabled, |el| el.cursor_default())
            .focus_visible(move |style| style.border_color(c.focus))
            .styles(|styles| styles.disabled(|style| style.opacity(0.5)))
            .children(self.children)
            .refine_style(&self.style)
    }
}

/// An owner-controlled checkbox card with native checked/disabled semantics.
#[derive(IntoElement)]
pub struct SelectableCard {
    base: gpui_base::Checkbox,
    disabled: bool,
    selected: bool,
    children: Vec<AnyElement>,
    theme: Theme,
    style: StyleRefinement,
}
impl SelectableCard {
    pub fn new(id: impl Into<ElementId>, label: impl Into<SharedString>) -> Self {
        Self {
            base: gpui_base::Checkbox::new(id).accessibility_label(label),
            disabled: false,
            selected: false,
            children: vec![],
            theme: Theme::default(),
            style: StyleRefinement::default(),
        }
    }
    pub fn selected(mut self, selected: bool) -> Self {
        self.selected = selected;
        self.base = self.base.checked(selected);
        self
    }
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self.base = self.base.disabled(disabled);
        self
    }
    pub fn theme(mut self, theme: Theme) -> Self {
        self.theme = theme;
        self
    }
    pub fn on_change(mut self, handler: impl Fn(bool, &mut Window, &mut App) + 'static) -> Self {
        self.base = self.base.on_change(move |value, _, window, cx| {
            handler(value == gpui_base::CheckboxState::Checked, window, cx)
        });
        self
    }
}
impl Styled for SelectableCard {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}
impl ParentElement for SelectableCard {
    fn extend(&mut self, children: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(children)
    }
}
impl RenderOnce for SelectableCard {
    fn render(self, _: &mut Window, _: &mut App) -> impl IntoElement {
        let c = self.theme.colors;
        self.base
            .flex()
            .flex_col()
            .items_start()
            .justify_start()
            .gap(px(8.))
            .p(px(12.))
            .rounded(px(12.))
            .border_1()
            .border_color(if self.selected { c.accent } else { c.border })
            .bg(if self.selected { c.neutral } else { c.card })
            .when(!self.disabled, |el| el.cursor_pointer())
            .when(self.disabled, |el| el.cursor_default())
            .text_color(c.text)
            .text_size(px(14.))
            .line_height(px(20.))
            .focus_visible(move |style| style.border_color(c.focus))
            .styles(|styles| styles.disabled(|style| style.opacity(0.5)))
            .children(self.children)
            .refine_style(&self.style)
    }
}

/// A controlled pressed action, with native ToggleButton accessibility metadata.
#[derive(IntoElement)]
pub struct ToggleButton {
    base: gpui_base::Toggle,
    disabled: bool,
    label: SharedString,
    pressed: bool,
    theme: Theme,
    style: StyleRefinement,
}
impl ToggleButton {
    pub fn new(id: impl Into<ElementId>, label: impl Into<SharedString>) -> Self {
        let label = label.into();
        Self {
            base: gpui_base::Toggle::new(id).accessibility_label(label.clone()),
            disabled: false,
            label,
            pressed: false,
            theme: Theme::default(),
            style: StyleRefinement::default(),
        }
    }
    pub fn pressed(mut self, pressed: bool) -> Self {
        self.pressed = pressed;
        self.base = self.base.pressed(pressed);
        self
    }
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self.base = self.base.disabled(disabled);
        self
    }
    pub fn theme(mut self, theme: Theme) -> Self {
        self.theme = theme;
        self
    }
    pub fn on_change(mut self, handler: impl Fn(bool, &mut Window, &mut App) + 'static) -> Self {
        self.base = self
            .base
            .on_change(move |pressed, _, window, cx| handler(pressed, window, cx));
        self
    }
}
impl Styled for ToggleButton {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}
impl RenderOnce for ToggleButton {
    fn render(self, _: &mut Window, _: &mut App) -> impl IntoElement {
        let c = self.theme.colors;
        self.base
            .flex()
            .items_center()
            .justify_center()
            .h(px(32.))
            .px(px(12.))
            .rounded(px(10.))
            .border_1()
            .border_color(if self.pressed {
                c.accent
            } else {
                c.border_emphasized
            })
            .bg(if self.pressed {
                c.neutral
            } else {
                gpui::transparent_black()
            })
            .text_color(c.text)
            .text_size(px(14.))
            .line_height(px(20.))
            .when(!self.disabled, |el| {
                el.cursor_pointer().hover(move |style| style.bg(c.hover))
            })
            .when(self.disabled, |el| el.cursor_default())
            .focus_visible(move |style| style.border_color(c.focus))
            .styles(|styles| styles.disabled(|style| style.opacity(0.5)))
            .child(self.label)
            .refine_style(&self.style)
    }
}

type Select = Rc<dyn Fn(SharedString, &mut Window, &mut App)>;
#[derive(Default)]
struct SegmentFocus(HashMap<SharedString, FocusHandle>);
/// Compact radio choices with a native horizontal radio group and roving focus.
#[derive(IntoElement)]
pub struct SegmentedControl {
    id: ElementId,
    items: Vec<SelectOption>,
    selected: Option<SharedString>,
    disabled: bool,
    on_change: Option<Select>,
    theme: Theme,
    style: StyleRefinement,
}
impl SegmentedControl {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            items: vec![],
            selected: None,
            disabled: false,
            on_change: None,
            theme: Theme::default(),
            style: StyleRefinement::default(),
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
    pub fn disabled(mut self, value: bool) -> Self {
        self.disabled = value;
        self
    }
    pub fn on_change(
        mut self,
        handler: impl Fn(SharedString, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_change = Some(Rc::new(handler));
        self
    }
    pub fn theme(mut self, theme: Theme) -> Self {
        self.theme = theme;
        self
    }
}
impl Styled for SegmentedControl {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}
impl RenderOnce for SegmentedControl {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let c = self.theme.colors;
        let state = window.use_keyed_state((self.id.clone(), "segment-focus"), cx, |_, _| {
            SegmentFocus::default()
        });
        let handles = Rc::new(state.update(cx, |state, cx| {
            state
                .0
                .retain(|key, _| self.items.iter().any(|item| &item.value == key));
            self.items
                .iter()
                .map(|item| {
                    state
                        .0
                        .entry(item.value.clone())
                        .or_insert_with(|| cx.focus_handle())
                        .clone()
                })
                .collect::<Vec<_>>()
        }));
        let items = Rc::new(self.items);
        let enabled: Rc<Vec<usize>> = Rc::new(
            items
                .iter()
                .enumerate()
                .filter_map(|(index, item)| (!self.disabled && !item.disabled).then_some(index))
                .collect(),
        );
        let active = items
            .iter()
            .position(|item| !item.disabled && Some(&item.value) == self.selected.as_ref())
            .or_else(|| enabled.first().copied());
        gpui_base::RadioGroup::new(self.id.clone())
            .axis(gpui::Axis::Horizontal)
            .flex()
            .items_center()
            .p(px(4.))
            .gap(px(4.))
            .rounded(px(12.))
            .bg(c.neutral)
            .children(items.iter().enumerate().map(|(index, item)| {
                let callback = self.on_change.clone();
                let keys = self.on_change.clone();
                let value = item.value.clone();
                let nav = items.clone();
                let handles = handles.clone();
                let enabled = enabled.clone();
                let disabled = self.disabled || item.disabled;
                let selected = Some(&item.value) == self.selected.as_ref();
                gpui_base::Radio::new((self.id.clone(), item.value.clone()))
                    .checked(selected)
                    .disabled(disabled)
                    .accessibility_label(item.label.clone())
                    .track_focus(&handles[index])
                    .tab_stop(!disabled && active == Some(index))
                    .flex()
                    .items_center()
                    .justify_center()
                    .h(px(28.))
                    .px(px(12.))
                    .rounded(px(8.))
                    .bg(if selected {
                        c.surface
                    } else {
                        gpui::transparent_black()
                    })
                    .text_color(if selected { c.text } else { c.muted })
                    .text_size(px(14.))
                    .line_height(px(20.))
                    .focus_visible(move |style| style.border_1().border_color(c.focus))
                    .styles(|styles| styles.disabled(|style| style.opacity(0.5)))
                    .on_change(move |_, _, window, cx| {
                        if let Some(handler) = &callback {
                            handler(value.clone(), window, cx);
                        }
                    })
                    .on_key_down(move |event, window, cx| {
                        if disabled || event.keystroke.modifiers != gpui::Modifiers::default() {
                            return;
                        }
                        let Some(position) = enabled.iter().position(|&i| i == index) else {
                            return;
                        };
                        let next = match event.keystroke.key.as_str() {
                            "left" | "up" => {
                                Some(enabled[(position + enabled.len() - 1) % enabled.len()])
                            }
                            "right" | "down" => Some(enabled[(position + 1) % enabled.len()]),
                            "home" => enabled.first().copied(),
                            "end" => enabled.last().copied(),
                            _ => None,
                        };
                        if let Some(next) = next {
                            cx.stop_propagation();
                            handles[next].focus(window, cx);
                            if next != index
                                && let Some(handler) = &keys
                            {
                                handler(nav[next].value.clone(), window, cx);
                            }
                        }
                    })
                    .child(item.label.clone())
            }))
            .refine_style(&self.style)
    }
}
