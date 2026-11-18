//! Unofficial Astryx-inspired native GPUI components.
//!
//! Call [`init`] once before opening your first GPUI window. Components are
//! controlled by their owning view and use an explicit light or dark [`Theme`].

mod badge;
mod button;
mod card;
mod checkbox;
mod collapsible;
mod divider;
mod tabs;
mod text;
pub mod theme;

pub use badge::{Badge, Tone};
pub use button::{Button, ButtonVariant};
pub use card::Card;
pub use checkbox::Checkbox;
pub use collapsible::Collapsible;
pub use divider::Divider;
pub use gpui_base::init;
pub use tabs::{Tab, Tabs};
pub use text::{Text, TextKind};
pub use theme::{Appearance, Colors, ControlSize, Theme};
