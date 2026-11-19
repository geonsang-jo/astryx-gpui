use std::{collections::HashMap, rc::Rc};

use gpui::{
    App, ElementId, FocusHandle, FontWeight, InteractiveElement, IntoElement, ParentElement,
    RenderOnce, Role, SharedString, StatefulInteractiveElement, StyleRefinement, Styled, Window,
    div, prelude::FluentBuilder as _, px,
};
use gpui_base::StyledExt as _;

use crate::Theme;

type SelectHandler = Rc<dyn Fn(SharedString, bool, &mut Window, &mut App)>;

/// One uniquely keyed tab in a controlled tab list.
#[derive(Clone, Debug)]
pub struct Tab {
    pub key: SharedString,
    pub label: SharedString,
    pub disabled: bool,
    pub icon_svg: Option<SharedString>,
}

impl Tab {
    pub fn new(key: impl Into<SharedString>, label: impl Into<SharedString>) -> Self {
        Self {
            key: key.into(),
            label: label.into(),
            disabled: false,
            icon_svg: None,
        }
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Uses caller-supplied inline SVG; the library bundles no icon collection.
    pub fn icon_svg(mut self, data: impl Into<SharedString>) -> Self {
        self.icon_svg = Some(data.into());
        self
    }
}

#[derive(Default)]
struct TabFocus {
    handles: HashMap<SharedString, FocusHandle>,
}

/// Horizontal automatic-activation tabs with roving native focus.
///
/// Left/Right wrap through enabled tabs. Home/End move to the first/last enabled
/// tab. The owning view must apply the value reported by `on_select`.
#[derive(IntoElement)]
pub struct Tabs {
    id: ElementId,
    items: Vec<Tab>,
    selected: Option<SharedString>,
    on_select: Option<SelectHandler>,
    theme: Theme,
    style: StyleRefinement,
}

impl Tabs {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            items: Vec::new(),
            selected: None,
            on_select: None,
            theme: Theme::light(),
            style: StyleRefinement::default(),
        }
    }

    /// Replaces the tab list. Keys must be unique within this list.
    pub fn items(mut self, items: Vec<Tab>) -> Self {
        self.items = items;
        self
    }

    pub fn selected(mut self, selected: impl Into<SharedString>) -> Self {
        self.selected = Some(selected.into());
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
        self.on_select = Some(Rc::new(move |value, _, window, cx| {
            handler(value, window, cx)
        }));
        self
    }

    /// Reports whether the selected tab owns native focus at activation time.
    /// Hosts can use this to preserve a keyboard-focused tab strip or return
    /// focus to their working content after a pointer-driven selection.
    pub fn on_select_with_focus(
        mut self,
        handler: impl Fn(SharedString, bool, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_select = Some(Rc::new(handler));
        self
    }
}

