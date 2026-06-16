---
title: Markdown
layout: doc
sidebar: true
search: true
access: public
---

# Markdown

RustPress 的 Markdown 能力来自 `rustpress-md`。它负责解析 frontmatter、渲染 HTML、收集标题、提取搜索文本，并把代码块包装成主题可增强的结构。

## 支持的语法

当前启用的 `pulldown-cmark` 扩展包括：

- 表格
- 脚注
- 删除线
- 任务列表
- 标题属性

常规 Markdown 语法如段落、标题、链接、图片、引用、列表、行内代码和 fenced code block 都可以直接使用。

## Frontmatter 默认值

```yaml
---
title: Page Title
layout: doc
sidebar: true
search: true
access: public
---
```

如果没有写 `title`，RustPress 会使用第一个标题作为页面标题；如果页面没有标题，则使用 `Untitled`。

## 标题锚点

开启 `heading_anchors = true` 后，每个标题会获得稳定 `id` 和可点击的 `#` 锚点。

```toml
[markdown]
heading_anchors = true
```

锚点规则：

- 英文会转为小写。
- 空格和 `_` 会转成 `-`。
- 标点会被移除。
- 重复标题会追加 `-2`、`-3`。
- CJK 字符会保留。

## 代码块增强

普通 fenced code block 会被渲染为主题代码块：

- 使用 syntect 进行高亮。
- 默认显示行号。
- 显示语言标签。
- 提供复制按钮，复制内容不包含行号。

```toml
[markdown]
code_highlight = true
code_line_numbers = true
```

如果语言写成 `language-rust`、`rust,{...}` 等形式，RustPress 会归一化成 `rust`。

## Mermaid

```toml
[markdown]
mermaid = true
```

语言为 `mermaid` 的代码块会输出为 `<pre class="mermaid">`，再由主题 JavaScript 在浏览器里调用 Mermaid 渲染。切换 Light/Dark 主题时，Mermaid 图也会重新渲染并使用当前主题变量。

## 搜索文本提取

RustPress 会从 Markdown 事件中提取纯文本用于搜索。默认不索引 fenced code block：

```toml
[search]
index_code = false
```

如果文档面向 API 或代码片段查询，可以设置 `index_code = true`。

## Markdown 源文件

构建时每个页面都会在同一路由目录写入 `index.md.txt`。例如 `/guide/cli/` 会输出 `/guide/cli/index.md.txt`。主题提供“复制 Markdown”和“复制 Markdown URL”，方便把页面源码交给审阅、AI 工具或外部自动化流程。
