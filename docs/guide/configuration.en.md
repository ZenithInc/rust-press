---
title: Configuration
layout: doc
sidebar: true
search: true
access: public
---

# Configuration

RustPress uses `rustpress.toml` for site basics, top navigation, theme, search, locales, and access masking. The config file directory is the project root, and relative paths are resolved from there.

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
| `base` | Deployment prefix, such as `/rust-press/` for GitHub Pages project sites |

`base` is normalized with leading and trailing `/`.

`build` writes `base` into HTML links, search URLs, and Markdown source URLs. `dev` temporarily uses `/` for local serving so subpath deployment settings do not break local preview.

## Top Navigation

`top_nav` configures only the top-level entries. An entry can be a direct link or a dropdown group.

```toml
[[top_nav]]
text = "Guide"
link = "/guide/cli/"

[[top_nav.items]]
text = "Quick Start"
link = "/guide/installation/"

[[top_nav.items]]
text = "Site Config"
link = "/guide/configuration/"
```

| Field | Meaning |
| --- | --- |
| `text` | Label shown in the top bar |
| `link` | Top-level entry link; omit it for a dropdown-only trigger |
| `items` | Dropdown links |

Links starting with `http://`, `https://`, `mailto:`, or `#` are preserved as ordinary links. Local links are normalized as site paths; on localized pages they are rendered under the current locale prefix, so `/guide/cli/` becomes `/en/guide/cli/`.

The old `nav` key has been removed. `top_nav.sidebar`, `sidebars`, `locales.*.top_nav`, and `locales.*.sidebars` are also removed and fail during config loading.

## Sidebars

Sidebars are generated from `docs/` paths and are no longer configured in TOML. The current page's first path segment selects the sidebar section; home pages do not show a directory sidebar.

```text
docs/
  index.md                  -> /
  guide.md                  -> /guide/
  guide/cli.md              -> /guide/cli/
  guide/configuration.md    -> /guide/configuration/
  features/search.md        -> /features/search/
```

`/guide/cli/` shows only pages from `guide`; `/features/search/` shows only pages from `features`. Sidebar rendering is capped at 2 levels. Deeper pages still get routes, but they collapse to their second-level item in the sidebar.

Page titles come from frontmatter `title`, falling back to the first Markdown heading. Use `sidebar: false` to exclude a page from the generated sidebar.

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
| `code_line_numbers` | `true` | Show line numbers in code blocks |
| `heading_anchors` | `true` | Add copyable heading anchors |

## Search

```toml
[search]
enabled = true
languages = ["zh", "en", "ja", "ko"]
index_code = false
```

`enabled = true` writes a local search index. When `index_code = false`, code blocks are excluded from the search body. Use frontmatter `search: false` to exclude one page.

## Theme

```toml
[theme]
name = "default"
skin = "light"
allow_switch = true
github_url = "https://github.com/your-org/your-repo"
```

The current implementation uses the built-in default theme. `skin` supports `light` and `dark`; other values fall back to `light`. `allow_switch = true` renders the color mode switcher. `github_url` renders the GitHub icon in the top bar.

## Access Mask

```toml
[access]
enabled = true
mode = "mask"
password = "rustpress"
password_hint = "Enter password"
```

When access masking is enabled and a page uses frontmatter `access: masked`, the page displays a front-end password mask. This is not a security boundary; the HTML still exists in the static output.

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
| `layout` | `doc` | The current theme uses the doc layout |
| `sidebar` | `true` | Include in the generated sidebar |
| `search` | `true` | Include in the search index |
| `access` | `public` | `public` or `masked` |

Invalid `access` values are normalized to `public`.

## Multilingual Docs

When `locales` is configured, `locales.root` is required. Root files have no suffix; other languages use `.<locale>.md` filename suffixes.

```toml
[locales.root]
label = "简体中文"
lang = "zh-CN"
title = "Chinese Docs"

[locales.en]
label = "English"
lang = "en-US"
link = "/en/"
title = "English Docs"
```

```text
docs/index.md          -> /
docs/index.en.md       -> /en/
docs/guide/cli.md      -> /guide/cli/
docs/guide/cli.en.md   -> /en/guide/cli/
```

Translation matching strips the locale suffix before computing the key. The language switcher links to the matching translation when it exists, and falls back to the target locale home page when it does not. Top navigation is configured once at the root and is not repeated under `locales`.

## Static Assets

The project root `public/` directory is copied into `out_dir`. Use it for images, downloads, favicons, and similar assets.

```text
public/logo.png -> dist/logo.png
```
