---
title: 설치
layout: doc
sidebar: true
search: true
access: public
---

# 설치

RustPress CLI crate 이름은 `rust-press`이며, 설치되는 바이너리 이름도 `rust-press`입니다.

이 저장소는 crates.io 게시를 준비했지만, `cargo install rust-press`는 crate가 게시된 뒤에만 동작합니다. 그전에는 소스 checkout 또는 Git에서 설치하세요.

## 소스 checkout

로컬 checkout에서 CLI를 설치할 수 있습니다.

```bash
git clone https://github.com/ZenithInc/rust-press.git
cd rust-press
cargo install --path crates/rust-press
```

Rust 1.93 이상이 필요합니다.

## Git

GitHub의 최신 커밋에서 설치합니다.

```bash
cargo install --git https://github.com/ZenithInc/rust-press rust-press
```

특정 릴리스 태그를 설치하려면 실제 존재하는 tag로 바꾸세요.

```bash
cargo install --git https://github.com/ZenithInc/rust-press --tag v0.1.2 rust-press
```

## crates.io

crates.io에 게시된 뒤에는 Cargo로 설치할 수 있습니다.

```bash
cargo install rust-press
rust-press --version
```

## 사전 빌드 바이너리

릴리스 tag가 게시된 뒤에는 GitHub Releases가 Linux, macOS, Windows용 사전 빌드 아카이브를 제공합니다. 플랫폼에 맞는 아카이브를 다운로드해 압축을 풀고 `rust-press`를 `PATH`에 두거나, 압축을 푼 디렉터리에서 직접 실행합니다.

```bash
rust-press --help
```

각 릴리스 아카이브에는 SHA256 체크섬 파일도 함께 게시됩니다.

## 업데이트

crate 게시 후에는 최신 Cargo 릴리스로 업데이트할 수 있습니다.

```bash
cargo install rust-press --force
```
