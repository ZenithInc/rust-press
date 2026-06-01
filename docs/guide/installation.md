---
title: 安装
layout: doc
sidebar: true
search: true
access: public
---

# 安装

RustPress 的 CLI crate 名为 `rust-press`，安装后的二进制文件也叫 `rust-press`。

当前仓库已经为 crates.io 发布做好准备，但 `cargo install rust-press` 只有在 crate 发布后才可用。未发布前请使用源码目录或 Git 安装。

## 源码目录

本地 checkout 可以直接安装 CLI：

```bash
git clone https://github.com/ZenithInc/rust-press.git
cd rust-press
cargo install --path crates/rust-press
```

需要 Rust 1.93 或更新版本。

## Git

从 GitHub 安装最新提交：

```bash
cargo install --git https://github.com/ZenithInc/rust-press rust-press
```

安装指定发布标签时，把示例版本替换为实际存在的 tag：

```bash
cargo install --git https://github.com/ZenithInc/rust-press --tag v0.1.1 rust-press
```

## crates.io

发布到 crates.io 之后，可以通过 Cargo 安装：

```bash
cargo install rust-press
rust-press --version
```

## 预编译二进制

发布 tag 之后，GitHub Releases 会提供 Linux、macOS 和 Windows 的预编译包。下载匹配平台的压缩包，解压后把 `rust-press` 放入 `PATH`，或直接从解压目录运行：

```bash
rust-press --help
```

每个发布包都会同时提供 SHA256 校验文件。

## 更新

crate 发布之后，通过 Cargo 更新到最新发布版：

```bash
cargo install rust-press --force
```
