---
title: Markdown
layout: doc
sidebar: true
search: true
access: public
---

# Markdown

Markdown 由 `pulldown-cmark` 解析。当前 MVP 启用了表格、任务列表、删除线、脚注、标题属性、标题锚点和 Mermaid fenced blocks。

## 标题锚点

每个标题都会获得稳定锚点。非 ASCII 标题会保留，例如 `中文 标题` 会变成 `#中文-标题`。

## Mermaid

语言为 `mermaid` 的 fenced code block 会输出为 Mermaid block，并由客户端 Mermaid 脚本渲染。

```mermaid
sequenceDiagram
    participant User
    participant CLI
    participant Builder
    User->>CLI: rust-press build
    CLI->>Builder: BuildOptions
    Builder-->>User: dist/
```

## 搜索文本

当 `index_code = false` 时，代码块会从搜索索引中排除。
