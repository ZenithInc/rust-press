---
title: Configuration
layout: doc
sidebar: true
search: true
access: public
---

# Configuration

RustPress uses `rustpress.toml` for the site, navigation, theme, search, locales, and access mask. The config file directory is the project root for relative paths.

## Minimal Config

```toml
title = "My Docs"
src_dir = "docs"
out_dir = "dist"
base = "/"
```

| Field | Meaning |
| --- | --- |
| `title` | Site title and page title suffix |
| `src_dir` | Markdown source directory |
| `out_dir` | Static output directory, cleaned before each build |
| `base` | Deployment path prefix, such as `/rust-press/` for GitHub Pages project sites |

`base` is normalized to start and end with `/`.

## Top Navigation

`top_nav` controls only the top bar. It can be a direct link or a dropdown group.

```toml
[[top_nav]]
text = "Guide"
link = "/guide/cli/"
sidebar = "guide"

[[top_nav.items]]
text = "Quick Start"
link = "/guide/installation/"

[[top_nav.items]]
text = "Site Config"
link = "/guide/configuration/"
```

| Field | Meaning |
| --- | --- |
| `text` | Visible label |
| `link` | Top-level entry link; omit it for a dropdown-only trigger |
| `sidebar` | Binds this top section to a sidebar group |
| `items` | Top dropdown links only |

The old `nav` key has been removed. `[[nav]]` and `[[locales.en.nav]]` fail during config loading; use `[[top_nav]]` and `[[locales.en.top_nav]]`.

## Sidebars

`sidebars.<id>` controls the left sidebar. It does not reuse `top_nav.items`.

```toml
[[sidebars.guide]]
text = "Guide"
link = "/guide/cli/"

[[sidebars.guide.items]]
text = "CLI"
link = "/guide/cli/"

[[sidebars.guide.items]]
text = "Installation"
link = "/guide/installation/"

[[sidebars.guide.items]]
text = "Configuration"
link = "/guide/configuration/"
```

`sidebar = "guide"` on a top navigation item only connects that top section to `sidebars.guide`. A page that appears only in `top_nav.items` and not in `sidebars.guide.items` will not be added to the left sidebar.

If no `sidebars` are configured, RustPress builds a sidebar from Markdown paths and uses top-level `top_nav` entries to help order groups.

## Markdown

```toml
[markdown]
mermaid = true
code_highlight = true
code_line_numbers = true
heading_anchors = true
```

| Field | Default | Effect |
| --- | --- | --- |
| `mermaid` | `true` | Render `mermaid` code fences as diagrams |
| `code_highlight` | `true` | Highlight code with syntect |
| `code_line_numbers` | `true` | Show line numbers |
| `heading_anchors` | `true` | Add stable heading anchors |

## Search

```toml
[search]
enabled = true
languages = ["zh", "en", "ja", "ko"]
index_code = false
```

`enabled = true` writes the local search index. `index_code = false` excludes code blocks from search text. Use `search: false` in page frontmatter to exclude one page.

## Theme

```toml
[theme]
name = "default"
skin = "light"
allow_switch = true
github_url = "https://github.com/your-org/your-repo"
```

The current implementation uses the built-in default theme. `skin` supports `light` and `dark`; other values fall back to `light`. `allow_switch = true` shows the color mode switcher. `github_url` shows a GitHub icon in the top bar.

## Access Mask

```toml
[access]
enabled = true
mode = "mask"
password = "rustpress"
password_hint = "Enter password"
```

When access masking is enabled and a page uses `access: masked`, the page shows a front-end password overlay. It is not a security boundary; the HTML still exists in the static output.

## Page Frontmatter

```yaml
---
title: Page Title
layout: doc
sidebar: true
search: true
access: public
---
```

| Field | Default | Meaning |
| --- | --- | --- |
| `title` | First heading or `Untitled` | Page title |
| `layout` | `doc` | Current theme document layout |
| `sidebar` | `true` | Include in generated sidebars when no explicit sidebars exist |
| `search` | `true` | Include in search index |
| `access` | `public` | `public` or `masked` |

Invalid `access` values normalize to `public`.

## Multilingual Docs

When `locales` is configured, `locales.root` is required. Root files live directly under `docs/`; other locales live under `docs/<locale>/`.

```toml
[locales.root]
label = "简体中文"
lang = "zh-CN"
title = "中文文档"

[locales.en]
label = "English"
lang = "en-US"
link = "/en/"
title = "English Docs"

[[locales.en.top_nav]]
text = "Guide"
link = "guide/cli/"
sidebar = "guide"

[[locales.en.sidebars.guide]]
text = "Guide"
link = "guide/cli/"
```

Relative locale links are resolved under the locale prefix. `guide/cli/` in `locales.en.top_nav` becomes `/en/guide/cli/`.

The language switcher links to the matching translated page when it exists. If a translation is missing, it falls back to that locale's home page.

## Static Assets

Files in `public/` are copied to `out_dir`.

```text
public/logo.png -> dist/logo.png
```
