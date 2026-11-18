use gpui::{
    App, ElementId, InteractiveElement, IntoElement, ParentElement, RenderOnce, SharedString,
    StyleRefinement, Styled, Window, div, px,
};
use gpui_base::StyledExt as _;

use crate::Theme;

/// An owner-controlled binary switch using native pointer, keyboard and role behavior.
#[derive(IntoElement)]
pub struct Switch {
    base: gpui_base::Switch,
    label: SharedString,
    checked: bool,
    theme: Theme,
    style: StyleRefinement,
}

impl Switch {
    pub fn new(id: impl Into<ElementId>, label: impl Into<SharedString>) -> Self {
        let label = label.into();
        Self {
            base: gpui_base::Switch::new(id).accessibility_label(label.clone()),
            label,
            checked: false,
            theme: Theme::default(),
            style: StyleRefinement::default(),
        }
    }

    pub fn checked(mut self, checked: bool) -> Self {
        self.checked = checked;
        self.base = self.base.checked(checked);
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
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
            .on_change(move |value, _, window, cx| handler(value, window, cx));
        self
    }
}

impl Styled for Switch {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for Switch {
    fn render(self, _: &mut Window, _: &mut App) -> impl IntoElement {
        let c = self.theme.colors;
        self.base
            .flex()
            .items_center()
            .gap(px(8.))
            .py(px(4.))
            .text_size(px(14.))
            .line_height(px(20.))
            .text_color(c.text)
            .border_1()
            .border_color(gpui::transparent_black())
            .rounded(px(6.))
            .focus_visible(move |style| style.border_color(c.focus))
            .styles(|styles| styles.disabled(|style| style.opacity(0.5)))
            .child(
                div()
                    .flex()
                    .flex_none()
                    .items_center()
                    .w(px(32.))
                    .h(px(20.))
                    .p(px(2.))
                    .rounded_full()
                    .bg(if self.checked {
                        c.accent
                    } else {
                        c.border_emphasized
                    })
                    .child(
                        div()
                            .size(px(16.))
                            .rounded_full()
                            .ml(px(if self.checked { 12. } else { 0. }))
                            .bg(if self.checked { c.on_accent } else { c.surface }),
                    ),
            )
            .child(self.label)
            .refine_style(&self.style)
    }
}
