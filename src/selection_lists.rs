use crate::{Checkbox, SelectOption, Theme};
use gpui::{
    App, ElementId, FocusHandle, InteractiveElement as _, IntoElement, ParentElement, RenderOnce,
    SharedString, StyleRefinement, Styled, Window, div, prelude::FluentBuilder as _, px,
};
use gpui_base::StyledExt as _;
use std::{collections::HashMap, rc::Rc};
type Many = Rc<dyn Fn(Vec<SharedString>, &mut Window, &mut App)>;
type One = Rc<dyn Fn(SharedString, &mut Window, &mut App)>;

#[derive(IntoElement)]
pub struct CheckboxList {
    id: ElementId,
    items: Vec<SelectOption>,
    selected: Vec<SharedString>,
    theme: Theme,
    change: Option<Many>,
    style: StyleRefinement,
}
impl CheckboxList {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            items: vec![],
            selected: vec![],
            theme: Theme::default(),
            change: None,
            style: StyleRefinement::default(),
        }
    }
    pub fn items(mut self, items: Vec<SelectOption>) -> Self {
        self.items = items;
        self
    }
    pub fn selected(mut self, selected: Vec<SharedString>) -> Self {
        self.selected = selected;
        self
    }
    pub fn theme(mut self, theme: Theme) -> Self {
        self.theme = theme;
        self
    }
    pub fn on_change(
        mut self,
        f: impl Fn(Vec<SharedString>, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.change = Some(Rc::new(f));
        self
    }
}
impl Styled for CheckboxList {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}
impl RenderOnce for CheckboxList {
    fn render(self, _: &mut Window, _: &mut App) -> impl IntoElement {
        let mut group = div().id(self.id.clone()).flex().flex_col().gap(px(10.));
        for item in self.items {
            let change = self.change.clone();
            let selected = self.selected.clone();
            let value = item.value.clone();
            group = group.child(
                Checkbox::new((self.id.clone(), item.value.clone()), item.label)
                    .theme(self.theme)
                    .checked(self.selected.contains(&item.value))
                    .disabled(item.disabled)
                    .on_change(move |checked, window, cx| {
                        let mut next = selected.clone();
                        next.retain(|v| v != &value);
                        if checked {
                            next.push(value.clone());
                        }
                        if let Some(f) = &change {
                            f(next, window, cx);
                        }
                    }),
            );
        }
        group.refine_style(&self.style)
    }
}

#[derive(Default)]
struct RadioFocus(HashMap<SharedString, FocusHandle>);
#[derive(IntoElement)]
pub struct RadioList {
    id: ElementId,
    items: Vec<SelectOption>,
    selected: Option<SharedString>,
    theme: Theme,
    change: Option<One>,
    style: StyleRefinement,
}
impl RadioList {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            items: vec![],
            selected: None,
            theme: Theme::default(),
            change: None,
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
    pub fn theme(mut self, theme: Theme) -> Self {
        self.theme = theme;
        self
    }
    pub fn on_change(mut self, f: impl Fn(SharedString, &mut Window, &mut App) + 'static) -> Self {
        self.change = Some(Rc::new(f));
        self
    }
}
impl Styled for RadioList {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}
impl RenderOnce for RadioList {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let c = self.theme.colors;
        let state = window.use_keyed_state((self.id.clone(), "radio-focus"), cx, |_, _| {
            RadioFocus::default()
        });
        let handles = state.update(cx, |state, cx| {
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
        });
        let active = self
            .items
            .iter()
            .position(|item| !item.disabled && Some(&item.value) == self.selected.as_ref())
            .or_else(|| self.items.iter().position(|item| !item.disabled));
        let items = Rc::new(self.items);
        let handles = Rc::new(handles);
        let mut group = gpui_base::RadioGroup::new(self.id.clone())
            .flex()
            .flex_col()
            .gap(px(10.));
        for (index, item) in items.iter().enumerate() {
            let change = self.change.clone();
            let keyboard = self.change.clone();
            let value = item.value.clone();
            let nav_items = items.clone();
            let nav_handles = handles.clone();
            let checked = Some(&item.value) == self.selected.as_ref();
            group = group.child(
                gpui_base::Radio::new((self.id.clone(), item.value.clone()))
                    .checked(checked)
                    .disabled(item.disabled)
                    .accessibility_label(item.label.clone())
                    .track_focus(&handles[index])
                    .tab_stop(active == Some(index))
                    .flex()
                    .items_center()
                    .gap(px(8.))
                    .text_size(px(14.))
                    .line_height(px(20.))
                    .text_color(c.text)
                    .when(item.disabled, |el| el.opacity(0.5))
                    .on_change(move |_, _, window, cx| {
                        if let Some(f) = &change {
                            f(value.clone(), window, cx);
                        }
                    })
                    .on_key_down(move |event, window, cx| {
                        if event.keystroke.modifiers != gpui::Modifiers::default() {
                            return;
                        }
                        let enabled: Vec<usize> = nav_items
                            .iter()
                            .enumerate()
                            .filter_map(|(i, v)| (!v.disabled).then_some(i))
                            .collect();
                        let Some(at) = enabled.iter().position(|&i| i == index) else {
                            return;
                        };
                        let next = match event.keystroke.key.as_str() {
                            "up" | "left" => {
                                Some(enabled[(at + enabled.len() - 1) % enabled.len()])
                            }
                            "down" | "right" => Some(enabled[(at + 1) % enabled.len()]),
                            "home" => enabled.first().copied(),
                            "end" => enabled.last().copied(),
                            _ => None,
                        };
                        if let Some(next) = next {
                            cx.stop_propagation();
                            nav_handles[next].focus(window, cx);
                            if let Some(f) = &keyboard {
                                f(nav_items[next].value.clone(), window, cx);
                            }
                        }
                    })
                    .child(
                        div()
                            .size(px(16.))
                            .rounded_full()
                            .border_1()
                            .border_color(if checked {
                                c.accent
                            } else {
                                c.border_emphasized
                            })
                            .flex()
                            .items_center()
                            .justify_center()
                            .when(checked, |el| {
                                el.child(div().size(px(8.)).rounded_full().bg(c.accent))
                            }),
                    )
                    .child(item.label.clone()),
            );
        }
        group.refine_style(&self.style)
    }
}
