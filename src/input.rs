use gpui::{
    App, ElementId, Entity, Focusable as _, InteractiveElement as _, IntoElement, ParentElement,
    RenderOnce, SharedString, StyleRefinement, Styled, Window, prelude::FluentBuilder as _, px,
};
use gpui_base::{InputBase, StyledExt as _};

use crate::Theme;

pub use gpui_base::input::{InputEvent, InputState, TextareaState};

/// A single-line Astryx frame around a caller-owned native editing state.
///
/// Configure the value, placeholder, disabled and read-only flags on `InputState`.
/// Rebuilding this wrapper never replaces that state or its composition/history.
#[derive(IntoElement)]
pub struct Input {
    id: Option<ElementId>,
    state: Entity<InputState>,
    theme: Theme,
    invalid: bool,
    disabled: Option<bool>,
    label: Option<SharedString>,
    style: StyleRefinement,
}

/// A multi-line Astryx frame around a caller-owned native text-area state.
///
/// Configure rows, value, placeholder, disabled and read-only flags on
/// `TextareaState`. The native editing engine owns IME, selection and history.
#[derive(IntoElement)]
pub struct TextArea {
    id: Option<ElementId>,
    state: Entity<TextareaState>,
    theme: Theme,
    invalid: bool,
    disabled: Option<bool>,
    label: Option<SharedString>,
    style: StyleRefinement,
}

macro_rules! presentation {
    ($name:ident, $state:ty, $native:ty, $height:expr) => {
        impl $name {
            pub fn new(state: &Entity<$state>) -> Self {
                Self {
                    id: None,
                    state: state.clone(),
                    theme: Theme::light(),
                    invalid: false,
                    disabled: None,
                    label: None,
                    style: StyleRefinement::default(),
                }
            }

            pub fn theme(mut self, theme: Theme) -> Self {
                self.theme = theme;
                self
            }
            /// Uses the host's stable identity for the actual input frame.
            pub fn id(mut self, id: impl Into<ElementId>) -> Self {
                self.id = Some(id.into());
                self
            }
            pub fn invalid(mut self, invalid: bool) -> Self {
                self.invalid = invalid;
                self
            }
            /// Overrides the native state's disabled flag. When omitted, the
            /// caller's state remains authoritative. This does not replace its
            /// text, composition, selection, or undo history.
            pub fn disabled(mut self, disabled: bool) -> Self {
                self.disabled = Some(disabled);
                self
            }
            pub fn label(mut self, label: impl Into<SharedString>) -> Self {
                self.label = Some(label.into());
                self
            }
        }

        impl Styled for $name {
            fn style(&mut self) -> &mut StyleRefinement {
                &mut self.style
            }
        }

        impl RenderOnce for $name {
            fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
                let c = self.theme.colors;
                self.state.update(cx, |state, cx| {
                    if let Some(disabled) = self.disabled
                        && state.presentation().is_disabled() != disabled
                    {
                        state.set_disabled(disabled, cx);
                    }
                    state.set_editor_style(gpui_base::input::InputEditorStyle {
                        foreground: c.text,
                        muted_foreground: c.faint,
                        background: c.surface,
                        border: c.border_emphasized,
                        caret: c.text,
                        selection: gpui::Hsla { a: 0.22, ..c.info },
                        ..Default::default()
                    });
                });
                let state = self.state.read(cx);
                let presentation = state.presentation();
                let disabled = presentation.is_disabled();
                let focused = state.focus_handle(cx).is_focused(window);
                let handle = presentation.focus_handle().clone();
                InputBase::new(
                    self.id
                        .unwrap_or_else(|| ("astryx-input", self.state.entity_id()).into()),
                )
                .w_full()
                .min_w(px(0.))
                .h(px($height))
                .px(px(12.))
                .rounded(px(10.))
                .border_1()
                .border_color(if self.invalid {
                    c.danger
                } else {
                    c.border_emphasized
                })
                .bg(c.surface)
                .text_color(c.text)
                .text_size(px(14.))
                .line_height(px(20.))
                .focused(focused)
                .disabled(disabled)
                .when(!disabled, |frame| {
                    frame
                        .cursor_text()
                        .on_mouse_down(gpui::MouseButton::Left, move |_, window, cx| {
                            handle.focus(window, cx)
                        })
                })
                .styles(|styles| {
                    styles
                        .focused(|style| {
                            style.border_color(if self.invalid { c.danger } else { c.focus })
                        })
                        .disabled(|style| style.opacity(0.5))
                })
                .when_some(self.label, |frame, label| frame.accessibility_label(label))
                .child(<$native>::new(&self.state))
                .refine_style(&self.style)
            }
        }
    };
}

presentation!(Input, InputState, gpui_base::Input, 36.);
presentation!(TextArea, TextareaState, gpui_base::Textarea, 100.);
