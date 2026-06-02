---
title: Configuration
layout: doc
sidebar: true
search: true
access: public
---

# Configuration

RustPress uses `rustpress.toml` instead of VitePress project conventions.

## Example

```toml
title = "My Docs"
src_dir = "docs"
out_dir = "dist"
base = "/"

[[nav]]
text = "Guide"
link = "/guide/cli/"

[[nav.items]]
text = "CLI"
link = "/guide/cli/"

[[nav.items]]
text = "Configuration"
link = "/guide/configuration/"

[theme]
name = "default"
skin = "light"
allow_switch = true
github_url = "https://github.com/your-org/your-repo"

[markdown]
mermaid = true
code_highlight = true
code_line_numbers = true
heading_anchors = true

[search]
enabled = true
languages = ["zh", "en", "ja", "ko"]
index_code = false

[access]
enabled = true
mode = "mask"
password = "demo123"
password_hint = "Enter password"
```

## Frontmatter

```yaml
---
title: Page Title
layout: doc
sidebar: true
search: true
access: public
---
```

`access` can be `public` or `masked`. Masked pages show the access overlay only when `[access].password` is configured. `search: false` excludes the page from the generated search index.

## Top Navigation and Sidebars

Use `[[nav]]` to render top navigation links or grouped menus.

```toml
[[nav]]
text = "Guide"
link = "/guide/cli/"
sidebar = "guide"

[[nav.items]]
text = "CLI"
link = "/guide/cli/"

[[nav.items]]
text = "Configuration"
link = "/guide/configuration/"

[[nav]]
text = "Reference"
link = "/internals/crates/"
sidebar = "reference"

[[sidebars.guide]]
text = "Guide"
link = "/guide/cli/"

[[sidebars.guide.items]]
text = "CLI"
link = "/guide/cli/"

[[sidebars.guide.items]]
text = "Configuration"
link = "/guide/configuration/"

[[sidebars.reference]]
text = "Reference"
link = "/internals/crates/"

[[sidebars.reference.items]]
text = "Crates"
link = "/internals/crates/"
```

When `items` are present, the theme renders a dropdown menu. When `items` are omitted, the item renders as a direct top-level link.

Add `sidebar = "name"` to a top navigation item to bind pages in that section to `sidebars.name`. If no `sidebars` are configured, RustPress keeps the legacy behavior and builds the sidebar from Markdown pages plus `nav` ordering.

## Multilingual Docs

RustPress is single-language by default. Add `locales` to opt in to URL-based multilingual docs. When `locales` is configured, `locales.root` is required and represents the default language at `/`.

```toml
[locales.root]
label = "简体中文"
lang = "zh-CN"

[locales.en]
label = "English"
lang = "en-US"
link = "/en/"

[locales.ja]
label = "日本語"
lang = "ja-JP"
link = "/ja/"

[locales.ko]
label = "한국어"
lang = "ko-KR"
link = "/ko/"

[[locales.en.nav]]
text = "Guide"
link = "guide/cli/"
sidebar = "guide"

[[locales.en.sidebars.guide]]
text = "Guide"
link = "guide/cli/"

[[locales.en.sidebars.guide.items]]
text = "CLI"
link = "guide/cli/"
```

The root language keeps using files directly under `docs/`. Other locale files live in `docs/<locale>/`.

```text
docs/index.md              -> /
docs/guide/cli.md          -> /guide/cli/
docs/en/index.md           -> /en/
docs/en/guide/cli.md       -> /en/guide/cli/
docs/ja/index.md           -> /ja/
docs/ko/index.md           -> /ko/
```

Non-root locale links default to `/<locale>/`; use `link` to override that prefix. Locale `nav`, `sidebars`, and `title` override the global values, and fall back to global config when omitted. Relative locale nav and sidebar links are resolved under that locale prefix, so `guide/cli/` in `locales.en.nav` or `locales.en.sidebars.guide` becomes `/en/guide/cli/`.

The language selector appears in the top bar only when `locales` is configured. It switches to the matching translated page when one exists. If a translation is missing, it links to that language's home page.
