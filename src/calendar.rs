use crate::Theme;
use chrono::{Datelike, NaiveDate};
use gpui::{
    App, ElementId, Entity, FocusHandle, InteractiveElement as _, IntoElement, ParentElement,
    RenderOnce, SharedString, StyleRefinement, Styled, Window, div, prelude::FluentBuilder as _,
    px,
};
use gpui_base::StyledExt as _;
use std::{collections::HashMap, rc::Rc};
type Change = Rc<dyn Fn(Date, &mut Window, &mut App)>;
pub use gpui_base::{CalendarEvent, CalendarState, CalendarView, Date, Matcher as DateMatcher};

#[derive(Default)]
struct CalendarFocus(HashMap<NaiveDate, FocusHandle>);
/// A native calendar over GPUI's date/range model. Observe the caller-owned
/// state for redraws and subscribe to CalendarEvent::Selected for commits.
/// Arrow keys move through the enabled visible-day grid; Home/End move to its
/// edges. Month and year views keep the same validated date model.
#[derive(IntoElement)]
pub struct Calendar {
    id: ElementId,
    state: Entity<CalendarState>,
    theme: Theme,
    change: Option<Change>,
    style: StyleRefinement,
}
impl Calendar {
    pub fn new(id: impl Into<ElementId>, state: &Entity<CalendarState>) -> Self {
        Self {
            id: id.into(),
            state: state.clone(),
            theme: Theme::default(),
            change: None,
            style: StyleRefinement::default(),
        }
    }
    pub fn theme(mut self, theme: Theme) -> Self {
        self.theme = theme;
        self
    }
    pub fn on_change(mut self, f: impl Fn(Date, &mut Window, &mut App) + 'static) -> Self {
        self.change = Some(Rc::new(f));
        self
    }
}
impl Styled for Calendar {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}
fn button(
    id: impl Into<ElementId>,
    label: impl Into<SharedString>,
    theme: Theme,
) -> gpui_base::Button {
    let c = theme.colors;
    gpui_base::Button::new(id)
        .h(px(32.))
        .min_w(px(32.))
        .px(px(6.))
        .rounded(px(8.))
        .text_size(px(14.))
        .line_height(px(20.))
        .text_color(c.text)
        .hover(move |s| s.bg(c.hover))
        .focus_visible(move |s| s.bg(c.neutral))
        .styles(|styles| styles.disabled(|s| s.opacity(0.4)))
        .child(label.into())
}
impl RenderOnce for Calendar {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let c = self.theme.colors;
        let state = self.state.read(cx);
        let view = state.view();
        let year = state.current_year();
        let month = state.current_month();
        let previous_disabled = view.is_month() || (view.is_year() && !state.has_prev_year_page());
        let next_disabled = view.is_month() || (view.is_year() && !state.has_next_year_page());
        let selected = state.date();
        let dates: Vec<NaiveDate> = state.days().into_iter().flatten().collect();
        let enabled: Vec<usize> = dates
            .iter()
            .enumerate()
            .filter_map(|(i, date)| {
                (!state
                    .disabled_matcher_ref()
                    .is_some_and(|matcher| matcher.matched(date)))
                .then_some(i)
            })
            .collect();
        let years = state.years_on_page().to_vec();
        let focus = window.use_keyed_state((self.id.clone(), "calendar-focus"), cx, |_, _| {
            CalendarFocus::default()
        });
        let handles = focus.update(cx, |focus, cx| {
            focus.0.retain(|date, _| dates.contains(date));
            dates
                .iter()
                .map(|date| {
                    focus
                        .0
                        .entry(*date)
                        .or_insert_with(|| cx.focus_handle())
                        .clone()
                })
                .collect::<Vec<_>>()
        });
        let handles = Rc::new(handles);
        let enabled = Rc::new(enabled);
        let active = dates
            .iter()
            .position(|date| selected.is_active(date))
            .filter(|i| enabled.contains(i))
            .or_else(|| {
                enabled
                    .iter()
                    .copied()
                    .find(|&i| dates[i].month() == month as u32)
            });
        let previous = self.state.clone();
        let next = self.state.clone();
        let month_state = self.state.clone();
        let year_state = self.state.clone();
        let mut root = div()
            .id(self.id.clone())
            .flex()
            .flex_col()
            .w(px(280.))
            .gap(px(8.))
            .p(px(12.))
            .rounded(px(12.))
            .bg(c.surface)
            .text_color(c.text)
            .child(
                div()
                    .flex()
                    .items_center()
                    .justify_between()
                    .gap(px(4.))
                    .child(
                        button((self.id.clone(), "previous"), "‹", self.theme)
                            .disabled(previous_disabled)
                            .accessibility_label("Previous month or year page")
                            .on_click(move |_, _, cx| {
                                previous.update(cx, |s, cx| {
                                    if s.view().is_year() {
                                        s.prev_year_page();
                                    } else {
                                        s.prev_month();
                                    }
                                    cx.notify();
                                });
                            }),
                    )
                    .child(
                        button(
                            (self.id.clone(), "month"),
                            chrono::Month::try_from(month)
                                .map(|m| m.name())
                                .unwrap_or("Month"),
                            self.theme,
                        )
                        .on_click(move |_, _, cx| {
                            month_state.update(cx, |s, cx| {
                                s.set_view(if s.view().is_month() {
                                    CalendarView::Day
                                } else {
                                    CalendarView::Month
                                });
                                cx.notify();
                            });
                        }),
                    )
                    .child(
                        button((self.id.clone(), "year"), year.to_string(), self.theme).on_click(
                            move |_, _, cx| {
                                year_state.update(cx, |s, cx| {
                                    s.set_view(if s.view().is_year() {
                                        CalendarView::Day
                                    } else {
                                        CalendarView::Year
                                    });
                                    cx.notify();
                                });
                            },
                        ),
                    )
                    .child(
                        button((self.id.clone(), "next"), "›", self.theme)
                            .disabled(next_disabled)
                            .accessibility_label("Next month or year page")
                            .on_click(move |_, _, cx| {
                                next.update(cx, |s, cx| {
                                    if s.view().is_year() {
                                        s.next_year_page();
                                    } else {
                                        s.next_month();
                                    }
                                    cx.notify();
                                });
                            }),
                    ),
            );
        if view.is_month() {
            let mut grid = div().grid().grid_cols(3).gap(px(4.));
            for m in 1u8..=12 {
                let state = self.state.clone();
                grid = grid.child(
                    button(
                        (self.id.clone(), m.to_string()),
                        chrono::Month::try_from(m).unwrap().name(),
                        self.theme,
                    )
                    .on_click(move |_, _, cx| {
                        state.update(cx, |s, cx| {
                            s.select_month(m);
                            cx.notify();
                        });
                    }),
                );
            }
            root = root.child(grid);
        } else if view.is_year() {
            let mut grid = div().grid().grid_cols(3).gap(px(4.));
            for year in years {
                let state = self.state.clone();
                grid = grid.child(
                    button(
                        (self.id.clone(), year.to_string()),
                        year.to_string(),
                        self.theme,
                    )
                    .on_click(move |_, _, cx| {
                        state.update(cx, |s, cx| {
                            s.select_year(year);
                            cx.notify();
                        });
                    }),
                );
            }
            root = root.child(grid);
        } else {
            let mut grid = div().grid().grid_cols(7).gap(px(2.));
            for day in ["Su", "Mo", "Tu", "We", "Th", "Fr", "Sa"] {
                grid = grid.child(
                    div()
                        .h(px(24.))
                        .text_size(px(12.))
                        .text_center()
                        .text_color(c.muted)
                        .child(day),
                );
            }
            for (index, date) in dates.into_iter().enumerate() {
                let state = self.state.clone();
                let change = self.change.clone();
                let nav_handles = handles.clone();
                let nav_enabled = enabled.clone();
                let disabled = !enabled.contains(&index);
                let chosen = selected.is_active(&date);
                let in_range = selected.is_in_range(&date);
                grid = grid.child(
                    button(
                        (self.id.clone(), date.to_string()),
                        date.day().to_string(),
                        self.theme,
                    )
                    .accessibility_label(date.to_string())
                    .disabled(disabled)
                    .track_focus(&handles[index])
                    .tab_stop(active == Some(index))
                    .bg(if chosen {
                        c.accent
                    } else if in_range {
                        c.neutral
                    } else {
                        gpui::transparent_black()
                    })
                    .text_color(if chosen {
                        c.on_accent
                    } else if date.month() != month as u32 {
                        c.muted
                    } else {
                        c.text
                    })
                    .when(disabled, |el| el.opacity(0.35))
                    .on_click(move |_, window, cx| {
                        let value = state.update(cx, |s, cx| {
                            s.activate_date(date, cx);
                            cx.notify();
                            s.date()
                        });
                        if let Some(change) = &change {
                            change(value, window, cx);
                        }
                    })
                    .on_key_down(move |event, window, cx| {
                        if event.keystroke.modifiers != gpui::Modifiers::default() {
                            return;
                        }
                        let Some(at) = nav_enabled.iter().position(|&i| i == index) else {
                            return;
                        };
                        let next = match event.keystroke.key.as_str() {
                            "left" => Some(at.saturating_sub(1)),
                            "right" => Some((at + 1).min(nav_enabled.len() - 1)),
                            "up" => nav_enabled
                                .iter()
                                .rposition(|&i| i <= index.saturating_sub(7)),
                            "down" => nav_enabled
                                .iter()
                                .position(|&i| i >= index.saturating_add(7)),
                            "home" => Some(0),
                            "end" => Some(nav_enabled.len() - 1),
                            _ => None,
                        };
                        if let Some(next) = next {
                            cx.stop_propagation();
                            nav_handles[nav_enabled[next]].focus(window, cx);
                        }
                    }),
                );
            }
            root = root.child(grid);
        }
        root.refine_style(&self.style)
    }
}
