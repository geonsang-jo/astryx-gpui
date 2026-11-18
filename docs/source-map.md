# Upstream source map

All Astryx links below refer to the same reviewed commit:
`f4fa3661a25403ded7ee4c4c2ac5c2a355eb74ef`.

| Native surface | Pinned upstream reference | Adaptation |
| --- | --- | --- |
| Theme | [neutralTheme.ts](https://github.com/meta/astryx/blob/f4fa3661a25403ded7ee4c4c2ac5c2a355eb74ef/packages/themes/neutral/src/neutralTheme.ts), [neutralPaletteRefs.generated.ts](https://github.com/meta/astryx/blob/f4fa3661a25403ded7ee4c4c2ac5c2a355eb74ef/packages/themes/neutral/src/neutralPaletteRefs.generated.ts) | Explicit Rust semantic color/geometry values for two appearances |
| Text | [Text.tsx](https://github.com/meta/astryx/blob/f4fa3661a25403ded7ee4c4c2ac5c2a355eb74ef/packages/core/src/Text/Text.tsx) | Three desktop typography roles with absolute pixel line heights |
| Button | [Button.tsx](https://github.com/meta/astryx/blob/f4fa3661a25403ded7ee4c4c2ac5c2a355eb74ef/packages/core/src/Button/Button.tsx), [interactionOverlay.stylex.ts](https://github.com/meta/astryx/blob/f4fa3661a25403ded7ee4c4c2ac5c2a355eb74ef/packages/core/src/utils/interactionOverlay.stylex.ts) | Four visual treatments on native `gpui-base` activation; contrasting keyboard focus |
| Card | [Card.tsx](https://github.com/meta/astryx/blob/f4fa3661a25403ded7ee4c4c2ac5c2a355eb74ef/packages/core/src/Card/Card.tsx) and Neutral overrides | Flat 12px radius / 12px padding container with native child composition |
| Badge | [Badge.tsx](https://github.com/meta/astryx/blob/f4fa3661a25403ded7ee4c4c2ac5c2a355eb74ef/packages/core/src/Badge/Badge.tsx) and Neutral overrides | Exact named background/foreground status pairs |
| Divider | [Divider.tsx](https://github.com/meta/astryx/blob/f4fa3661a25403ded7ee4c4c2ac5c2a355eb74ef/packages/core/src/Divider/Divider.tsx) | Horizontal native color separator |
| Checkbox | [CheckboxInput.tsx](https://github.com/meta/astryx/blob/f4fa3661a25403ded7ee4c4c2ac5c2a355eb74ef/packages/core/src/CheckboxInput/CheckboxInput.tsx) | Boolean controlled value; native semantic checkbox behavior; system check glyph |
| Tabs | [TabList.tsx](https://github.com/meta/astryx/blob/f4fa3661a25403ded7ee4c4c2ac5c2a355eb74ef/packages/core/src/TabList/TabList.tsx), [Tab.tsx](https://github.com/meta/astryx/blob/f4fa3661a25403ded7ee4c4c2ac5c2a355eb74ef/packages/core/src/TabList/Tab.tsx) | Horizontal controlled selection; retained keyed focus and native roving keyboard navigation |
| Collapsible | [Collapsible.tsx](https://github.com/meta/astryx/blob/f4fa3661a25403ded7ee4c4c2ac5c2a355eb74ef/packages/core/src/Collapsible/Collapsible.tsx) | Controlled disclosure; hidden content is not laid out |

The Rust API and behavior are native adaptations, not copied React internals.
The first slice omits animation, React polymorphism, rich content slots,
indeterminate checkbox state, vertical tabs, overflow menus, panels and advanced
accessibility relationships. Host applications compose their own tab content.
No trademark graphics, custom font files or icon library assets were copied.

The source license and copyright notice are reproduced in
[THIRD_PARTY_NOTICES.md](../THIRD_PARTY_NOTICES.md).