impl Styled for Tabs {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

fn destination(items: &[Tab], current: usize, key: &str) -> Option<usize> {
    let enabled: Vec<usize> = items
        .iter()
        .enumerate()
        .filter_map(|(index, tab)| (!tab.disabled).then_some(index))
        .collect();
    if enabled.is_empty() {
        return None;
    }
    let position = enabled
        .iter()
        .position(|index| *index == current)
        .unwrap_or(0);
    match key {
        "left" => Some(enabled[(position + enabled.len() - 1) % enabled.len()]),
        "right" => Some(enabled[(position + 1) % enabled.len()]),
        "home" => enabled.first().copied(),
        "end" => enabled.last().copied(),
        _ => None,
    }
}

impl RenderOnce for Tabs {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let c = self.theme.colors;
        let active = self
            .items
            .iter()
            .position(|tab| !tab.disabled && Some(&tab.key) == self.selected.as_ref())
            .or_else(|| self.items.iter().position(|tab| !tab.disabled));
        let focus = window.use_keyed_state((self.id.clone(), "tab-focus"), cx, |_, _| {
            TabFocus::default()
        });
        let handles = focus.update(cx, |state, cx| {
            state
                .handles
                .retain(|key, _| self.items.iter().any(|tab| &tab.key == key));
            self.items
                .iter()
                .map(|tab| {
                    state
                        .handles
                        .entry(tab.key.clone())
                        .or_insert_with(|| cx.focus_handle())
                        .clone()
                })
                .collect::<Vec<_>>()
        });
        let items = Rc::new(self.items);
        let handles = Rc::new(handles);
        let buttons = items
            .iter()
            .enumerate()
            .map(|(index, item)| {
                let callback = self.on_select.clone();
                let keyboard_callback = self.on_select.clone();
                let key = item.key.clone();
                let selected = Some(index) == active;
                let keyboard_items = items.clone();
                let keyboard_handles = handles.clone();
                let click_handle = handles[index].clone();
                gpui_base::Button::new((self.id.clone(), item.key.clone()))
                    .role(Role::Tab)
                    .accessibility_label(item.label.clone())
                    .aria_selected(selected)
                    .track_focus(&handles[index])
                    .tab_stop(selected)
                    .disabled(item.disabled)
                    .flex()
                    .items_center()
                    .justify_center()
                    .gap(px(8.))
                    .h(px(36.))
                    .px(px(12.))
                    .text_size(px(14.))
                    .line_height(px(20.))
                    .font_weight(FontWeight::MEDIUM)
                    .text_color(if selected { c.text } else { c.muted })
                    .border_b(px(2.))
                    .border_color(if selected {
                        c.accent
                    } else {
                        gpui::transparent_black()
                    })
                    .hover(move |style| style.bg(c.hover))
                    .active(move |style| style.bg(c.pressed))
                    .focus_visible(move |style| style.bg(c.neutral))
                    .styles(|styles| styles.disabled(|style| style.opacity(0.5)))
                    .when_some(callback, |button, callback| {
                        button.on_click(move |_, window, cx| {
                            let focused = click_handle.is_focused(window);
                            callback(key.clone(), focused, window, cx)
                        })
                    })
                    .on_key_down(move |event, window, cx| {
                        if event.keystroke.modifiers != gpui::Modifiers::default() {
                            return;
                        }
                        if let Some(next) =
                            destination(&keyboard_items, index, &event.keystroke.key)
                        {
                            cx.stop_propagation();
                            keyboard_handles[next].focus(window, cx);
                            if let Some(callback) = &keyboard_callback {
                                let focused = keyboard_handles[next].is_focused(window);
                                callback(keyboard_items[next].key.clone(), focused, window, cx);
                            }
                        }
                    })
                    .when_some(item.icon_svg.clone(), |button, icon| {
                        button.child(
                            gpui::svg()
                                .data(icon.as_bytes())
                                .size(px(14.))
                                .flex_none()
                                .text_color(if selected { c.text } else { c.muted }),
                        )
                    })
                    .child(item.label.clone())
            })
            .collect::<Vec<_>>();
        div()
            .id(self.id)
            .role(Role::TabList)
            .flex()
            .min_w_0()
            .gap(px(4.))
            .border_b_1()
            .border_color(c.border)
            .children(buttons)
            .refine_style(&self.style)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn navigation_wraps_skips_disabled_and_handles_empty_lists() {
        let tabs = [
            Tab::new("a", "A"),
            Tab::new("b", "B").disabled(true),
            Tab::new("c", "C"),
        ];
        assert_eq!(destination(&tabs, 0, "right"), Some(2));
        assert_eq!(destination(&tabs, 2, "right"), Some(0));
        assert_eq!(destination(&tabs, 0, "left"), Some(2));
        assert_eq!(destination(&tabs, 2, "home"), Some(0));
        assert_eq!(destination(&tabs, 0, "end"), Some(2));
        assert_eq!(destination(&[], 0, "right"), None);
        assert_eq!(
            destination(&[Tab::new("a", "A").disabled(true)], 0, "end"),
            None
        );
    }
}
