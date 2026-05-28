---
title: 命令行
layout: doc
sidebar: true
search: true
access: public
---

# 命令行

二进制文件名为 `rust-press`。

## init

```bash
rust-press init [dir]
```

创建：

- `rustpress.toml`
- `docs/index.md`
- `docs/private.md`
- `public/.gitkeep`

该命令会拒绝覆盖已有文件。

## build

```bash
rust-press build --config rustpress.toml
```

构建产物会写入配置中的 `out_dir`，默认是 `dist`。

## dev

```bash
rust-press dev --host 0.0.0.0 --port 5190
```

开发服务器会先执行一次构建，提供 `dist` 服务，监听 Markdown 和配置文件变化，并向 HTML 响应注入一个小型刷新脚本。

## preview

```bash
rust-press preview --host 127.0.0.1 --port 4177
```

Preview 会提供已经构建好的静态输出，不监听文件变化。
