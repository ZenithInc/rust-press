---
title: 安装
layout: doc
sidebar: true
search: true
access: public
---

# 安装

RustPress 的 CLI 包名是 `rust-press`，安装后的命令也是 `rust-press`。

## 环境要求

- Rust 1.93 或更新版本。
- 一个可以执行 `cargo` 的 shell。
- 发布静态站点时只需要普通静态文件托管，不需要 Rust 运行时。

检查本机 Rust：

```bash
rustc --version
cargo --version
```

## 从 crates.io 安装

发布到 crates.io 后，推荐使用 Cargo 安装：

```bash
cargo install rust-press
rust-press --version
```

更新到最新发布版：

```bash
cargo install rust-press --force
```

## 从源码安装

本地 checkout 可以直接安装当前源码：

```bash
git clone https://github.com/ZenithInc/rust-press.git
cd rust-press
cargo install --path crates/rust-press
```

如果只想临时运行，也可以不安装：

```bash
cargo run -p rust-press -- --help
```

## 从 Git 安装

安装 main 分支最新提交：

```bash
cargo install --git https://github.com/ZenithInc/rust-press rust-press
```

安装指定 tag：

```bash
cargo install --git https://github.com/ZenithInc/rust-press --tag v0.1.11 rust-press
```

## 预编译二进制

推送 `v*` tag 后，GitHub Release workflow 会构建 Linux、macOS 和 Windows 包。下载匹配平台的压缩包，解压后把 `rust-press` 放到 `PATH`，或直接从解压目录运行：

```bash
rust-press --help
```

每个压缩包都会附带 SHA256 校验文件。

## 创建第一个站点

```bash
rust-press init my-docs
cd my-docs
rust-press dev
```

`init` 会创建：

- `rustpress.toml`
- `docs/index.md`
- `docs/private.md`
- `public/.gitkeep`

`init` 不会覆盖已经存在的文件。

## 构建和部署

构建静态输出：

```bash
rust-press build --config rustpress.toml
```

默认输出目录是 `dist/`。把 `dist/` 上传到 GitHub Pages、Nginx、S3、Cloudflare Pages 或任何静态托管服务即可。
