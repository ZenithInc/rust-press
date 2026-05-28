---
title: 検索
layout: doc
sidebar: true
search: true
access: public
---

# 検索

RustPress はビルド時にローカル検索インデックスを書き込みます。

## 出力ファイル

- `dist/assets/search-index.json`
- `dist/assets/search-index.json.br`
- `dist/assets/rustpress_search_bg.wasm`

現在の runtime は JavaScript fallback を使って JSON インデックスを検索します。WASM ファイルは MVP の出力契約を保つために存在します。

## English

英語 token は大文字小文字を区別せずに一致します。`BUILD`、`build`、`search` を検索してみてください。

## 中国語検索

中国語コンテンツは文字 token としてインデックスに入ります。`搜索`、`中文`、`访问遮罩` を検索してローカル検索結果を確認できます。
