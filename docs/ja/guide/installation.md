---
title: インストール
layout: doc
sidebar: true
search: true
access: public
---

# インストール

CLI パッケージ名は `rust-press`、実行ファイル名も `rust-press` です。

## 必要条件

- Rust 1.93 以上
- `cargo` を実行できる環境
- デプロイ時は静的ホスティングだけでよく、Rust ランタイムは不要

```bash
rustc --version
cargo --version
```

## crates.io からインストール

```bash
cargo install rust-press
rust-press --version
```

更新:

```bash
cargo install rust-press --force
```

## ソースからインストール

```bash
git clone https://github.com/ZenithInc/rust-press.git
cd rust-press
cargo install --path crates/rust-press
```

インストールせずに実行することもできます。

```bash
cargo run -p rust-press -- --help
```

## Git からインストール

```bash
cargo install --git https://github.com/ZenithInc/rust-press rust-press
```

特定 tag:

```bash
cargo install --git https://github.com/ZenithInc/rust-press --tag v0.1.10 rust-press
```

## 最初のサイト

```bash
rust-press init my-docs
cd my-docs
rust-press dev
```

`init` は `rustpress.toml`、`docs/index.md`、`docs/private.md`、`public/.gitkeep` を作成します。既存ファイルは上書きしません。

## ビルドとデプロイ

```bash
rust-press build --config rustpress.toml
```

既定の出力先は `dist/` です。このディレクトリを GitHub Pages、Nginx、S3、Cloudflare Pages などへアップロードします。
