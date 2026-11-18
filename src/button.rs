use gpui::{
    AnyElement, AnyView, App, ClickEvent, ElementId, FocusHandle, FontWeight, Hsla,
    InteractiveElement, IntoElement, ParentElement, RenderOnce, Rgba, SharedString,
    StatefulInteractiveElement, StyleRefinement, Styled, Window, prelude::FluentBuilder as _, px,
};
use gpui_base::StyledExt as _;

use crate::{ControlSize, Theme};

/// Astryx's button treatments, independent of the native activation behavior.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum ButtonVariant {
    #[default]
    Primary,
    Secondary,
    Outline,
    Ghost,
}

/// A styled native button. Enter and Space activation are supplied by GPUI.
#[derive(IntoElement)]
pub struct Button {
    base: gpui_base::Button,
    label: SharedString,
    theme: Theme,
    variant: ButtonVariant,
    size: ControlSize,
    selected: bool,
    disabled: bool,
    leading: Option<AnyElement>,
    trailing: Option<AnyElement>,
    style: StyleRefinement,
}

impl Button {
    pub fn new(id: impl Into<ElementId>, label: impl Into<SharedString>) -> Self {
        let label = label.into();
        Self {
            base: gpui_base::Button::new(id).accessibility_label(label.clone()),
            label,
            theme: Theme::light(),
            variant: ButtonVariant::Primary,
            size: ControlSize::Medium,
            selected: false,
            disabled: false,
            leading: None,
            trailing: None,
            style: StyleRefinement::default(),
        }
    }

    pub fn theme(mut self, theme: Theme) -> Self {
        self.theme = theme;
        self
    }

    pub fn variant(mut self, variant: ButtonVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn size(mut self, size: ControlSize) -> Self {
        self.size = size;
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self.base = self.base.disabled(disabled);
        self
    }

    pub fn selected(mut self, selected: bool) -> Self {
        self.selected = selected;
        self.base = self.base.selected(selected);
        self
    }

    /// Overrides the accessible name, for example when the visible label is empty.
    pub fn accessibility_label(mut self, label: impl Into<SharedString>) -> Self {
        self.base = self.base.accessibility_label(label);
        self
    }

    /// Uses a caller-owned focus target instead of keyed internal button focus.
    pub fn focus_handle(mut self, handle: &FocusHandle) -> Self {
        self.base = self.base.track_focus(handle);
        self
    }

    /// Adds caller-supplied native content before the label (for example an icon).
    pub fn leading(mut self, content: impl IntoElement) -> Self {
        self.leading = Some(content.into_any_element());
        self
    }

    /// Adds caller-supplied native content after the label.
    pub fn trailing(mut self, content: impl IntoElement) -> Self {
        self.trailing = Some(content.into_any_element());
        self
    }

    /// Uses GPUI's native tooltip lifecycle with caller-supplied content.
    pub fn tooltip(mut self, builder: impl Fn(&mut Window, &mut App) -> AnyView + 'static) -> Self {
        self.base = self.base.tooltip(builder);
        self
    }

    pub fn on_click(
        mut self,
        handler: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.base = self.base.on_click(handler);
        self
    }
}

impl Styled for Button {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

// Source-over preserves translucent Neutral/ghost fills without assuming the
// host's background. GPUI's color blend keeps the base alpha unchanged.
fn overlay(base: Hsla, tint: Hsla) -> Hsla {
    let base = Rgba::from(base);
    let tint = Rgba::from(tint);
    let alpha = tint.a + base.a * (1. - tint.a);
    if alpha == 0. {
        return gpui::transparent_black();
    }
    let channel = |base_channel: f32, tint_channel: f32| {
        (tint_channel * tint.a + base_channel * base.a * (1. - tint.a)) / alpha
    };
    Rgba {
        r: channel(base.r, tint.r),
        g: channel(base.g, tint.g),
        b: channel(base.b, tint.b),
        a: alpha,
    }
    .into()
}

impl RenderOnce for Button {
    fn render(self, _: &mut Window, _: &mut App) -> impl IntoElement {
        let c = self.theme.colors;
        let primary = self.variant == ButtonVariant::Primary;
        let fill = if primary {
            c.accent
        } else if self.variant == ButtonVariant::Secondary || self.selected {
            c.neutral
        } else {
            gpui::transparent_black()
        };
        // Astryx uses layered interaction overlays. On solid primary fills,
        // use the contrasting on-accent tone at the theme's overlay strengths.
        let interaction = |tint: Hsla| {
            if primary {
                Hsla {
                    a: tint.a,
                    ..c.on_accent
                }
            } else {
                tint
            }
        };
        let hover = overlay(fill, interaction(c.hover));
        let pressed = overlay(fill, interaction(c.pressed));
        self.base
            .flex_none()
            .h(px(self.size.height()))
            .px(px(12.))
            .gap(px(8.))
            .rounded(px(10.))
            .border_1()
            .border_color(if self.variant == ButtonVariant::Outline {
                c.border_emphasized
            } else {
                gpui::transparent_black()
            })
            .bg(fill)
            .text_color(if primary { c.on_accent } else { c.text })
            .text_size(px(14.))
            .line_height(px(20.))
            .font_weight(FontWeight::MEDIUM)
            .when(!self.disabled, |button| {
                button
                    .cursor_pointer()
                    .hover(move |style| style.bg(hover))
                    .active(move |style| style.bg(pressed))
            })
            .when(self.disabled, |button| button.cursor_default())
            // Primary's neutral focus token equals its fill. An inset
            // on-accent border keeps keyboard focus visible on that surface.
            .focus_visible(move |style| {
                style
                    .border_color(if primary { c.on_accent } else { c.focus })
                    .border_2()
            })
            .styles(|styles| styles.disabled(|style| style.opacity(0.5)))
            .children(self.leading)
            .when(!self.label.is_empty(), |button| button.child(self.label))
            .children(self.trailing)
            .refine_style(&self.style)
    }
}
