# astryx-gpui

An unofficial native Rust port of the Astryx visual language. Preserve source
attribution in `THIRD_PARTY_NOTICES.md`. Do not imply Meta endorsement.

- Public dependencies only; no local/private paths in committed manifests.
- Keep component state controlled by the owner. Reuse `gpui-base` behavior for
  keyboard activation, disabled state and stable focus identity.
- The library must not install host-wide Tab or application keybindings.
- Use native GPUI lengths (`px`) for absolute line heights.
- First release targets the Herdr-compatible `gpui-pre 0.3.8` family. Do not
  claim type compatibility with the separate upstream `gpui` package.
- Run `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`,
  `cargo test --lib --tests` and `cargo check --examples` before publishing.
- Keep examples, artifacts and tests synthetic. Never copy app/session state,
  credentials, private repositories or vendored Herdr history into this repo.
- Native screenshots come from the gallery's headless capture mode; no HTML
  screenshot mocks. Document the platform and renderer actually tested.
