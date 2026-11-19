use crate::Theme;
use gpui::{
    Anchor, AnyElement, App, Bounds, ElementId, IntoElement, ParentElement, Pixels, RenderOnce,
    StyleRefinement, Styled, Window, div, px,
};
pub use gpui_base::ResolvedPosition;
use gpui_base::StyledExt as _;

/// Low-level context layer positioning with native deferred paint and edge snapping.
/// The host owns open state and focus/dismissal policy; use Popover/Dialog when
/// those higher-level interaction policies are desired.
#[derive(IntoElement)]
pub struct Layer {
    base: gpui_base::Popup,
    open: bool,
    children: Vec<AnyElement>,
    theme: Theme,
    style: StyleRefinement,
}
impl Layer {
    pub fn new(id: impl Into<ElementId>, trigger: impl IntoElement) -> Self {
        Self {
            base: gpui_base::Popup::new(id, trigger),
            open: false,
            children: vec![],
            theme: Theme::default(),
            style: StyleRefinement::default(),
        }
    }
    pub fn open(mut self, value: bool) -> Self {
        self.open = value;
        self
    }
    pub fn anchor(mut self, value: Anchor) -> Self {
        self.base = self.base.anchor(value);
        self
    }
    pub fn offset(mut self, pixels: f32) -> Self {
        self.base = self.base.offset(px(pixels));
        self
    }
    pub fn on_position(
        mut self,
        handler: impl Fn(ResolvedPosition, Bounds<Pixels>) + 'static,
    ) -> Self {
        self.base = self.base.on_position(handler);
        self
    }
    pub fn theme(mut self, theme: Theme) -> Self {
        self.theme = theme;
        self
    }
}
impl Styled for Layer {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}
impl ParentElement for Layer {
    fn extend(&mut self, children: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(children)
    }
}
impl RenderOnce for Layer {
    fn render(self, _: &mut Window, _: &mut App) -> impl IntoElement {
        let mut layer = self.base;
        if self.open {
            layer = layer.content(
                div()
                    .text_color(self.theme.colors.text)
                    .text_size(px(14.))
                    .line_height(px(20.))
                    .children(self.children),
            );
        }
        layer.refine_style(&self.style)
    }
}
