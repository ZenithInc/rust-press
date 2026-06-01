---
title: Crates
layout: doc
sidebar: true
search: true
access: public
---

# Crates

워크스페이스는 작은 crate들로 나뉘어 있습니다.

## rust-press

명령줄 인자를 파싱하고 명령을 dispatch합니다.

## rustpress-core

설정을 로드하고, 소스 Markdown을 스캔하고, 렌더링을 조율하고, public 자산을 복사하고, 검색 자산을 씁니다.

## rustpress-md

frontmatter를 파싱하고, Markdown을 렌더링하고, 제목 앵커를 생성하고, 검색 텍스트를 추출합니다.

## rustpress-theme

기본 HTML을 렌더링하고 CSS 및 JavaScript runtime 자산을 씁니다.

## rustpress-search

로컬 검색 인덱스를 구축하고 tokenization helpers를 노출합니다.

## rustpress-dev

정적 파일을 제공하고, 소스 파일을 감시하고, 변경 시 다시 빌드하고, 개발 모드에서 새로고침 스크립트를 주입합니다.
