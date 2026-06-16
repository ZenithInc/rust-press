---
title: 検索
layout: doc
sidebar: true
search: true
access: public
---

# 検索

RustPress の検索は完全にローカルです。ビルド時に索引を作成し、ブラウザが JSON を読んで検索します。

## 有効化

```toml
[search]
enabled = true
languages = ["zh", "en", "ja", "ko"]
index_code = false
```

`enabled = false` なら検索入口と索引生成を無効化します。

## 出力ファイル

```text
dist/assets/search-index.json
dist/assets/search-index.json.br
dist/assets/rustpress_search_bg.wasm
```

現在の実行時スクリプトは JSON 索引を検索します。

## ページ単位の制御

```yaml
---
title: Internal Note
search: false
---
```

ページは通常通り表示されますが、検索索引には入りません。

## コード索引

既定ではコードブロックは索引に入りません。

```toml
[search]
index_code = false
```

API 文書やコード例中心の文書では `true` にできます。

## 分かち書き

英語は大小文字を無視し、軽い語幹処理を行います。CJK は文字単位の token で中国語、日本語、韓国語の検索を扱います。

## 使い方

検索ボタンまたは `Shift` 2 回で検索ダイアログを開きます。クエリは外部サービスに送信されません。
