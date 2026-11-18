# Initial native port

The initial scope is a small independent Rust library for the primitives used
by Herdr: Text, Button, Card, Badge, Divider, Checkbox, Tabs and Collapsible,
with explicit light/dark themes. Input, Select, Dialog, Popover and rich chat
editing remain future slices; the project does not claim a complete Astryx API.

The visual reference is Meta Astryx commit
`f4fa3661a25403ded7ee4c4c2ac5c2a355eb74ef`. Tokens and source measurements are
mapped to native GPUI lengths and semantic colors. No React runtime, Meta logo,
font files or icon assets are included.

The public GPUI family is `gpui-pre 0.3.8`, matching the Herdr integration
target. `gpui-base 0.7.1` supplies unstyled behavior primitives where appropriate.
This avoids copying Herdr's private GPUI patches and avoids reimplementing input
activation, stable keyed focus and disabled controls.

Every interactive component is controlled by the owning view. It receives its
state and reports the desired next state through a callback. Theme values are
explicit and copyable; no application-global theme or hidden component state is
introduced. Component-specific builders can refine styles without bypassing
disabled behavior. Library initialization is the existing gpui-base initializer;
host focus traversal bindings belong to the host, not to the library.

The native gallery demonstrates both themes, each component and real controlled
interactions. Native tests cover activation, disabled suppression, owner state
and disclosure. A genuine headless GPU capture provides visual evidence.

This is a community project, not an official Meta package. Its code is MIT,
with the complete upstream Meta MIT notice preserved alongside dependency
license provenance. The first publication is a public GitHub repository, not
a crates.io release.
