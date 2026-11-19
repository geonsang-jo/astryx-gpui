use crate::Theme;
use gpui::{
    AnyElement, App, AppContext as _, ClickEvent, Context, ElementId, Entity, EntityId,
    FocusHandle, InteractiveElement as _, IntoElement, MouseButton, ParentElement, Render,
    RenderOnce, Role, SharedString, StatefulInteractiveElement as _, StyleRefinement, Styled,
    Window, div, prelude::FluentBuilder as _, px,
};
use gpui_base::StyledExt as _;
use std::{collections::HashMap, rc::Rc};

type Value = Rc<dyn Fn(SharedString, &mut Window, &mut App)>;
type Click = Rc<dyn Fn(&ClickEvent, &mut Window, &mut App)>;
type Move = Rc<dyn Fn(SharedString, Option<SharedString>, &mut Window, &mut App)>;

/// A composable native row. Supplying on_click gives it native Button activation.
#[derive(IntoElement)]
pub struct Item {
    id: ElementId,
    label: SharedString,
    description: Option<SharedString>,
    leading: Option<AnyElement>,
    trailing: Option<AnyElement>,
    selected: bool,
    disabled: bool,
    theme: Theme,
    click: Option<Click>,
    style: StyleRefinement,
}
impl Item {
    pub fn new(id: impl Into<ElementId>, label: impl Into<SharedString>) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            description: None,
            leading: None,
            trailing: None,
            selected: false,
            disabled: false,
            theme: Theme::default(),
            click: None,
            style: StyleRefinement::default(),
        }
    }
    pub fn description(mut self, value: impl Into<SharedString>) -> Self {
        self.description = Some(value.into());
        self
    }
    pub fn leading(mut self, value: impl IntoElement) -> Self {
        self.leading = Some(value.into_any_element());
        self
    }
    pub fn trailing(mut self, value: impl IntoElement) -> Self {
        self.trailing = Some(value.into_any_element());
        self
    }
    pub fn selected(mut self, value: bool) -> Self {
        self.selected = value;
        self
    }
    pub fn disabled(mut self, value: bool) -> Self {
        self.disabled = value;
        self
    }
    pub fn theme(mut self, value: Theme) -> Self {
        self.theme = value;
        self
    }
    pub fn on_click(
        mut self,
        handler: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.click = Some(Rc::new(handler));
        self
    }
}
impl Styled for Item {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}
impl RenderOnce for Item {
    fn render(self, _: &mut Window, _: &mut App) -> impl IntoElement {
        let c = self.theme.colors;
        let content = div()
            .flex()
            .items_center()
            .gap(px(10.))
            .children(self.leading)
            .child(
                div()
                    .flex_1()
                    .min_w_0()
                    .flex()
                    .flex_col()
                    .gap(px(2.))
                    .child(self.label.clone())
                    .children(self.description.map(|text| {
                        div()
                            .text_size(px(12.))
                            .line_height(px(20.))
                            .text_color(c.muted)
                            .child(text)
                    })),
            )
            .children(self.trailing);
        if let Some(click) = self.click {
            gpui_base::Button::new(self.id)
                .accessibility_label(self.label)
                .disabled(self.disabled)
                .selected(self.selected)
                .w_full()
                .min_h(px(40.))
                .px(px(12.))
                .py(px(8.))
                .rounded(px(10.))
                .text_size(px(14.))
                .line_height(px(20.))
                .text_color(c.text)
                .bg(if self.selected {
                    c.neutral
                } else {
                    gpui::transparent_black()
                })
                .when(!self.disabled, |el| el.hover(move |s| s.bg(c.hover)))
                .when(self.disabled, |el| el.opacity(0.5))
                .focus_visible(move |s| s.bg(c.neutral))
                .on_click(move |event, window, cx| click(event, window, cx))
                .child(content)
                .refine_style(&self.style)
                .into_any_element()
        } else {
            div()
                .id(self.id)
                .role(Role::ListItem)
                .min_h(px(40.))
                .px(px(12.))
                .py(px(8.))
                .rounded(px(10.))
                .text_size(px(14.))
                .line_height(px(20.))
                .text_color(c.text)
                .bg(if self.selected {
                    c.neutral
                } else {
                    gpui::transparent_black()
                })
                .when(self.disabled, |el| el.opacity(0.5))
                .child(content)
                .refine_style(&self.style)
                .into_any_element()
        }
    }
}

