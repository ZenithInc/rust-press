---
title: 命令行
layout: doc
sidebar: true
search: true
access: public
---

# 命令行

`rust-press` 提供四个命令：`init`、`build`、`dev`、`preview`。

## 命令概览

| 命令 | 用途 | 默认行为 |
| --- | --- | --- |
| `init [dir]` | 创建新文档项目 | 目标目录默认为当前目录 |
| `build` | 生成静态站点 | 读取 `rustpress.toml`，写入 `dist/` |
| `dev` | 本地开发 | 构建、启动服务、监听变更、自动刷新 |
| `preview` | 预览产物 | 只服务已构建的 `out_dir` |

查看帮助：

```bash
rust-press --help
rust-press build --help
```

## init

```bash
rust-press init my-docs
```

创建一个最小项目：

```text
my-docs/
├── rustpress.toml
├── docs/
│   ├── index.md
│   └── private.md
└── public/
    └── .gitkeep
```

生成的配置已经包含顶部导航、侧边栏、主题、搜索和访问遮罩示例。

## build

```bash
rust-press build --config rustpress.toml
```

构建流程会：

1. 加载并规范化配置。
2. 扫描 `src_dir` 下的 Markdown。
3. 解析 frontmatter、标题和正文。
4. 渲染页面、顶部导航、侧边栏、目录、语言切换器。
5. 写入搜索索引和主题资源。
6. 复制 `public/` 中的静态资源。

构建前会清理 `out_dir`。不要把需要保留的手工文件直接放在 `dist/` 中。

## dev

```bash
rust-press dev --config rustpress.toml --host 127.0.0.1 --port 5177
```

`dev` 会先构建一次，然后：

- 服务 `out_dir`。
- 监听 `src_dir` 和配置文件。
- 文件创建、修改、删除后重新构建。
- 向 HTML 注入一个小型 live reload 脚本。

默认地址是 `http://127.0.0.1:5177/`。

## preview

```bash
rust-press preview --config rustpress.toml --host 127.0.0.1 --port 4177
```

`preview` 不监听文件变化，不重新构建，只把当前 `out_dir` 当静态目录服务。它适合发布前检查已经生成的内容。

## 配置路径

所有需要配置的命令都支持 `--config`：

```bash
rust-press build --config site/rustpress.toml
rust-press dev --config site/rustpress.toml
rust-press preview --config site/rustpress.toml
```

相对路径会以配置文件所在目录作为项目根目录。因此 `src_dir = "docs"` 表示配置文件旁边的 `docs/`。
