# Initial port implementation

Approved by the user: create public `geonsang-jo/astryx-gpui`, start with Herdr's
basic components, and have subagents carry the project through publication.

- [x] Implement theme and presentation primitives with source attribution.
- [x] Implement controlled interactive primitives using native behavior.
- [x] Build the native gallery and a reproducible headless capture mode.
- [x] Verify keyboard/pointer/disabled behavior and both light/dark rendering.
- [x] Run formatting, clippy, tests and example compilation.
- [x] Independently review source, public file inventory and license notices.
- [x] Create the public personal-account repository and push verified commits.
- [x] Verify remote visibility and pushed SHA; record evidence and limitations.

## Public API contract

All components accept `.theme(Theme)`, defaulting to light. `Theme` and `Colors`
are copyable. `Theme.colors` exposes surface, canvas, text, muted, faint, border,
border_emphasized, accent, on_accent, neutral, hover, pressed, success, warning,
danger, info and focus colors. `ControlSize::{Small,Medium,Large}` maps to
28/32/36px. Body type is 14/20px, supporting 12/20px, heading 17/24px.

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

## Publication evidence

The initial implementation was published to the verified public
[geonsang-jo/astryx-gpui](https://github.com/geonsang-jo/astryx-gpui) repository.
GitHub `main` matched local implementation commit
`da450a36740c08120e7a81a022ef95e87c5b8e45` on 2026-11-18.
The account credential was scoped to the publishing process; no global GitHub
account configuration was changed. See `verification.md` for native evidence
and the deliberately limited first-slice scope.
