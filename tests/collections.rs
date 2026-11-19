use astryx_gpui::{
    Item, List, ListEntry, SortDirection, Table, TableColumn, TableRow, TreeItem, TreeList,
    TreeState,
};
use gpui::{
    AppContext as _, Context, Entity, FocusHandle, InteractiveElement as _, IntoElement,
    KeyDownEvent, KeyUpEvent, Keystroke, Modifiers, MouseButton, ParentElement, Render,
    SharedString, Styled, TestAppContext, VisualTestContext, Window, div, point, px,
};
#[derive(Clone, Copy)]
enum Kind {
    List,
    Dnd,
    Table,
    Tree,
    Item,
}
struct Harness {
    kind: Kind,
    values: Vec<SharedString>,
    selected: SharedString,
    selections: Vec<SharedString>,
    activations: Vec<SharedString>,
    moves: Vec<(SharedString, Option<SharedString>)>,
    sort: Option<SortDirection>,
    sorts: Vec<SortDirection>,
    tree: Entity<TreeState>,
    tree_focus: FocusHandle,
    clicks: usize,
    disabled: bool,
}
impl Render for Harness {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let selected = cx.entity().downgrade();
        let activated = selected.clone();
        let moved = selected.clone();
        let sorted = selected.clone();
        let control = match self.kind {
            Kind::List | Kind::Dnd => {
                let list = List::new("list")
                    .entries(
                        self.values
                            .iter()
                            .map(|id| {
                                ListEntry::new(id.clone(), id.clone()).disabled(id.as_str() == "b")
                            })
                            .collect(),
                    )
                    .selected(self.selected.clone())
                    .on_select(move |value, _, cx| {
                        selected
                            .update(cx, |this, cx| {
                                this.selected = value.clone();
                                this.selections.push(value);
                                cx.notify();
                            })
                            .unwrap();
                    })
                    .on_activate(move |value, _, cx| {
                        activated
                            .update(cx, |this, cx| {
                                this.activations.push(value);
                                cx.notify();
                            })
                            .unwrap();
                    });
                let list = if matches!(self.kind, Kind::Dnd) {
                    list.on_move(move |value, before, _, cx| {
                        moved
                            .update(cx, |this, cx| {
                                this.moves.push((value.clone(), before.clone()));
                                this.values.retain(|id| id != &value);
                                let target = before
                                    .as_ref()
                                    .and_then(|before| {
                                        this.values.iter().position(|id| id == before)
                                    })
                                    .unwrap_or(this.values.len());
                                this.values.insert(target, value);
                                cx.notify();
                            })
                            .unwrap();
                    })
                } else {
                    list
                };
                list.into_any_element()
            }
            Kind::Table => {
                let table = Table::new("table")
                    .columns(vec![
                        TableColumn::new("name", "Name").sortable(true),
                        TableColumn::new("value", "Value"),
                    ])
                    .rows(
                        self.values
                            .iter()
                            .map(|id| {
                                TableRow::new(id.clone())
                                    .disabled(id.as_str() == "b")
                                    .cell(id.clone())
                                    .cell("Cell")
                            })
                            .collect(),
                    )
                    .selected(self.selected.clone())
                    .on_sort(move |_, direction, _, cx| {
                        sorted
                            .update(cx, |this, cx| {
                                this.sort = Some(direction);
                                this.sorts.push(direction);
                                this.values.sort();
                                if direction == SortDirection::Descending {
                                    this.values.reverse();
                                }
                                cx.notify();
                            })
                            .unwrap();
                    })
                    .on_row_select(move |value, _, cx| {
                        selected
                            .update(cx, |this, cx| {
                                this.selected = value.clone();
                                this.selections.push(value);
                                cx.notify();
                            })
                            .unwrap();
                    });
                let table = if let Some(sort) = self.sort {
                    table.sorted_by("name", sort)
                } else {
                    table
                };
                table.into_any_element()
            }
            Kind::Tree => TreeList::new(&self.tree)
                .track_focus(&self.tree_focus)
                .h(px(240.))
                .on_activate(move |value, _, cx| {
                    activated
                        .update(cx, |this, cx| {
                            this.activations.push(value);
                            cx.notify();
                        })
                        .unwrap();
                })
                .into_any_element(),
            Kind::Item => Item::new("item", "Task")
                .description("Supporting details")
                .disabled(self.disabled)
                .leading(div().size(px(12.)).child("•"))
                .trailing(div().child("⌘1"))
                .on_click(cx.listener(|this, _, _, cx| {
                    this.clicks += 1;
                    cx.notify();
                }))
                .into_any_element(),
        };
        div()
            .id("collection-owner")
            .w(px(400.))
            .h(px(400.))
            .child(control)
    }
}
fn mount(cx: &mut TestAppContext, kind: Kind) -> (Entity<Harness>, &mut VisualTestContext) {
    cx.update(astryx_gpui::init);
    let (view, cx) = cx.add_window_view(move |_, cx| Harness {
        kind,
        values: vec!["a".into(), "b".into(), "c".into()],
        selected: "a".into(),
        selections: vec![],
        activations: vec![],
        moves: vec![],
        sort: None,
        sorts: vec![],
        tree: cx.new(|cx| {
            TreeState::new(cx).items(vec![
                TreeItem::new("root", "Root")
                    .child(TreeItem::new("skip", "Skip").disabled(true))
                    .child(TreeItem::new("leaf", "Leaf")),
                TreeItem::new("last", "Last"),
            ])
        }),
        tree_focus: cx.focus_handle(),
        clicks: 0,
        disabled: false,
    });
    draw(cx);
    (view, cx)
}
fn draw(cx: &mut VisualTestContext) {
    cx.update(|w, cx| w.draw(cx).clear(cx));
}
fn key(cx: &mut VisualTestContext, name: &str) {
    let stroke = Keystroke::parse(name).unwrap();
    cx.simulate_event(KeyDownEvent {
        keystroke: stroke.clone(),
        is_held: false,
        prefer_character_input: false,
    });
    cx.simulate_event(KeyUpEvent { keystroke: stroke });
    draw(cx);
}
#[gpui::test]
fn list_arrows_skip_disabled_and_native_keyboard_activation_runs_once(cx: &mut TestAppContext) {
    let (view, cx) = mount(cx, Kind::List);
    cx.simulate_click(point(px(10.), px(16.)), Modifiers::default());
    draw(cx);
    key(cx, "down");
    assert_eq!(view.read_with(cx, |v, _| v.selected.to_string()), "c");
    key(cx, "enter");
    assert_eq!(
        view.read_with(cx, |v, _| v.activations.clone()),
        vec![SharedString::from("c")]
    );
    assert_eq!(view.read_with(cx, |v, _| v.selections.len()), 3);
}
#[gpui::test]
fn list_click_release_cannot_arm_a_later_hover_drag(cx: &mut TestAppContext) {
    let (view, cx) = mount(cx, Kind::Dnd);
    cx.simulate_click(point(px(10.), px(16.)), Modifiers::default());
    cx.simulate_mouse_move(point(px(80.), px(100.)), None, Modifiers::default());
    draw(cx);
    assert!(view.read_with(cx, |v, _| v.moves.is_empty()));
    assert_eq!(view.read_with(cx, |v, _| v.selections.len()), 1);
}
#[gpui::test]
fn list_native_drag_reports_a_model_move_and_owner_reorders(cx: &mut TestAppContext) {
    let (view, cx) = mount(cx, Kind::Dnd);
    cx.simulate_mouse_down(
        point(px(10.), px(16.)),
        MouseButton::Left,
        Modifiers::default(),
    );
    cx.simulate_mouse_move(
        point(px(40.), px(20.)),
        Some(MouseButton::Left),
        Modifiers::default(),
    );
    draw(cx);
    cx.simulate_mouse_move(
        point(px(30.), px(100.)),
        Some(MouseButton::Left),
        Modifiers::default(),
    );
    draw(cx);
    cx.simulate_mouse_up(
        point(px(30.), px(100.)),
        MouseButton::Left,
        Modifiers::default(),
    );
    draw(cx);
    assert_eq!(
        view.read_with(cx, |v, _| v.moves.clone()),
        vec![("a".into(), Some("c".into()))]
    );
    assert_eq!(
        view.read_with(cx, |v, _| v.values.clone()),
        vec![SharedString::from("b"), "a".into(), "c".into()]
    );
}

