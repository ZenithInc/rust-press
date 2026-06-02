---
title: 設定
layout: doc
sidebar: true
search: true
access: public
---

# 設定

RustPress は VitePress のプロジェクト規約ではなく、`rustpress.toml` を使用します。

## 例

```toml
title = "My Docs"
src_dir = "docs"
out_dir = "dist"
base = "/"

[[nav]]
text = "ガイド"
link = "/guide/cli/"

[[nav.items]]
text = "CLI"
link = "/guide/cli/"

[[nav.items]]
text = "設定"
link = "/guide/configuration/"

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
languages = ["zh", "en", "ja", "ko"]
index_code = false

[access]
enabled = true
mode = "mask"
password = "demo123"
password_hint = "Enter password"
```

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

`access` は `public` または `masked` にできます。`[access].password` が設定されている場合のみ、`masked` ページにアクセスマスクが表示されます。`search: false` を指定すると、そのページは生成される検索インデックスから除外されます。

## トップナビゲーションとサイドバー

`[[nav]]` を使ってトップナビゲーションリンクまたはグループメニューを表示します。

```toml
[[nav]]
text = "ガイド"
link = "/guide/cli/"
sidebar = "guide"

[[nav.items]]
text = "CLI"
link = "/guide/cli/"

[[nav.items]]
text = "設定"
link = "/guide/configuration/"

[[nav]]
text = "リファレンス"
link = "/internals/crates/"
sidebar = "reference"

[[sidebars.guide]]
text = "ガイド"
link = "/guide/cli/"

[[sidebars.guide.items]]
text = "CLI"
link = "/guide/cli/"

[[sidebars.guide.items]]
text = "設定"
link = "/guide/configuration/"

[[sidebars.reference]]
text = "リファレンス"
link = "/internals/crates/"

[[sidebars.reference.items]]
text = "Crates"
link = "/internals/crates/"
```

`items` がある場合、テーマはドロップダウンメニューをレンダリングします。`items` が省略された場合、その項目は直接のトップレベルリンクとしてレンダリングされます。

トップナビゲーション項目に `sidebar = "name"` を追加すると、そのセクションのページが `sidebars.name` に紐づきます。`sidebars` を設定しない場合、RustPress は従来どおり Markdown ページと `nav` の順序からサイドバーを自動生成します。

## 多言語ドキュメント

RustPress はデフォルトでは単一言語です。`locales` を追加すると、URL ベースの多言語ドキュメントが有効になります。`locales` を設定する場合、`locales.root` は必須で、`/` にあるデフォルト言語を表します。

```toml
[locales.root]
label = "简体中文"
lang = "zh-CN"

[locales.en]
label = "English"
lang = "en-US"
link = "/en/"

[locales.ja]
label = "日本語"
lang = "ja-JP"
link = "/ja/"

[locales.ko]
label = "한국어"
lang = "ko-KR"
link = "/ko/"

[[locales.en.nav]]
text = "Guide"
link = "guide/cli/"
sidebar = "guide"

[[locales.en.sidebars.guide]]
text = "Guide"
link = "guide/cli/"

[[locales.en.sidebars.guide.items]]
text = "CLI"
link = "guide/cli/"
```

ルート言語は引き続き `docs/` 直下のファイルを使用します。その他の言語ファイルは `docs/<locale>/` に配置します。

```text
docs/index.md              -> /
docs/guide/cli.md          -> /guide/cli/
docs/en/index.md           -> /en/
docs/en/guide/cli.md       -> /en/guide/cli/
docs/ja/index.md           -> /ja/
docs/ko/index.md           -> /ko/
```

root 以外の言語リンクはデフォルトで `/<locale>/` になります。`link` でこのプレフィックスを上書きできます。Locale の `nav`、`sidebars`、`title` はグローバル値を上書きし、省略時はグローバル設定にフォールバックします。Locale ナビゲーションとサイドバー内の相対リンクは、その言語プレフィックスの下に解決されます。たとえば `locales.en.nav` または `locales.en.sidebars.guide` の `guide/cli/` は `/en/guide/cli/` になります。

言語セレクターは `locales` が設定されている場合にのみトップバーに表示されます。言語を切り替えると対応する翻訳ページに移動します。対象言語にそのページがない場合は、その言語のホームページに移動します。
