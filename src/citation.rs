use crate::{Theme, Tooltip};
use gpui::{
    App, ClickEvent, ElementId, IntoElement, ParentElement, RenderOnce, SharedString,
    StatefulInteractiveElement as _, StyleRefinement, Styled, Window, px,
};
use gpui_base::StyledExt as _;
use std::rc::Rc;
type Open = Rc<dyn Fn(&str, &ClickEvent, &mut Window, &mut App)>;
/// A numbered/source citation with native link semantics and a source tooltip.
#[derive(IntoElement)]
pub struct Citation {
    base: gpui_base::Link,
    label: SharedString,
    source: Option<SharedString>,
    theme: Theme,
    on_open: Option<Open>,
    style: StyleRefinement,
}
impl Citation {
    pub fn new(
        id: impl Into<ElementId>,
        label: impl Into<SharedString>,
        href: impl Into<SharedString>,
    ) -> Self {
        let label = label.into();
        Self {
            base: gpui_base::Link::new(id)
                .accessibility_label(label.clone())
                .href(href),
            label,
            source: None,
            theme: Theme::default(),
            on_open: None,
            style: StyleRefinement::default(),
        }
    }
    pub fn source(mut self, source: impl Into<SharedString>) -> Self {
        self.source = Some(source.into());
        self
    }
    pub fn on_open(
        mut self,
        handler: impl Fn(&str, &ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_open = Some(Rc::new(handler));
        self
    }
    pub fn theme(mut self, theme: Theme) -> Self {
        self.theme = theme;
        self
    }
}
impl Styled for Citation {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}
impl RenderOnce for Citation {
    fn render(self, _: &mut Window, _: &mut App) -> impl IntoElement {
        let theme = self.theme;
        let callback = self.on_open;
        let mut base = self
            .base
            .open_with(move |url, event, window, cx| {
                if let Some(handler) = &callback {
                    handler(url, event, window, cx);
                } else {
                    cx.open_url(url);
                }
            })
            .flex_none()
            .px(px(4.))
            .py(px(1.))
            .rounded(px(4.))
            .bg(theme.colors.neutral)
            .text_color(theme.colors.info)
            .text_size(px(12.))
            .line_height(px(16.));
        if let Some(source) = self.source {
            base = base.tooltip(Tooltip::with_theme(source, theme));
        }
        base.child(self.label).refine_style(&self.style)
    }
}
