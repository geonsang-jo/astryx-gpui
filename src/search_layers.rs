use crate::{Dialog, Input, InputState, Theme};
use gpui::{
    AnyElement, App, ElementId, Entity, EntityId, EntityInputHandler as _, Focusable as _,
    InteractiveElement as _, IntoElement, ParentElement, RenderOnce, Role, SharedString,
    StatefulInteractiveElement as _, StyleRefinement, Styled, Subscription, Window, div,
    prelude::FluentBuilder as _, px, relative,
};
use gpui_base::StyledExt as _;
use std::rc::Rc;

type Value = Rc<dyn Fn(SharedString, &mut Window, &mut App)>;
type Open = Rc<dyn Fn(bool, &mut Window, &mut App)>;
type Highlight = Rc<dyn Fn(usize, &mut Window, &mut App)>;
#[derive(Clone, Debug)]
pub struct CommandItem {
    pub value: SharedString,
    pub label: SharedString,
    pub description: Option<SharedString>,
    pub keywords: SharedString,
    pub disabled: bool,
}
impl CommandItem {
    pub fn new(value: impl Into<SharedString>, label: impl Into<SharedString>) -> Self {
        Self {
            value: value.into(),
            label: label.into(),
            description: None,
            keywords: "".into(),
            disabled: false,
        }
    }
    pub fn description(mut self, value: impl Into<SharedString>) -> Self {
        self.description = Some(value.into());
        self
    }
    pub fn keywords(mut self, value: impl Into<SharedString>) -> Self {
        self.keywords = value.into();
        self
    }
    pub fn disabled(mut self, value: bool) -> Self {
        self.disabled = value;
        self
    }
}
#[derive(Clone, Debug)]
pub struct SearchSection {
    pub title: SharedString,
    pub results: Vec<CommandItem>,
}
impl SearchSection {
    pub fn new(title: impl Into<SharedString>, results: Vec<CommandItem>) -> Self {
        Self {
            title: title.into(),
            results,
        }
    }
}
struct QueryWatch {
    entity: EntityId,
    _subscription: Subscription,
}
struct SearchPanel {
    id: ElementId,
    query: Entity<InputState>,
    title: SharedString,
    sections: Vec<SearchSection>,
    filter: bool,
    open: bool,
    disabled: bool,
    loading: bool,
    highlighted: Option<usize>,
    theme: Theme,
    on_select: Option<Value>,
    on_submit: Option<Value>,
    on_open: Option<Open>,
    on_highlight: Option<Highlight>,
    style: StyleRefinement,
}
impl SearchPanel {
    fn new(
        id: impl Into<ElementId>,
        query: &Entity<InputState>,
        title: &str,
        filter: bool,
    ) -> Self {
        Self {
            id: id.into(),
            query: query.clone(),
            title: title.into(),
            sections: vec![],
            filter,
            open: false,
            disabled: false,
            loading: false,
            highlighted: None,
            theme: Theme::default(),
            on_select: None,
            on_submit: None,
            on_open: None,
            on_highlight: None,
            style: StyleRefinement::default(),
        }
    }
    fn render(self, window: &mut Window, cx: &mut App) -> AnyElement {
        let watch = window.use_keyed_state((self.id.clone(), "query-watch"), cx, |window, cx| {
            QueryWatch {
                entity: self.query.entity_id(),
                _subscription: window.observe(&self.query, cx, |_, window, _| window.refresh()),
            }
        });
        watch.update(cx, |watch, cx| {
            if watch.entity != self.query.entity_id() {
                watch.entity = self.query.entity_id();
                watch._subscription =
                    window.observe(&self.query, cx, |_, window, _| window.refresh());
            }
        });
        let text = self.query.read(cx).value().to_string();
        let needle = text.to_lowercase();
        let words: Vec<&str> = needle.split_whitespace().collect();
        let mut entries = vec![];
        for section in self.sections {
            for item in section.results {
                let haystack = format!(
                    "{} {} {}",
                    item.label,
                    item.keywords,
                    item.description.as_deref().unwrap_or("")
                )
                .to_lowercase();
                if !self.filter || words.iter().all(|word| haystack.contains(word)) {
                    entries.push((section.title.clone(), item));
                }
            }
        }
        let active = self
            .highlighted
            .filter(|&i| entries.get(i).is_some_and(|(_, item)| !item.disabled))
            .or_else(|| entries.iter().position(|(_, item)| !item.disabled));
        let enabled: Vec<usize> = entries
            .iter()
            .enumerate()
            .filter_map(|(i, (_, item))| (!item.disabled).then_some(i))
            .collect();
        let c = self.theme.colors;
        let input_focus = self.query.read(cx).focus_handle(cx);
        let confirm_query = self.query.clone();
        let confirm = self.on_select.clone();
        let submit = self.on_submit;
        let confirm_open = self.on_open.clone();
        let confirm_entries = entries.clone();
        let up = self.on_highlight.clone();
        let up_enabled = enabled.clone();
        let up_query = self.query.clone();
        let down = self.on_highlight;
        let down_query = self.query.clone();
        let escape = self.on_open.clone();
        let escape_query = self.query.clone();
        let mut body = div()
            .id((self.id.clone(), "search-content"))
            .flex()
            .flex_col()
            .min_h_0()
            .gap(px(12.))
            .capture_action(move |_: &gpui_base::input::MoveUp, window, cx| {
                if composing(&up_query, window, cx) {
                    cx.propagate();
                    return;
                }
                cx.stop_propagation();
                let at = up_enabled.iter().position(|&i| Some(i) == active);
                if let Some(index) =
                    up_enabled.get(at.map_or(0, |i| (i + up_enabled.len() - 1) % up_enabled.len()))
                    && let Some(callback) = &up
                {
                    callback(*index, window, cx);
                }
            })
            .capture_action(move |_: &gpui_base::input::MoveDown, window, cx| {
                if composing(&down_query, window, cx) {
                    cx.propagate();
                    return;
                }
                cx.stop_propagation();
                let at = enabled.iter().position(|&i| Some(i) == active);
                if let Some(index) = enabled.get(at.map_or(0, |i| (i + 1) % enabled.len()))
                    && let Some(callback) = &down
                {
                    callback(*index, window, cx);
                }
            })
            .capture_action(move |action: &gpui_base::input::Enter, window, cx| {
                if action.secondary || action.shift || composing(&confirm_query, window, cx) {
                    cx.propagate();
                    return;
                }
                cx.stop_propagation();
                if self.loading {
                    return;
                }
                if let Some((_, item)) = active.and_then(|i| confirm_entries.get(i)) {
                    if let Some(callback) = &confirm_open {
                        callback(false, window, cx);
                    }
                    if let Some(callback) = &confirm {
                        callback(item.value.clone(), window, cx);
                    }
                } else if let Some(callback) = &submit {
                    if let Some(open) = &confirm_open {
                        open(false, window, cx);
                    }
                    callback(text.clone().into(), window, cx);
                }
            })
            .capture_action(move |_: &gpui_base::input::Escape, window, cx| {
                cx.stop_propagation();
                if composing(&escape_query, window, cx) {
                    escape_query.update(cx, |state, cx| state.unmark_text(window, cx));
                } else if let Some(callback) = &escape {
                    callback(false, window, cx);
                }
            })
            .child(
                Input::new(&self.query)
                    .theme(self.theme)
                    .label(self.title.clone()),
            );
        let mut list = div()
            .id((self.id.clone(), "results"))
            .role(Role::ListBox)
            .flex()
            .flex_col()
            .gap(px(2.))
            .min_h_0()
            .max_h(px(360.))
            .overflow_y_scroll();
        if self.loading {
            list = list.child(div().p(px(12.)).text_color(c.muted).child("Searching…"));
        } else {
            let mut section: Option<SharedString> = None;
            for (index, (group, item)) in entries.into_iter().enumerate() {
                if section.as_ref() != Some(&group) && !group.is_empty() {
                    list = list.child(
                        div()
                            .px(px(10.))
                            .py(px(6.))
                            .text_size(px(12.))
                            .text_color(c.muted)
                            .child(group.clone()),
                    );
                }
                section = Some(group);
                let callback = self.on_select.clone();
                let close = self.on_open.clone();
                list = list.child(
                    gpui_base::Button::new((self.id.clone(), item.value.clone()))
                        .role(Role::ListBoxOption)
                        .accessibility_label(item.label.clone())
                        .disabled(item.disabled)
                        .focusable(false)
                        .tab_stop(false)
                        .flex()
                        .flex_col()
                        .items_start()
                        .min_h(px(36.))
                        .px(px(10.))
                        .py(px(8.))
                        .gap(px(2.))
                        .rounded(px(8.))
                        .bg(if active == Some(index) {
                            c.neutral
                        } else {
                            gpui::transparent_black()
                        })
                        .when(active == Some(index), |el| el.aria_active_descendant())
                        .when(item.disabled, |el| el.opacity(0.5))
                        .when(!item.disabled, |el| el.hover(move |s| s.bg(c.hover)))
                        .on_click(move |_, window, cx| {
                            cx.stop_propagation();
                            if let Some(close) = &close {
                                close(false, window, cx);
                            }
                            if let Some(callback) = &callback {
                                callback(item.value.clone(), window, cx);
                            }
                        })
                        .child(item.label)
                        .children(item.description.map(|text| {
                            div()
                                .text_size(px(12.))
                                .line_height(px(20.))
                                .text_color(c.muted)
                                .child(text)
                        })),
                );
            }
            if section.is_none() {
                list = list.child(div().p(px(12.)).text_color(c.muted).child("No results"));
            }
        }
        body = body.child(list);
        Dialog::new(self.id)
            .open(self.open && !self.disabled)
            .theme(self.theme)
            .title(self.title)
            .initial_focus(&input_focus)
            .on_open_change(move |open, _, window, cx| {
                if let Some(callback) = &self.on_open {
                    callback(open, window, cx);
                }
            })
            .w(px(600.))
            .max_h(relative(0.8))
            .child(body)
            .refine_style(&self.style)
            .into_any_element()
    }
}
fn composing(query: &Entity<InputState>, window: &mut Window, cx: &mut App) -> bool {
    query.update(cx, |state, cx| {
        state.marked_text_range(window, cx).is_some()
    })
}

