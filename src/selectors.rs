use crate::{Button, ButtonVariant, CheckboxList, Input, InputState, Popover, SelectOption, Theme};
use gpui::{
    App, ElementId, Entity, EntityId, EntityInputHandler as _, Focusable as _,
    InteractiveElement as _, IntoElement, ParentElement, RenderOnce, Role, SharedString,
    StatefulInteractiveElement as _, StyleRefinement, Styled, Subscription, Window, div,
    prelude::FluentBuilder as _, px,
};
use gpui_base::StyledExt as _;
use std::rc::Rc;
type Open = Rc<dyn Fn(bool, &mut Window, &mut App)>;
type One = Rc<dyn Fn(SharedString, &mut Window, &mut App)>;
type Many = Rc<dyn Fn(Vec<SharedString>, &mut Window, &mut App)>;
type Highlight = Rc<dyn Fn(usize, &mut Window, &mut App)>;
struct QueryWatch {
    entity: EntityId,
    _subscription: Subscription,
}
fn watch_query(id: ElementId, input: &Entity<InputState>, window: &mut Window, cx: &mut App) {
    let watch = window.use_keyed_state((id, "query-watch"), cx, |window, cx| QueryWatch {
        entity: input.entity_id(),
        _subscription: window.observe(input, cx, |_, window, _| window.refresh()),
    });
    watch.update(cx, |watch, cx| {
        if watch.entity != input.entity_id() {
            watch.entity = input.entity_id();
            watch._subscription = window.observe(input, cx, |_, window, _| window.refresh());
        }
    });
}
fn composing(input: &Entity<InputState>, window: &mut Window, cx: &mut App) -> bool {
    input.update(cx, |state, cx| {
        state.marked_text_range(window, cx).is_some()
    })
}

/// An owner-controlled popup checklist. The selected values remain visible in
/// its trigger; checking one option leaves the picker open for further choices.
#[derive(IntoElement)]
pub struct MultiSelector {
    id: ElementId,
    items: Vec<SelectOption>,
    selected: Vec<SharedString>,
    open: bool,
    disabled: bool,
    theme: Theme,
    change: Option<Many>,
    opened: Option<Open>,
    style: StyleRefinement,
}
impl MultiSelector {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            items: vec![],
            selected: vec![],
            open: false,
            disabled: false,
            theme: Theme::default(),
            change: None,
            opened: None,
            style: StyleRefinement::default(),
        }
    }
    pub fn items(mut self, v: Vec<SelectOption>) -> Self {
        self.items = v;
        self
    }
    pub fn selected(mut self, v: Vec<SharedString>) -> Self {
        self.selected = v;
        self
    }
    pub fn open(mut self, v: bool) -> Self {
        self.open = v;
        self
    }
    pub fn disabled(mut self, v: bool) -> Self {
        self.disabled = v;
        self
    }
    pub fn theme(mut self, v: Theme) -> Self {
        self.theme = v;
        self
    }
    pub fn on_change(
        mut self,
        f: impl Fn(Vec<SharedString>, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.change = Some(Rc::new(f));
        self
    }
    pub fn on_open_change(mut self, f: impl Fn(bool, &mut Window, &mut App) + 'static) -> Self {
        self.opened = Some(Rc::new(f));
        self
    }
}
impl Styled for MultiSelector {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}
impl RenderOnce for MultiSelector {
    fn render(self, _: &mut Window, _: &mut App) -> impl IntoElement {
        let names: Vec<&str> = self
            .items
            .iter()
            .filter(|item| self.selected.contains(&item.value))
            .map(|item| item.label.as_ref())
            .collect();
        let label = if names.is_empty() {
            "Select options…".to_string()
        } else {
            names.join(", ")
        };
        let trigger = Button::new((self.id.clone(), "trigger"), label)
            .theme(self.theme)
            .variant(ButtonVariant::Outline)
            .disabled(self.disabled);
        let opened = self.opened;
        let change = self.change;
        let theme = self.theme;
        Popover::new(self.id.clone())
            .theme(theme)
            .open(self.open)
            .disabled(self.disabled)
            .trigger(trigger)
            .on_open_change(move |open, w, cx| {
                if let Some(f) = &opened {
                    f(open, w, cx);
                }
            })
            .content(move |_, _, _| {
                CheckboxList::new((self.id, "options"))
                    .theme(theme)
                    .items(self.items)
                    .selected(self.selected)
                    .on_change(move |values, w, cx| {
                        if let Some(f) = &change {
                            f(values, w, cx);
                        }
                    })
            })
            .refine_style(&self.style)
            .into_any_element()
    }
}

