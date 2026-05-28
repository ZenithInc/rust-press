---
title: Search
layout: doc
sidebar: true
search: true
access: public
---

# Search

RustPress writes a local search index at build time.

## Output Files

- `dist/assets/search-index.json`
- `dist/assets/search-index.json.br`
- `dist/assets/rustpress_search_bg.wasm`

The current runtime uses JavaScript fallback querying against the JSON index. The WASM file is present to keep the MVP output contract.

## English

English tokens are matched case-insensitively. Try searching for `BUILD`, `build`, or `search`.

## 中文搜索

Chinese content enters the index as character tokens. Search for `搜索`, `中文`, or `访问遮罩` to verify local search results.
