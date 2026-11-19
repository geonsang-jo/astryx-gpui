//! Unofficial Astryx-inspired native GPUI components.
//!
//! Call [`init`] once before opening your first GPUI window. Components are
//! controlled by their owning view and use an explicit light or dark [`Theme`].

mod alert_dialog;
mod badge;
mod button;
mod calendar;
mod card;
mod cards;
mod chat;
mod checkbox;
mod citation;
mod collapsible;
mod collections;
mod content;
mod date_input;
mod dialog;
mod divider;
mod feedback;
mod field;
mod file_input;
mod geometry;
mod hover_card;
mod input;
mod layer;
mod layout;
mod lightbox;
mod markdown;
mod media;
mod menus;
mod modal_surfaces;
mod navigation;
mod number_input;
mod popover;
mod search_layers;
mod select;
mod selection_lists;
mod selectors;
mod slider;
mod switch;
mod tabs;
mod text;
pub mod theme;
mod time;
mod time_input;
mod toast;
mod tokens;
mod tooltip;

pub use alert_dialog::AlertDialog;
pub use badge::{Badge, Tone};
pub use button::{Button, ButtonVariant};
pub use calendar::{Calendar, CalendarEvent, CalendarState, CalendarView, Date, DateMatcher};
pub use card::Card;
pub use cards::{ClickableCard, SegmentedControl, SelectableCard, ToggleButton};
pub use chat::{Chat, ChatMessage, ChatRole};
pub use checkbox::Checkbox;
pub use citation::Citation;
pub use collapsible::Collapsible;
pub use collections::{
    Item, List, ListEntry, SortDirection, Table, TableColumn, TableRow, TreeEntry, TreeEvent,
    TreeItem, TreeList, TreeState,
};
pub use content::{Blockquote, MetadataList, Outline, VisuallyHidden};
pub use date_input::{DateInput, DateRangeInput, DateTimeInput};
pub use dialog::{Dialog, DialogChangeReason};
pub use divider::Divider;
pub use feedback::{Banner, EmptyState, Kbd, ProgressBar, Skeleton, StatusDot};
pub use field::{Field, FormLayout, InputGroup};
pub use file_input::FileInput;
pub use geometry::{
    Carousel, OverflowList, Resizable, ResizablePane, ResizablePanelEvent, ResizableState,
    ScrollAxis, ScrollableArea, SnapPaging, Swipe, SwipeDirection,
};
pub use gpui_base::init;
pub use hover_card::{HoverCard, HoverCardState, InfoTip};
pub use input::{Input, InputEvent, InputState, TextArea, TextareaState};
pub use layer::{Layer, ResolvedPosition};
pub use layout::{
    AppShell, AspectRatio, ButtonGroup, Center, Direction, Grid, Layout, Section, Stack,
};
pub use lightbox::{Lightbox, LightboxItem};
pub use markdown::{CodeBlock, Markdown};
pub use media::{Avatar, AvatarGroup, Icon, IconButton, Indicator, NavIcon, Spinner, Thumbnail};
pub use menus::{ContextMenu, DropdownMenu, Menu, MenuItem, MoreMenu, NavMenu};
pub use modal_surfaces::{BottomSheet, Drawer, DrawerSide, Overlay};
pub use navigation::{
    Breadcrumb, Breadcrumbs, Link, MobileNav, NavItem, Pagination, SideNav, Step, Stepper, Toolbar,
    TopNav,
};
pub use number_input::{NumberInput, NumberInputEvent, NumberStep, StepAction};
pub use popover::{Popover, PopoverState};
pub use search_layers::{CommandItem, CommandPalette, PowerSearch, SearchSection};
pub use select::{Select, SelectOption};
pub use selection_lists::{CheckboxList, RadioList};
pub use selectors::{ComplexSelector, MultiSelector, SelectGroup, Typeahead};
pub use slider::{Slider, SliderEvent, SliderScale, SliderState, SliderValue};
pub use switch::Switch;
pub use tabs::{Tab, Tabs};
pub use text::{Text, TextKind};
pub use theme::{Appearance, Colors, ControlSize, Theme};
pub use time::{Timer, TimerState, Timestamp};
pub use time_input::{HourCycle, TimeFieldEvent, TimeFieldState, TimeInput, TimePrecision};
pub use toast::{
    Toast, ToastAdvance, ToastManager, ToastMotion, ToastOptions, ToastStackState,
    ToastTransitionStatus,
};
pub use tokens::{InlineToken, InlineTokenError, InputContent, Token, Tokenizer};
pub use tooltip::Tooltip;
