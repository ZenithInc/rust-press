---
title: Markdown
layout: doc
sidebar: true
search: true
access: public
---

# Markdown

Markdown 処理は `rustpress-md` が担当します。frontmatter、HTML 生成、見出し収集、検索テキスト抽出、コードブロック拡張を行います。

## 対応構文

- 表
- 脚注
- 取り消し線
- タスクリスト
- 見出し属性
- 通常の Markdown 構文

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

`title` がない場合は最初の見出しを使い、見出しもない場合は `Untitled` になります。

## 見出しアンカー

```toml
[markdown]
heading_anchors = true
```

英語は小文字化され、空白は `-` になり、重複見出しには番号が付きます。CJK 文字は保持されます。

## コードブロック

```toml
[markdown]
code_highlight = true
code_line_numbers = true
```

コードブロックには syntect ハイライト、行番号、言語ラベル、コピーボタンが付きます。コピー内容に行番号は含まれません。

## Mermaid

```toml
[markdown]
mermaid = true
```

`mermaid` fenced block はブラウザで図としてレンダリングされます。テーマ切替時にも再描画されます。

## 検索テキスト

既定ではコードブロックは検索本文に入りません。

```toml
[search]
index_code = false
```

API 文書などでは `true` にできます。

## Markdown ソース

各ページに `index.md.txt` が作られ、テーマから Markdown と Markdown URL をコピーできます。
