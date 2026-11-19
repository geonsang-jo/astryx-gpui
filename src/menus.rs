use std::rc::Rc;

use crate::Theme;
use gpui::{
    AnyElement, App, ElementId, InteractiveElement, IntoElement, MouseButton, ParentElement,
    RenderOnce, Role, SharedString, StatefulInteractiveElement, StyleRefinement, Styled, Window,
    div, prelude::FluentBuilder as _, px,
};
use gpui_base::StyledExt as _;

type Open = Rc<dyn Fn(bool, &mut Window, &mut App)>;
type Select = Rc<dyn Fn(SharedString, &mut Window, &mut App)>;
type Highlight = Rc<dyn Fn(usize, &mut Window, &mut App)>;

/// One uniquely valued menu action. Disabled actions stay visible but cannot activate.
#[derive(Clone, Debug)]
pub struct MenuItem {
    pub value: SharedString,
    pub label: SharedString,
    pub disabled: bool,
    pub shortcut: Option<SharedString>,
}
impl MenuItem {
    pub fn new(value: impl Into<SharedString>, label: impl Into<SharedString>) -> Self {
        Self {
            value: value.into(),
            label: label.into(),
            disabled: false,
            shortcut: None,
        }
    }
    pub fn disabled(mut self, value: bool) -> Self {
        self.disabled = value;
        self
    }
    pub fn shortcut(mut self, value: impl Into<SharedString>) -> Self {
        self.shortcut = Some(value.into());
        self
    }
}

