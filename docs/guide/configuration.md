---
title: 配置
layout: doc
sidebar: true
search: true
access: public
---

# 配置

RustPress 使用一个 `rustpress.toml` 描述站点、导航、主题、搜索、多语言和访问遮罩。配置文件所在目录就是项目根目录，相对路径都会从这里解析。

## 最小配置

```toml
title = "My Docs"
src_dir = "docs"
out_dir = "dist"
base = "/"
```

| 字段 | 说明 |
| --- | --- |
| `title` | 站点标题，也会作为页面标题后缀 |
| `src_dir` | Markdown 源目录 |
| `out_dir` | 静态输出目录，构建前会被清理 |
| `base` | 部署路径前缀，例如 GitHub Pages 项目站点可设为 `/rust-press/` |

`base` 会自动补齐开头和结尾的 `/`。

## 顶部导航

`top_nav` 只控制顶部栏。它可以是直接链接，也可以带下拉菜单。

```toml
[[top_nav]]
text = "指南"
link = "/guide/cli/"
sidebar = "guide"

[[top_nav.items]]
text = "快速开始"
link = "/guide/installation/"

[[top_nav.items]]
text = "站点配置"
link = "/guide/configuration/"
```

| 字段 | 说明 |
| --- | --- |
| `text` | 顶部显示文字 |
| `link` | 顶层入口链接；省略时只作为下拉触发器 |
| `sidebar` | 把这个顶部分区绑定到某个侧边栏组 |
| `items` | 只用于顶部下拉菜单 |

旧配置名 `nav` 已被移除。使用 `[[nav]]` 或 `[[locales.en.nav]]` 会直接报错，请改成 `[[top_nav]]` 或 `[[locales.en.top_nav]]`。

## 侧边栏

`sidebars.<id>` 只控制左侧菜单。它不会复用 `top_nav.items`。

```toml
[[sidebars.guide]]
text = "指南"
link = "/guide/cli/"

[[sidebars.guide.items]]
text = "命令行"
link = "/guide/cli/"

[[sidebars.guide.items]]
text = "安装"
link = "/guide/installation/"

[[sidebars.guide.items]]
text = "配置"
link = "/guide/configuration/"
```

顶部的 `sidebar = "guide"` 只是说明当前顶部分区和 `sidebars.guide` 有关联。只出现在 `top_nav.items`、没有出现在 `sidebars.guide.items` 的页面，不会自动加入左侧侧边栏。

如果没有配置任何 `sidebars`，RustPress 会根据 Markdown 页面路径自动生成侧边栏，并用顶层 `top_nav` 的顺序辅助分组排序。

## Markdown

```toml
[markdown]
mermaid = true
code_highlight = true
code_line_numbers = true
heading_anchors = true
```

| 字段 | 默认值 | 效果 |
| --- | --- | --- |
| `mermaid` | `true` | `mermaid` 代码块渲染为图 |
| `code_highlight` | `true` | 使用 syntect 高亮代码 |
| `code_line_numbers` | `true` | 代码块显示行号 |
| `heading_anchors` | `true` | 标题生成可复制锚点 |

## 搜索

```toml
[search]
enabled = true
languages = ["zh", "en", "ja", "ko"]
index_code = false
```

`enabled = true` 会写入本地搜索索引。`index_code = false` 时，代码块不会进入搜索正文。页面 frontmatter 里的 `search: false` 可以排除单页。

## 主题

```toml
[theme]
name = "default"
skin = "light"
allow_switch = true
github_url = "https://github.com/your-org/your-repo"
```

当前实现使用内置默认主题。`skin` 支持 `light` 和 `dark`；其他值会回退到 `light`。`allow_switch = true` 会在顶部显示颜色模式切换器。配置 `github_url` 后顶部会显示 GitHub 图标。

## 访问遮罩

```toml
[access]
enabled = true
mode = "mask"
password = "rustpress"
password_hint = "Enter password"
```

当站点开启访问遮罩，并且页面 frontmatter 设置 `access: masked` 时，页面会显示前端密码遮罩。遮罩不是安全机制，HTML 仍然在静态输出中。

## 页面 frontmatter

```yaml
---
title: Page Title
layout: doc
sidebar: true
search: true
access: public
---
```

| 字段 | 默认值 | 说明 |
| --- | --- | --- |
| `title` | 第一个标题或 `Untitled` | 页面标题 |
| `layout` | `doc` | 当前主题使用文档布局 |
| `sidebar` | `true` | 自动侧边栏模式下是否参与侧边栏 |
| `search` | `true` | 是否进入搜索索引 |
| `access` | `public` | `public` 或 `masked` |

无效的 `access` 值会被规范化为 `public`。

## 多语言文档

配置 `locales` 后，必须提供 `locales.root`。根语言继续使用 `docs/` 下的文件，其他语言使用 `docs/<locale>/`。

```toml
[locales.root]
label = "简体中文"
lang = "zh-CN"
title = "中文文档"

[locales.en]
label = "English"
lang = "en-US"
link = "/en/"
title = "English Docs"

[[locales.en.top_nav]]
text = "Guide"
link = "guide/cli/"
sidebar = "guide"

[[locales.en.sidebars.guide]]
text = "Guide"
link = "guide/cli/"
```

相对链接会解析到 locale 前缀下。例如 `guide/cli/` 在 `locales.en.top_nav` 中会变成 `/en/guide/cli/`。

语言切换器会优先跳到当前页面的对应译文。如果目标语言没有同一页面，会回退到该语言首页。

## 静态资源

项目根目录下的 `public/` 会被复制到 `out_dir`。适合放图片、下载文件、favicon 等静态资源。

```text
public/logo.png -> dist/logo.png
```