#[gpui::test]
fn removing_a_drag_source_before_drop_cannot_reinsert_a_stale_row(cx: &mut TestAppContext) {
    let (view, cx) = mount(cx, Kind::Dnd);
    cx.simulate_mouse_down(
        point(px(10.), px(16.)),
        MouseButton::Left,
        Modifiers::default(),
    );
    cx.simulate_mouse_move(
        point(px(40.), px(20.)),
        Some(MouseButton::Left),
        Modifiers::default(),
    );
    draw(cx);
    view.update(cx, |view, cx| {
        view.values.retain(|value| value.as_str() != "a");
        cx.notify();
    });
    draw(cx);
    cx.simulate_mouse_move(
        point(px(30.), px(60.)),
        Some(MouseButton::Left),
        Modifiers::default(),
    );
    draw(cx);
    cx.simulate_mouse_up(
        point(px(30.), px(60.)),
        MouseButton::Left,
        Modifiers::default(),
    );
    draw(cx);
    assert!(view.read_with(cx, |v, _| v.moves.is_empty()));
    assert!(!view.read_with(cx, |v, _| {
        v.values.iter().any(|value| value.as_str() == "a")
    }));
}
#[gpui::test]
fn table_native_sort_button_toggles_owner_order(cx: &mut TestAppContext) {
    let (view, cx) = mount(cx, Kind::Table);
    cx.simulate_click(point(px(25.), px(18.)), Modifiers::default());
    draw(cx);
    key(cx, "enter");
    assert_eq!(
        view.read_with(cx, |v, _| v.sorts.clone()),
        vec![SortDirection::Ascending, SortDirection::Descending]
    );
    assert_eq!(
        view.read_with(cx, |v, _| v.values.clone()),
        vec![SharedString::from("c"), "b".into(), "a".into()]
    );
}
#[gpui::test]
fn table_row_selection_is_keyboard_accessible_and_skips_disabled(cx: &mut TestAppContext) {
    let (view, cx) = mount(cx, Kind::Table);
    cx.simulate_click(point(px(25.), px(60.)), Modifiers::default());
    draw(cx);
    key(cx, "down");
    key(cx, "enter");
    assert_eq!(view.read_with(cx, |v, _| v.selected.to_string()), "c");
    assert_eq!(view.read_with(cx, |v, _| v.selections.len()), 3);
}
#[gpui::test]
fn virtual_tree_keyboard_selects_enabled_children_and_navigates_parent(cx: &mut TestAppContext) {
    let (view, cx) = mount(cx, Kind::Tree);
    cx.simulate_click(point(px(20.), px(16.)), Modifiers::default());
    draw(cx);
    key(cx, "down");
    assert_eq!(
        view.read_with(cx, |v, cx| v
            .tree
            .read(cx)
            .selected_item()
            .map(|i| i.id.to_string())),
        Some("leaf".into())
    );
    key(cx, "left");
    assert_eq!(
        view.read_with(cx, |v, cx| v
            .tree
            .read(cx)
            .selected_item()
            .map(|i| i.id.to_string())),
        Some("root".into())
    );
    key(cx, "right");
    key(cx, "enter");
    assert_eq!(
        view.read_with(cx, |v, _| v.activations.clone()),
        vec![SharedString::from("leaf")]
    );
    key(cx, "left");
    key(cx, "left");
    assert_eq!(
        view.read_with(cx, |v, cx| v
            .tree
            .read(cx)
            .entry(1)
            .map(|i| i.item().id.to_string())),
        Some("last".into())
    );
}
#[gpui::test]
fn tree_external_native_state_focus_still_skips_disabled_on_arrow(cx: &mut TestAppContext) {
    let (view, cx) = mount(cx, Kind::Tree);
    cx.simulate_click(point(px(20.), px(16.)), Modifiers::default());
    draw(cx);
    let tree = view.read_with(cx, |v, _| v.tree.clone());
    cx.update(|window, cx| tree.update(cx, |state, cx| state.focus(window, cx)));
    key(cx, "down");
    assert_eq!(
        tree.read_with(cx, |state, _| state
            .selected_item()
            .map(|i| i.id.to_string())),
        Some("leaf".into())
    );
}
#[gpui::test]
fn item_native_pointer_and_space_activation_respect_disabled(cx: &mut TestAppContext) {
    let (view, cx) = mount(cx, Kind::Item);
    cx.simulate_click(point(px(20.), px(16.)), Modifiers::default());
    key(cx, "space");
    assert_eq!(view.read_with(cx, |v, _| v.clicks), 2);
    view.update(cx, |v, cx| {
        v.disabled = true;
        cx.notify();
    });
    draw(cx);
    cx.simulate_click(point(px(20.), px(16.)), Modifiers::default());
    key(cx, "enter");
    assert_eq!(view.read_with(cx, |v, _| v.clicks), 2);
}
