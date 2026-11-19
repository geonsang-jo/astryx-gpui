# Full native catalogue port

This port covers the original Astryx catalogue for native GPUI hosts. The source
inventory is pinned, with 102 UI families and 11 infrastructure/support export
groups tracked in [coverage](coverage.md).

## Contracts

- Each completed family must expose a working native representation of its core
  purpose, with controlled state or caller-owned native entities as appropriate.
- Reuse public `gpui-base 0.7.1` primitives for native editing, focus, activation,
  layers and collection behavior. Do not copy private host patches into this repo.
- Aliases, inert placeholders and README promises do not count as completed ports.
- Platform adapters may leave file dialogs, URL navigation, provider/network work,
  persistence and app-global focus policy to explicit caller callbacks.
- Themed surfaces/controls use explicit Theme; stable IDs and native disabled behavior
  are part of interactive controls. No business I/O starts while rendering.
- Public manifests remain reproducible and independent of Herdr. Native MIT
  code retains the full Meta notice and clearly identifies the unofficial port.

## Batches

- [x] Existing theme and eight native component families, public bootstrap.
- [x] Input/TextArea/Select, Dialog/Popover/Tooltip, Switch and status primitives.
- [x] Form groups, radio/slider/calendar/date/time/number/file controls and selectors.
- [x] Modal variants, menus, hover content, notifications, command/search layers.
- [x] Layout/app/navigation shells, media, lists, tables and text/chat content.
- [x] Stateful geometry/collections, pagination, resizable/scroll and overflow.
- [x] Verify the complete source inventory has a concrete native mapping and tests.
- [x] Publish final audited code with reproducible native host pin guidance.

## Verification and release

Each coherent batch runs formatting, clippy with warnings denied, native tests
and example compilation. Behavioral changes get focused native event tests.
Gallery screenshots are captured through public HeadlessAppContext with the
platform text system and Metal renderer, including light and dark appearance.
Independent workers cross-review meaningful controls before publication.

Verified batches are published separately so a consuming application can pin
real library source while remaining families continue. The host owns its native
adapter and installation; no host files, sessions or credentials enter this repo.
