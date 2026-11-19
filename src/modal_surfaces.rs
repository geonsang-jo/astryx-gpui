use gpui::{
    AnyElement, App, ElementId, IntoElement, ParentElement, RenderOnce, StyleRefinement, Styled,
    Window, px, relative,
};
use gpui_base::StyledExt as _;

use crate::{Dialog, DialogChangeReason, Theme};

/// The physical edge of a desktop drawer. The host chooses its writing-direction mapping.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum DrawerSide {
    Left,
    #[default]
    Right,
}

/// A full-height modal inspector that overlays the page without reflowing it.
#[derive(IntoElement)]
pub struct Drawer {
    dialog: Dialog,
    side: DrawerSide,
    width: f32,
    style: StyleRefinement,
}

impl Drawer {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            dialog: Dialog::new(id),
            side: DrawerSide::Right,
            width: 360.,
            style: StyleRefinement::default(),
        }
    }
    pub fn side(mut self, side: DrawerSide) -> Self {
        self.side = side;
        self
    }
    pub fn width(mut self, width: f32) -> Self {
        if width.is_finite() {
            self.width = width.max(160.)
        }
        self
    }
    pub fn open(mut self, open: bool) -> Self {
        self.dialog = self.dialog.open(open);
        self
    }
    pub fn theme(mut self, theme: Theme) -> Self {
        self.dialog = self.dialog.theme(theme);
        self
    }
    pub fn title(mut self, title: impl Into<gpui::SharedString>) -> Self {
        self.dialog = self.dialog.title(title);
        self
    }
    pub fn dismissible(mut self, dismissible: bool) -> Self {
        self.dialog = self.dialog.dismissible(dismissible);
        self
    }
    pub fn on_open_change(
        mut self,
        callback: impl Fn(bool, DialogChangeReason, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.dialog = self.dialog.on_open_change(callback);
        self
    }
}
impl Styled for Drawer {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}
impl ParentElement for Drawer {
    fn extend(&mut self, children: impl IntoIterator<Item = AnyElement>) {
        self.dialog.extend(children)
    }
}
impl RenderOnce for Drawer {
    fn render(self, _: &mut Window, _: &mut App) -> impl IntoElement {
        let dialog = self
            .dialog
            .absolute()
            .top_0()
            .bottom_0()
            .h_full()
            .max_h(relative(1.))
            .w(px(self.width))
            .max_w(relative(0.95))
            .rounded(px(0.));
        let dialog = match self.side {
            DrawerSide::Left => dialog.left_0(),
            DrawerSide::Right => dialog.right_0(),
        };
        dialog.refine_style(&self.style)
    }
}

/// A controlled bottom-anchored modal surface. Touch drag/snap motion remains a host concern.
#[derive(IntoElement)]
pub struct BottomSheet {
    dialog: Dialog,
    style: StyleRefinement,
}
impl BottomSheet {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            dialog: Dialog::new(id),
            style: StyleRefinement::default(),
        }
    }
    pub fn open(mut self, open: bool) -> Self {
        self.dialog = self.dialog.open(open);
        self
    }
    pub fn title(mut self, title: impl Into<gpui::SharedString>) -> Self {
        self.dialog = self.dialog.title(title);
        self
    }
    pub fn theme(mut self, theme: Theme) -> Self {
        self.dialog = self.dialog.theme(theme);
        self
    }
    pub fn dismissible(mut self, value: bool) -> Self {
        self.dialog = self.dialog.dismissible(value);
        self
    }
    pub fn on_open_change(
        mut self,
        callback: impl Fn(bool, DialogChangeReason, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.dialog = self.dialog.on_open_change(callback);
        self
    }
}
impl Styled for BottomSheet {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}
impl ParentElement for BottomSheet {
    fn extend(&mut self, children: impl IntoIterator<Item = AnyElement>) {
        self.dialog.extend(children)
    }
}
impl RenderOnce for BottomSheet {
    fn render(self, _: &mut Window, _: &mut App) -> impl IntoElement {
        self.dialog
            .absolute()
            .bottom_0()
            .left_0()
            .right_0()
            .w_full()
            .max_w(relative(1.))
            .max_h(relative(0.85))
            .rounded(px(12.))
            .rounded_bl(px(0.))
            .rounded_br(px(0.))
            .refine_style(&self.style)
    }
}

/// A transparent, owner-controlled modal layer for custom native content.
/// The owner styles/composes its own surface; the layer supplies backdrop, Escape and focus return.
#[derive(IntoElement)]
pub struct Overlay {
    dialog: Dialog,
    style: StyleRefinement,
}
impl Overlay {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            dialog: Dialog::new(id),
            style: StyleRefinement::default(),
        }
    }
    pub fn open(mut self, value: bool) -> Self {
        self.dialog = self.dialog.open(value);
        self
    }
    pub fn dismissible(mut self, value: bool) -> Self {
        self.dialog = self.dialog.dismissible(value);
        self
    }
    pub fn on_open_change(
        mut self,
        callback: impl Fn(bool, DialogChangeReason, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.dialog = self.dialog.on_open_change(callback);
        self
    }
}
impl Styled for Overlay {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}
impl ParentElement for Overlay {
    fn extend(&mut self, children: impl IntoIterator<Item = AnyElement>) {
        self.dialog.extend(children)
    }
}
impl RenderOnce for Overlay {
    fn render(self, _: &mut Window, _: &mut App) -> impl IntoElement {
        self.dialog
            .bg(gpui::transparent_black())
            .border_0()
            .rounded(px(0.))
            .p(px(0.))
            .refine_style(&self.style)
    }
}
