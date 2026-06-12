---
title: Installation
layout: doc
sidebar: true
search: true
access: public
---

# Installation

The CLI package is named `rust-press`, and the installed binary is also `rust-press`.

## Requirements

- Rust 1.93 or newer.
- A shell that can run `cargo`.
- Static hosting for deployment; the generated site does not need a Rust runtime.

Check your toolchain:

```bash
rustc --version
cargo --version
```

## Install From crates.io

After publishing, install with Cargo:

```bash
cargo install rust-press
rust-press --version
```

Update to the latest release:

```bash
cargo install rust-press --force
```

## Install From Source

```bash
git clone https://github.com/ZenithInc/rust-press.git
cd rust-press
cargo install --path crates/rust-press
```

You can also run without installing:

```bash
cargo run -p rust-press -- --help
```

## Install From Git

Install the latest main branch:

```bash
cargo install --git https://github.com/ZenithInc/rust-press rust-press
```

Install a specific tag:

```bash
cargo install --git https://github.com/ZenithInc/rust-press --tag v0.1.10 rust-press
```

## Prebuilt Binaries

Pushing a `v*` tag runs the GitHub Release workflow and builds packages for Linux, macOS, and Windows. Download the archive for your platform, unpack it, and put `rust-press` on your `PATH`.

```bash
rust-press --help
```

Each archive includes a SHA256 checksum file.

## Create a Site

```bash
rust-press init my-docs
cd my-docs
rust-press dev
```

`init` creates:

- `rustpress.toml`
- `docs/index.md`
- `docs/private.md`
- `public/.gitkeep`

It refuses to overwrite existing files.

## Build and Deploy

```bash
rust-press build --config rustpress.toml
```

The default output directory is `dist/`. Upload that directory to GitHub Pages, Nginx, S3, Cloudflare Pages, or any static hosting provider.
