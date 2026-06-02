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

[[top_nav]]
text = "Guide"
link = "/guide/installation/"

[[top_nav.items]]
text = "Quick Start"
link = "/guide/installation/"

[[top_nav.items]]
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

Use `[[top_nav]]` to render top navigation links or grouped menus.

```toml
[[top_nav]]
text = "Guide"
link = "/guide/installation/"
sidebar = "guide"

[[top_nav.items]]
text = "Quick Start"
link = "/guide/installation/"

[[top_nav.items]]
text = "Markdown"
link = "/guide/markdown-tutorial/"

[[top_nav]]
text = "Reference"
link = "/internals/crates/"
sidebar = "reference"

[[sidebars.guide]]
text = "Guide"
link = "/guide/installation/"

[[sidebars.guide.items]]
text = "Installation"
link = "/guide/installation/"

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

`top_nav.items` only controls the top dropdown menu. When `items` are omitted, the item renders as a direct top-level link.

`sidebars.<name>.items` controls the left sidebar. Add `sidebar = "name"` to a top navigation item to bind that top-level section to `sidebars.name`; it does not reuse `top_nav.items` as sidebar entries. A page that appears only in `top_nav.items` and not in `sidebars.<name>.items` will not be added to the left sidebar.

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

[[locales.en.top_nav]]
text = "Guide"
link = "guide/installation/"
sidebar = "guide"

[[locales.en.top_nav.items]]
text = "Quick Start"
link = "guide/installation/"

[[locales.en.sidebars.guide]]
text = "Guide"
link = "guide/installation/"

[[locales.en.sidebars.guide.items]]
text = "Installation"
link = "guide/installation/"
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

Non-root locale links default to `/<locale>/`; use `link` to override that prefix. Locale `top_nav`, `sidebars`, and `title` override the global values, and fall back to global config when omitted. Relative locale top navigation and sidebar links are resolved under that locale prefix, so `guide/installation/` in `locales.en.top_nav` or `locales.en.sidebars.guide` becomes `/en/guide/installation/`.

The language selector appears in the top bar only when `locales` is configured. It switches to the matching translated page when one exists. If a translation is missing, it links to that language's home page.
