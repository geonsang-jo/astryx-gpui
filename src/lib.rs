//! Unofficial Astryx-inspired native GPUI components.
//!
//! Call [`init`] once before opening your first GPUI window. Components are
//! controlled by their owning view and use an explicit light or dark [`Theme`].

mod badge;
mod button;
mod card;
mod checkbox;
mod collapsible;
mod dialog;
mod divider;
mod feedback;
mod input;
mod popover;
mod select;
mod switch;
mod tabs;
mod text;
pub mod theme;
mod tooltip;

pub use badge::{Badge, Tone};
pub use button::{Button, ButtonVariant};
pub use card::Card;
pub use checkbox::Checkbox;
pub use collapsible::Collapsible;
pub use dialog::{Dialog, DialogChangeReason};
pub use divider::Divider;
pub use feedback::{Banner, EmptyState, Kbd, ProgressBar, Skeleton, StatusDot};
pub use gpui_base::init;
pub use input::{Input, InputEvent, InputState, TextArea, TextareaState};
pub use popover::{Popover, PopoverState};
pub use select::{Select, SelectOption};
pub use switch::Switch;
pub use tabs::{Tab, Tabs};
pub use text::{Text, TextKind};
pub use theme::{Appearance, Colors, ControlSize, Theme};
pub use tooltip::Tooltip;
