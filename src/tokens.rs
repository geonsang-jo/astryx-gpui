use crate::{InputState, Theme};
use gpui::{
    App, ElementId, Entity, EntityInputHandler as _, Focusable as _, InteractiveElement as _,
    IntoElement, ParentElement, RenderOnce, SharedString, StyleRefinement, Styled, Window, div,
    prelude::FluentBuilder as _, px,
};
use gpui_base::StyledExt as _;
pub use gpui_base::input::{InlineToken, InlineTokenError, InputContent};
use std::rc::Rc;
type Remove = Rc<dyn Fn(&mut Window, &mut App)>;
type Commit = Rc<dyn Fn(InputContent, &mut Window, &mut App)>;

/// A compact selected-value/reference chip with an optional native remove action.
#[derive(IntoElement)]
pub struct Token {
    id: ElementId,
    label: SharedString,
    theme: Theme,
    selected: bool,
    disabled: bool,
    remove: Option<Remove>,
    style: StyleRefinement,
}
impl Token {
    pub fn new(id: impl Into<ElementId>, label: impl Into<SharedString>) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            theme: Theme::default(),
            selected: false,
            disabled: false,
            remove: None,
            style: StyleRefinement::default(),
        }
    }
    pub fn theme(mut self, v: Theme) -> Self {
        self.theme = v;
        self
    }
    pub fn selected(mut self, v: bool) -> Self {
        self.selected = v;
        self
    }
    pub fn disabled(mut self, v: bool) -> Self {
        self.disabled = v;
        self
    }
    pub fn on_remove(mut self, f: impl Fn(&mut Window, &mut App) + 'static) -> Self {
        self.remove = Some(Rc::new(f));
        self
    }
}
impl Styled for Token {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}
impl RenderOnce for Token {
    fn render(self, _: &mut Window, _: &mut App) -> impl IntoElement {
        let c = self.theme.colors;
        div()
            .flex()
            .items_center()
            .gap(px(4.))
            .h(px(22.))
            .px(px(6.))
            .rounded(px(6.))
            .text_size(px(12.))
            .line_height(px(20.))
            .bg(if self.selected { c.accent } else { c.neutral })
            .text_color(if self.selected { c.on_accent } else { c.text })
            .when(self.disabled, |el| el.opacity(0.5))
            .child(self.label.clone())
            .when_some(self.remove, |el, remove| {
                el.child(
                    gpui_base::Button::new((self.id, "remove"))
                        .accessibility_label(format!("Remove {}", self.label))
                        .disabled(self.disabled)
                        .size(px(18.))
                        .rounded(px(4.))
                        .on_click(move |_, w, cx| {
                            cx.stop_propagation();
                            remove(w, cx);
                        })
                        .child("×"),
                )
            })
            .refine_style(&self.style)
    }
}

/// A native text editor whose InlineToken spans render as Astryx chips.
///
/// Values are native InputContent on the caller's InputState. GPUI keeps each
/// token atomic during selection, Backspace/Delete, IME composition and undo.
/// The optional Enter callback requests a commit; the host decides how query
/// text becomes tokens and can replace the content through the same state.
#[derive(IntoElement)]
pub struct Tokenizer {
    state: Entity<InputState>,
    theme: Theme,
    label: Option<SharedString>,
    commit: Option<Commit>,
    style: StyleRefinement,
}
impl Tokenizer {
    pub fn new(state: &Entity<InputState>) -> Self {
        Self {
            state: state.clone(),
            theme: Theme::default(),
            label: None,
            commit: None,
            style: StyleRefinement::default(),
        }
    }
    pub fn theme(mut self, v: Theme) -> Self {
        self.theme = v;
        self
    }
    pub fn label(mut self, v: impl Into<SharedString>) -> Self {
        self.label = Some(v.into());
        self
    }
    pub fn on_commit(mut self, f: impl Fn(InputContent, &mut Window, &mut App) + 'static) -> Self {
        self.commit = Some(Rc::new(f));
        self
    }
}
impl Styled for Tokenizer {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}
impl RenderOnce for Tokenizer {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let c = self.theme.colors;
        let theme = self.theme;
        self.state.update(cx, |s, _| {
            s.set_editor_style(gpui_base::input::InputEditorStyle {
                foreground: c.text,
                muted_foreground: c.faint,
                background: c.surface,
                border: c.border_emphasized,
                caret: c.text,
                selection: gpui::Hsla { a: 0.22, ..c.info },
                ..Default::default()
            })
        });
        let model = self.state.read(cx);
        let presentation = model.presentation();
        let disabled = presentation.is_disabled();
        let editable = presentation.is_editable();
        let focused = model.focus_handle(cx).is_focused(window);
        let focus = presentation.focus_handle().clone();
        let state = self.state.clone();
        let commit = self.commit;
        gpui_base::InputBase::new(("astryx-tokenizer", self.state.entity_id()))
            .w_full()
            .h(px(36.))
            .px(px(10.))
            .rounded(px(10.))
            .border_1()
            .border_color(if focused {
                c.focus
            } else {
                c.border_emphasized
            })
            .bg(c.surface)
            .text_color(c.text)
            .text_size(px(14.))
            .line_height(px(20.))
            .when(disabled, |el| el.opacity(0.5))
            .when(!disabled, |el| {
                el.on_mouse_down(gpui::MouseButton::Left, move |_, w, cx| focus.focus(w, cx))
            })
            .when_some(self.label, |el, label| el.accessibility_label(label))
            .capture_action(move |action: &gpui_base::input::Enter, w, cx| {
                let composing =
                    state.update(cx, |state, cx| state.marked_text_range(w, cx).is_some());
                if editable
                    && !composing
                    && !action.shift
                    && !action.secondary
                    && let Some(commit) = &commit
                {
                    cx.stop_propagation();
                    let content = state.read(cx).content();
                    commit(content, w, cx);
                } else {
                    cx.propagate();
                }
            })
            .child(
                gpui_base::Input::new(&self.state).token(move |context, _, _| {
                    Token::new(
                        context.token().id().clone(),
                        context.token().label().clone(),
                    )
                    .theme(theme)
                    .selected(context.is_selected())
                    .disabled(context.is_disabled())
                }),
            )
            .refine_style(&self.style)
    }
}
