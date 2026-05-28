---
title: CLI
layout: doc
sidebar: true
search: true
access: public
---

# CLI

バイナリ名は `rust-press` です。

## init

```bash
rust-press init [dir]
```

作成されるもの:

- `rustpress.toml`
- `docs/index.md`
- `docs/private.md`
- `public/.gitkeep`

このコマンドは既存ファイルの上書きを拒否します。

## build

```bash
rust-press build --config rustpress.toml
```

ビルド出力は設定された `out_dir` に書き込まれます。デフォルトは `dist` です。

## dev

```bash
rust-press dev --host 0.0.0.0 --port 5190
```

開発サーバーは初回ビルドを実行し、`dist` を配信し、Markdown と設定ファイルを監視し、HTML レスポンスに小さな更新スクリプトを注入します。

## preview

```bash
rust-press preview --host 127.0.0.1 --port 4177
```

Preview は、ファイル監視なしでビルド済みの静的出力を配信します。
