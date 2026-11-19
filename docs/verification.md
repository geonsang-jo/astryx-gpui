# Native catalogue verification

Verified locally on 2026-11-19 with Rust 1.95.0 on Apple Silicon macOS 26.5
(25F71). This records the independent component library and native galleries.
A consuming application's installation and private adapter checks are separate.

## Required gates

| Command | Result |
| --- | --- |
| `cargo fmt --check` | Passed |
| `cargo clippy --locked --all-targets -- -D warnings` | Passed, no warnings |
| `cargo test --locked --lib --tests` | 116 passed: 12 unit + 2 inventory/type checks + 102 native interaction tests |
| `cargo check --locked --examples` | Passed |

The package pins Rust 1.95 and the public `gpui-pre 0.3.8` / `gpui-base 0.7.1`
type family. No local patches, host-specific dependencies or compiler feature
overrides are committed. The gallery dependency lockfile is public and locked.

## Source coverage and native interactions

[Coverage](coverage.md) maps all 102 UI families from the pinned public source
index, separately from 11 supporting exports. `tests/catalogue.rs` checks exact
source-inventory equality and each exported native element/view type; it does
not substitute for interaction testing. Core behavior is exercised by these
native GPUI event suites:

| Suite | Evidence |
| --- | --- |
| `interactions` | Button pointer/Enter/Space, disabled guards, leading content, retained focus; checkbox, disclosure and Tabs roving/focus reporting |
| `forms` | Korean text and native edit-state retention, multiline/read-only/disabled, stable frame IDs and controlled disabled override; Select keys, focus and consumer focus ownership |
| `forms_extended` | Radio/checkbox lists, both range-slider thumbs, calendar date rules, date/range/time/number editing and popup disable focus |
| `pickers` | Mocked native OS path prompt, cancellation/disabled, multi/grouped selectors, query IME guards, atomic token deletion and commit |
| `navigation` | Link routing, breadcrumbs, top/side/mobile navigation, toolbar arrows, pagination boundaries and step activation |
| `overlays` | Modal focus trap/restore/veto, popover pointer/keyboard/outside/Escape, disabled-close and focused child Enter |
| `overlay_families` | Drawer/sheet geometry, native context/dropdown menus, disabled-open focus restoration, delayed hover help and explicit toast dismissal |
| `advanced_overlays` | Alert default cancel, busy focus/guards, lightbox navigation, command query/disabled selection and IME |
| `collections` | Row selection, owner reordering, native drag/drop identity isolation, table sort/selection and hierarchical virtual-tree navigation |
| `geometry` | Carousel own-focus keys, native swipe/outside cancellation, actual snap scroll offset, preserved scrolling, real constrained panel-handle drag and overflow popup lifecycle |
| `presentation` | Actual grid/ratio/centering geometry, IconButton disabled activation, button-group arrows and hidden application navigation |
| `content` | Korean chat Enter/Shift+Enter/pending/IME/input swap, raw code clipboard, controlled cards/segments and hidden layer layout |
| `time` | Native action starts retained timer, scheduled notifications, pause cancellation and reset; no real-time sleep |

Pure tests cover light/dark contrast, sizing, navigation, bounded geometry,
Unicode initials, safe code fencing, elapsed-time accumulation and relative dates.
The library installs no application-wide Tab bindings. Galleries bind traversal
only inside their own root key context.

## Independent review and corrections

Independent workers reviewed forms, layers, geometry, chat/composer and content
contracts. Native regressions reproduced and corrected dialog/popover interception
of a focused child's Enter action; Select's deferred consumer-focus overwrite;
disabling an open date/menu/popover; busy alert focus becoming invalid; calendar
vertical navigation across disabled dates; both range-slider thumbs; stale or
cross-view drag payloads; chat draft entity/subscription replacement; and capacity/child-count changes removing
an open overflow popup. The latter retains a valid fallback focus without
stealing focus from an external control or changing the owner's controlled flag.

The first real content gallery also exposed AvatarGroup's overflow count being
abbreviated to a single initial. Overflow now uses explicit fallback text while
retaining the full accessible count. Visual artifacts were regenerated from the
corrected native source. Native SVG inspection also caught a GPUI-specific
requirement: each SVG itself needs an explicit paint color. Icon, Spinner, Tabs
and navigation icons now set that color rather than relying on parent text inheritance.
Dark inline-code foreground/background also explicitly use semantic theme colors;
the native rich-text default otherwise retained a light background.

## Native visual evidence

All images come from public `HeadlessAppContext`, the platform text system and
macOS `MetalHeadlessRenderer`. They are real GPU frames, not HTML or reconstructed
mockups. Examples use synthetic local state; no agent, private repository, actual
file chooser or AI/network provider is started by rendering or capture.

| Gallery | Native artifacts |
| --- | --- |
| Original primitives | [Light](assets/gallery-light.png), [Dark](assets/gallery-dark.png), [Light focus](assets/gallery-focus-light.png), [Dark focus](assets/gallery-focus-dark.png) |
| Core forms/layers | [Light](assets/controls-light.png), [Dark](assets/controls-dark.png), [Dialog](assets/controls-dialog.png) |
| Forms and selectors | [Light](assets/forms-light.png), [Dark](assets/forms-dark.png) |
| Menus, collections and layers | [Light](assets/layers-light.png), [Dark](assets/layers-dark.png), [Alert](assets/layers-dialog.png) |
| Rich content and chat | [Light](assets/content-light.png), [Dark](assets/content-dark.png) |
| Geometry and overflow | [Light](assets/geometry-light.png), [Dark](assets/geometry-dark.png) |

The original/core images are 2240×1720 pixels for 1120×860 logical points; the
forms/layers/geometry galleries use 2240×1800 for 1120×900 points; content uses
2240×1840 for 1120×920 points. Layouts were inspected in
both appearances for readable labels, focus, fitting controls and unclipped
composition. Actual keyboard/pointer tests establish behavior beyond the images.

Reproduce captures with `cargo run --locked --example NAME -- --snapshot PATH`;
add `--dark`, `--dialog` for the layer examples, or `--navigation` for forms.

## Publication and limits

Public files are audited to contain only independent source, dependency metadata,
synthetic tests/examples/artifacts and documentation. The complete Meta MIT notice
is retained, dependencies remain public, and no Meta logo/font/icon set is bundled.
Earlier verified batches were published separately for a consuming host to pin.
This final catalogue extends those batches rather than replacing native behavior
with wrappers around a web runtime.

Family coverage does not establish every upstream prop/variant, browser/touch
behavior, image codec/network source, platform build or assistive-technology
relationship. Linux/Windows and physical VoiceOver remain unverified. Skeleton
is static; BottomSheet does not implement touch snap points; SnapPaging takes an
explicit pixel width; OverflowList capacity is host-supplied; translated labels
and provider/search/router data remain host-owned. Tabs delegate panel content
and relationships to the host. Details are explicit per family in coverage.md.