/// Shared controlled action-menu behavior. Open/highlight values belong to the host.
/// Arrows, Home/End and first-character navigation skip disabled entries.
#[derive(IntoElement)]
pub struct Menu {
    id: ElementId,
    theme: Theme,
    items: Vec<MenuItem>,
    open: bool,
    disabled: bool,
    highlighted: Option<usize>,
    trigger: Option<AnyElement>,
    trigger_label: SharedString,
    trigger_name: SharedString,
    mouse_button: MouseButton,
    on_open: Option<Open>,
    on_select: Option<Select>,
    on_highlight: Option<Highlight>,
    style: StyleRefinement,
}
impl Menu {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            theme: Theme::default(),
            items: vec![],
            open: false,
            disabled: false,
            highlighted: None,
            trigger: None,
            trigger_label: "Options".into(),
            trigger_name: "Options".into(),
            mouse_button: MouseButton::Left,
            on_open: None,
            on_select: None,
            on_highlight: None,
            style: StyleRefinement::default(),
        }
    }
    pub fn theme(mut self, theme: Theme) -> Self {
        self.theme = theme;
        self
    }
    pub fn items(mut self, items: Vec<MenuItem>) -> Self {
        self.items = items;
        self
    }
    pub fn open(mut self, open: bool) -> Self {
        self.open = open;
        self
    }
    pub fn disabled(mut self, value: bool) -> Self {
        self.disabled = value;
        self
    }
    pub fn highlighted(mut self, index: usize) -> Self {
        self.highlighted = Some(index);
        self
    }
    pub fn trigger(mut self, trigger: impl IntoElement) -> Self {
        self.trigger = Some(trigger.into_any_element());
        self
    }
    pub fn on_open_change(
        mut self,
        handler: impl Fn(bool, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_open = Some(Rc::new(handler));
        self
    }
    pub fn on_select(
        mut self,
        handler: impl Fn(SharedString, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_select = Some(Rc::new(handler));
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
impl Styled for Menu {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

fn destination(items: &[MenuItem], current: Option<usize>, key: &str) -> Option<usize> {
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
        _ if key.chars().count() == 1 => {
            let needle = key.to_lowercase();
            let start = position.map_or(0, |i| i + 1);
            (0..enabled.len())
                .map(|offset| enabled[(start + offset) % enabled.len()])
                .find(|&i| items[i].label.to_lowercase().starts_with(&needle))
        }
        _ => None,
    }
}

impl RenderOnce for Menu {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let c = self.theme.colors;
        let active = self
            .highlighted
            .filter(|&i| self.items.get(i).is_some_and(|item| !item.disabled))
            .or_else(|| self.items.iter().position(|item| !item.disabled));
        let trigger = self.trigger.unwrap_or_else(|| {
            gpui_base::Button::new((self.id.clone(), "trigger"))
                .accessibility_label(self.trigger_name)
                .disabled(self.disabled)
                .aria_expanded(self.open)
                .h(px(32.))
                .px(px(12.))
                .rounded(px(10.))
                .border_1()
                .border_color(c.border_emphasized)
                .bg(c.surface)
                .text_color(c.text)
                .child(self.trigger_label)
                .into_any_element()
        });
        let disabled = self.disabled;
        let frame_id = self.id.clone();
        let mouse_button = self.mouse_button;
        let shortcut_open = self.on_open.clone();
        let context = self.mouse_button == MouseButton::Right;
        let trigger = div()
            .id((self.id.clone(), "anchor"))
            .child(trigger)
            .on_key_down(move |event, window, cx| {
                if !disabled
                    && context
                    && event.keystroke.key == "f10"
                    && event.keystroke.modifiers.shift
                    && !event.keystroke.modifiers.control
                    && !event.keystroke.modifiers.alt
                    && !event.keystroke.modifiers.platform
                {
                    cx.stop_propagation();
                    if let Some(open) = &shortcut_open {
                        open(true, window, cx);
                    }
                }
            });
        let opened = self.on_open.clone();
        let id = self.id.clone();
        let focus = window
            .use_keyed_state((id.clone(), "menu-focus"), cx, |_, cx| cx.focus_handle())
            .read(cx)
            .clone();
        // Keep the native lifecycle mounted when disabled so sync_open(false)
        // can release its deferred context and restore the trigger's focus.
        let popover = gpui_base::Popover::new(self.id)
            .open(self.open && !disabled)
            .track_focus(&focus)
            .mouse_button(self.mouse_button)
            .offset(px(4.))
            .trigger_with(move |_, _, _| trigger.into_any_element())
            .on_open_change(move |value, window, cx| {
                if let Some(handler) = &opened {
                    handler(*value, window, cx)
                }
            })
            .content(move |_, _, cx| {
                let state = cx.entity().downgrade();
                let confirm_state = state.clone();
                let confirm = self.on_select.clone();
                let confirm_items = self.items.clone();
                let highlight = self.on_highlight.clone();
                let key_items = self.items.clone();
                let mut content = div()
                    .id((id.clone(), "menu"))
                    .role(Role::Menu)
                    .track_focus(&focus)
                    .flex()
                    .flex_col()
                    .w(px(240.))
                    .max_h(px(320.))
                    .overflow_y_scroll()
                    .p(px(4.))
                    .gap(px(2.))
                    .rounded(px(12.))
                    .border_1()
                    .border_color(c.border_emphasized)
                    .bg(c.surface)
                    .text_color(c.text)
                    .text_size(px(14.))
                    .line_height(px(20.))
                    .on_action(move |_: &gpui_base::actions::Confirm, window, cx| {
                        cx.stop_propagation();
                        if let Some(item) = active.and_then(|i| confirm_items.get(i)) {
                            let _ = confirm_state.update(cx, |state, cx| state.dismiss(window, cx));
                            if let Some(callback) = &confirm {
                                callback(item.value.clone(), window, cx);
                            }
                        }
                    })
                    .on_key_down(move |event, window, cx| {
                        if event.keystroke.modifiers != gpui::Modifiers::default() {
                            return;
                        }
                        if let Some(index) = destination(&key_items, active, &event.keystroke.key) {
                            cx.stop_propagation();
                            if let Some(callback) = &highlight {
                                callback(index, window, cx);
                            }
                        }
                    });
                for (index, item) in self.items.into_iter().enumerate() {
                    let callback = self.on_select.clone();
                    let state = state.clone();
                    content = content.child(
                        gpui_base::Button::new((id.clone(), item.value.clone()))
                            .accessibility_label(item.label.clone())
                            .role(Role::MenuItem)
                            .disabled(item.disabled)
                            .focusable(false)
                            .tab_stop(false)
                            .flex()
                            .items_center()
                            .justify_between()
                            .h(px(32.))
                            .px(px(10.))
                            .gap(px(12.))
                            .rounded(px(8.))
                            .bg(if active == Some(index) {
                                c.neutral
                            } else {
                                gpui::transparent_black()
                            })
                            .when(!item.disabled, |el| {
                                el.hover(move |style| style.bg(c.hover))
                            })
                            .when(item.disabled, |el| el.opacity(0.5))
                            .on_click(move |_, window, cx| {
                                cx.stop_propagation();
                                let _ = state.update(cx, |state, cx| state.dismiss(window, cx));
                                if let Some(callback) = &callback {
                                    callback(item.value.clone(), window, cx);
                                }
                            })
                            .child(item.label)
                            .children(item.shortcut.map(|shortcut| {
                                div().text_size(px(12.)).text_color(c.muted).child(shortcut)
                            })),
                    );
                }
                content
            })
            .refine_style(&self.style)
            .into_any_element();
        div()
            .id((frame_id, "menu-frame"))
            .when(disabled, |el| {
                el.opacity(0.5)
                    .capture_any_mouse_down(move |event, _, cx| {
                        if event.button == mouse_button {
                            cx.stop_propagation();
                        }
                    })
                    .capture_action(|_: &gpui_base::actions::Confirm, _, cx| {
                        cx.stop_propagation();
                    })
                    .capture_key_down(|event, _, cx| {
                        if matches!(event.keystroke.key.as_str(), "enter" | "space" | "f10") {
                            cx.stop_propagation();
                        }
                    })
            })
            .child(popover)
            .into_any_element()
    }
}

macro_rules! menu_wrapper {
    ($name:ident,$doc:literal,$button:expr,$label:expr,$name_label:expr) => {
        #[doc=$doc]
        #[derive(IntoElement)]
        pub struct $name(Menu);
        impl $name {
            pub fn new(id: impl Into<ElementId>) -> Self {
                let mut menu = Menu::new(id);
                menu.mouse_button = $button;
                menu.trigger_label = $label.into();
                menu.trigger_name = $name_label.into();
                Self(menu)
            }
            pub fn theme(mut self, value: Theme) -> Self {
                self.0 = self.0.theme(value);
                self
            }
            pub fn items(mut self, value: Vec<MenuItem>) -> Self {
                self.0 = self.0.items(value);
                self
            }
            pub fn open(mut self, value: bool) -> Self {
                self.0 = self.0.open(value);
                self
            }
            pub fn disabled(mut self, value: bool) -> Self {
                self.0 = self.0.disabled(value);
                self
            }
            pub fn highlighted(mut self, value: usize) -> Self {
                self.0 = self.0.highlighted(value);
                self
            }
            pub fn trigger(mut self, value: impl IntoElement) -> Self {
                self.0 = self.0.trigger(value);
                self
            }
            pub fn on_open_change(
                mut self,
                handler: impl Fn(bool, &mut Window, &mut App) + 'static,
            ) -> Self {
                self.0 = self.0.on_open_change(handler);
                self
            }
            pub fn on_select(
                mut self,
                handler: impl Fn(SharedString, &mut Window, &mut App) + 'static,
            ) -> Self {
                self.0 = self.0.on_select(handler);
                self
            }
            pub fn on_highlight(
                mut self,
                handler: impl Fn(usize, &mut Window, &mut App) + 'static,
            ) -> Self {
                self.0 = self.0.on_highlight(handler);
                self
            }
        }
        impl Styled for $name {
            fn style(&mut self) -> &mut StyleRefinement {
                self.0.style()
            }
        }
        impl RenderOnce for $name {
            fn render(self, _: &mut Window, _: &mut App) -> impl IntoElement {
                self.0
            }
        }
    };
}
menu_wrapper!(
    DropdownMenu,
    "A left-activated controlled dropdown action menu.",
    MouseButton::Left,
    "Options",
    "Options"
);
menu_wrapper!(
    ContextMenu,
    "A right-activated action menu; Shift+F10 opens from a focusable trigger.",
    MouseButton::Right,
    "Context actions",
    "Context actions"
);
menu_wrapper!(
    MoreMenu,
    "An overflow menu with a compact ellipsis trigger.",
    MouseButton::Left,
    "⋯",
    "More options"
);
menu_wrapper!(
    NavMenu,
    "A controlled navigation-action menu; the host performs navigation on selection.",
    MouseButton::Left,
    "Navigate",
    "Navigate"
);
