---
title: 設定
layout: doc
sidebar: true
search: true
access: public
---

# 設定

RustPress は `rustpress.toml` でサイト基本情報、トップナビ、テーマ、検索、多言語、アクセスマスクを設定します。設定ファイルのあるディレクトリがプロジェクトルートです。

## 最小設定

```toml
title = "My Docs"
src_dir = "docs"
out_dir = "dist"
base = "/"
```

| フィールド | 説明 |
| --- | --- |
| `title` | サイトタイトル。ページタイトルの suffix にも使われます |
| `src_dir` | Markdown ソースディレクトリ |
| `out_dir` | 静的出力ディレクトリ。ビルド前に削除されます |
| `base` | デプロイ先のパス prefix。例: `/rust-press/` |

`base` は先頭と末尾の `/` が自動補完されます。

`build` は HTML リンク、検索 URL、Markdown ソース URL に `base` を反映します。`dev` はローカル配信用に一時的に `/` を使うため、サブパス配備用の設定でも手元で確認できます。

## トップナビ

`top_nav` はトップバーの入口だけを設定します。直接リンクにも、ドロップダウンにもできます。

```toml
[[top_nav]]
text = "ガイド"
link = "/guide/cli/"

[[top_nav.items]]
text = "クイックスタート"
link = "/guide/installation/"

[[top_nav.items]]
text = "サイト設定"
link = "/guide/configuration/"
```

`http://`、`https://`、`mailto:`、`#` で始まるリンクはそのまま保持されます。ローカルリンクはサイト内パスとして正規化され、多言語ページでは現在の locale prefix が付きます。たとえば `/guide/cli/` は `/ja/guide/cli/` として表示されます。

旧 `nav`、`top_nav.sidebar`、`sidebars`、`locales.*.top_nav`、`locales.*.sidebars` は削除済みです。設定に残っている場合は読み込み時にエラーになります。

## サイドバー

サイドバーは `docs/` のパスから自動生成され、TOML には書きません。現在ページの第 1 パスセグメントに属するページだけを表示し、ホームページではディレクトリサイドバーを表示しません。

```text
docs/
  index.md                  -> /
  guide.md                  -> /guide/
  guide/cli.md              -> /guide/cli/
  guide/configuration.md    -> /guide/configuration/
  features/search.md        -> /features/search/
```

`/guide/cli/` では `guide` 配下だけ、`/features/search/` では `features` 配下だけを表示します。サイドバーは最大 2 階層です。より深いページもルートは生成されますが、サイドバー上は第 2 階層に集約されます。

ページタイトルは frontmatter `title`、なければ最初の Markdown 見出しから取得します。`sidebar: false` で自動サイドバーから除外できます。

## Markdown

```toml
[markdown]
mermaid = true
code_highlight = true
code_line_numbers = true
heading_anchors = true
```

## 検索

```toml
[search]
enabled = true
languages = ["zh", "en", "ja", "ko"]
index_code = false
```

`search: false` をページ frontmatter に設定すると、そのページは検索インデックスから除外されます。

## テーマ

```toml
[theme]
name = "default"
skin = "light"
allow_switch = true
github_url = "https://github.com/your-org/your-repo"
```

`skin` は `light` と `dark` をサポートします。`github_url` を設定するとトップバーに GitHub アイコンが表示されます。

## アクセスマスク

```toml
[access]
enabled = true
mode = "mask"
password = "rustpress"
password_hint = "Enter password"
```

ページ frontmatter に `access: masked` を設定すると、フロントエンドのパスワードマスクが表示されます。静的 HTML は出力に残るため、セキュリティ境界ではありません。

## ページ frontmatter

```yaml
---
title: Page Title
layout: doc
sidebar: true
search: true
access: public
---
```

| フィールド | デフォルト | 説明 |
| --- | --- | --- |
| `title` | 最初の見出しまたは `Untitled` | ページタイトル |
| `layout` | `doc` | 現在のテーマは doc layout を使います |
| `sidebar` | `true` | 自動サイドバーに含めるか |
| `search` | `true` | 検索インデックスに含めるか |
| `access` | `public` | `public` または `masked` |

## 多言語ドキュメント

`locales` を設定する場合、`locales.root` が必要です。root 言語は suffix なし、他の言語は `.<locale>.md` というファイル名 suffix を使います。

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
docs/index.ja.md       -> /ja/
docs/guide/cli.md      -> /guide/cli/
docs/guide/cli.ja.md   -> /ja/guide/cli/
```

翻訳キーは locale suffix を外して計算されます。対応する翻訳がない場合、言語切替は対象 locale のホームへ戻ります。トップナビは root に一度だけ設定します。

## 静的アセット

プロジェクトルートの `public/` は `out_dir` にコピーされます。

```text
public/logo.png -> dist/logo.png
```
