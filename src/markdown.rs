use std::sync::Arc;

use gpui::{
    App, ClickEvent, ElementId, IntoElement, ParentElement, Refineable as _, RenderOnce,
    SharedString, StyleRefinement, Styled, Window, div, prelude::FluentBuilder as _, px,
};
use gpui_base::StyledExt as _;

use crate::{Appearance, Button, ButtonVariant, Theme};

type LinkHandler = Arc<dyn Fn(&SharedString, &ClickEvent, &mut Window, &mut App) + Send + Sync>;

fn rich_style(theme: Theme) -> gpui_base::TextViewStyle {
    let c = theme.colors;
    gpui_base::TextViewStyle::default()
        .with_dark(theme.appearance == Appearance::Dark)
        .with_foreground(c.text)
        .with_muted_foreground(c.muted)
        .with_link(c.info)
        .with_selection(gpui::Hsla { a: 0.22, ..c.info })
        .with_code_background(c.canvas)
        .with_inline_code(gpui::HighlightStyle {
            color: Some(c.text),
            background_color: Some(c.neutral),
            ..Default::default()
        })
        .with_border(c.border)
        .with_heading(move |level| {
            let mut style = div()
                .text_color(c.text)
                .font_weight(gpui::FontWeight::SEMIBOLD)
                .text_size(px(match level {
                    1 => 26.,
                    2 => 22.,
                    _ => 17.,
                }))
                .line_height(px(match level {
                    1 => 32.,
                    2 => 28.,
                    _ => 24.,
                }));
            style.style().clone()
        })
}

/// Selectable native rich text with GPUI's Markdown parser and link activation.
#[derive(IntoElement)]
pub struct Markdown {
    id: ElementId,
    text: SharedString,
    theme: Theme,
    on_link: Option<LinkHandler>,
    style: StyleRefinement,
}
impl Markdown {
    pub fn new(id: impl Into<ElementId>, text: impl Into<SharedString>) -> Self {
        Self {
            id: id.into(),
            text: text.into(),
            theme: Theme::default(),
            on_link: None,
            style: StyleRefinement::default(),
        }
    }
    pub fn theme(mut self, theme: Theme) -> Self {
        self.theme = theme;
        self
    }
    /// Replaces native external URL opening with the host's routing strategy.
    pub fn on_link(
        mut self,
        handler: impl Fn(&SharedString, &ClickEvent, &mut Window, &mut App) + Send + Sync + 'static,
    ) -> Self {
        self.on_link = Some(Arc::new(handler));
        self
    }
}
impl Styled for Markdown {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}
impl RenderOnce for Markdown {
    fn render(self, _: &mut Window, _: &mut App) -> impl IntoElement {
        let mut view = gpui_base::TextView::markdown(self.id, self.text)
            .style(rich_style(self.theme))
            .text_size(px(14.))
            .line_height(px(20.));
        if let Some(handler) = self.on_link {
            view =
                view.on_link_click(move |url, event, window, cx| handler(url, event, window, cx));
        }
        view.refine_style(&self.style)
    }
}

/// A selectable fenced code block with an explicit native clipboard action.
#[derive(IntoElement)]
pub struct CodeBlock {
    id: ElementId,
    code: SharedString,
    language: SharedString,
    copy: bool,
    theme: Theme,
    style: StyleRefinement,
}
impl CodeBlock {
    pub fn new(id: impl Into<ElementId>, code: impl Into<SharedString>) -> Self {
        Self {
            id: id.into(),
            code: code.into(),
            language: "text".into(),
            copy: true,
            theme: Theme::default(),
            style: StyleRefinement::default(),
        }
    }
    pub fn language(mut self, language: impl Into<SharedString>) -> Self {
        self.language = language.into();
        self
    }
    pub fn copy_button(mut self, visible: bool) -> Self {
        self.copy = visible;
        self
    }
    pub fn theme(mut self, theme: Theme) -> Self {
        self.theme = theme;
        self
    }
}
impl Styled for CodeBlock {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}
fn fenced(code: &str, language: &str) -> String {
    let longest = code
        .split(|character| character != '`')
        .map(str::len)
        .max()
        .unwrap_or(0);
    let fence = "`".repeat((longest + 1).max(3));
    let language: String = language
        .chars()
        .filter(|c| c.is_ascii_alphanumeric() || matches!(c, '_' | '-' | '+' | '.'))
        .collect();
    format!("{fence}{language}\n{code}\n{fence}")
}
impl RenderOnce for CodeBlock {
    fn render(self, _: &mut Window, _: &mut App) -> impl IntoElement {
        let text = fenced(&self.code, &self.language);
        let theme = self.theme;
        let code = self.code;
        let c = theme.colors;
        let mut root = div()
            .flex()
            .flex_col()
            .min_w_0()
            .gap(px(8.))
            .p(px(12.))
            .rounded(px(10.))
            .border_1()
            .border_color(c.border)
            .bg(c.canvas)
            .text_color(c.text);
        root.style().refine(&self.style);
        root.when(self.copy, |el| {
            el.child(
                div()
                    .flex()
                    .justify_between()
                    .items_center()
                    .child(
                        div()
                            .text_size(px(12.))
                            .line_height(px(20.))
                            .text_color(c.muted)
                            .child(self.language),
                    )
                    .child(
                        Button::new((self.id.clone(), "copy"), "Copy")
                            .theme(theme)
                            .variant(ButtonVariant::Ghost)
                            .on_click(move |_, _, cx| {
                                cx.write_to_clipboard(gpui::ClipboardItem::new_string(
                                    code.to_string(),
                                ))
                            }),
                    ),
            )
        })
        .child(Markdown::new((self.id, "content"), text).theme(theme))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn fencing_preserves_untrusted_code_as_code() {
        let result = fenced("```\n[link](https://example.test)\n```", "rs\n<script>");
        assert!(result.starts_with("````rsscript\n```"));
        assert!(result.ends_with("\n````"));
    }
}
