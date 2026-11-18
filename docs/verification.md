# Initial slice verification

Verified locally on 2026-11-18 with Rust 1.95.0 on Apple Silicon macOS 26.5
(25F71). This records the independent component library and gallery only; it
does not claim a Herdr integration or cross-platform validation.

## Checks

| Command | Result |
| --- | --- |
| `cargo fmt --check` | Passed |
| `cargo clippy --locked --all-targets -- -D warnings` | Passed, no warnings |
| `cargo test --locked --lib --tests` | 11 passed: 4 unit tests and 7 native interaction tests |
| `cargo check --locked --examples` | Passed |

The published GPUI dependency requires APIs absent from Rust 1.94; a local
1.94 build failed in that dependency at `std::hint::cold_path`. The project
pins the successfully tested 1.95 toolchain rather than introducing private
patches or compiler feature overrides.

The interaction suite dispatches native pointer events and paired key-down /
key-up events. It verifies single Button activation by pointer, Enter and
Space; disabled activation/focus suppression; controlled Checkbox changes;
Tabs roving arrows, Home/End, disabled skipping and modifier isolation; and
controlled Collapsible content layout only while expanded. Unit tests cover
text/status contrast in both appearances, label fit and navigation edge cases.

## Native visual evidence

The gallery was rendered through public `HeadlessAppContext` and the macOS
`MetalHeadlessRenderer`, using the platform text system. Each image is
2240 × 1720 pixels at a 1120 × 860 logical window size. These are native GPU
renders, not HTML or externally reconstructed component images.

```sh
cargo run --locked --example gallery -- --snapshot docs/assets/gallery-light.png
cargo run --locked --example gallery -- --dark --snapshot docs/assets/gallery-dark.png
cargo run --locked --example gallery -- --focus-primary --snapshot docs/assets/gallery-focus-light.png
cargo run --locked --example gallery -- --dark --focus-primary --snapshot docs/assets/gallery-focus-dark.png
```

| Normal | Keyboard focus after Tab twice |
| --- | --- |
| [Light](assets/gallery-light.png) | [Light focus](assets/gallery-focus-light.png) |
| [Dark](assets/gallery-dark.png) | [Dark focus](assets/gallery-focus-dark.png) |

Visual inspection confirmed readable labels, unclipped control geometry,
status color pairs and a contrasting primary-button keyboard focus border in
both themes. The gallery establishes root focus when it opens; Tab does not
require an initial mouse click.

Independent cross-review found and corrected an invisible primary focus border
and missing primary hover/pressed feedback. Primary interaction overlays use a
contrasting on-accent tint as a native adaptation; exact upstream pixel parity
is not claimed. Disabled Buttons suppress pointer feedback and activation.

## Publication and limits

Two independent component cross-reviews checked controlled state, stable focus,
style refinement order, public dependency boundaries, source provenance and
license notices. The final public inventory contains only library source,
synthetic examples/tests/screenshots, dependency lock data and documentation.

The initial public [macOS CI run](https://github.com/geonsang-jo/astryx-gpui/actions/runs/44260800129)
also passed formatting, clippy, all 11 tests and example compilation for code
commit `da450a36740c08120e7a81a022ef95e87c5b8e45`. Later documentation-only
publication records skip duplicate CI; the library, tests and gallery are unchanged.

Windows/Linux compilation,
interactive assistive-technology testing, broad visual regression testing and
full upstream Astryx behavior remain unverified. Text heading styling has no
semantic heading role. Tabs do not implement panel relationships or overflow;
the host composes their content. The remaining controls are listed in the
README roadmap.

## Expanded controls batch

On 2026-11-18, the first expanded batch passed formatting, clippy with warnings
denied, all examples, and24tests (5unit +19nativeinteraction tests). New checks
cover Korean editing-state retention, multiline text, disabled/read-only input,
Select enabled-option navigation/single confirmation/Escape/focus and consumer
focus overrides, modal hidden-content layout, dismissal veto/focus restoration
and focused child-button Enter handling. Switch and icon-child button activation
are also covered, including caller-owned Button focus and tab focus reporting.

Independent forms/layers cross-review found the dialog default-Enter conflict;
the modal now confirms only from its own exact focus target and lets a focused
child control own Enter. The native regression exercises both paths.

Real offscreen Metal images for the new forms/layers gallery:
[Light](assets/controls-light.png), [Dark](assets/controls-dark.png),
[Open dialog](assets/controls-dialog.png). Editing and popup tests dispatch native
events; no business/provider/OS-file operation starts in these synthetic examples.
The full102-family port remains in progress, as recorded in coverage.md.
