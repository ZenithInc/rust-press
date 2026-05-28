---
title: CLI
layout: doc
sidebar: true
search: true
access: public
---

# CLI

The binary is named `rust-press`.

## init

```bash
rust-press init [dir]
```

Creates:

- `rustpress.toml`
- `docs/index.md`
- `docs/private.md`
- `public/.gitkeep`

The command refuses to overwrite existing files.

## build

```bash
rust-press build --config rustpress.toml
```

Build output goes to the configured `out_dir`, which defaults to `dist`.

## dev

```bash
rust-press dev --host 0.0.0.0 --port 5190
```

The dev server performs an initial build, serves `dist`, watches Markdown and config files, and injects a small refresh script into HTML responses.

## preview

```bash
rust-press preview --host 127.0.0.1 --port 4177
```

Preview serves the already built static output without watching files.
