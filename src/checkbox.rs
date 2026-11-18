use gpui::{
    App, ElementId, FontWeight, InteractiveElement, IntoElement, ParentElement, RenderOnce,
    SharedString, StyleRefinement, Styled, Window, div, px,
};
use gpui_base::{CheckboxState, StyledExt as _};

use crate::Theme;

/// A caller-controlled checkbox with a native semantic toggle and focus target.
#[derive(IntoElement)]
pub struct Checkbox {
    base: gpui_base::Checkbox,
    label: SharedString,
    theme: Theme,
    checked: bool,
    style: StyleRefinement,
}

impl Checkbox {
    pub fn new(id: impl Into<ElementId>, label: impl Into<SharedString>) -> Self {
        let label = label.into();
        Self {
            base: gpui_base::Checkbox::new(id).accessibility_label(label.clone()),
            label,
            theme: Theme::light(),
            checked: false,
            style: StyleRefinement::default(),
        }
    }

    pub fn theme(mut self, theme: Theme) -> Self {
        self.theme = theme;
        self
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

    pub fn on_change(mut self, handler: impl Fn(bool, &mut Window, &mut App) + 'static) -> Self {
        self.base = self.base.on_change(move |state, _, window, cx| {
            handler(state == CheckboxState::Checked, window, cx);
        });
        self
    }
}

impl Styled for Checkbox {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for Checkbox {
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
            .rounded(px(6.))
            .border_1()
            .border_color(gpui::transparent_black())
            .cursor_pointer()
            .focus_visible(move |style| style.border_color(c.focus))
            .styles(|styles| styles.disabled(|style| style.opacity(0.5)))
            .child(
                div()
                    .flex()
                    .flex_none()
                    .items_center()
                    .justify_center()
                    .size(px(18.))
                    .rounded(px(6.))
                    .border_1()
                    .border_color(if self.checked {
                        c.accent
                    } else {
                        c.border_emphasized
                    })
                    .bg(if self.checked { c.accent } else { c.surface })
                    .text_color(c.on_accent)
                    .text_size(px(13.))
                    .line_height(px(16.))
                    .font_weight(FontWeight::SEMIBOLD)
                    .child(if self.checked { "✓" } else { "" }),
            )
            .child(self.label)
            .refine_style(&self.style)
    }
}
