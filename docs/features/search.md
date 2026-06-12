---
title: 搜索
layout: doc
sidebar: true
search: true
access: public
---

# 搜索

RustPress 的搜索是纯本地搜索。构建时生成索引，浏览器运行时读取 JSON 并在前端匹配。

## 如何启用

```toml
[search]
enabled = true
languages = ["zh", "en", "ja", "ko"]
index_code = false
```

`enabled = false` 会隐藏搜索入口，并跳过搜索索引写入。

## 输出文件

启用搜索后会生成：

```text
dist/assets/search-index.json
dist/assets/search-index.json.br
dist/assets/rustpress_search_bg.wasm
```

当前前端 runtime 使用 JSON 索引执行搜索。`.br` 和 wasm 文件保留在输出中，用于稳定资源约定和后续优化空间。

## 页面级控制

frontmatter 可以排除单页：

```yaml
---
title: Internal Note
search: false
---
```

被排除页面仍会正常渲染，只是不进入索引。

## 代码是否入索引

默认设置：

```toml
[search]
index_code = false
```

这样可以避免代码块里的大量标识符干扰文档搜索。如果你的站点是 API 手册或代码片段库，可以改成 `true`。

## 语言和分词

搜索索引会保留页面标题、URL、标题列表和正文 token。

- 英文按大小写不敏感匹配，并做轻量词干归一。
- CJK 内容按字符进入索引，适合中文、日文、韩文关键词。
- 前端搜索会从标题和正文中生成结果摘要。

## 使用方式

用户可以：

- 点击顶部栏搜索按钮。
- 连续按两次 `Shift` 打开搜索。
- 输入关键词后点击结果跳转。

搜索不需要服务端，也不会把查询发送到外部服务。
