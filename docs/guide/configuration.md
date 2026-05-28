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

[[nav]]
text = "指南"
link = "/guide/cli/"

[[nav.items]]
text = "命令行"
link = "/guide/cli/"

[[nav.items]]
text = "配置"
link = "/guide/configuration/"

[theme]
name = "default"
skin = "light"
allow_switch = true

[markdown]
mermaid = true
code_highlight = true
heading_anchors = true

[search]
enabled = true
languages = ["zh", "en", "ja", "ko"]
index_code = false

[access]
enabled = true
mode = "mask"
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

`access` 可以是 `public` 或 `masked`。`search: false` 会将页面排除在生成的搜索索引之外。

## 顶部导航

使用 `[[nav]]` 渲染顶部导航链接或分组菜单。

```toml
[[nav]]
text = "指南"
link = "/guide/cli/"

[[nav.items]]
text = "命令行"
link = "/guide/cli/"

[[nav.items]]
text = "配置"
link = "/guide/configuration/"

[[nav]]
text = "参考"
link = "/internals/crates/"
```

存在 `items` 时，主题会渲染下拉菜单。省略 `items` 时，该项会渲染为直接的顶层链接。

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

[[locales.en.nav]]
text = "Guide"
link = "guide/cli/"
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

非 root 语言的链接默认是 `/<locale>/`，也可以用 `link` 覆盖该前缀。Locale 的 `nav` 和 `title` 会覆盖全局值；未配置时回退到全局配置。Locale 导航中的相对链接会解析到该语言前缀下，例如 `locales.en.nav` 中的 `guide/cli/` 会变成 `/en/guide/cli/`。

只有配置了 `locales` 时，顶部栏才会显示语言选择器。切换语言时会跳到对应译文页面；如果目标语言缺少该页面，则跳到该语言首页。
