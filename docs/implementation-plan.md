# Initial port implementation

Approved by the user: create public `geonsang-jo/astryx-gpui`, start with Herdr's
basic components, and have subagents carry the project through publication.

- [x] Implement theme and presentation primitives with source attribution.
- [x] Implement controlled interactive primitives using native behavior.
- [x] Build the native gallery and a reproducible headless capture mode.
- [x] Verify keyboard/pointer/disabled behavior and both light/dark rendering.
- [x] Run formatting, clippy, tests and example compilation.
- [x] Independently review source, public file inventory and license notices.
- [ ] Create the public personal-account repository and push verified commits.
- [ ] Verify remote visibility and pushed SHA; record evidence and limitations.

## Public API contract

All components accept `.theme(Theme)`, defaulting to light. `Theme` and `Colors`
are copyable. `Theme.colors` exposes surface, canvas, text, muted, faint, border,
border_emphasized, accent, on_accent, neutral, hover, pressed, success, warning,
danger, info and focus colors. `ControlSize::{Small,Medium,Large}` maps to
28/32/36px. Body type is14/20px, supporting12/20px, heading17/24px.

- `Text::new(label).kind(TextKind::{Body,Supporting,Heading}).muted()`
- `Card::new().padding(16.).child(...)`
- `Badge::new(label).tone(Tone::{Neutral,Success,Warning,Danger,Info})`
- `Divider::new()`
- `Button::new(id,label).variant(ButtonVariant::{Primary,Secondary,Outline,Ghost})
  .size(ControlSize).disabled(bool).selected(bool).on_click(callback)`
- `Checkbox::new(id,label).checked(bool).disabled(bool).on_change(callback)`
  where callback is `Fn(bool,&mut Window,&mut App)`.
- `Tab::new(key,label).disabled(bool)` and
  `Tabs::new(id).items(Vec<Tab>).selected(key).on_select(callback)` where callback
  is `Fn(SharedString,&mut Window,&mut App)`.
- `Collapsible::new(id,label).open(bool).disabled(bool).on_open_change(callback)
  .child(...)`, callback `Fn(bool,&mut Window,&mut App)`.

Container components implement `ParentElement`. Native callback closures are
`'static`. Only the owner mutates state; rendering does not initiate I/O.