/// A local, token-filtered command dialog over caller-owned native query state.
/// The host receives the selected command; rendering never executes a command.
#[derive(IntoElement)]
pub struct CommandPalette(SearchPanel);
impl CommandPalette {
    pub fn new(id: impl Into<ElementId>, query: &Entity<InputState>) -> Self {
        Self(SearchPanel::new(id, query, "Commands", true))
    }
    pub fn items(mut self, items: Vec<CommandItem>) -> Self {
        self.0.sections = vec![SearchSection::new("", items)];
        self
    }
}
/// A provider-fed grouped search dialog. The host owns fetching, results and
/// loading state. Enter with no selectable result reports the raw query.
#[derive(IntoElement)]
pub struct PowerSearch(SearchPanel);
impl PowerSearch {
    pub fn new(id: impl Into<ElementId>, query: &Entity<InputState>) -> Self {
        Self(SearchPanel::new(id, query, "Search", false))
    }
    pub fn sections(mut self, sections: Vec<SearchSection>) -> Self {
        self.0.sections = sections;
        self
    }
    pub fn loading(mut self, loading: bool) -> Self {
        self.0.loading = loading;
        self
    }
    pub fn on_query_submit(
        mut self,
        callback: impl Fn(SharedString, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.0.on_submit = Some(Rc::new(callback));
        self
    }
}
macro_rules! search_api {
    ($name:ident) => {
        impl $name {
            pub fn open(mut self, value: bool) -> Self {
                self.0.open = value;
                self
            }
            pub fn disabled(mut self, value: bool) -> Self {
                self.0.disabled = value;
                self
            }
            pub fn theme(mut self, value: Theme) -> Self {
                self.0.theme = value;
                self
            }
            pub fn title(mut self, value: impl Into<SharedString>) -> Self {
                self.0.title = value.into();
                self
            }
            pub fn highlighted(mut self, index: usize) -> Self {
                self.0.highlighted = Some(index);
                self
            }
            pub fn on_open_change(
                mut self,
                callback: impl Fn(bool, &mut Window, &mut App) + 'static,
            ) -> Self {
                self.0.on_open = Some(Rc::new(callback));
                self
            }
            pub fn on_select(
                mut self,
                callback: impl Fn(SharedString, &mut Window, &mut App) + 'static,
            ) -> Self {
                self.0.on_select = Some(Rc::new(callback));
                self
            }
            pub fn on_highlight(
                mut self,
                callback: impl Fn(usize, &mut Window, &mut App) + 'static,
            ) -> Self {
                self.0.on_highlight = Some(Rc::new(callback));
                self
            }
        }
        impl Styled for $name {
            fn style(&mut self) -> &mut StyleRefinement {
                &mut self.0.style
            }
        }
        impl RenderOnce for $name {
            fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
                self.0.render(window, cx)
            }
        }
    };
}
search_api!(CommandPalette);
search_api!(PowerSearch);
