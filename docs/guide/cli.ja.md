---
title: CLI
layout: doc
sidebar: true
search: true
access: public
---

# CLI

`rust-press` には `init`、`build`、`dev`、`preview` があります。

## コマンド

| コマンド | 用途 | 既定 |
| --- | --- | --- |
| `init [dir]` | 新しい docs プロジェクトを作成 | `dir` 省略時は現在のディレクトリ |
| `build` | 静的サイトを生成 | `rustpress.toml` を読み `dist/` へ出力 |
| `dev` | ローカル開発 | ビルド、配信、監視、リロード |
| `preview` | 生成済み出力を確認 | 現在の `out_dir` を配信 |

```bash
rust-press --help
rust-press build --help
```

## init

```bash
rust-press init my-docs
```

トップナビ、テーマ、検索、アクセスマスクの例を含む最小構成を作ります。サイドバーは Markdown パスから自動生成されます。

## build

```bash
rust-press build --config rustpress.toml
```

処理内容:

1. 設定を読み込み正規化する。
2. `src_dir` の Markdown を読む。
3. frontmatter、見出し、本文を解析する。
4. ページ、ナビ、自動サイドバー、目次、言語切替を生成する。
5. 検索インデックスとテーマ資産を書き込む。
6. `public/` をコピーする。

ビルド前に `out_dir` は削除されます。

## dev

```bash
rust-press dev --config rustpress.toml --host 127.0.0.1 --port 5177
```

`dev` は一度ビルドし、`src_dir` と設定ファイルを監視し、変更時に再ビルドします。HTML には live reload script が注入されます。

`dev` 中は `base = "/"` としてレンダリングされるため、ローカル URL はサーバールートから動作します。`build` と `preview` は設定されたデプロイ用 `base` を使います。

## preview

```bash
rust-press preview --config rustpress.toml --host 127.0.0.1 --port 4177
```

`preview` は監視も再ビルドもせず、現在の出力だけを配信します。

## 設定パス

```bash
rust-press build --config site/rustpress.toml
```

相対パスは設定ファイルの場所から解決されます。`site/rustpress.toml` の `src_dir = "docs"` は `site/docs/` を指します。
