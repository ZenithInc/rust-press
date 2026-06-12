---
title: 設定
layout: doc
sidebar: true
search: true
access: public
---

# 設定

RustPress は `rustpress.toml` でサイト、ナビゲーション、テーマ、検索、多言語、アクセスマスクを設定します。設定ファイルの場所がプロジェクトルートです。

## 最小設定

```toml
title = "My Docs"
src_dir = "docs"
out_dir = "dist"
base = "/"
```

| 項目 | 説明 |
| --- | --- |
| `title` | サイトタイトル |
| `src_dir` | Markdown ソースディレクトリ |
| `out_dir` | 静的出力先。ビルド前に削除される |
| `base` | デプロイ先のパス接頭辞 |

## トップナビ

`top_nav` はトップバーだけを制御します。

```toml
[[top_nav]]
text = "ガイド"
link = "/guide/cli/"
sidebar = "guide"

[[top_nav.items]]
text = "クイックスタート"
link = "/guide/installation/"
```

`sidebar = "guide"` はトップセクションを `sidebars.guide` に関連付けるだけです。`top_nav.items` はサイドバーには再利用されません。旧 `nav` 設定は無効です。

## サイドバー

```toml
[[sidebars.guide]]
text = "ガイド"
link = "/guide/cli/"

[[sidebars.guide.items]]
text = "CLI"
link = "/guide/cli/"

[[sidebars.guide.items]]
text = "インストール"
link = "/guide/installation/"
```

`sidebars.<id>.items` が左側サイドバーを決めます。明示的な `sidebars` がない場合、Markdown のパスから自動生成されます。

## Markdown

```toml
[markdown]
mermaid = true
code_highlight = true
code_line_numbers = true
heading_anchors = true
```

Mermaid、コードハイライト、行番号、見出しアンカーを制御します。

## 検索

```toml
[search]
enabled = true
languages = ["zh", "en", "ja", "ko"]
index_code = false
```

`search: false` を frontmatter に書くと、そのページだけ検索から除外できます。

## テーマ

```toml
[theme]
name = "default"
skin = "light"
allow_switch = true
github_url = "https://github.com/your-org/your-repo"
```

`skin` は `light` または `dark`。`allow_switch` で切替ボタンを表示します。

## アクセスマスク

```toml
[access]
enabled = true
mode = "mask"
password = "rustpress"
password_hint = "Enter password"
```

ページ側で `access: masked` を指定するとフロントエンドのマスクが表示されます。これはセキュリティ機構ではありません。

## Frontmatter

```yaml
---
title: Page Title
layout: doc
sidebar: true
search: true
access: public
---
```

| 項目 | 既定 | 説明 |
| --- | --- | --- |
| `title` | 最初の見出しまたは `Untitled` | ページタイトル |
| `layout` | `doc` | 文書レイアウト |
| `sidebar` | `true` | 自動サイドバーに含めるか |
| `search` | `true` | 検索に含めるか |
| `access` | `public` | `public` または `masked` |

## 多言語ドキュメント

`locales` を設定する場合、`locales.root` が必要です。root は `docs/`、他の言語は `docs/<locale>/` に置きます。

```toml
[locales.root]
label = "简体中文"
lang = "zh-CN"

[locales.en]
label = "English"
lang = "en-US"
link = "/en/"

[[locales.en.top_nav]]
text = "Guide"
link = "guide/cli/"
sidebar = "guide"
```

locale 内の相対リンクは locale 接頭辞の下に解決されます。

## 静的資産

`public/` の内容は `out_dir` にコピーされます。
