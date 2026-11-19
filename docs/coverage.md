# Native coverage inventory

Pinned source: Meta Astryx [`f4fa3661a25403ded7ee4c4c2ac5c2a355eb74ef`](https://github.com/meta/astryx/blob/f4fa3661a25403ded7ee4c4c2ac5c2a355eb74ef/packages/core/src/index.ts).

All **102 public UI families** have concrete native GPUI counterparts. This is
family-level coverage of their core purpose, not React API compatibility, all
source props/variants, exact pixel parity, or a claim that every control has been
exercised with physical assistive technology. No alias or inert stub is counted.
The separate 11 supporting exports map to native host responsibilities below.

The inventory test checks all 102 exported native elements/views and the pinned
source family list. Behavioral suites exercise native events and state; their
scope and visual evidence are recorded in [verification](verification.md).

## Public UI families

| Upstream family | Native API | Supported native contract |
| --- | --- | --- |
| [AppShell](https://github.com/meta/astryx/tree/f4fa3661a25403ded7ee4c4c2ac5c2a355eb74ef/packages/core/src/AppShell) | `AppShell` | Header and controlled navigation visibility around native layout slots. |
| [AspectRatio](https://github.com/meta/astryx/tree/f4fa3661a25403ded7ee4c4c2ac5c2a355eb74ef/packages/core/src/AspectRatio) | `AspectRatio` | Native aspect-ratio constraint; invalid ratios fall back to square. |
| [Avatar](https://github.com/meta/astryx/tree/f4fa3661a25403ded7ee4c4c2ac5c2a355eb74ef/packages/core/src/Avatar) | `Avatar` | Caller image source with native loading/error fallback and Unicode initials. |
| [AvatarGroup](https://github.com/meta/astryx/tree/f4fa3661a25403ded7ee4c4c2ac5c2a355eb74ef/packages/core/src/AvatarGroup) | `AvatarGroup` | Overlapping avatars, explicit visible capacity and remaining count. |
| [Badge](https://github.com/meta/astryx/tree/f4fa3661a25403ded7ee4c4c2ac5c2a355eb74ef/packages/core/src/Badge) | `Badge` | Neutral and semantic tone presentation. |
| [Banner](https://github.com/meta/astryx/tree/f4fa3661a25403ded7ee4c4c2ac5c2a355eb74ef/packages/core/src/Banner) | `Banner` | Semantic tone with composable status content. |
| [Blockquote](https://github.com/meta/astryx/tree/f4fa3661a25403ded7ee4c4c2ac5c2a355eb74ef/packages/core/src/Blockquote) | `Blockquote` | Indented quote content with optional attribution. |
| [BottomSheet](https://github.com/meta/astryx/tree/f4fa3661a25403ded7ee4c4c2ac5c2a355eb74ef/packages/core/src/BottomSheet) | `BottomSheet` | Bottom-aligned controlled modal, Escape/backdrop/focus lifecycle; no touch snap points. |
| [Breadcrumbs](https://github.com/meta/astryx/tree/f4fa3661a25403ded7ee4c4c2ac5c2a355eb74ef/packages/core/src/Breadcrumbs) | `Breadcrumbs` | Controlled path/current item, disabled entries and native link activation. |
| [Button](https://github.com/meta/astryx/tree/f4fa3661a25403ded7ee4c4c2ac5c2a355eb74ef/packages/core/src/Button) | `Button` | Native pointer/keyboard activation, disabled state, focus override, leading/trailing content. |
| [ButtonGroup](https://github.com/meta/astryx/tree/f4fa3661a25403ded7ee4c4c2ac5c2a355eb74ef/packages/core/src/ButtonGroup) | `ButtonGroup` | Native toolbar grouping with scoped roving focus. |
| [IconButton](https://github.com/meta/astryx/tree/f4fa3661a25403ded7ee4c4c2ac5c2a355eb74ef/packages/core/src/IconButton) | `IconButton` | Named native button with caller-supplied icon and disabled handling. |
| [Card](https://github.com/meta/astryx/tree/f4fa3661a25403ded7ee4c4c2ac5c2a355eb74ef/packages/core/src/Card) | `Card` | Composable themed surface; optional native frame ID and caller styling. |
| [ClickableCard](https://github.com/meta/astryx/tree/f4fa3661a25403ded7ee4c4c2ac5c2a355eb74ef/packages/core/src/ClickableCard) | `ClickableCard` | Native button semantics and disabled activation around card content. |
| [Carousel](https://github.com/meta/astryx/tree/f4fa3661a25403ded7ee4c4c2ac5c2a355eb74ef/packages/core/src/Carousel) | `Carousel` | Controlled page, previous/next actions, own-root arrow keys and focus. |
| [SnapPaging](https://github.com/meta/astryx/tree/f4fa3661a25403ded7ee4c4c2ac5c2a355eb74ef/packages/core/src/SnapPaging) | `SnapPaging` | Real scroll-handle snapping, horizontal wheel/swipe and scoped keys; explicit pixel page width. |
| [Calendar](https://github.com/meta/astryx/tree/f4fa3661a25403ded7ee4c4c2ac5c2a355eb74ef/packages/core/src/Calendar) | `Calendar` | Retained native date/range model, day/month navigation and disabled-date rules. |
| [Center](https://github.com/meta/astryx/tree/f4fa3661a25403ded7ee4c4c2ac5c2a355eb74ef/packages/core/src/Center) | `Center` | Two-axis native flex alignment. |
| [CodeBlock](https://github.com/meta/astryx/tree/f4fa3661a25403ded7ee4c4c2ac5c2a355eb74ef/packages/core/src/CodeBlock) | `CodeBlock` | Native selectable code, safe dynamic fences and optional raw-code clipboard action. |
| [CommandPalette](https://github.com/meta/astryx/tree/f4fa3661a25403ded7ee4c4c2ac5c2a355eb74ef/packages/core/src/CommandPalette) | `CommandPalette` | Controlled modal query/results, keyboard selection, disabled commands and focus restoration. |
| [ComplexSelector](https://github.com/meta/astryx/tree/f4fa3661a25403ded7ee4c4c2ac5c2a355eb74ef/packages/core/src/ComplexSelector) | `ComplexSelector` | Grouped controlled single selection with query, disabled options and native focus. |
| [Chat](https://github.com/meta/astryx/tree/f4fa3661a25403ded7ee4c4c2ac5c2a355eb74ef/packages/core/src/Chat) | `Chat` | Controlled messages, retained native draft, IME-safe submit, Shift+Enter and send/cancel callbacks. |
| [Markdown](https://github.com/meta/astryx/tree/f4fa3661a25403ded7ee4c4c2ac5c2a355eb74ef/packages/core/src/Markdown) | `Markdown` | Native rich-text parsing, selection and link callbacks; no HTML webview. |
| [Citation](https://github.com/meta/astryx/tree/f4fa3661a25403ded7ee4c4c2ac5c2a355eb74ef/packages/core/src/Citation) | `Citation` | Native source link and tooltip; host can override URL opening. |
| [CheckboxInput](https://github.com/meta/astryx/tree/f4fa3661a25403ded7ee4c4c2ac5c2a355eb74ef/packages/core/src/CheckboxInput) | `Checkbox` | Controlled native checked state, pointer/keyboard activation and disabled guards. |
| [CheckboxList](https://github.com/meta/astryx/tree/f4fa3661a25403ded7ee4c4c2ac5c2a355eb74ef/packages/core/src/CheckboxList) | `CheckboxList` | Multiple controlled choices with native checkbox behavior. |
| [Collapsible](https://github.com/meta/astryx/tree/f4fa3661a25403ded7ee4c4c2ac5c2a355eb74ef/packages/core/src/Collapsible) | `Collapsible` | Controlled content visibility and native toggle trigger. |
| [RadioList](https://github.com/meta/astryx/tree/f4fa3661a25403ded7ee4c4c2ac5c2a355eb74ef/packages/core/src/RadioList) | `RadioList` | Controlled exclusive selection with native roving keyboard focus. |
| [Resizable](https://github.com/meta/astryx/tree/f4fa3661a25403ded7ee4c4c2ac5c2a355eb74ef/packages/core/src/Resizable) | `Resizable` | Caller-owned native panel state, real drag handles, min/max constraints and resize callback. |
| [ScrollableArea](https://github.com/meta/astryx/tree/f4fa3661a25403ded7ee4c4c2ac5c2a355eb74ef/packages/core/src/ScrollableArea) | `ScrollableArea` | Caller-owned scroll handle and native scrollbar; preserved offset across rerenders. |
| [Divider](https://github.com/meta/astryx/tree/f4fa3661a25403ded7ee4c4c2ac5c2a355eb74ef/packages/core/src/Divider) | `Divider` | Semantic horizontal separator. |
| [VisuallyHidden](https://github.com/meta/astryx/tree/f4fa3661a25403ded7ee4c4c2ac5c2a355eb74ef/packages/core/src/VisuallyHidden) | `VisuallyHidden` | Unpainted labelled native accessibility node; physical VoiceOver behavior unverified. |
| [EmptyState](https://github.com/meta/astryx/tree/f4fa3661a25403ded7ee4c4c2ac5c2a355eb74ef/packages/core/src/EmptyState) | `EmptyState` | Title, description and composed actions. |
| [Lightbox](https://github.com/meta/astryx/tree/f4fa3661a25403ded7ee4c4c2ac5c2a355eb74ef/packages/core/src/Lightbox) | `Lightbox` | Controlled image index, modal focus, previous/next and Escape; caller owns image assets. |
| [Link](https://github.com/meta/astryx/tree/f4fa3661a25403ded7ee4c4c2ac5c2a355eb74ef/packages/core/src/Link) | `Link` | Native link activation with host routing override and disabled handling. |
| [List](https://github.com/meta/astryx/tree/f4fa3661a25403ded7ee4c4c2ac5c2a355eb74ef/packages/core/src/List) | `List` | Controlled selection/reorder, keyboard navigation and scoped native drag/drop payloads. |
| [MetadataList](https://github.com/meta/astryx/tree/f4fa3661a25403ded7ee4c4c2ac5c2a355eb74ef/packages/core/src/MetadataList) | `MetadataList` | Label/value rows with composed native values. |
| [NavIcon](https://github.com/meta/astryx/tree/f4fa3661a25403ded7ee4c4c2ac5c2a355eb74ef/packages/core/src/NavIcon) | `NavIcon` | Caller-supplied icon with active styling and count. |
| [NavMenu](https://github.com/meta/astryx/tree/f4fa3661a25403ded7ee4c4c2ac5c2a355eb74ef/packages/core/src/NavMenu) | `NavMenu` | Native menu selection suitable for host navigation; no implicit router. |
| [Slider](https://github.com/meta/astryx/tree/f4fa3661a25403ded7ee4c4c2ac5c2a355eb74ef/packages/core/src/Slider) | `Slider` | Retained single/range values, pointer and both-thumb keyboard changes with constraints. |
| [Stack](https://github.com/meta/astryx/tree/f4fa3661a25403ded7ee4c4c2ac5c2a355eb74ef/packages/core/src/Stack) | `Stack` | Horizontal/vertical native flex layout and explicit spacing. |
| [Stepper](https://github.com/meta/astryx/tree/f4fa3661a25403ded7ee4c4c2ac5c2a355eb74ef/packages/core/src/Stepper) | `Stepper` | Controlled current step, completed/disabled steps and activation callback. |
| [Switch](https://github.com/meta/astryx/tree/f4fa3661a25403ded7ee4c4c2ac5c2a355eb74ef/packages/core/src/Switch) | `Switch` | Controlled native binary switch with pointer/keyboard and disabled handling. |
| [DateInput](https://github.com/meta/astryx/tree/f4fa3661a25403ded7ee4c4c2ac5c2a355eb74ef/packages/core/src/DateInput) | `DateInput` | Retained date model, native calendar popup, keyboard editing and popup focus lifecycle. |
| [DateTimeInput](https://github.com/meta/astryx/tree/f4fa3661a25403ded7ee4c4c2ac5c2a355eb74ef/packages/core/src/DateTimeInput) | `DateTimeInput` | Composed date/time editing over caller-owned models. |
| [DateRangeInput](https://github.com/meta/astryx/tree/f4fa3661a25403ded7ee4c4c2ac5c2a355eb74ef/packages/core/src/DateRangeInput) | `DateRangeInput` | Retained range editing and calendar range selection. |
| [Field](https://github.com/meta/astryx/tree/f4fa3661a25403ded7ee4c4c2ac5c2a355eb74ef/packages/core/src/Field) | `Field` | Label/description/error/required presentation around composed controls. |
| [FileInput](https://github.com/meta/astryx/tree/f4fa3661a25403ded7ee4c4c2ac5c2a355eb74ef/packages/core/src/FileInput) | `FileInput` | Native OS path chooser and controlled path list; cancellation preserves value, no upload/read. |
| [FormLayout](https://github.com/meta/astryx/tree/f4fa3661a25403ded7ee4c4c2ac5c2a355eb74ef/packages/core/src/FormLayout) | `FormLayout` | Composed field layout; validation/submission remain host policy. |
| [Grid](https://github.com/meta/astryx/tree/f4fa3661a25403ded7ee4c4c2ac5c2a355eb74ef/packages/core/src/Grid) | `Grid` | Native grid columns and gap constraints. |
| [Section](https://github.com/meta/astryx/tree/f4fa3661a25403ded7ee4c4c2ac5c2a355eb74ef/packages/core/src/Section) | `Section` | Title/description/divider around composed content. |
| [SegmentedControl](https://github.com/meta/astryx/tree/f4fa3661a25403ded7ee4c4c2ac5c2a355eb74ef/packages/core/src/SegmentedControl) | `SegmentedControl` | Controlled radio choices, disabled skipping and scoped Arrow/Home/End focus. |
| [SelectableCard](https://github.com/meta/astryx/tree/f4fa3661a25403ded7ee4c4c2ac5c2a355eb74ef/packages/core/src/SelectableCard) | `SelectableCard` | Native checkbox semantics around controlled card selection. |
| [Selector](https://github.com/meta/astryx/tree/f4fa3661a25403ded7ee4c4c2ac5c2a355eb74ef/packages/core/src/Selector) | `Select` | Controlled selected/open/highlight, native popup and explicit trigger/content focus handles. |
| [MultiSelector](https://github.com/meta/astryx/tree/f4fa3661a25403ded7ee4c4c2ac5c2a355eb74ef/packages/core/src/MultiSelector) | `MultiSelector` | Controlled set of selected options in native popup. |
| [Icon](https://github.com/meta/astryx/tree/f4fa3661a25403ded7ee4c4c2ac5c2a355eb74ef/packages/core/src/Icon) | `Icon` | Caller-supplied native SVG path/inline data and optional accessible label; no icon set bundled. |
| [InfoTip](https://github.com/meta/astryx/tree/f4fa3661a25403ded7ee4c4c2ac5c2a355eb74ef/packages/core/src/InfoTip) | `InfoTip` | Pointer/keyboard-triggered native information popover. |
| [Indicator](https://github.com/meta/astryx/tree/f4fa3661a25403ded7ee4c4c2ac5c2a355eb74ef/packages/core/src/Indicator) | `Indicator` | Composed content with count indicator. |
| [InputGroup](https://github.com/meta/astryx/tree/f4fa3661a25403ded7ee4c4c2ac5c2a355eb74ef/packages/core/src/InputGroup) | `InputGroup` | Native input layout with leading/trailing composed content. |
| [Item](https://github.com/meta/astryx/tree/f4fa3661a25403ded7ee4c4c2ac5c2a355eb74ef/packages/core/src/Item) | `Item` | Native focusable selectable row with disabled activation guards. |
| [Swipe](https://github.com/meta/astryx/tree/f4fa3661a25403ded7ee4c4c2ac5c2a355eb74ef/packages/core/src/Swipe) | `Swipe` | Native pointer gesture threshold/direction callback and outside cancellation; no OS gesture hook. |
| [Text](https://github.com/meta/astryx/tree/f4fa3661a25403ded7ee4c4c2ac5c2a355eb74ef/packages/core/src/Text) | `Text` | Body/supporting/heading typography; heading style does not create a semantic heading role. |
| [TextInput](https://github.com/meta/astryx/tree/f4fa3661a25403ded7ee4c4c2ac5c2a355eb74ef/packages/core/src/TextInput) | `Input` | Caller-owned native editing entity, selection/history/IME, optional controlled disabled and frame ID. |
| [TabList](https://github.com/meta/astryx/tree/f4fa3661a25403ded7ee4c4c2ac5c2a355eb74ef/packages/core/src/TabList) | `Tabs` | Controlled selection, native roving arrows/Home/End and focus callback; host composes panels. |
| [TextArea](https://github.com/meta/astryx/tree/f4fa3661a25403ded7ee4c4c2ac5c2a355eb74ef/packages/core/src/TextArea) | `TextArea` | Caller-owned multiline native editing entity with IME/history and optional disabled/frame ID. |
| [TimeInput](https://github.com/meta/astryx/tree/f4fa3661a25403ded7ee4c4c2ac5c2a355eb74ef/packages/core/src/TimeInput) | `TimeInput` | Retained time segments, precision/hour-cycle constraints and keyboard stepping. |
| [NumberInput](https://github.com/meta/astryx/tree/f4fa3661a25403ded7ee4c4c2ac5c2a355eb74ef/packages/core/src/NumberInput) | `NumberInput` | Native numeric editing and bounded stepping over caller-configured input state. |
| [Table](https://github.com/meta/astryx/tree/f4fa3661a25403ded7ee4c4c2ac5c2a355eb74ef/packages/core/src/Table) | `Table` | Native indexed table structure, controlled row selection and column sort requests; host sorts data. |
| [ToggleButton](https://github.com/meta/astryx/tree/f4fa3661a25403ded7ee4c4c2ac5c2a355eb74ef/packages/core/src/ToggleButton) | `ToggleButton` | Controlled native pressed state and disabled activation. |
| [Token](https://github.com/meta/astryx/tree/f4fa3661a25403ded7ee4c4c2ac5c2a355eb74ef/packages/core/src/Token) | `Token` | Selected-value chip with optional native remove action. |
| [Typeahead](https://github.com/meta/astryx/tree/f4fa3661a25403ded7ee4c4c2ac5c2a355eb74ef/packages/core/src/Typeahead) | `Typeahead` | Caller-owned query and supplied suggestions; keyboard commit respects IME composition. |
| [Tokenizer](https://github.com/meta/astryx/tree/f4fa3661a25403ded7ee4c4c2ac5c2a355eb74ef/packages/core/src/Tokenizer) | `Tokenizer` | Native atomic InlineToken spans, selection/delete/history/IME and explicit commit callback. |
| [Thumbnail](https://github.com/meta/astryx/tree/f4fa3661a25403ded7ee4c4c2ac5c2a355eb74ef/packages/core/src/Thumbnail) | `Thumbnail` | Caller image source, native loading/error presentation and label. |
| [PowerSearch](https://github.com/meta/astryx/tree/f4fa3661a25403ded7ee4c4c2ac5c2a355eb74ef/packages/core/src/PowerSearch) | `PowerSearch` | Sectioned native query/results layer; search data and ranking remain host-owned. |
| [TreeList](https://github.com/meta/astryx/tree/f4fa3661a25403ded7ee4c4c2ac5c2a355eb74ef/packages/core/src/TreeList) | `TreeList` | Retained native virtual tree, expansion/selection and scoped hierarchical keyboard navigation. |
| [Kbd](https://github.com/meta/astryx/tree/f4fa3661a25403ded7ee4c4c2ac5c2a355eb74ef/packages/core/src/Kbd) | `Kbd` | Shortcut-label presentation; does not install app keybindings. |
| [AlertDialog](https://github.com/meta/astryx/tree/f4fa3661a25403ded7ee4c4c2ac5c2a355eb74ef/packages/core/src/AlertDialog) | `AlertDialog` | Controlled confirm/cancel modal with destructive/busy behavior and valid focus trap. |
| [Dialog](https://github.com/meta/astryx/tree/f4fa3661a25403ded7ee4c4c2ac5c2a355eb74ef/packages/core/src/Dialog) | `Dialog` | Controlled modal, vetoable dismissal, initial focus/trap/restore and child-owned Enter actions. |
| [ContextMenu](https://github.com/meta/astryx/tree/f4fa3661a25403ded7ee4c4c2ac5c2a355eb74ef/packages/core/src/ContextMenu) | `ContextMenu` | Native secondary-pointer menu plus keyboard opening, selection and Escape. |
| [DropdownMenu](https://github.com/meta/astryx/tree/f4fa3661a25403ded7ee4c4c2ac5c2a355eb74ef/packages/core/src/DropdownMenu) | `DropdownMenu` | Controlled anchored menu, enabled-item keyboard navigation and focus restore. |
| [Drawer](https://github.com/meta/astryx/tree/f4fa3661a25403ded7ee4c4c2ac5c2a355eb74ef/packages/core/src/Drawer) | `Drawer` | Controlled side-aligned modal, backdrop and focus lifecycle. |
| [MoreMenu](https://github.com/meta/astryx/tree/f4fa3661a25403ded7ee4c4c2ac5c2a355eb74ef/packages/core/src/MoreMenu) | `MoreMenu` | Overflow-menu presentation over actual native menu behavior. |
| [Toolbar](https://github.com/meta/astryx/tree/f4fa3661a25403ded7ee4c4c2ac5c2a355eb74ef/packages/core/src/Toolbar) | `Toolbar` | Scoped native toolbar grouping and focus navigation. |
| [TopNav](https://github.com/meta/astryx/tree/f4fa3661a25403ded7ee4c4c2ac5c2a355eb74ef/packages/core/src/TopNav) | `TopNav` | Controlled horizontal items and native activation; routing remains host-owned. |
| [SideNav](https://github.com/meta/astryx/tree/f4fa3661a25403ded7ee4c4c2ac5c2a355eb74ef/packages/core/src/SideNav) | `SideNav` | Controlled vertical items, disabled skipping and selection callback. |
| [MobileNav](https://github.com/meta/astryx/tree/f4fa3661a25403ded7ee4c4c2ac5c2a355eb74ef/packages/core/src/MobileNav) | `MobileNav` | Compact controlled navigation presentation using native buttons. |
| [Pagination](https://github.com/meta/astryx/tree/f4fa3661a25403ded7ee4c4c2ac5c2a355eb74ef/packages/core/src/Pagination) | `Pagination` | Controlled page/count and bounded previous/next/direct navigation. |
| [ProgressBar](https://github.com/meta/astryx/tree/f4fa3661a25403ded7ee4c4c2ac5c2a355eb74ef/packages/core/src/ProgressBar) | `ProgressBar` | Finite clamped 0–100 value and native labelled progress semantics. |
| [Layout](https://github.com/meta/astryx/tree/f4fa3661a25403ded7ee4c4c2ac5c2a355eb74ef/packages/core/src/Layout) | `Layout` | Header/navigation/main/aside/footer slots with explicit widths. |
| [Layer](https://github.com/meta/astryx/tree/f4fa3661a25403ded7ee4c4c2ac5c2a355eb74ef/packages/core/src/Layer) | `Layer` | Controlled native anchored/deferred layer with edge resolution; host supplies dismissal/focus policy. |
| [Toast](https://github.com/meta/astryx/tree/f4fa3661a25403ded7ee4c4c2ac5c2a355eb74ef/packages/core/src/Toast) | `Toast` | Native toast manager, controlled timing advancement, dismiss/action callbacks and stack state. |
| [Popover](https://github.com/meta/astryx/tree/f4fa3661a25403ded7ee4c4c2ac5c2a355eb74ef/packages/core/src/Popover) | `Popover` | Controlled native anchored layer, disabled-close, outside/Escape dismissal and focus restore. |
| [HoverCard](https://github.com/meta/astryx/tree/f4fa3661a25403ded7ee4c4c2ac5c2a355eb74ef/packages/core/src/HoverCard) | `HoverCard` | Retained hover/focus open state and delayed native popup lifecycle. |
| [Tooltip](https://github.com/meta/astryx/tree/f4fa3661a25403ded7ee4c4c2ac5c2a355eb74ef/packages/core/src/Tooltip) | `Tooltip` | Native Render view/factory attached to host tooltip lifecycle; inherited hover timing. |
| [Skeleton](https://github.com/meta/astryx/tree/f4fa3661a25403ded7ee4c4c2ac5c2a355eb74ef/packages/core/src/Skeleton) | `Skeleton` | Static loading placeholder; shimmer/motion policy is not implemented implicitly. |
| [StatusDot](https://github.com/meta/astryx/tree/f4fa3661a25403ded7ee4c4c2ac5c2a355eb74ef/packages/core/src/StatusDot) | `StatusDot` | Labelled semantic status presentation. |
| [Spinner](https://github.com/meta/astryx/tree/f4fa3661a25403ded7ee4c4c2ac5c2a355eb74ef/packages/core/src/Spinner) | `Spinner` | Native rotation animation with explicit static-motion option and progress label. |
| [Timestamp](https://github.com/meta/astryx/tree/f4fa3661a25403ded7ee4c4c2ac5c2a355eb74ef/packages/core/src/Timestamp) | `Timestamp` | Absolute or caller-clock-relative label, explicit timezone and absolute tooltip. |
| [Timer](https://github.com/meta/astryx/tree/f4fa3661a25403ded7ee4c4c2ac5c2a355eb74ef/packages/core/src/Timer) | `Timer` | Retained elapsed time; explicit start/pause/reset and cancellable one-second task. |
| [Overlay](https://github.com/meta/astryx/tree/f4fa3661a25403ded7ee4c4c2ac5c2a355eb74ef/packages/core/src/Overlay) | `Overlay` | Controlled modal scrim/content with native dismissal and focus lifecycle. |
| [Outline](https://github.com/meta/astryx/tree/f4fa3661a25403ded7ee4c4c2ac5c2a355eb74ef/packages/core/src/Outline) | `Outline` | Controlled visual outline around composed native content. |
| [OverflowList](https://github.com/meta/astryx/tree/f4fa3661a25403ded7ee4c4c2ac5c2a355eb74ef/packages/core/src/OverflowList) | `OverflowList` | Explicit visible capacity, native overflow popup and retained fallback focus when overflow disappears; host may update capacity from layout. |

## Supporting exports and native adaptations

The source's React/context/utility APIs are not re-exported or simulated. These
11 source export groups are recorded separately from the 102 native UI families.

| Upstream export | Native responsibility |
| --- | --- |
| `BaseProps` | Typed Rust builders, `Styled` and `ParentElement`; stable native IDs. |
| `InteractiveRoleContext` | Native roles on behavior primitives; explicit labels on named controls. |
| `SizeContext` | Explicit `ControlSize` and caller styles rather than a React provider. |
| `hooks` | GPUI `Entity`, `Context`, `Subscription`, `Task` and scoped view listeners; no React hook API. |
| `animator/createAnimator` | Native GPUI animation elements and component-owned motion; no browser animator API. |
| `dragToSnapPoints/createDragToSnapPoints` | `SnapPaging` native pointer/scroll snapping; no general-purpose web snap-point helper. |
| `reorderable/createReorderable` | `List` native scoped drag/drop and owner reorder callback; no DOM reorder utility. |
| `gestures/createPanGesture` | `Swipe` pointer gesture and native resizable/scroll backends; no browser gesture helper. |
| `utils` | Rust/GPUI standard geometry, callbacks and state; source JS utility API is not a public Rust compatibility layer. |
| `theme` | Explicit customizable `Theme` on themed surfaces/controls; decoration-free geometry inherits caller styles. |
| `i18n` | Caller labels, date/time model options and explicit timestamp timezone; no translated locale catalog or browser locale provider. |

## Host boundaries

Hosts supply data, URL routing, chat providers, persistence, image assets, file
contents, search ranking and application-wide focus/key policy. File selection
returns paths without reading/uploading contents. Rendering does not launch an
agent, access private app state, or start business/network jobs. Retained models
such as input, tree, slider and timer need the owning view to observe their
changes when it should redraw, as shown in the native examples.

The tested platform is Apple Silicon macOS. Physical VoiceOver, Linux/Windows,
touch-specific web behaviors, every image codec/network source, automatic text
translation, every upstream prop/variant and universal responsive parity remain
unverified or explicitly outside these native contracts.
