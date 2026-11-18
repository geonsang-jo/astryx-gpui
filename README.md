# Astryx GPUI

An **unofficial community port** of [Meta's Astryx](https://github.com/meta/astryx) Neutral design language to native Rust GPUI. It draws with GPUI rather than a web view. This project is independent of Meta and does not provide drop-in compatibility with the React package.

![Light native gallery](docs/assets/gallery-light.png)

[Dark gallery](docs/assets/gallery-dark.png) · [Pinned source map](docs/source-map.md)

## First slice

| Component | Available behavior |
| --- | --- |
| Theme | Explicit light/dark semantic tokens; caller can customize `Theme.colors` |
| Text | Body, supporting and heading typography |
| Button | Primary, secondary, outline and ghost; three sizes; native keyboard and disabled behavior |
| Card | Composable children and caller styling |
| Badge | Neutral, success, warning, danger and info |
| Divider | Horizontal semantic color separator |
| Checkbox | Caller-owned checked state and change callback |
| Tabs | Caller-owned selection; roving focus; Left/Right/Home/End; disabled items |
| Collapsible | Caller-owned open state; pointer and keyboard trigger |

This is an initial useful subset, not the full Astryx kit. Input, Select, Dialog and Popover are the next phase. No application integration is included.

## Run the native gallery

A macOS build environment with Command Line Tools is required for the currently verified platform. Rust **1.95.0**, rustfmt and clippy are pinned in `rust-toolchain.toml`; installing them does not change your global default toolchain.

```sh
git clone https://github.com/geonsang-jo/astryx-gpui.git
cd astryx-gpui
cargo run --locked --example gallery
cargo run --locked --example gallery -- --dark
```

The gallery supports real checkbox, tab, disclosure, button counter and theme state. Its own root handles Tab / Shift+Tab focus traversal. The library does not install application-wide navigation keybindings.

Capture a real offscreen GPUI render through the same gallery:

```sh
cargo run --locked --example gallery -- --snapshot artifacts/gallery-light.png
cargo run --locked --example gallery -- --dark --snapshot artifacts/gallery-dark.png
cargo run --locked --example gallery -- --focus-primary --snapshot artifacts/gallery-focus.png
```

The capture path uses the public GPUI test renderer and the platform text system. It does not render HTML or substitute a screenshot mockup.

## Use in a GPUI host

This crate targets the **`gpui-pre` 0.3.8** type family used by `gpui-base` 0.7.1. Depend on that package under the `gpui` name in your host. A different GPUI fork/version may expose different Rust types.

```toml
[dependencies]
astryx-gpui = { git = "https://github.com/geonsang-jo/astryx-gpui.git" }
gpui = { package = "gpui-pre", version = "=0.3.8", default-features = false }
```

Call `astryx_gpui::init(cx)` once during app initialization, before opening windows. It initializes the public `gpui-base` behavior primitives. Pin the git dependency to a reviewed commit for a reproducible application build.

Inside a view's `Render` implementation:

```rust,ignore
use astryx_gpui::{Button, ButtonVariant, Card, Checkbox, Text, TextKind, Theme};
use gpui::prelude::*;

let theme = Theme::light(); // or a theme retained/customized by the host
let change = cx.listener(|this, value: &bool, _, cx| {
    this.notifications = *value;
    cx.notify();
});

Card::new()
    .theme(theme)
    .child(Text::new("Notifications").kind(TextKind::Heading).theme(theme))
    .child(
        Checkbox::new("notifications", "Notify me about activity")
            .theme(theme)
            .checked(self.notifications)
            .on_change(move |value, window, cx| change(&value, window, cx)),
    )
    .child(
        Button::new("save", "Save")
            .theme(theme)
            .variant(ButtonVariant::Primary)
            .on_click(cx.listener(|this, _, _, cx| {
                this.save();
                cx.notify();
            })),
    )
```

Use stable, distinct control IDs in the owning window. Checkbox, Tabs and Collapsible request changes through callbacks; the host updates its value and calls `cx.notify()`. Rebuilding a component does not own or persist application state. Components implement GPUI `Styled`; caller refinements apply after their defaults. Containers implement `ParentElement`.

The host owns page layout, fonts, focus traversal policy and persistence. Components use inherited/system fonts; no Meta logo, font or icon asset is bundled.

## Verification

```sh
cargo fmt --check
cargo clippy --locked --all-targets -- -D warnings
cargo test --locked --lib --tests
cargo check --locked --examples
```

The integration suite dispatches native pointer/keyboard events to GPUI views and checks controlled state, disabled controls and tab navigation. Theme tests verify text and badge contrast. CI runs on macOS. Linux/Windows builds and assistive-technology behavior have not been validated; semantic roles are supplied by public GPUI primitives where available, and typography-only `TextKind::Heading` is not a semantic heading.

See [verification evidence](docs/verification.md), [design scope](docs/design.md), and [source and license provenance](THIRD_PARTY_NOTICES.md).

## Design provenance and license

The first slice follows Astryx's Neutral theme at commit `f4fa3661a25403ded7ee4c4c2ac5c2a355eb74ef`: neutral/status colors, 4px spacing scale, 12px cards, 10px buttons, and 14/20, 12/20, 17/24 typography. Native controls adapt React interactions to GPUI; exact pixel parity with the full upstream system is not claimed.

Original Rust work is [MIT licensed](LICENSE). The complete Meta MIT notice and dependency license guidance are in [THIRD_PARTY_NOTICES.md](THIRD_PARTY_NOTICES.md).
