use crate::{Button, ButtonVariant, Theme};
use gpui::{
    App, ElementId, IntoElement, ParentElement, PathPromptOptions, RenderOnce, SharedString,
    StyleRefinement, Styled, Window, div, prelude::FluentBuilder as _, px,
};
use gpui_base::StyledExt as _;
use std::{path::PathBuf, rc::Rc};
type Change = Rc<dyn Fn(Vec<PathBuf>, &mut Window, &mut App)>;
type Error = Rc<dyn Fn(SharedString, &mut Window, &mut App)>;

/// A native file chooser and owner-controlled selection list. Selecting paths
/// never reads or uploads file contents. Cancellation leaves the value intact.
#[derive(IntoElement)]
pub struct FileInput {
    id: ElementId,
    files: Vec<PathBuf>,
    multiple: bool,
    disabled: bool,
    theme: Theme,
    change: Option<Change>,
    error: Option<Error>,
    style: StyleRefinement,
}
impl FileInput {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            files: vec![],
            multiple: false,
            disabled: false,
            theme: Theme::default(),
            change: None,
            error: None,
            style: StyleRefinement::default(),
        }
    }
    pub fn files(mut self, files: Vec<PathBuf>) -> Self {
        self.files = files;
        self
    }
    pub fn multiple(mut self, multiple: bool) -> Self {
        self.multiple = multiple;
        self
    }
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
    pub fn theme(mut self, theme: Theme) -> Self {
        self.theme = theme;
        self
    }
    pub fn on_change(mut self, f: impl Fn(Vec<PathBuf>, &mut Window, &mut App) + 'static) -> Self {
        self.change = Some(Rc::new(f));
        self
    }
    pub fn on_error(mut self, f: impl Fn(SharedString, &mut Window, &mut App) + 'static) -> Self {
        self.error = Some(Rc::new(f));
        self
    }
}
impl Styled for FileInput {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}
impl RenderOnce for FileInput {
    fn render(self, _: &mut Window, _: &mut App) -> impl IntoElement {
        let c = self.theme.colors;
        let change = self.change.clone();
        let error = self.error;
        let multiple = self.multiple;
        let mut root = div().flex().flex_col().gap(px(8.)).child(
            Button::new(
                (self.id.clone(), "browse"),
                if multiple {
                    "Choose files…"
                } else {
                    "Choose file…"
                },
            )
            .theme(self.theme)
            .variant(ButtonVariant::Outline)
            .disabled(self.disabled)
            .on_click(move |_, window, cx| {
                let receiver = cx.prompt_for_paths(PathPromptOptions {
                    files: true,
                    directories: false,
                    multiple,
                    prompt: Some("Choose files".into()),
                });
                let handle = window.window_handle();
                let change = change.clone();
                let error = error.clone();
                cx.spawn(async move |cx| match receiver.await {
                    Ok(Ok(Some(paths))) => {
                        let _ = handle.update(cx, |_, window, cx| {
                            if let Some(change) = change {
                                change(paths, window, cx);
                            }
                        });
                    }
                    Ok(Ok(None)) => {}
                    _ => {
                        let _ = handle.update(cx, |_, window, cx| {
                            if let Some(error) = error {
                                error(
                                    "The native file chooser could not be opened.".into(),
                                    window,
                                    cx,
                                );
                            }
                        });
                    }
                })
                .detach();
            }),
        );
        for (index, path) in self.files.iter().enumerate() {
            let value = self.files.clone();
            let change = self.change.clone();
            let name = path
                .file_name()
                .unwrap_or(path.as_os_str())
                .to_string_lossy()
                .to_string();
            root = root.child(
                div()
                    .flex()
                    .items_center()
                    .gap(px(8.))
                    .text_size(px(14.))
                    .line_height(px(20.))
                    .text_color(c.text)
                    .child(
                        div()
                            .flex_1()
                            .min_w(px(0.))
                            .text_ellipsis()
                            .child(name.clone()),
                    )
                    .child(
                        Button::new((self.id.clone(), index.to_string()), "×")
                            .accessibility_label(format!("Remove {name}"))
                            .theme(self.theme)
                            .variant(ButtonVariant::Ghost)
                            .disabled(self.disabled)
                            .on_click(move |_, w, cx| {
                                let mut next = value.clone();
                                next.remove(index);
                                if let Some(change) = &change {
                                    change(next, w, cx);
                                }
                            }),
                    ),
            );
        }
        root.when(self.files.is_empty(), |el| {
            el.child(
                div()
                    .text_size(px(12.))
                    .text_color(c.muted)
                    .child("No file selected"),
            )
        })
        .refine_style(&self.style)
    }
}
