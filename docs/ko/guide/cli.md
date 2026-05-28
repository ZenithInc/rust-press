---
title: CLI
layout: doc
sidebar: true
search: true
access: public
---

# CLI

바이너리 이름은 `rust-press`입니다.

## init

```bash
rust-press init [dir]
```

생성 항목:

- `rustpress.toml`
- `docs/index.md`
- `docs/private.md`
- `public/.gitkeep`

이 명령은 기존 파일 덮어쓰기를 거부합니다.

## build

```bash
rust-press build --config rustpress.toml
```

빌드 출력은 설정된 `out_dir`에 기록됩니다. 기본값은 `dist`입니다.

## dev

```bash
rust-press dev --host 0.0.0.0 --port 5190
```

개발 서버는 초기 빌드를 수행하고, `dist`를 제공하며, Markdown과 설정 파일을 감시하고, HTML 응답에 작은 새로고침 스크립트를 주입합니다.

## preview

```bash
rust-press preview --host 127.0.0.1 --port 4177
```

Preview는 파일 감시 없이 이미 빌드된 정적 출력을 제공합니다.