#[derive(Clone, Debug)]
pub struct SelectGroup {
    pub label: SharedString,
    pub items: Vec<SelectOption>,
}
impl SelectGroup {
    pub fn new(label: impl Into<SharedString>, items: Vec<SelectOption>) -> Self {
        Self {
            label: label.into(),
            items,
        }
    }
}
/// A grouped single-choice picker with optional caller-owned query editing.
#[derive(IntoElement)]
pub struct ComplexSelector {
    id: ElementId,
    groups: Vec<SelectGroup>,
    selected: Option<SharedString>,
    query: Option<Entity<InputState>>,
    open: bool,
    disabled: bool,
    theme: Theme,
    selected_change: Option<One>,
    opened: Option<Open>,
    style: StyleRefinement,
}
impl ComplexSelector {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            groups: vec![],
            selected: None,
            query: None,
            open: false,
            disabled: false,
            theme: Theme::default(),
            selected_change: None,
            opened: None,
            style: StyleRefinement::default(),
        }
    }
    pub fn groups(mut self, v: Vec<SelectGroup>) -> Self {
        self.groups = v;
        self
    }
    pub fn selected(mut self, v: impl Into<SharedString>) -> Self {
        self.selected = Some(v.into());
        self
    }
    pub fn query(mut self, v: &Entity<InputState>) -> Self {
        self.query = Some(v.clone());
        self
    }
    pub fn open(mut self, v: bool) -> Self {
        self.open = v;
        self
    }
    pub fn disabled(mut self, v: bool) -> Self {
        self.disabled = v;
        self
    }
    pub fn theme(mut self, v: Theme) -> Self {
        self.theme = v;
        self
    }
    pub fn on_select(mut self, f: impl Fn(SharedString, &mut Window, &mut App) + 'static) -> Self {
        self.selected_change = Some(Rc::new(f));
        self
    }
    pub fn on_open_change(mut self, f: impl Fn(bool, &mut Window, &mut App) + 'static) -> Self {
        self.opened = Some(Rc::new(f));
        self
    }
}
impl Styled for ComplexSelector {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}
impl RenderOnce for ComplexSelector {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        if let Some(query) = &self.query {
            watch_query(self.id.clone(), query, window, cx);
        }
        let c = self.theme.colors;
        let label = self
            .groups
            .iter()
            .flat_map(|g| &g.items)
            .find(|item| Some(&item.value) == self.selected.as_ref())
            .map(|item| item.label.clone())
            .unwrap_or("Select…".into());
        let trigger = Button::new((self.id.clone(), "trigger"), label)
            .theme(self.theme)
            .variant(ButtonVariant::Outline)
            .disabled(self.disabled);
        let needle = self
            .query
            .as_ref()
            .map(|q| q.read(cx).value().to_lowercase())
            .unwrap_or_default();
        let opened = self.opened;
        let change = self.selected_change;
        let theme = self.theme;
        Popover::new(self.id.clone())
            .theme(theme)
            .open(self.open)
            .disabled(self.disabled)
            .trigger(trigger)
            .on_open_change(move |open, w, cx| {
                if let Some(f) = &opened {
                    f(open, w, cx);
                }
            })
            .content(move |_, _, cx| {
                let state = cx.entity().downgrade();
                let mut list = div()
                    .id((self.id.clone(), "groups"))
                    .flex()
                    .flex_col()
                    .gap(px(8.))
                    .w(px(280.))
                    .max_h(px(360.))
                    .overflow_y_scroll();
                if let Some(query) = self.query {
                    list = list.child(Input::new(&query).theme(theme).label("Filter options"));
                }
                for group in self.groups {
                    let items: Vec<SelectOption> = group
                        .items
                        .into_iter()
                        .filter(|item| {
                            needle.is_empty() || item.label.to_lowercase().contains(&needle)
                        })
                        .collect();
                    if items.is_empty() {
                        continue;
                    }
                    list = list.child(
                        div()
                            .px(px(8.))
                            .text_size(px(12.))
                            .text_color(c.muted)
                            .child(group.label),
                    );
                    for item in items {
                        let state = state.clone();
                        let change = change.clone();
                        let value = item.value.clone();
                        let selected = Some(&item.value) == self.selected.as_ref();
                        list = list.child(
                            gpui_base::Button::new((self.id.clone(), item.value))
                                .disabled(item.disabled)
                                .accessibility_label(item.label.clone())
                                .h(px(32.))
                                .w_full()
                                .justify_start()
                                .px(px(10.))
                                .rounded(px(8.))
                                .text_size(px(14.))
                                .text_color(c.text)
                                .bg(if selected {
                                    c.neutral
                                } else {
                                    gpui::transparent_black()
                                })
                                .when(item.disabled, |el| el.opacity(0.5))
                                .on_click(move |_, w, cx| {
                                    let _ = state.update(cx, |s, cx| s.dismiss(w, cx));
                                    if let Some(f) = &change {
                                        f(value.clone(), w, cx);
                                    }
                                })
                                .child(item.label),
                        );
                    }
                }
                list
            })
            .refine_style(&self.style)
            .into_any_element()
    }
}

