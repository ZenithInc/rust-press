---
title: 설치
layout: doc
sidebar: true
search: true
access: public
---

# 설치

CLI 패키지 이름은 `rust-press`이고 실행 파일 이름도 `rust-press`입니다.

## 요구 사항

- Rust 1.93 이상
- `cargo`를 실행할 수 있는 shell
- 배포 시에는 정적 호스팅만 필요하며 Rust 런타임은 필요하지 않음

```bash
rustc --version
cargo --version
```

## crates.io 설치

```bash
cargo install rust-press
rust-press --version
```

업데이트:

```bash
cargo install rust-press --force
```

## 소스에서 설치

```bash
git clone https://github.com/ZenithInc/rust-press.git
cd rust-press
cargo install --path crates/rust-press
```

설치하지 않고 실행할 수도 있습니다.

```bash
cargo run -p rust-press -- --help
```

## Git에서 설치

```bash
cargo install --git https://github.com/ZenithInc/rust-press rust-press
```

특정 tag:

```bash
cargo install --git https://github.com/ZenithInc/rust-press --tag v0.1.10 rust-press
```

## 첫 사이트 만들기

```bash
rust-press init my-docs
cd my-docs
rust-press dev
```

`init`은 `rustpress.toml`, `docs/index.md`, `docs/private.md`, `public/.gitkeep`을 만듭니다. 기존 파일은 덮어쓰지 않습니다.

## 빌드와 배포

```bash
rust-press build --config rustpress.toml
```

기본 출력 디렉터리는 `dist/`입니다. 이 디렉터리를 GitHub Pages, Nginx, S3, Cloudflare Pages 등 정적 호스팅에 업로드합니다.
