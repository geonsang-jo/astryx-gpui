# Native coverage inventory

Source: Meta Astryx `f4fa3661a25403ded7ee4c4c2ac5c2a355eb74ef`, public `packages/core/src/index.ts`.

This inventory tracks **102 UI families** and **11 supporting exports**. A native adaptation is a working GPUI counterpart, not React API compatibility. A source family remains pending until its core native behavior and relevant interaction checks are present. Aliases and documentation alone do not count as completed ports.

The first expanded batch supplies 21 native UI counterparts. Controls remain
explicit about supported behavior; this is not full source-prop or React API parity.
`Skeleton` is static and follows host animation policy.

## Public UI families

| Upstream family | Native API | Status |
| --- | --- | --- |
| [AppShell](https://github.com/meta/astryx/tree/f4fa3661a25403ded7ee4c4c2ac5c2a355eb74ef/packages/core/src/AppShell) | `—` | Pending |
| [AspectRatio](https://github.com/meta/astryx/tree/f4fa3661a25403ded7ee4c4c2ac5c2a355eb74ef/packages/core/src/AspectRatio) | `—` | Pending |
| [Avatar](https://github.com/meta/astryx/tree/f4fa3661a25403ded7ee4c4c2ac5c2a355eb74ef/packages/core/src/Avatar) | `—` | Pending |
| [AvatarGroup](https://github.com/meta/astryx/tree/f4fa3661a25403ded7ee4c4c2ac5c2a355eb74ef/packages/core/src/AvatarGroup) | `—` | Pending |
| [Badge](https://github.com/meta/astryx/tree/f4fa3661a25403ded7ee4c4c2ac5c2a355eb74ef/packages/core/src/Badge) | `Badge` | Native counterpart |
| [Banner](https://github.com/meta/astryx/tree/f4fa3661a25403ded7ee4c4c2ac5c2a355eb74ef/packages/core/src/Banner) | `Banner` | Native counterpart |
| [Blockquote](https://github.com/meta/astryx/tree/f4fa3661a25403ded7ee4c4c2ac5c2a355eb74ef/packages/core/src/Blockquote) | `—` | Pending |
| [BottomSheet](https://github.com/meta/astryx/tree/f4fa3661a25403ded7ee4c4c2ac5c2a355eb74ef/packages/core/src/BottomSheet) | `—` | Pending |
| [Breadcrumbs](https://github.com/meta/astryx/tree/f4fa3661a25403ded7ee4c4c2ac5c2a355eb74ef/packages/core/src/Breadcrumbs) | `—` | Pending |
| [Button](https://github.com/meta/astryx/tree/f4fa3661a25403ded7ee4c4c2ac5c2a355eb74ef/packages/core/src/Button) | `Button` | Native counterpart |
| [ButtonGroup](https://github.com/meta/astryx/tree/f4fa3661a25403ded7ee4c4c2ac5c2a355eb74ef/packages/core/src/ButtonGroup) | `—` | Pending |
| [IconButton](https://github.com/meta/astryx/tree/f4fa3661a25403ded7ee4c4c2ac5c2a355eb74ef/packages/core/src/IconButton) | `—` | Pending |
| [Card](https://github.com/meta/astryx/tree/f4fa3661a25403ded7ee4c4c2ac5c2a355eb74ef/packages/core/src/Card) | `Card` | Native counterpart |
| [ClickableCard](https://github.com/meta/astryx/tree/f4fa3661a25403ded7ee4c4c2ac5c2a355eb74ef/packages/core/src/ClickableCard) | `—` | Pending |
| [Carousel](https://github.com/meta/astryx/tree/f4fa3661a25403ded7ee4c4c2ac5c2a355eb74ef/packages/core/src/Carousel) | `—` | Pending |
| [SnapPaging](https://github.com/meta/astryx/tree/f4fa3661a25403ded7ee4c4c2ac5c2a355eb74ef/packages/core/src/SnapPaging) | `—` | Pending |
| [Calendar](https://github.com/meta/astryx/tree/f4fa3661a25403ded7ee4c4c2ac5c2a355eb74ef/packages/core/src/Calendar) | `—` | Pending |
| [Center](https://github.com/meta/astryx/tree/f4fa3661a25403ded7ee4c4c2ac5c2a355eb74ef/packages/core/src/Center) | `—` | Pending |
| [CodeBlock](https://github.com/meta/astryx/tree/f4fa3661a25403ded7ee4c4c2ac5c2a355eb74ef/packages/core/src/CodeBlock) | `—` | Pending |
| [CommandPalette](https://github.com/meta/astryx/tree/f4fa3661a25403ded7ee4c4c2ac5c2a355eb74ef/packages/core/src/CommandPalette) | `—` | Pending |
| [ComplexSelector](https://github.com/meta/astryx/tree/f4fa3661a25403ded7ee4c4c2ac5c2a355eb74ef/packages/core/src/ComplexSelector) | `—` | Pending |
| [Chat](https://github.com/meta/astryx/tree/f4fa3661a25403ded7ee4c4c2ac5c2a355eb74ef/packages/core/src/Chat) | `—` | Pending |
| [Markdown](https://github.com/meta/astryx/tree/f4fa3661a25403ded7ee4c4c2ac5c2a355eb74ef/packages/core/src/Markdown) | `—` | Pending |
| [Citation](https://github.com/meta/astryx/tree/f4fa3661a25403ded7ee4c4c2ac5c2a355eb74ef/packages/core/src/Citation) | `—` | Pending |
| [CheckboxInput](https://github.com/meta/astryx/tree/f4fa3661a25403ded7ee4c4c2ac5c2a355eb74ef/packages/core/src/CheckboxInput) | `Checkbox` | Native counterpart |
| [CheckboxList](https://github.com/meta/astryx/tree/f4fa3661a25403ded7ee4c4c2ac5c2a355eb74ef/packages/core/src/CheckboxList) | `—` | Pending |
| [Collapsible](https://github.com/meta/astryx/tree/f4fa3661a25403ded7ee4c4c2ac5c2a355eb74ef/packages/core/src/Collapsible) | `Collapsible` | Native counterpart |
| [RadioList](https://github.com/meta/astryx/tree/f4fa3661a25403ded7ee4c4c2ac5c2a355eb74ef/packages/core/src/RadioList) | `—` | Pending |
| [Resizable](https://github.com/meta/astryx/tree/f4fa3661a25403ded7ee4c4c2ac5c2a355eb74ef/packages/core/src/Resizable) | `—` | Pending |
| [ScrollableArea](https://github.com/meta/astryx/tree/f4fa3661a25403ded7ee4c4c2ac5c2a355eb74ef/packages/core/src/ScrollableArea) | `—` | Pending |
| [Divider](https://github.com/meta/astryx/tree/f4fa3661a25403ded7ee4c4c2ac5c2a355eb74ef/packages/core/src/Divider) | `Divider` | Native counterpart |
| [VisuallyHidden](https://github.com/meta/astryx/tree/f4fa3661a25403ded7ee4c4c2ac5c2a355eb74ef/packages/core/src/VisuallyHidden) | `—` | Pending |
| [EmptyState](https://github.com/meta/astryx/tree/f4fa3661a25403ded7ee4c4c2ac5c2a355eb74ef/packages/core/src/EmptyState) | `EmptyState` | Native counterpart |
| [Lightbox](https://github.com/meta/astryx/tree/f4fa3661a25403ded7ee4c4c2ac5c2a355eb74ef/packages/core/src/Lightbox) | `—` | Pending |
| [Link](https://github.com/meta/astryx/tree/f4fa3661a25403ded7ee4c4c2ac5c2a355eb74ef/packages/core/src/Link) | `—` | Pending |
| [List](https://github.com/meta/astryx/tree/f4fa3661a25403ded7ee4c4c2ac5c2a355eb74ef/packages/core/src/List) | `—` | Pending |
| [MetadataList](https://github.com/meta/astryx/tree/f4fa3661a25403ded7ee4c4c2ac5c2a355eb74ef/packages/core/src/MetadataList) | `—` | Pending |
| [NavIcon](https://github.com/meta/astryx/tree/f4fa3661a25403ded7ee4c4c2ac5c2a355eb74ef/packages/core/src/NavIcon) | `—` | Pending |
| [NavMenu](https://github.com/meta/astryx/tree/f4fa3661a25403ded7ee4c4c2ac5c2a355eb74ef/packages/core/src/NavMenu) | `—` | Pending |
| [Slider](https://github.com/meta/astryx/tree/f4fa3661a25403ded7ee4c4c2ac5c2a355eb74ef/packages/core/src/Slider) | `—` | Pending |
| [Stack](https://github.com/meta/astryx/tree/f4fa3661a25403ded7ee4c4c2ac5c2a355eb74ef/packages/core/src/Stack) | `—` | Pending |
| [Stepper](https://github.com/meta/astryx/tree/f4fa3661a25403ded7ee4c4c2ac5c2a355eb74ef/packages/core/src/Stepper) | `—` | Pending |
| [Switch](https://github.com/meta/astryx/tree/f4fa3661a25403ded7ee4c4c2ac5c2a355eb74ef/packages/core/src/Switch) | `Switch` | Native counterpart |
| [DateInput](https://github.com/meta/astryx/tree/f4fa3661a25403ded7ee4c4c2ac5c2a355eb74ef/packages/core/src/DateInput) | `—` | Pending |
| [DateTimeInput](https://github.com/meta/astryx/tree/f4fa3661a25403ded7ee4c4c2ac5c2a355eb74ef/packages/core/src/DateTimeInput) | `—` | Pending |
| [DateRangeInput](https://github.com/meta/astryx/tree/f4fa3661a25403ded7ee4c4c2ac5c2a355eb74ef/packages/core/src/DateRangeInput) | `—` | Pending |
| [Field](https://github.com/meta/astryx/tree/f4fa3661a25403ded7ee4c4c2ac5c2a355eb74ef/packages/core/src/Field) | `—` | Pending |
| [FileInput](https://github.com/meta/astryx/tree/f4fa3661a25403ded7ee4c4c2ac5c2a355eb74ef/packages/core/src/FileInput) | `—` | Pending |
| [FormLayout](https://github.com/meta/astryx/tree/f4fa3661a25403ded7ee4c4c2ac5c2a355eb74ef/packages/core/src/FormLayout) | `—` | Pending |
| [Grid](https://github.com/meta/astryx/tree/f4fa3661a25403ded7ee4c4c2ac5c2a355eb74ef/packages/core/src/Grid) | `—` | Pending |
| [Section](https://github.com/meta/astryx/tree/f4fa3661a25403ded7ee4c4c2ac5c2a355eb74ef/packages/core/src/Section) | `—` | Pending |
| [SegmentedControl](https://github.com/meta/astryx/tree/f4fa3661a25403ded7ee4c4c2ac5c2a355eb74ef/packages/core/src/SegmentedControl) | `—` | Pending |
| [SelectableCard](https://github.com/meta/astryx/tree/f4fa3661a25403ded7ee4c4c2ac5c2a355eb74ef/packages/core/src/SelectableCard) | `—` | Pending |
| [Selector](https://github.com/meta/astryx/tree/f4fa3661a25403ded7ee4c4c2ac5c2a355eb74ef/packages/core/src/Selector) | `Select` | Native counterpart |
| [MultiSelector](https://github.com/meta/astryx/tree/f4fa3661a25403ded7ee4c4c2ac5c2a355eb74ef/packages/core/src/MultiSelector) | `—` | Pending |
| [Icon](https://github.com/meta/astryx/tree/f4fa3661a25403ded7ee4c4c2ac5c2a355eb74ef/packages/core/src/Icon) | `—` | Pending |
| [InfoTip](https://github.com/meta/astryx/tree/f4fa3661a25403ded7ee4c4c2ac5c2a355eb74ef/packages/core/src/InfoTip) | `—` | Pending |
| [Indicator](https://github.com/meta/astryx/tree/f4fa3661a25403ded7ee4c4c2ac5c2a355eb74ef/packages/core/src/Indicator) | `—` | Pending |
| [InputGroup](https://github.com/meta/astryx/tree/f4fa3661a25403ded7ee4c4c2ac5c2a355eb74ef/packages/core/src/InputGroup) | `—` | Pending |
| [Item](https://github.com/meta/astryx/tree/f4fa3661a25403ded7ee4c4c2ac5c2a355eb74ef/packages/core/src/Item) | `—` | Pending |
| [Swipe](https://github.com/meta/astryx/tree/f4fa3661a25403ded7ee4c4c2ac5c2a355eb74ef/packages/core/src/Swipe) | `—` | Pending |
| [Text](https://github.com/meta/astryx/tree/f4fa3661a25403ded7ee4c4c2ac5c2a355eb74ef/packages/core/src/Text) | `Text` | Native counterpart |
| [TextInput](https://github.com/meta/astryx/tree/f4fa3661a25403ded7ee4c4c2ac5c2a355eb74ef/packages/core/src/TextInput) | `Input` | Native counterpart |
| [TabList](https://github.com/meta/astryx/tree/f4fa3661a25403ded7ee4c4c2ac5c2a355eb74ef/packages/core/src/TabList) | `Tabs` | Native counterpart |
| [TextArea](https://github.com/meta/astryx/tree/f4fa3661a25403ded7ee4c4c2ac5c2a355eb74ef/packages/core/src/TextArea) | `TextArea` | Native counterpart |
| [TimeInput](https://github.com/meta/astryx/tree/f4fa3661a25403ded7ee4c4c2ac5c2a355eb74ef/packages/core/src/TimeInput) | `—` | Pending |
| [NumberInput](https://github.com/meta/astryx/tree/f4fa3661a25403ded7ee4c4c2ac5c2a355eb74ef/packages/core/src/NumberInput) | `—` | Pending |
| [Table](https://github.com/meta/astryx/tree/f4fa3661a25403ded7ee4c4c2ac5c2a355eb74ef/packages/core/src/Table) | `—` | Pending |
| [ToggleButton](https://github.com/meta/astryx/tree/f4fa3661a25403ded7ee4c4c2ac5c2a355eb74ef/packages/core/src/ToggleButton) | `—` | Pending |
| [Token](https://github.com/meta/astryx/tree/f4fa3661a25403ded7ee4c4c2ac5c2a355eb74ef/packages/core/src/Token) | `—` | Pending |
| [Typeahead](https://github.com/meta/astryx/tree/f4fa3661a25403ded7ee4c4c2ac5c2a355eb74ef/packages/core/src/Typeahead) | `—` | Pending |
| [Tokenizer](https://github.com/meta/astryx/tree/f4fa3661a25403ded7ee4c4c2ac5c2a355eb74ef/packages/core/src/Tokenizer) | `—` | Pending |
| [Thumbnail](https://github.com/meta/astryx/tree/f4fa3661a25403ded7ee4c4c2ac5c2a355eb74ef/packages/core/src/Thumbnail) | `—` | Pending |
| [PowerSearch](https://github.com/meta/astryx/tree/f4fa3661a25403ded7ee4c4c2ac5c2a355eb74ef/packages/core/src/PowerSearch) | `—` | Pending |
| [TreeList](https://github.com/meta/astryx/tree/f4fa3661a25403ded7ee4c4c2ac5c2a355eb74ef/packages/core/src/TreeList) | `—` | Pending |
| [Kbd](https://github.com/meta/astryx/tree/f4fa3661a25403ded7ee4c4c2ac5c2a355eb74ef/packages/core/src/Kbd) | `Kbd` | Native counterpart |
| [AlertDialog](https://github.com/meta/astryx/tree/f4fa3661a25403ded7ee4c4c2ac5c2a355eb74ef/packages/core/src/AlertDialog) | `—` | Pending |
| [Dialog](https://github.com/meta/astryx/tree/f4fa3661a25403ded7ee4c4c2ac5c2a355eb74ef/packages/core/src/Dialog) | `Dialog` | Native counterpart |
| [ContextMenu](https://github.com/meta/astryx/tree/f4fa3661a25403ded7ee4c4c2ac5c2a355eb74ef/packages/core/src/ContextMenu) | `—` | Pending |
| [DropdownMenu](https://github.com/meta/astryx/tree/f4fa3661a25403ded7ee4c4c2ac5c2a355eb74ef/packages/core/src/DropdownMenu) | `—` | Pending |
| [Drawer](https://github.com/meta/astryx/tree/f4fa3661a25403ded7ee4c4c2ac5c2a355eb74ef/packages/core/src/Drawer) | `—` | Pending |
| [MoreMenu](https://github.com/meta/astryx/tree/f4fa3661a25403ded7ee4c4c2ac5c2a355eb74ef/packages/core/src/MoreMenu) | `—` | Pending |
| [Toolbar](https://github.com/meta/astryx/tree/f4fa3661a25403ded7ee4c4c2ac5c2a355eb74ef/packages/core/src/Toolbar) | `—` | Pending |
| [TopNav](https://github.com/meta/astryx/tree/f4fa3661a25403ded7ee4c4c2ac5c2a355eb74ef/packages/core/src/TopNav) | `—` | Pending |
| [SideNav](https://github.com/meta/astryx/tree/f4fa3661a25403ded7ee4c4c2ac5c2a355eb74ef/packages/core/src/SideNav) | `—` | Pending |
| [MobileNav](https://github.com/meta/astryx/tree/f4fa3661a25403ded7ee4c4c2ac5c2a355eb74ef/packages/core/src/MobileNav) | `—` | Pending |
| [Pagination](https://github.com/meta/astryx/tree/f4fa3661a25403ded7ee4c4c2ac5c2a355eb74ef/packages/core/src/Pagination) | `—` | Pending |
| [ProgressBar](https://github.com/meta/astryx/tree/f4fa3661a25403ded7ee4c4c2ac5c2a355eb74ef/packages/core/src/ProgressBar) | `ProgressBar` | Native counterpart |
| [Layout](https://github.com/meta/astryx/tree/f4fa3661a25403ded7ee4c4c2ac5c2a355eb74ef/packages/core/src/Layout) | `—` | Pending |
| [Layer](https://github.com/meta/astryx/tree/f4fa3661a25403ded7ee4c4c2ac5c2a355eb74ef/packages/core/src/Layer) | `—` | Pending |
| [Toast](https://github.com/meta/astryx/tree/f4fa3661a25403ded7ee4c4c2ac5c2a355eb74ef/packages/core/src/Toast) | `—` | Pending |
| [Popover](https://github.com/meta/astryx/tree/f4fa3661a25403ded7ee4c4c2ac5c2a355eb74ef/packages/core/src/Popover) | `Popover` | Native counterpart |
| [HoverCard](https://github.com/meta/astryx/tree/f4fa3661a25403ded7ee4c4c2ac5c2a355eb74ef/packages/core/src/HoverCard) | `—` | Pending |
| [Tooltip](https://github.com/meta/astryx/tree/f4fa3661a25403ded7ee4c4c2ac5c2a355eb74ef/packages/core/src/Tooltip) | `Tooltip` | Native counterpart |
| [Skeleton](https://github.com/meta/astryx/tree/f4fa3661a25403ded7ee4c4c2ac5c2a355eb74ef/packages/core/src/Skeleton) | `Skeleton` | Native counterpart |
| [StatusDot](https://github.com/meta/astryx/tree/f4fa3661a25403ded7ee4c4c2ac5c2a355eb74ef/packages/core/src/StatusDot) | `StatusDot` | Native counterpart |
| [Spinner](https://github.com/meta/astryx/tree/f4fa3661a25403ded7ee4c4c2ac5c2a355eb74ef/packages/core/src/Spinner) | `—` | Pending |
| [Timestamp](https://github.com/meta/astryx/tree/f4fa3661a25403ded7ee4c4c2ac5c2a355eb74ef/packages/core/src/Timestamp) | `—` | Pending |
| [Timer](https://github.com/meta/astryx/tree/f4fa3661a25403ded7ee4c4c2ac5c2a355eb74ef/packages/core/src/Timer) | `—` | Pending |
| [Overlay](https://github.com/meta/astryx/tree/f4fa3661a25403ded7ee4c4c2ac5c2a355eb74ef/packages/core/src/Overlay) | `—` | Pending |
| [Outline](https://github.com/meta/astryx/tree/f4fa3661a25403ded7ee4c4c2ac5c2a355eb74ef/packages/core/src/Outline) | `—` | Pending |
| [OverflowList](https://github.com/meta/astryx/tree/f4fa3661a25403ded7ee4c4c2ac5c2a355eb74ef/packages/core/src/OverflowList) | `—` | Pending |

## Supporting exports

| Upstream export | Native responsibility |
| --- | --- |
| `BaseProps` | Rust builders, Styled and ParentElement |
| `InteractiveRoleContext` | Native GPUI roles on behavior primitives |
| `SizeContext` | Explicit ControlSize |
| `hooks` | Native counterpart pending; not counted as a UI family |
| `animator/createAnimator` | Native counterpart pending; not counted as a UI family |
| `dragToSnapPoints/createDragToSnapPoints` | Native counterpart pending; not counted as a UI family |
| `reorderable/createReorderable` | Native counterpart pending; not counted as a UI family |
| `gestures/createPanGesture` | Native counterpart pending; not counted as a UI family |
| `utils` | Native counterpart pending; not counted as a UI family |
| `theme` | Explicit Theme and semantic colors |
| `i18n` | Caller-supplied text and locale-dependent formatting; no implicit browser locale |