/// Search suggestions over a native text-input state. The host observes that
/// state and controls open/highlight. ArrowDown opens suggestions; Enter commits
/// one suggestion and writes its label to the same native input state.
#[derive(IntoElement)]
pub struct Typeahead {
    id: ElementId,
    input: Entity<InputState>,
    items: Vec<SelectOption>,
    open: bool,
    highlighted: usize,
    theme: Theme,
    select: Option<One>,
    opened: Option<Open>,
    highlight: Option<Highlight>,
    style: StyleRefinement,
}
impl Typeahead {
    pub fn new(id: impl Into<ElementId>, input: &Entity<InputState>) -> Self {
        Self {
            id: id.into(),
            input: input.clone(),
            items: vec![],
            open: false,
            highlighted: 0,
            theme: Theme::default(),
            select: None,
            opened: None,
            highlight: None,
            style: StyleRefinement::default(),
        }
    }
    pub fn items(mut self, v: Vec<SelectOption>) -> Self {
        self.items = v;
        self
    }
    pub fn open(mut self, v: bool) -> Self {
        self.open = v;
        self
    }
    pub fn highlighted(mut self, v: usize) -> Self {
        self.highlighted = v;
        self
    }
    pub fn theme(mut self, v: Theme) -> Self {
        self.theme = v;
        self
    }
    pub fn on_select(mut self, f: impl Fn(SharedString, &mut Window, &mut App) + 'static) -> Self {
        self.select = Some(Rc::new(f));
        self
    }
    pub fn on_open_change(mut self, f: impl Fn(bool, &mut Window, &mut App) + 'static) -> Self {
        self.opened = Some(Rc::new(f));
        self
    }
    pub fn on_highlight(mut self, f: impl Fn(usize, &mut Window, &mut App) + 'static) -> Self {
        self.highlight = Some(Rc::new(f));
        self
    }
}
impl Styled for Typeahead {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}
impl RenderOnce for Typeahead {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        watch_query(self.id.clone(), &self.input, window, cx);
        let c = self.theme.colors;
        let state = self.input.read(cx);
        let disabled = !state.is_editable();
        let needle = state.value().to_lowercase();
        let focus = state.focus_handle(cx);
        let items = Rc::new(
            self.items
                .into_iter()
                .filter(|item| needle.is_empty() || item.label.to_lowercase().contains(&needle))
                .collect::<Vec<_>>(),
        );
        let active = items
            .get(self.highlighted)
            .filter(|item| !item.disabled)
            .map(|_| self.highlighted)
            .or_else(|| items.iter().position(|item| !item.disabled));
        let open = self.open && !disabled;
        let input = self.input.clone();
        let chosen = self.select;
        let opened = self.opened.clone();
        let commit: Highlight = Rc::new({
            let items = items.clone();
            let focus = focus.clone();
            move |index, w, cx| {
                if let Some(item) = items.get(index).filter(|item| !item.disabled) {
                    if let Some(opened) = &opened {
                        opened(false, w, cx);
                    }
                    input.update(cx, |s, cx| s.set_value(item.label.clone(), w, cx));
                    focus.focus(w, cx);
                    if let Some(chosen) = &chosen {
                        chosen(item.value.clone(), w, cx);
                    }
                }
            }
        });
        let mut popup = gpui_base::Popup::new(
            (self.id.clone(), "popup"),
            Input::new(&self.input).theme(self.theme),
        )
        .offset(px(4.))
        .w_full();
        if open {
            let outside = self.opened.clone();
            let mut list = div()
                .id((self.id.clone(), "suggestions"))
                .role(Role::ListBox)
                .w(px(280.))
                .max_h(px(260.))
                .overflow_y_scroll()
                .p(px(4.))
                .rounded(px(12.))
                .border_1()
                .border_color(c.border)
                .bg(c.surface)
                .on_mouse_down_out(move |_, w, cx| {
                    if let Some(f) = &outside {
                        f(false, w, cx);
                    }
                });
            for (index, item) in items.iter().enumerate() {
                let choose = commit.clone();
                list = list.child(
                    gpui_base::Button::new((self.id.clone(), item.value.clone()))
                        .role(Role::ListBoxOption)
                        .disabled(item.disabled)
                        .focusable(false)
                        .aria_selected(active == Some(index))
                        .h(px(32.))
                        .px(px(10.))
                        .w_full()
                        .justify_start()
                        .text_size(px(14.))
                        .text_color(c.text)
                        .bg(if active == Some(index) {
                            c.neutral
                        } else {
                            gpui::transparent_black()
                        })
                        .when(item.disabled, |el| el.opacity(0.5))
                        .on_click(move |_, w, cx| choose(index, w, cx))
                        .child(item.label.clone()),
                );
            }
            popup = popup.content(list);
        }
        let down_items = items.clone();
        let down_query = self.input.clone();
        let up_query = self.input.clone();
        let confirm_query = self.input.clone();
        let escape_query = self.input.clone();
        let down_open = self.opened.clone();
        let down_highlight = self.highlight.clone();
        let up_items = items;
        let up_open = self.opened.clone();
        let up_highlight = self.highlight;
        let close = self.opened;
        let confirm = commit;
        div()
            .id(self.id)
            .role(Role::ComboBox)
            .aria_expanded(open)
            .capture_action(move |_: &gpui_base::input::MoveDown, w, cx| {
                if disabled || composing(&down_query, w, cx) {
                    cx.propagate();
                    return;
                }
                cx.stop_propagation();
                let enabled: Vec<usize> = down_items
                    .iter()
                    .enumerate()
                    .filter_map(|(i, item)| (!item.disabled).then_some(i))
                    .collect();
                if let Some(next) = if open {
                    active
                        .and_then(|current| enabled.iter().position(|&i| i == current))
                        .map(|i| enabled[(i + 1) % enabled.len()])
                } else {
                    enabled.first().copied()
                } && let Some(f) = &down_highlight
                {
                    f(next, w, cx);
                }
                if let Some(f) = &down_open {
                    f(true, w, cx);
                }
            })
            .capture_action(move |_: &gpui_base::input::MoveUp, w, cx| {
                if disabled || composing(&up_query, w, cx) {
                    cx.propagate();
                    return;
                }
                cx.stop_propagation();
                let enabled: Vec<usize> = up_items
                    .iter()
                    .enumerate()
                    .filter_map(|(i, item)| (!item.disabled).then_some(i))
                    .collect();
                if let Some(next) = if open {
                    active
                        .and_then(|current| enabled.iter().position(|&i| i == current))
                        .map(|i| enabled[(i + enabled.len() - 1) % enabled.len()])
                } else {
                    enabled.last().copied()
                } && let Some(f) = &up_highlight
                {
                    f(next, w, cx);
                }
                if let Some(f) = &up_open {
                    f(true, w, cx);
                }
            })
            .capture_action(move |action: &gpui_base::input::Enter, w, cx| {
                if open && !action.secondary && !action.shift && !composing(&confirm_query, w, cx) {
                    cx.stop_propagation();
                    if let Some(index) = active {
                        confirm(index, w, cx);
                    }
                } else {
                    cx.propagate();
                }
            })
            .capture_action(move |_: &gpui_base::input::Escape, w, cx| {
                if open {
                    cx.stop_propagation();
                    if composing(&escape_query, w, cx) {
                        escape_query.update(cx, |state, cx| state.unmark_text(w, cx));
                    } else if let Some(f) = &close {
                        f(false, w, cx);
                    }
                } else {
                    cx.propagate();
                }
            })
            .child(popup)
            .refine_style(&self.style)
    }
}
