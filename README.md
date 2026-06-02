# RustPress

RustPress is a Rust-first static documentation generator. It reads a
`rustpress.toml` file, renders Markdown from `docs/`, writes static HTML into
`dist/`, and serves the generated site for development or preview.

The project is currently an MVP, focused on a small but complete documentation
workflow.

## Features

- Markdown rendering with tables, task lists, strikethrough, footnotes, heading
  attributes, and stable heading anchors.
- Mermaid fenced code blocks rendered in the browser.
- Built-in documentation theme with responsive navigation, sidebar, table of
  contents, local search, subtle grid background, and Light/Dark switching.
- Global search shortcut: press `Shift` twice to open search.
- Multilingual docs using locale-prefixed routes.
- Local search index for English and CJK content.
- Front-end access mask for demo or lightweight viewing flows.

Access masking is a UI overlay only. It does not encrypt static files, remove
HTML content, or provide server-side authentication.

## Requirements

- Rust 1.93 or newer

## Quick Start

Install the CLI from this checkout:

```bash
cargo install --path crates/rust-press
rust-press --version
```

After the first crates.io release is published, you can use `cargo install rust-press` instead.

Create a new documentation project:

```bash
rust-press init my-docs
cd my-docs
rust-press build
rust-press preview --port 4177
```

When working from this repository, pass the generated config path explicitly:

```bash
cargo run -p rust-press -- init site
cargo run -p rust-press -- build --config site/rustpress.toml
cargo run -p rust-press -- preview --config site/rustpress.toml
```

You can also run the CLI without installing it:

```bash
cargo run -p rust-press -- --help
```

## Commands

```bash
rust-press init [dir]
rust-press build --config rustpress.toml
rust-press dev --config rustpress.toml --host 127.0.0.1 --port 5177
rust-press preview --config rustpress.toml --host 127.0.0.1 --port 4177
```

- `init` creates `rustpress.toml`, starter Markdown pages, and `public/.gitkeep`.
- `build` renders the static site into the configured `out_dir`.
- `dev` builds, serves the site, watches Markdown/config changes, and injects a
  small live reload script.
- `preview` serves the already built static output.

## Configuration

Minimal `rustpress.toml`:

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
languages = ["zh", "en"]
index_code = false

[access]
enabled = true
mode = "mask"
password = "demo123"
password_hint = "Enter password"
```

Markdown frontmatter:

```yaml
---
title: Page Title
layout: doc
sidebar: true
search: true
access: public
---
```

`access` can be `public` or `masked`. Set `search: false` to exclude a page from
the generated search index.

## Repository Layout

```text
crates/rust-press         CLI entry point
crates/rustpress-core     Config, routing, site build pipeline
crates/rustpress-dev      Dev and preview servers
crates/rustpress-md       Markdown parsing and HTML rendering
crates/rustpress-search   Search index builder
crates/rustpress-theme    Static theme HTML, CSS, and JavaScript
docs/                     Documentation source for this repository
```

## Development

Run the local docs server:

```bash
./start.sh
```

Run tests:

```bash
cargo test
```

Format code:

```bash
cargo fmt
```

## Publishing

The crates.io install package is `rust-press`; it installs the `rust-press`
binary. Internal crates must be published first because Cargo resolves packaged
path dependencies from crates.io.

Publish order:

```bash
cargo publish -p rustpress-md
cargo publish -p rustpress-search
cargo publish -p rustpress-theme
cargo publish -p rustpress-core
cargo publish -p rustpress-dev
cargo publish -p rust-press
```

Before publishing, run `cargo test` and then `cargo publish --dry-run -p <crate>`
in the same order. Pushing a tag such as `v0.1.3` builds GitHub Releases
archives for Linux, macOS, and Windows.

## License

RustPress is licensed under the MIT License. See [LICENSE](LICENSE).
