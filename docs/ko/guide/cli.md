---
title: CLI
layout: doc
sidebar: true
search: true
access: public
---

# CLI

`rust-press`는 `init`, `build`, `dev`, `preview` 네 가지 명령을 제공합니다.

## 명령

| 명령 | 용도 | 기본 동작 |
| --- | --- | --- |
| `init [dir]` | 새 문서 프로젝트 생성 | `dir` 생략 시 현재 디렉터리 |
| `build` | 정적 사이트 생성 | `rustpress.toml`을 읽고 `dist/`에 출력 |
| `dev` | 로컬 개발 | 빌드, 서빙, 감시, 자동 새로고침 |
| `preview` | 생성된 출력 확인 | 현재 `out_dir` 서빙 |

```bash
rust-press --help
rust-press build --help
```

## init

```bash
rust-press init my-docs
```

상단 내비게이션, 사이드바, 테마, 검색, 접근 마스크 예제가 포함된 최소 프로젝트를 만듭니다.

## build

```bash
rust-press build --config rustpress.toml
```

빌드는 설정을 로드하고 Markdown을 파싱한 뒤 페이지, 내비게이션, 사이드바, 목차, 언어 전환, 검색 인덱스, 테마 자산을 생성합니다. 빌드 전에 `out_dir`은 삭제됩니다.

## dev

```bash
rust-press dev --config rustpress.toml --host 127.0.0.1 --port 5177
```

`dev`는 한 번 빌드한 뒤 `src_dir`과 설정 파일을 감시하고 변경 시 다시 빌드합니다. HTML에는 live reload script가 삽입됩니다.

## preview

```bash
rust-press preview --config rustpress.toml --host 127.0.0.1 --port 4177
```

`preview`는 감시나 재빌드 없이 현재 출력만 서빙합니다.

## 설정 경로

```bash
rust-press build --config site/rustpress.toml
```

상대 경로는 설정 파일 위치 기준으로 해석됩니다. `site/rustpress.toml`의 `src_dir = "docs"`는 `site/docs/`를 의미합니다.
