use std::rc::Rc;

use gpui::{
    AnyElement, App, ElementId, FontWeight, InteractiveElement, IntoElement, ParentElement,
    RenderOnce, SharedString, StatefulInteractiveElement, StyleRefinement, Styled, Window, div,
    prelude::FluentBuilder as _, px,
};
use gpui_base::StyledExt as _;

use crate::Theme;

type OpenHandler = Rc<dyn Fn(bool, &mut Window, &mut App)>;

/// A disclosure whose open state belongs to its owning view.
#[derive(IntoElement)]
pub struct Collapsible {
    id: ElementId,
    label: SharedString,
    open: bool,
    disabled: bool,
    theme: Theme,
    on_open_change: Option<OpenHandler>,
    children: Vec<AnyElement>,
    style: StyleRefinement,
}

impl Collapsible {
    pub fn new(id: impl Into<ElementId>, label: impl Into<SharedString>) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            open: false,
            disabled: false,
            theme: Theme::light(),
            on_open_change: None,
            children: Vec::new(),
            style: StyleRefinement::default(),
        }
    }

    pub fn theme(mut self, theme: Theme) -> Self {
        self.theme = theme;
        self
    }

    pub fn open(mut self, open: bool) -> Self {
        self.open = open;
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn on_open_change(
        mut self,
        handler: impl Fn(bool, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_open_change = Some(Rc::new(handler));
        self
    }
}

impl ParentElement for Collapsible {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl Styled for Collapsible {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for Collapsible {
    fn render(self, _: &mut Window, _: &mut App) -> impl IntoElement {
        let c = self.theme.colors;
        let next = !self.open;
        let trigger = gpui_base::Button::new((self.id.clone(), "trigger"))
            .accessibility_label(self.label.clone())
            .aria_expanded(self.open)
            .disabled(self.disabled)
            .w_full()
            .justify_between()
            .py(px(8.))
            .gap(px(8.))
            .text_size(px(17.))
            .line_height(px(24.))
            .font_weight(FontWeight::SEMIBOLD)
            .text_color(c.text)
            .rounded(px(6.))
            .border_1()
            .border_color(gpui::transparent_black())
            .focus_visible(move |style| style.border_color(c.focus))
            .active(move |style| style.bg(c.pressed))
            .styles(|styles| styles.disabled(|style| style.opacity(0.5)))
            .when_some(self.on_open_change, |button, callback| {
                button.on_click(move |_, window, cx| callback(next, window, cx))
            })
            .child(self.label)
            .child(
                div()
                    .text_color(c.faint)
                    .child(if self.open { "⌄" } else { "›" }),
            );

        div()
            .id(self.id)
            .flex()
            .flex_col()
            .min_w_0()
            .child(trigger)
            .when(self.open, |this| {
                this.child(
                    div()
                        .flex()
                        .flex_col()
                        .pt(px(4.))
                        .pb(px(8.))
                        .gap(px(8.))
                        .text_size(px(14.))
                        .line_height(px(20.))
                        .text_color(c.text)
                        .children(self.children),
                )
            })
            .refine_style(&self.style)
    }
}