#[derive(Clone, Debug)]
pub struct ListEntry {
    pub value: SharedString,
    pub label: SharedString,
    pub description: Option<SharedString>,
    pub disabled: bool,
}
impl ListEntry {
    pub fn new(value: impl Into<SharedString>, label: impl Into<SharedString>) -> Self {
        Self {
            value: value.into(),
            label: label.into(),
            description: None,
            disabled: false,
        }
    }
    pub fn description(mut self, value: impl Into<SharedString>) -> Self {
        self.description = Some(value.into());
        self
    }
    pub fn disabled(mut self, value: bool) -> Self {
        self.disabled = value;
        self
    }
}
#[derive(Default)]
struct ListFocus(HashMap<SharedString, FocusHandle>);
#[derive(Clone)]
struct ListDrag {
    list: ElementId,
    scope: EntityId,
    value: SharedString,
    label: SharedString,
    theme: Theme,
}
impl Render for ListDrag {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        let c = self.theme.colors;
        div()
            .p(px(10.))
            .rounded(px(10.))
            .bg(c.surface)
            .border_1()
            .border_color(c.border_emphasized)
            .text_color(c.text)
            .text_size(px(14.))
            .line_height(px(20.))
            .child(self.label.clone())
    }
}
/// A controlled selectable list. Keys are stable and unique. Optional native
/// drag reordering reports "move value before target", where None means end;
/// the owner updates its data. Rendering never reorders a model.
#[derive(IntoElement)]
pub struct List {
    id: ElementId,
    entries: Vec<ListEntry>,
    selected: Option<SharedString>,
    theme: Theme,
    select: Option<Value>,
    activate: Option<Value>,
    reorder: Option<Move>,
    style: StyleRefinement,
}
impl List {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            entries: vec![],
            selected: None,
            theme: Theme::default(),
            select: None,
            activate: None,
            reorder: None,
            style: StyleRefinement::default(),
        }
    }
    pub fn entries(mut self, value: Vec<ListEntry>) -> Self {
        self.entries = value;
        self
    }
    pub fn selected(mut self, value: impl Into<SharedString>) -> Self {
        self.selected = Some(value.into());
        self
    }
    pub fn theme(mut self, value: Theme) -> Self {
        self.theme = value;
        self
    }
    pub fn on_select(
        mut self,
        handler: impl Fn(SharedString, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.select = Some(Rc::new(handler));
        self
    }
    pub fn on_activate(
        mut self,
        handler: impl Fn(SharedString, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.activate = Some(Rc::new(handler));
        self
    }
    pub fn on_move(
        mut self,
        handler: impl Fn(SharedString, Option<SharedString>, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.reorder = Some(Rc::new(handler));
        self
    }
}
impl Styled for List {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}
fn flat_destination<T>(
    items: &[T],
    current: usize,
    key: &str,
    enabled: impl Fn(&T) -> bool,
) -> Option<usize> {
    let enabled: Vec<usize> = items
        .iter()
        .enumerate()
        .filter_map(|(i, item)| enabled(item).then_some(i))
        .collect();
    let at = enabled.iter().position(|&i| i == current)?;
    match key {
        "up" => Some(enabled[(at + enabled.len() - 1) % enabled.len()]),
        "down" => Some(enabled[(at + 1) % enabled.len()]),
        "home" => enabled.first().copied(),
        "end" => enabled.last().copied(),
        _ => None,
    }
}
impl RenderOnce for List {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let c = self.theme.colors;
        let drag_scope = window.current_view();
        let focus = window.use_keyed_state((self.id.clone(), "list-focus"), cx, |_, _| {
            ListFocus::default()
        });
        let handles = focus.update(cx, |state, cx| {
            state
                .0
                .retain(|key, _| self.entries.iter().any(|row| &row.value == key));
            self.entries
                .iter()
                .map(|row| {
                    state
                        .0
                        .entry(row.value.clone())
                        .or_insert_with(|| cx.focus_handle())
                        .clone()
                })
                .collect::<Vec<_>>()
        });
        let handles = Rc::new(handles);
        let entries = Rc::new(self.entries);
        let active = entries
            .iter()
            .position(|row| !row.disabled && Some(&row.value) == self.selected.as_ref())
            .or_else(|| entries.iter().position(|row| !row.disabled));
        let mut root = div()
            .id(self.id.clone())
            .role(Role::ListBox)
            .flex()
            .flex_col()
            .min_w_0()
            .gap(px(2.));
        for (index, row) in entries.iter().enumerate() {
            let select = self.select.clone();
            let activate = self.activate.clone();
            let keyboard = self.select.clone();
            let value = row.value.clone();
            let nav = entries.clone();
            let nav_focus = handles.clone();
            let mut button = gpui_base::Button::new((self.id.clone(), row.value.clone()))
                .role(Role::ListBoxOption)
                .accessibility_label(row.label.clone())
                .disabled(row.disabled)
                .track_focus(&handles[index])
                .tab_stop(active == Some(index))
                .aria_selected(Some(&row.value) == self.selected.as_ref())
                .w_full()
                .min_h(px(40.))
                .px(px(12.))
                .py(px(8.))
                .rounded(px(10.))
                .flex()
                .flex_col()
                .items_start()
                .gap(px(2.))
                .text_size(px(14.))
                .line_height(px(20.))
                .text_color(c.text)
                .bg(if Some(&row.value) == self.selected.as_ref() {
                    c.neutral
                } else {
                    gpui::transparent_black()
                })
                .when(!row.disabled, |el| el.hover(move |s| s.bg(c.hover)))
                .when(row.disabled, |el| el.opacity(0.5))
                .focus_visible(move |s| s.border_1().border_color(c.focus))
                .on_click(move |event, window, cx| {
                    cx.stop_propagation();
                    if let Some(callback) = &select {
                        callback(value.clone(), window, cx);
                    }
                    if (event.is_keyboard() || event.click_count() > 1)
                        && let Some(callback) = &activate
                    {
                        callback(value.clone(), window, cx);
                    }
                })
                .on_key_down(move |event, window, cx| {
                    if event.keystroke.modifiers != gpui::Modifiers::default() {
                        return;
                    }
                    if let Some(next) =
                        flat_destination(&nav, index, &event.keystroke.key, |row| !row.disabled)
                    {
                        cx.stop_propagation();
                        nav_focus[next].focus(window, cx);
                        if let Some(callback) = &keyboard {
                            callback(nav[next].value.clone(), window, cx);
                        }
                    }
                })
                .child(row.label.clone())
                .children(row.description.as_ref().map(|text| {
                    div()
                        .text_size(px(12.))
                        .line_height(px(20.))
                        .text_color(c.muted)
                        .child(text.clone())
                }));
            if let Some(reorder) = self.reorder.clone() {
                let list_id = self.id.clone();
                let target = row.value.clone();
                button = button.drag_over::<ListDrag>(move |style, drag, _, _| {
                    if drag.list == list_id && drag.scope == drag_scope {
                        style.border_t_2().border_color(c.focus)
                    } else {
                        style
                    }
                });
                let list_id = self.id.clone();
                let current_entries = entries.clone();
                button = button.on_drop(move |drag: &ListDrag, window, cx| {
                    if drag.list == list_id
                        && drag.scope == drag_scope
                        && drag.value != target
                        && current_entries
                            .iter()
                            .any(|row| row.value == drag.value && !row.disabled)
                    {
                        cx.stop_propagation();
                        reorder(drag.value.clone(), Some(target.clone()), window, cx);
                    }
                });
                if !row.disabled {
                    button = button.on_drag(
                        ListDrag {
                            list: self.id.clone(),
                            scope: drag_scope,
                            value: row.value.clone(),
                            label: row.label.clone(),
                            theme: self.theme,
                        },
                        |drag, _, _, cx| cx.new(|_| drag.clone()),
                    );
                }
            }
            root = root.child(button);
        }
        if let Some(reorder) = self.reorder {
            let id = self.id;
            let current_entries = entries;
            root = root.child(div().id("list-drop-end").h(px(12.)).w_full().on_drop(
                move |drag: &ListDrag, window, cx| {
                    if drag.list == id
                        && drag.scope == drag_scope
                        && current_entries
                            .iter()
                            .any(|row| row.value == drag.value && !row.disabled)
                    {
                        cx.stop_propagation();
                        reorder(drag.value.clone(), None, window, cx);
                    }
                },
            ));
        }
        root.refine_style(&self.style)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SortDirection {
    Ascending,
    Descending,
}
#[derive(Clone, Debug)]
pub struct TableColumn {
    pub key: SharedString,
    pub label: SharedString,
    pub width: Option<f32>,
    pub sortable: bool,
}
impl TableColumn {
    pub fn new(key: impl Into<SharedString>, label: impl Into<SharedString>) -> Self {
        Self {
            key: key.into(),
            label: label.into(),
            width: None,
            sortable: false,
        }
    }
    pub fn width(mut self, value: f32) -> Self {
        if value.is_finite() {
            self.width = Some(value.max(32.))
        }
        self
    }
    pub fn sortable(mut self, value: bool) -> Self {
        self.sortable = value;
        self
    }
}
pub struct TableRow {
    pub key: SharedString,
    pub disabled: bool,
    cells: Vec<AnyElement>,
}
impl TableRow {
    pub fn new(key: impl Into<SharedString>) -> Self {
        Self {
            key: key.into(),
            disabled: false,
            cells: vec![],
        }
    }
    pub fn disabled(mut self, value: bool) -> Self {
        self.disabled = value;
        self
    }
    pub fn cell(mut self, value: impl IntoElement) -> Self {
        self.cells.push(value.into_any_element());
        self
    }
}
type Sort = Rc<dyn Fn(SharedString, SortDirection, &mut Window, &mut App)>;
/// A semantic native table with owner-controlled ordering and row selection.
/// Sorting reports a request; the host supplies rows in the resulting order.
#[derive(IntoElement)]
pub struct Table {
    id: ElementId,
    columns: Vec<TableColumn>,
    rows: Vec<TableRow>,
    selected: Option<SharedString>,
    sort: Option<(SharedString, SortDirection)>,
    on_sort: Option<Sort>,
    on_select: Option<Value>,
    theme: Theme,
    style: StyleRefinement,
}
impl Table {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            columns: vec![],
            rows: vec![],
            selected: None,
            sort: None,
            on_sort: None,
            on_select: None,
            theme: Theme::default(),
            style: StyleRefinement::default(),
        }
    }
    pub fn columns(mut self, value: Vec<TableColumn>) -> Self {
        self.columns = value;
        self
    }
    pub fn rows(mut self, value: Vec<TableRow>) -> Self {
        self.rows = value;
        self
    }
    pub fn selected(mut self, value: impl Into<SharedString>) -> Self {
        self.selected = Some(value.into());
        self
    }
    pub fn sorted_by(mut self, key: impl Into<SharedString>, direction: SortDirection) -> Self {
        self.sort = Some((key.into(), direction));
        self
    }
    pub fn theme(mut self, value: Theme) -> Self {
        self.theme = value;
        self
    }
    pub fn on_sort(
        mut self,
        handler: impl Fn(SharedString, SortDirection, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_sort = Some(Rc::new(handler));
        self
    }
    pub fn on_row_select(
        mut self,
        handler: impl Fn(SharedString, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_select = Some(Rc::new(handler));
        self
    }
}
impl Styled for Table {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}
impl RenderOnce for Table {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let c = self.theme.colors;
        let columns = &self.columns;
        let row_count = self.rows.len() + 1;
        let mut header = gpui_base::TableRow::new((self.id.clone(), "head-row"), 1)
            .flex()
            .border_b_1()
            .border_color(c.border)
            .bg(c.canvas);
        for (column_index, column) in columns.iter().enumerate() {
            let mut cell =
                gpui_base::TableHead::new((self.id.clone(), column.key.clone()), column_index + 1)
                    .min_w_0()
                    .px(px(10.))
                    .py(px(8.));
            cell = if let Some(width) = column.width {
                cell.w(px(width)).flex_none()
            } else {
                cell.flex_1()
            };
            if column.sortable {
                let key = column.key.clone();
                let callback = self.on_sort.clone();
                let direction =
                    if self.sort.as_ref().is_some_and(|(key, d)| {
                        key == &column.key && *d == SortDirection::Ascending
                    }) {
                        SortDirection::Descending
                    } else {
                        SortDirection::Ascending
                    };
                let indicator = self
                    .sort
                    .as_ref()
                    .filter(|(key, _)| key == &column.key)
                    .map(|(_, direction)| {
                        if *direction == SortDirection::Ascending {
                            " ↑"
                        } else {
                            " ↓"
                        }
                    })
                    .unwrap_or("");
                cell = cell.child(
                    gpui_base::Button::new((self.id.clone(), format!("sort-{}", column.key)))
                        .accessibility_label(format!("Sort by {}", column.label))
                        .text_color(c.muted)
                        .text_size(px(12.))
                        .line_height(px(20.))
                        .on_click(move |_, window, cx| {
                            cx.stop_propagation();
                            if let Some(callback) = &callback {
                                callback(key.clone(), direction, window, cx);
                            }
                        })
                        .child(format!("{}{}", column.label, indicator)),
                );
            } else {
                cell = cell.child(column.label.clone());
            }
            header = header.child(cell);
        }
        let focus = window.use_keyed_state((self.id.clone(), "table-focus"), cx, |_, _| {
            ListFocus::default()
        });
        let handles = focus.update(cx, |state, cx| {
            state
                .0
                .retain(|key, _| self.rows.iter().any(|row| &row.key == key));
            self.rows
                .iter()
                .map(|row| {
                    state
                        .0
                        .entry(row.key.clone())
                        .or_insert_with(|| cx.focus_handle())
                        .clone()
                })
                .collect::<Vec<_>>()
        });
        let active = self
            .rows
            .iter()
            .position(|row| !row.disabled && Some(&row.key) == self.selected.as_ref())
            .or_else(|| self.rows.iter().position(|row| !row.disabled));
        let metadata = Rc::new(
            self.rows
                .iter()
                .map(|row| (row.key.clone(), row.disabled))
                .collect::<Vec<_>>(),
        );
        let handles = Rc::new(handles);
        let mut body = gpui_base::TableBody::new((self.id.clone(), "body"))
            .flex()
            .flex_col();
        for (index, row) in self.rows.into_iter().enumerate() {
            let selected = Some(&row.key) == self.selected.as_ref();
            let callback = self.on_select.clone();
            let pointer = callback.clone();
            let focus = handles[index].clone();
            let nav_focus = handles.clone();
            let nav_rows = metadata.clone();
            let key = row.key.clone();
            let mut native = gpui_base::TableRow::new((self.id.clone(), row.key), index + 2)
                .flex()
                .min_h(px(40.))
                .border_b_1()
                .border_color(c.border)
                .bg(if selected { c.neutral } else { c.surface })
                .when(row.disabled, |el| el.opacity(0.5));
            if !row.disabled && callback.is_some() {
                native = native
                    .track_focus(&focus.clone().tab_stop(active == Some(index)))
                    .focus_visible(move |s| s.bg(c.hover))
                    .on_click(move |_, window, cx| {
                        cx.stop_propagation();
                        if let Some(callback) = &pointer {
                            callback(key.clone(), window, cx);
                        }
                    })
                    .on_key_down(move |event, window, cx| {
                        if event.keystroke.modifiers != gpui::Modifiers::default() {
                            return;
                        }
                        if let Some(next) = flat_destination(
                            &nav_rows,
                            index,
                            &event.keystroke.key,
                            |(_, disabled)| !disabled,
                        ) {
                            cx.stop_propagation();
                            nav_focus[next].focus(window, cx);
                            if let Some(callback) = &callback {
                                callback(nav_rows[next].0.clone(), window, cx);
                            }
                        }
                    });
            }
            let mut cells = row.cells.into_iter();
            for (column_index, column) in columns.iter().enumerate() {
                let mut cell = gpui_base::TableCell::new(
                    (self.id.clone(), format!("{}-{}", index, column.key)),
                    column_index + 1,
                )
                .min_w_0()
                .px(px(10.))
                .py(px(10.));
                cell = if let Some(width) = column.width {
                    cell.w(px(width)).flex_none()
                } else {
                    cell.flex_1()
                };
                native = native.child(cell.children(cells.next()));
            }
            body = body.child(native);
        }
        gpui_base::Table::new(self.id)
            .column_count(columns.len())
            .row_count(row_count)
            .flex()
            .flex_col()
            .text_color(c.text)
            .text_size(px(14.))
            .line_height(px(20.))
            .border_1()
            .border_color(c.border)
            .rounded(px(10.))
            .child(gpui_base::TableHeader::new("table-header").child(header))
            .child(body)
            .refine_style(&self.style)
    }
}

pub use gpui_base::{TreeEntry, TreeEvent, TreeItem, TreeState};
/// Native virtualized tree presentation over an externally retained TreeState.
/// The wrapper supplies Tab focus, disabled-skipping navigation and leaf activation;
/// native state remains the single owner of selection and expansion.
#[derive(IntoElement)]
pub struct TreeList {
    state: Entity<TreeState>,
    theme: Theme,
    focus: Option<FocusHandle>,
    activate: Option<Value>,
    style: StyleRefinement,
}
impl TreeList {
    pub fn new(state: &Entity<TreeState>) -> Self {
        Self {
            state: state.clone(),
            theme: Theme::default(),
            focus: None,
            activate: None,
            style: StyleRefinement::default(),
        }
    }
    pub fn theme(mut self, value: Theme) -> Self {
        self.theme = value;
        self
    }
    pub fn track_focus(mut self, value: &FocusHandle) -> Self {
        self.focus = Some(value.clone());
        self
    }
    pub fn on_activate(
        mut self,
        handler: impl Fn(SharedString, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.activate = Some(Rc::new(handler));
        self
    }
}
impl Styled for TreeList {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}
fn tree_entries(state: &TreeState) -> Vec<TreeEntry> {
    (0..).map_while(|i| state.entry(i).cloned()).collect()
}
fn tree_toggle(state: &mut TreeState, index: usize, cx: &mut Context<TreeState>) {
    let Some(entry) = state.entry(index).cloned() else {
        return;
    };
    if !entry.is_folder() || entry.is_disabled() {
        return;
    }
    let item = entry.item().clone();
    let expanded = !item.is_expanded();
    item.clone().expanded(expanded);
    let roots = tree_entries(state)
        .into_iter()
        .filter(TreeEntry::is_root)
        .map(|entry| entry.item().clone())
        .collect::<Vec<_>>();
    state.set_items(roots, cx);
    state.set_selected_item(Some(&item), cx);
    cx.emit(if expanded {
        TreeEvent::Expanded(item.id)
    } else {
        TreeEvent::Collapsed(item.id)
    });
}
fn tree_key(
    state: &Entity<TreeState>,
    key: &str,
    activate: &Option<Value>,
    window: &mut Window,
    cx: &mut App,
) {
    let value = state.update(cx, |state, cx| {
        let entries = tree_entries(state);
        let selected = state.selected_index();
        let active = selected.and_then(|i| entries.get(i));
        let next = match key {
            "up" | "down" | "home" | "end" => {
                let enabled: Vec<usize> = entries
                    .iter()
                    .enumerate()
                    .filter_map(|(i, entry)| (!entry.is_disabled()).then_some(i))
                    .collect();
                let at = enabled.iter().position(|&i| Some(i) == selected);
                match key {
                    "home" => enabled.first().copied(),
                    "end" => enabled.last().copied(),
                    "up" => enabled
                        .get(at.map_or(0, |i| (i + enabled.len() - 1) % enabled.len()))
                        .copied(),
                    _ => enabled
                        .get(at.map_or(0, |i| (i + 1) % enabled.len()))
                        .copied(),
                }
            }
            "right" => {
                if let (Some(index), Some(entry)) = (selected, active) {
                    if entry.is_folder() && !entry.is_expanded() {
                        tree_toggle(state, index, cx);
                        None
                    } else {
                        ((index + 1)..entries.len())
                            .take_while(|&i| entries[i].depth() > entry.depth())
                            .find(|&i| !entries[i].is_disabled())
                    }
                } else {
                    None
                }
            }
            "left" => {
                if let (Some(index), Some(entry)) = (selected, active) {
                    if entry.is_folder() && entry.is_expanded() {
                        tree_toggle(state, index, cx);
                        None
                    } else {
                        (0..index).rev().find(|&i| {
                            entries[i].depth() < entry.depth() && !entries[i].is_disabled()
                        })
                    }
                } else {
                    None
                }
            }
            "enter" | "space" => {
                if let (Some(index), Some(entry)) = (selected, active) {
                    if entry.is_folder() {
                        tree_toggle(state, index, cx);
                    } else if !entry.is_disabled() {
                        return Some(entry.item().id.clone());
                    }
                }
                None
            }
            _ => None,
        };
        if let Some(next) = next {
            state.set_selected_index(Some(next), cx);
            state.scroll_to_item(next, gpui::ScrollStrategy::Top);
        }
        None
    });
    if let (Some(value), Some(callback)) = (value, activate) {
        callback(value, window, cx);
    }
}
impl RenderOnce for TreeList {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let c = self.theme.colors;
        let focus = self.focus.unwrap_or_else(|| {
            window
                .use_keyed_state(
                    ("astryx-tree-focus", self.state.entity_id()),
                    cx,
                    |_, cx| cx.focus_handle(),
                )
                .read(cx)
                .clone()
        });
        let pointer_focus = focus.clone();
        let state = self.state.clone();
        let activate = self.activate;
        let action_state = state.clone();
        let action_activate = activate.clone();
        let style = div().w_full().h_full().style().clone();
        let mut frame = div()
            .id(("astryx-tree", state.entity_id()))
            .role(Role::Group)
            .aria_label("Tree navigation")
            .track_focus(&focus.tab_stop(true))
            .key_context("AstryxTreeList")
            .w_full()
            .h(px(280.))
            .on_mouse_down(MouseButton::Left, move |_, window, cx| {
                pointer_focus.focus(window, cx)
            })
            .on_key_down(move |event, window, cx| {
                if event.keystroke.modifiers == gpui::Modifiers::default()
                    && matches!(
                        event.keystroke.key.as_str(),
                        "up" | "down" | "left" | "right" | "home" | "end" | "enter" | "space"
                    )
                {
                    cx.stop_propagation();
                    tree_key(&state, &event.keystroke.key, &activate, window, cx);
                }
            });
        macro_rules! capture_tree_action {
            ($action:ty,$key:expr) => {{
                let state = action_state.clone();
                let activate = action_activate.clone();
                frame = frame.capture_action(move |_: &$action, window, cx| {
                    cx.stop_propagation();
                    tree_key(&state, $key, &activate, window, cx);
                });
            }};
        }
        capture_tree_action!(gpui_base::actions::SelectUp, "up");
        capture_tree_action!(gpui_base::actions::SelectDown, "down");
        capture_tree_action!(gpui_base::actions::SelectLeft, "left");
        capture_tree_action!(gpui_base::actions::SelectRight, "right");
        capture_tree_action!(gpui_base::actions::Confirm, "enter");
        frame
            .child(
                gpui_base::Tree::new(&self.state)
                    .w_full()
                    .h_full()
                    .list_style(style)
                    .item(move |_, entry, state, _, _| {
                        div()
                            .h(px(32.))
                            .flex()
                            .items_center()
                            .gap(px(8.))
                            .pl(px(8. + entry.depth() as f32 * 16.))
                            .pr(px(8.))
                            .rounded(px(8.))
                            .text_size(px(14.))
                            .line_height(px(20.))
                            .text_color(c.text)
                            .bg(if state.is_selected() {
                                c.neutral
                            } else {
                                gpui::transparent_black()
                            })
                            .when(entry.is_disabled(), |el| el.opacity(0.5))
                            .child(div().w(px(12.)).text_color(c.muted).child(
                                if entry.is_folder() {
                                    if entry.is_expanded() { "▾" } else { "▸" }
                                } else {
                                    ""
                                },
                            ))
                            .child(entry.item().label.clone())
                            .into_any_element()
                    }),
            )
            .refine_style(&self.style)
    }
}
