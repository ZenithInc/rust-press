---
title: 配置
layout: doc
sidebar: true
search: true
access: public
---

# 配置

RustPress 使用一个 `rustpress.toml` 描述站点基础信息、顶部入口、主题、搜索、多语言和访问遮罩。配置文件所在目录就是项目根目录，相对路径都会从这里解析。

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

`build` 会把 `base` 写入 HTML、搜索索引和 Markdown 源文件 URL。`dev` 为了本地调试会临时使用 `/`，避免带子路径的部署配置影响本地预览。

## 顶部导航

`top_nav` 只配置顶部入口。它可以是直接链接，也可以带下拉菜单。

```toml
[[top_nav]]
text = "指南"
link = "/guide/cli/"

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
| `items` | 顶部下拉菜单 |

`http://`、`https://`、`mailto:` 和 `#` 开头的链接按普通链接保留。本地链接会自动规范化为站内路径；在多语言页面上会按当前 locale 前缀渲染，例如 `/guide/cli/` 会变成 `/en/guide/cli/`。

旧配置名 `nav` 已被移除。`top_nav.sidebar`、`sidebars`、`locales.*.top_nav` 和 `locales.*.sidebars` 也已移除，配置加载时会直接报错。

## 侧边栏

侧边栏由 `docs/` 目录自动生成，不再写在 TOML 中。当前页面属于哪个一级目录，就显示该目录下的页面；首页不显示目录侧边栏。

```text
docs/
  index.md                  -> /
  guide.md                  -> /guide/
  guide/cli.md              -> /guide/cli/
  guide/configuration.md    -> /guide/configuration/
  features/search.md        -> /features/search/
```

访问 `/guide/cli/` 时只显示 `guide` 下的页面；访问 `/features/search/` 时只显示 `features` 下的页面。侧边栏最多展示 2 级，更深的页面仍会生成路由，但在侧边栏中收敛到第二级。

页面标题优先使用 frontmatter `title`，没有时使用 Markdown 第一个标题。frontmatter `sidebar: false` 可以把单页排除出自动侧边栏。

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
| `sidebar` | `true` | 是否参与自动侧边栏 |
| `search` | `true` | 是否进入搜索索引 |
| `access` | `public` | `public` 或 `masked` |

无效的 `access` 值会被规范化为 `public`。

## 多语言文档

配置 `locales` 后，必须提供 `locales.root`。根语言使用无后缀文件，其他语言使用 `.<locale>.md` 文件名后缀。

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
```

```text
docs/index.md          -> /
docs/index.en.md       -> /en/
docs/guide/cli.md      -> /guide/cli/
docs/guide/cli.en.md   -> /en/guide/cli/
```

翻译匹配会去掉语言后缀后计算。同一页面存在译文时，语言切换器会跳到对应译文；缺失时回退到目标语言首页。顶部导航统一使用根配置，不在 `locales` 下重复配置。

## 静态资源

项目根目录下的 `public/` 会被复制到 `out_dir`。适合放图片、下载文件、favicon 等静态资源。

```text
public/logo.png -> dist/logo.png
```
