---
title: 主题
layout: doc
sidebar: true
search: true
access: public
---

# 主题

RustPress 目前提供一个内置默认主题。主题由静态 HTML、CSS 和 JavaScript runtime 组成，不依赖前端构建工具。

## 页面布局

默认页面包含：

- sticky 顶部栏。
- 顶部下拉导航。
- 左侧侧边栏。
- 正文区域。
- 二级和三级标题目录。
- 语言切换器。
- 搜索对话框。
- Light/Dark 切换器。
- GitHub 链接。
- 右下角 Markdown 复制菜单。

移动端会折叠布局，保留顶部导航和文档正文的可读性。

## 顶部导航和侧边栏

主题同时渲染两套导航：

- `top_nav` 渲染顶部菜单和顶部下拉。
- Markdown 路径自动生成左侧文档目录。

顶部导航不会决定侧边栏内容。当前页面属于哪个一级目录，左侧就显示该目录下的 Markdown 页面。

## 颜色模式

```toml
[theme]
skin = "light"
allow_switch = true
```

`skin` 支持：

- `light`
- `dark`

`allow_switch = true` 时，顶部栏显示切换按钮。用户选择会保存到 `localStorage`。

## GitHub 链接

```toml
[theme]
github_url = "https://github.com/your-org/your-repo"
```

配置后顶部栏会显示 GitHub 图标。空字符串会隐藏图标。

## 搜索 UI

开启 `[search].enabled` 后，主题会显示搜索按钮。用户也可以按两次 `Shift` 打开搜索对话框。

搜索结果在浏览器中读取 `assets/search-index.json`，不请求后端服务。

## Markdown 和代码复制

主题提供两类复制能力：

- 每个代码块的复制按钮。
- 页面右下角的 Markdown 复制菜单，可以复制当前页面源码或对应的 `index.md.txt` URL。

复制按钮会在成功后短暂显示完成状态。

## 访问遮罩 UI

`access: masked` 页面会显示前端遮罩面板。主题文案明确提示：这是查看遮挡，不是安全保护。
