# License

This project is multi-licensed. You may use it under **any one** of the
following licenses, at your option:

| License | SPDX identifier | Full text |
| ------- | --------------- | --------- |
| MIT License | `MIT` | [`LICENSES/MIT.txt`](LICENSES/MIT.txt) — also <https://opensource.org/license/mit/> |
| Apache License 2.0 | `Apache-2.0` | [`LICENSES/Apache-2.0.txt`](LICENSES/Apache-2.0.txt) — also <https://www.apache.org/licenses/LICENSE-2.0> |
| BSD 3-Clause License | `BSD-3-Clause` | [`LICENSES/BSD-3-Clause.txt`](LICENSES/BSD-3-Clause.txt) — also <https://opensource.org/license/bsd-3-clause/> |
| GNU General Public License v2.0 only | `GPL-2.0-only` | [`LICENSES/GPL-2.0-only.txt`](LICENSES/GPL-2.0-only.txt) — also <https://www.gnu.org/licenses/old-licenses/gpl-2.0.html> |
| GNU General Public License v3.0 only | `GPL-3.0-only` | [`LICENSES/GPL-3.0-only.txt`](LICENSES/GPL-3.0-only.txt) — also <https://www.gnu.org/licenses/gpl-3.0.html> |

The full text of every option is in [`LICENSES/`](LICENSES/), one file per
SPDX identifier — the [REUSE](https://reuse.software/) convention, which
exists for repositories that offer more than one license. Added 2026-08-26;
before then this file offered five licenses by URL and the repository
shipped the text of none of them. MIT, Apache-2.0, and BSD-3-Clause each
require the license text to travel with the software, so a URL was not
sufficient.

The SPDX license expression for every published crate in this workspace,
as it appears in each crate's `Cargo.toml`, is:

```
MIT OR Apache-2.0 OR BSD-3-Clause OR GPL-2.0-only OR GPL-3.0-only
```

Pick the one that fits your project and comply with that one. You do not
need to comply with all five, and you do not need to tell anyone which you
chose.

Copyright © Joel Parker Henderson <joel@joelparkerhenderson.com>

## Why five

Healthcare integration code ends up inside organisations with very
different legal constraints: a permissive license suits a vendor
integrating into a proprietary product, while a copyleft license suits a
public-sector project that wants derivatives kept open. Offering the choice
means neither has to ask.

## Scope

This file states the license for the workspace as a whole, including this
root's shared documentation and specification.

Each published crate also carries its own `LICENSE.md`, with the same five
licenses and the same expression, because a crate published to crates.io
must carry its license with it:

- [`er7/LICENSE.md`](er7/LICENSE.md)
- [`er7-redact/LICENSE.md`](er7-redact/LICENSE.md)
- [`serde-er7/LICENSE.md`](serde-er7/LICENSE.md)
- [`er7-rust.github.io/LICENSE.md`](er7-rust.github.io/LICENSE.md) — the
  documentation site

If a crate's own `LICENSE.md` ever disagrees with this file, that crate's
file governs that crate, and the disagreement is a bug worth reporting.

## Machine-readable summary

```yaml
spdx-license-expression: MIT OR Apache-2.0 OR BSD-3-Clause OR GPL-2.0-only OR GPL-3.0-only
copyright-holder: Joel Parker Henderson
contact: joel@joelparkerhenderson.com
```
