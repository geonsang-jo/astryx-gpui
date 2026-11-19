# Third-party notices

## Astryx

This is an unofficial community implementation for native GPUI. Meta does not
maintain or endorse this project. Astryx source measurements and design tokens
were adapted from [meta/astryx](https://github.com/meta/astryx), reference commit
`f4fa3661a25403ded7ee4c4c2ac5c2a355eb74ef`.

No Meta logo, Figtree font files or Lucide icon assets are bundled.

```text
MIT License

Copyright (c) 2026 Meta Platforms, Inc.

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
```

## Native dependencies

The public library uses `gpui-pre 0.3.8` and `gpui-base 0.7.1`, licensed
Apache-2.0. Their original notices are retained in their distributed crates.
This project does not vendor or modify those dependencies.
`chrono` supplies date/time values and formatting under its MIT OR Apache-2.0 license; its distributed crate retains its notices. Consumers who
redistribute a binary must include the required dependency licenses/notices
for their resolved dependency graph; Cargo.lock records the gallery's graph.

The project is not the Zed application and does not include its application
source. The separate upstream `gpui` package has a different Rust type identity
from `gpui-pre`; the current compatibility target is stated in the README.
