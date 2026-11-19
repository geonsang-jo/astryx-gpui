# Native port design

The first slice supplied eight primitives and explicit light/dark themes.
The expanded library now maps all 102 public UI families in the pinned Astryx
source to concrete native counterparts. [Coverage](coverage.md) records the
contract and adaptation for every family; React API/prop parity is not claimed.

The visual reference is Meta Astryx commit
`f4fa3661a25403ded7ee4c4c2ac5c2a355eb74ef`. Tokens and source measurements are
mapped to native GPUI lengths and semantic colors. No React runtime, Meta logo,
font files or icon assets are included.

The public GPUI family is `gpui-pre 0.3.8`, matching the Herdr integration
target. `gpui-base 0.7.1` supplies unstyled behavior primitives where appropriate.
This avoids copying Herdr's private GPUI patches and avoids reimplementing input
activation, stable keyed focus and disabled controls.

Interactive values are controlled by the owning view or retained in an explicit
caller-owned native model. Callbacks request application changes; native model
subscriptions notify the owner when it should redraw. Popup/focus lifecycles
may retain keyed native view state, without owning application persistence. Theme values are
explicit and copyable; no application-global theme is introduced. Component-specific builders can refine styles without bypassing
disabled behavior. Library initialization is the existing gpui-base initializer;
host focus traversal bindings belong to the host, not to the library.

Representative native galleries demonstrate both themes and real controlled
interactions. Native tests cover activation, disabled suppression, owner state
and disclosure. A genuine headless GPU capture provides visual evidence.

This is a community project, not an official Meta package. Its code is MIT,
with the complete upstream Meta MIT notice preserved alongside dependency
license provenance. The first publication is a public GitHub repository, not
a crates.io release.
