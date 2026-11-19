use crate::{Theme, Tone};
use gpui::{
    AnyElement, App, ElementId, IntoElement, ParentElement, RenderOnce, SharedString,
    StyleRefinement, Styled, Window, div, px,
};
use gpui_base::StyledExt as _;
use std::rc::Rc;

pub use gpui_base::{
    ToastAdvance, ToastManager, ToastMotion, ToastOptions, ToastStackState, ToastTransitionStatus,
};
type Dismiss = Rc<dyn Fn(&mut Window, &mut App)>;

/// A semantic native toast. A host-owned ToastManager supplies timers, pause and removal.
#[derive(IntoElement)]
pub struct Toast {
    id: ElementId,
    title: SharedString,
    description: Option<SharedString>,
    theme: Theme,
    tone: Tone,
    status: ToastTransitionStatus,
    on_dismiss: Option<Dismiss>,
    children: Vec<AnyElement>,
    style: StyleRefinement,
}
impl Toast {
    pub fn new(id: impl Into<ElementId>, title: impl Into<SharedString>) -> Self {
        Self {
            id: id.into(),
            title: title.into(),
            description: None,
            theme: Theme::default(),
            tone: Tone::Neutral,
            status: ToastTransitionStatus::Present,
            on_dismiss: None,
            children: vec![],
            style: StyleRefinement::default(),
        }
    }
    pub fn description(mut self, text: impl Into<SharedString>) -> Self {
        self.description = Some(text.into());
        self
    }
    pub fn theme(mut self, theme: Theme) -> Self {
        self.theme = theme;
        self
    }
    pub fn tone(mut self, tone: Tone) -> Self {
        self.tone = tone;
        self
    }
    pub fn transition_status(mut self, status: ToastTransitionStatus) -> Self {
        self.status = status;
        self
    }
    pub fn on_dismiss(mut self, callback: impl Fn(&mut Window, &mut App) + 'static) -> Self {
        self.on_dismiss = Some(Rc::new(callback));
        self
    }
}
impl Styled for Toast {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}
impl ParentElement for Toast {
    fn extend(&mut self, children: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(children)
    }
}
impl RenderOnce for Toast {
    fn render(self, _: &mut Window, _: &mut App) -> impl IntoElement {
        let c = self.theme.colors;
        let tint = match self.tone {
            Tone::Neutral => c.text,
            Tone::Success => c.success,
            Tone::Warning => c.warning,
            Tone::Danger => c.danger,
            Tone::Info => c.info,
        };
        let title = div()
            .flex_1()
            .min_w_0()
            .font_weight(gpui::FontWeight::SEMIBOLD)
            .text_color(tint)
            .child(self.title);
        gpui_base::Toast::new(self.id.clone())
            .transition_status(self.status)
            .flex()
            .flex_col()
            .w(px(360.))
            .max_w(px(480.))
            .p(px(12.))
            .gap(px(8.))
            .rounded(px(12.))
            .border_1()
            .border_color(c.border_emphasized)
            .bg(c.surface)
            .text_color(c.text)
            .text_size(px(14.))
            .line_height(px(20.))
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap(px(8.))
                    .child(title)
                    .children(self.on_dismiss.map(|dismiss| {
                        gpui_base::Button::new((self.id, "dismiss"))
                            .accessibility_label("Dismiss notification")
                            .size(px(24.))
                            .rounded(px(6.))
                            .text_color(c.muted)
                            .on_click(move |_, window, cx| dismiss(window, cx))
                            .child("×")
                    })),
            )
            .children(
                self.description
                    .map(|text| div().text_color(c.muted).child(text)),
            )
            .children(self.children)
            .refine_style(&self.style)
    }
}
