use std::time::{Duration, Instant};

use crate::{Theme, Tooltip};
use chrono::{DateTime, FixedOffset, Utc};
use gpui::{
    App, Context, ElementId, Entity, InteractiveElement, IntoElement, ParentElement,
    Refineable as _, RenderOnce, StatefulInteractiveElement as _, StyleRefinement, Styled, Task,
    Window, div, px,
};

fn relative_label(value: DateTime<Utc>, now: DateTime<Utc>) -> String {
    let seconds = now.signed_duration_since(value).num_seconds();
    let magnitude = seconds.unsigned_abs();
    let text = if magnitude < 60 {
        "just now".to_string()
    } else if magnitude < 3600 {
        format!("{}m", magnitude / 60)
    } else if magnitude < 86400 {
        format!("{}h", magnitude / 3600)
    } else {
        format!("{}d", magnitude / 86400)
    };
    if magnitude < 60 {
        text
    } else if seconds < 0 {
        format!("in {text}")
    } else {
        format!("{text} ago")
    }
}

/// An absolute or relative timestamp with an absolute native hover tooltip.
/// Pass `relative_to` on host clock updates for deterministic/live relative labels.
#[derive(IntoElement)]
pub struct Timestamp {
    id: ElementId,
    value: DateTime<Utc>,
    now: Option<DateTime<Utc>>,
    offset: FixedOffset,
    theme: Theme,
    style: StyleRefinement,
}
impl Timestamp {
    pub fn new(id: impl Into<ElementId>, value: DateTime<Utc>) -> Self {
        Self {
            id: id.into(),
            value,
            now: None,
            offset: FixedOffset::east_opt(0).expect("UTC offset"),
            theme: Theme::default(),
            style: StyleRefinement::default(),
        }
    }
    pub fn relative_to(mut self, now: DateTime<Utc>) -> Self {
        self.now = Some(now);
        self
    }
    pub fn timezone(mut self, offset: FixedOffset) -> Self {
        self.offset = offset;
        self
    }
    pub fn theme(mut self, theme: Theme) -> Self {
        self.theme = theme;
        self
    }
}
impl Styled for Timestamp {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}
impl RenderOnce for Timestamp {
    fn render(self, _: &mut Window, _: &mut App) -> impl IntoElement {
        let absolute = self
            .value
            .with_timezone(&self.offset)
            .format("%Y-%m-%d %H:%M:%S %:z")
            .to_string();
        let label = self
            .now
            .map(|now| relative_label(self.value, now))
            .unwrap_or_else(|| absolute.clone());
        let mut root = div()
            .id(self.id)
            .role(gpui::Role::Label)
            .aria_label(absolute.clone())
            .text_color(self.theme.colors.muted)
            .text_size(px(12.))
            .line_height(px(20.))
            .tooltip(Tooltip::with_theme(absolute, self.theme));
        root.style().refine(&self.style);
        root.child(label)
    }
}

/// Caller-owned elapsed-time model. A ticker starts only from an explicit start,
/// stops on pause/drop, and never performs provider/network work.
pub struct TimerState {
    elapsed: Duration,
    since: Option<Instant>,
    tick: Option<Task<()>>,
}
impl TimerState {
    pub fn new() -> Self {
        Self {
            elapsed: Duration::ZERO,
            since: None,
            tick: None,
        }
    }
    pub fn is_running(&self) -> bool {
        self.since.is_some()
    }
    pub fn elapsed_at(&self, now: Instant) -> Duration {
        self.elapsed
            + self
                .since
                .map(|since| now.saturating_duration_since(since))
                .unwrap_or_default()
    }
    pub fn elapsed(&self) -> Duration {
        self.elapsed_at(Instant::now())
    }
    pub fn start(&mut self, cx: &mut Context<Self>) {
        if self.since.is_none() {
            self.since = Some(Instant::now());
            self.schedule(cx);
            cx.notify();
        }
    }
    pub fn pause(&mut self, cx: &mut Context<Self>) {
        self.elapsed = self.elapsed();
        self.since = None;
        self.tick = None;
        cx.notify();
    }
    pub fn reset(&mut self, cx: &mut Context<Self>) {
        self.elapsed = Duration::ZERO;
        if self.since.is_some() {
            self.since = Some(Instant::now());
        }
        cx.notify();
    }
    fn schedule(&mut self, cx: &mut Context<Self>) {
        self.tick = Some(cx.spawn(async move |this, cx| {
            cx.background_executor().timer(Duration::from_secs(1)).await;
            if let Some(this) = this.upgrade() {
                this.update(cx, |state, cx| {
                    if state.is_running() {
                        cx.notify();
                        state.schedule(cx);
                    }
                });
            }
        }));
    }
}
impl Default for TimerState {
    fn default() -> Self {
        Self::new()
    }
}

fn timer_label(elapsed: Duration) -> String {
    let seconds = elapsed.as_secs();
    format!(
        "{:02}:{:02}:{:02}",
        seconds / 3600,
        (seconds / 60) % 60,
        seconds % 60
    )
}
/// Elapsed time from a retained native state. Observe that state in the owning view
/// to redraw on its one-second ticker, just as with native Calendar/Tree entities.
#[derive(IntoElement)]
pub struct Timer {
    id: ElementId,
    state: Entity<TimerState>,
    theme: Theme,
    style: StyleRefinement,
}
impl Timer {
    pub fn new(id: impl Into<ElementId>, state: &Entity<TimerState>) -> Self {
        Self {
            id: id.into(),
            state: state.clone(),
            theme: Theme::default(),
            style: StyleRefinement::default(),
        }
    }
    pub fn theme(mut self, theme: Theme) -> Self {
        self.theme = theme;
        self
    }
}
impl Styled for Timer {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}
impl RenderOnce for Timer {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let label = timer_label(self.state.read(cx).elapsed());
        let mut root = div()
            .id(self.id)
            .role(gpui::Role::Label)
            .aria_label(format!("Elapsed time {label}"))
            .font_family("monospace")
            .text_color(self.theme.colors.text)
            .text_size(px(14.))
            .line_height(px(20.));
        root.style().refine(&self.style);
        root.child(label)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn elapsed_accumulates_once_and_formats_without_wrapping_hours() {
        let start = Instant::now();
        let state = TimerState {
            elapsed: Duration::from_secs(90),
            since: Some(start),
            tick: None,
        };
        assert_eq!(
            state.elapsed_at(start + Duration::from_secs(10)),
            Duration::from_secs(100)
        );
        assert_eq!(timer_label(Duration::from_secs(90061)), "25:01:01");
    }
    #[test]
    fn relative_dates_distinguish_future_and_past() {
        let now: DateTime<Utc> = "2026-11-18T00:00:00Z".parse().unwrap();
        assert_eq!(
            relative_label(now + chrono::Duration::minutes(3), now),
            "in 3m"
        );
        assert_eq!(
            relative_label(now - chrono::Duration::hours(2), now),
            "2h ago"
        );
    }
}
