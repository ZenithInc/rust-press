---
title: CLI
layout: doc
sidebar: true
search: true
access: public
---

# CLI

`rust-press` provides four commands: `init`, `build`, `dev`, and `preview`.

## Commands

| Command | Purpose | Default Behavior |
| --- | --- | --- |
| `init [dir]` | Create a new docs project | Uses the current directory when `dir` is omitted |
| `build` | Generate the static site | Reads `rustpress.toml` and writes `dist/` |
| `dev` | Local development | Builds, serves, watches files, and reloads the browser |
| `preview` | Preview generated output | Serves the existing `out_dir` |

Help:

```bash
rust-press --help
rust-press build --help
```

## init

```bash
rust-press init my-docs
```

Creates:

```text
my-docs/
├── rustpress.toml
├── docs/
│   ├── index.md
│   └── private.md
└── public/
    └── .gitkeep
```

The generated config includes examples for top navigation, theme, search, and access masking. Sidebars are generated from Markdown paths.

## build

```bash
rust-press build --config rustpress.toml
```

The build:

1. Loads and normalizes config.
2. Scans Markdown from `src_dir`.
3. Parses frontmatter, headings, and body content.
4. Renders pages, top nav, generated sidebars, table of contents, and language switcher.
5. Writes search index and theme assets.
6. Copies static files from `public/`.

`out_dir` is cleaned before each build.

## dev

```bash
rust-press dev --config rustpress.toml --host 127.0.0.1 --port 5177
```

`dev` builds once, serves `out_dir`, watches `src_dir` and the config file, rebuilds on create/modify/remove events, and injects a small live reload script into HTML responses.

During `dev`, RustPress temporarily renders with `base = "/"` so local URLs work from the server root. `build` and `preview` keep the configured deployment `base`.

The default URL is `http://127.0.0.1:5177/`.

## preview

```bash
rust-press preview --config rustpress.toml --host 127.0.0.1 --port 4177
```

`preview` does not watch files and does not rebuild. It serves the current generated output and is useful before deployment.

## Config Path

All commands that need config accept `--config`:

```bash
rust-press build --config site/rustpress.toml
rust-press dev --config site/rustpress.toml
rust-press preview --config site/rustpress.toml
```

Relative paths are resolved from the config file directory. If `site/rustpress.toml` says `src_dir = "docs"`, RustPress reads `site/docs/`.
