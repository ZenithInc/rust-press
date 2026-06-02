---
title: 配置
layout: doc
sidebar: true
search: true
access: public
---

# 配置

RustPress 使用 `rustpress.toml`，而不是 VitePress 的项目约定。

## 示例

```toml
title = "My Docs"
src_dir = "docs"
out_dir = "dist"
base = "/"

[[top_nav]]
text = "指南"
link = "/guide/installation/"

[[top_nav.items]]
text = "快速开始"
link = "/guide/installation/"

[[top_nav.items]]
text = "配置"
link = "/guide/configuration/"

[theme]
name = "default"
skin = "light"
allow_switch = true
github_url = "https://github.com/your-org/your-repo"

[markdown]
mermaid = true
code_highlight = true
code_line_numbers = true
heading_anchors = true

[search]
enabled = true
languages = ["zh", "en", "ja", "ko"]
index_code = false

[access]
enabled = true
mode = "mask"
password = "demo123"
password_hint = "Enter password"
```

## Frontmatter

```yaml
---
title: Page Title
layout: doc
sidebar: true
search: true
access: public
---
```

`access` 可以是 `public` 或 `masked`。只有配置了 `[access].password` 时，`masked` 页面才会显示访问遮罩。`search: false` 会将页面排除在生成的搜索索引之外。

## 顶部导航和侧边栏

使用 `[[top_nav]]` 渲染顶部导航链接或分组菜单。

```toml
[[top_nav]]
text = "指南"
link = "/guide/installation/"
sidebar = "guide"

[[top_nav.items]]
text = "快速开始"
link = "/guide/installation/"

[[top_nav.items]]
text = "Markdown"
link = "/guide/markdown-tutorial/"

[[top_nav]]
text = "参考"
link = "/internals/crates/"
sidebar = "reference"

[[sidebars.guide]]
text = "指南"
link = "/guide/installation/"

[[sidebars.guide.items]]
text = "安装"
link = "/guide/installation/"

[[sidebars.guide.items]]
text = "配置"
link = "/guide/configuration/"

[[sidebars.reference]]
text = "参考"
link = "/internals/crates/"

[[sidebars.reference.items]]
text = "Crates"
link = "/internals/crates/"
```

`top_nav.items` 只控制顶部下拉菜单。省略 `items` 时，该项会渲染为直接的顶层链接。

`sidebars.<name>.items` 控制左侧侧边栏。在顶部导航项上添加 `sidebar = "name"` 可以把该顶部分区绑定到 `sidebars.name`；它不会把 `top_nav.items` 复用成侧边栏条目。只出现在 `top_nav.items`、没有出现在 `sidebars.<name>.items` 的页面，不会被加入左侧侧边栏。

## 多语言文档

RustPress 默认是单语言。添加 `locales` 后会启用基于 URL 的多语言文档。配置了 `locales` 时必须提供 `locales.root`，它代表 `/` 下的默认语言。

```toml
[locales.root]
label = "简体中文"
lang = "zh-CN"

[locales.en]
label = "English"
lang = "en-US"
link = "/en/"

[locales.ja]
label = "日本語"
lang = "ja-JP"
link = "/ja/"

[locales.ko]
label = "한국어"
lang = "ko-KR"
link = "/ko/"

[[locales.en.top_nav]]
text = "Guide"
link = "guide/installation/"
sidebar = "guide"

[[locales.en.top_nav.items]]
text = "Quick Start"
link = "guide/installation/"

[[locales.en.sidebars.guide]]
text = "Guide"
link = "guide/installation/"

[[locales.en.sidebars.guide.items]]
text = "Installation"
link = "guide/installation/"
```

根语言继续使用 `docs/` 下的文件。其他语言的文件放在 `docs/<locale>/` 中。

```text
docs/index.md              -> /
docs/guide/cli.md          -> /guide/cli/
docs/en/index.md           -> /en/
docs/en/guide/cli.md       -> /en/guide/cli/
docs/ja/index.md           -> /ja/
docs/ko/index.md           -> /ko/
```

非 root 语言的链接默认是 `/<locale>/`，也可以用 `link` 覆盖该前缀。Locale 的 `top_nav`、`sidebars` 和 `title` 会覆盖全局值；未配置时回退到全局配置。Locale 顶部导航和侧边栏中的相对链接会解析到该语言前缀下，例如 `locales.en.top_nav` 或 `locales.en.sidebars.guide` 中的 `guide/installation/` 会变成 `/en/guide/installation/`。

只有配置了 `locales` 时，顶部栏才会显示语言选择器。切换语言时会跳到对应译文页面；如果目标语言缺少该页面，则跳到该语言首页。
