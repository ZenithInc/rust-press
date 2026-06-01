---
title: インストール
layout: doc
sidebar: true
search: true
access: public
---

# インストール

RustPress の CLI crate 名は `rust-press` で、インストールされるバイナリ名も `rust-press` です。

このリポジトリは crates.io 公開の準備済みですが、`cargo install rust-press` は crate が公開された後にだけ使えます。それまではソース checkout または Git からインストールしてください。

## ソース checkout

ローカル checkout から CLI をインストールできます。

```bash
git clone https://github.com/ZenithInc/rust-press.git
cd rust-press
cargo install --path crates/rust-press
```

Rust 1.93 以降が必要です。

## Git

GitHub の最新コミットからインストールします。

```bash
cargo install --git https://github.com/ZenithInc/rust-press rust-press
```

特定のリリースタグをインストールする場合は、実在する tag に置き換えてください。

```bash
cargo install --git https://github.com/ZenithInc/rust-press --tag v0.1.1 rust-press
```

## crates.io

crates.io に公開された後は Cargo でインストールできます。

```bash
cargo install rust-press
rust-press --version
```

## ビルド済みバイナリ

リリース tag の公開後、GitHub Releases では Linux、macOS、Windows 向けのビルド済みアーカイブを提供します。対象プラットフォームのアーカイブをダウンロードして展開し、`rust-press` を `PATH` に置くか、展開先から直接実行します。

```bash
rust-press --help
```

各リリースアーカイブには SHA256 チェックサムファイルも付属します。

## 更新

crate 公開後は、最新の Cargo リリースへ更新できます。

```bash
cargo install rust-press --force
```
