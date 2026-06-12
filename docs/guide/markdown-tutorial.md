---
title: Markdown 教程
layout: doc
sidebar: true
search: true
access: public
---

# Markdown 教程

RustPress 使用 `pulldown-cmark` 解析 Markdown，并额外处理代码块、标题锚点、搜索文本和 Mermaid。这个页面可以当作写作速查表。

## Frontmatter

每个 `.md` 文件都可以在开头写 YAML frontmatter。

```yaml
---
title: 页面标题
layout: doc
sidebar: true
search: true
access: public
---
```

常用写法：

- 想隐藏某页搜索结果：`search: false`
- 想让页面显示访问遮罩：`access: masked`
- 想在自动侧栏模式下排除某页：`sidebar: false`

## 标题

```markdown
# 一级标题
## 二级标题
### 三级标题
```

RustPress 会为标题生成稳定锚点。重复标题会自动追加序号，中文、日文、韩文等非 ASCII 标题会保留在锚点里。

```markdown
## 配置
## 配置
```

会得到类似 `#配置` 和 `#配置-2` 的链接。

## 段落和强调

```markdown
这是第一段。

这是第二段。

*斜体*
**加粗**
***加粗斜体***
~~删除线~~
```

## 列表和任务

```markdown
- 写配置
- 写文档
  - 写功能页
  - 写配置页

1. 初始化项目
2. 构建站点
3. 部署 dist

- [x] 生成页面
- [ ] 发布文档
```

## 链接和图片

```markdown
[命令行指南](/guide/cli/)
[外部链接](https://github.com/ZenithInc/rust-press)

![Logo](/logo.png)
```

站内链接建议使用以 `/` 开头的绝对路径。多语言配置中的相对链接会自动加上 locale 前缀。

## 表格

```markdown
| 配置 | 用途 |
| --- | --- |
| `top_nav` | 顶部导航 |
| `sidebars` | 左侧侧边栏 |
| `locales` | 多语言 |
```

## 引用和脚注

```markdown
> 访问遮罩不是认证系统，只是前端显示层。

RustPress 支持脚注。[^note]

[^note]: 脚注会出现在文档末尾。
```

## 代码块

写上语言名可以启用高亮，并在代码块顶部显示语言标签。

````markdown
```bash
rust-press build --config rustpress.toml
```

```rust
fn main() {
    println!("hello");
}
```
````

代码块默认显示行号和复制按钮。可以在配置中关闭行号：

```toml
[markdown]
code_line_numbers = false
```

## Mermaid

`mermaid` 代码块会在浏览器中渲染为图：

````markdown
```mermaid
flowchart LR
    A[写 Markdown] --> B[运行 rust-press build]
    B --> C[生成 dist]
```
````

```mermaid
flowchart LR
    A[写 Markdown] --> B[运行 rust-press build]
    B --> C[生成 dist]
```

## 标题属性

启用的 Markdown 扩展支持标题属性：

```markdown
## 安装 {#install}
```

这样可以使用 `/guide/markdown-tutorial/#install` 跳到该标题。

## 原始 Markdown 复制

RustPress 会为每个页面写入 `index.md.txt`。主题右下角提供两个动作：

- 复制当前页面 Markdown。
- 复制当前页面 Markdown URL。

这适合把文档内容交给其他工具处理，或快速分享源码格式。

## 搜索文本

搜索文本来自 Markdown 正文。默认情况下代码块不会进入搜索正文；设置 `index_code = true` 后代码也会被索引。
