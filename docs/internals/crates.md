---
title: Crates
layout: doc
sidebar: true
search: true
access: public
---

# Crates

工作区被拆分为多个小 crate。

## rust-press

解析命令行参数并分发命令。

## rustpress-core

加载配置、扫描源 Markdown、编排渲染、复制 public 资源，并写入搜索资源。

## rustpress-md

解析 frontmatter、渲染 Markdown、生成标题锚点，并提取搜索文本。

## rustpress-theme

渲染默认 HTML，并写入 CSS 和 JavaScript runtime 资源。

## rustpress-search

构建本地搜索索引，并暴露 tokenization helpers。

## rustpress-dev

提供静态文件服务、监听源文件、在变更时重新构建，并在开发模式下注入刷新脚本。
