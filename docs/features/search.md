---
title: 搜索
layout: doc
sidebar: true
search: true
access: public
---

# 搜索

RustPress 会在构建时写入本地搜索索引。

## 输出文件

- `dist/assets/search-index.json`
- `dist/assets/search-index.json.br`
- `dist/assets/rustpress_search_bg.wasm`

当前 runtime 使用 JavaScript fallback 查询 JSON 索引。WASM 文件会保留，用于维持 MVP 输出约定。

## English

英文 token 会以大小写不敏感的方式匹配。可以搜索 `BUILD`、`build` 或 `search`。

## 中文搜索

中文内容按字符 token 进入索引。你可以搜索 `搜索`、`中文`、`访问遮罩` 来验证本地搜索结果。
