---
title: 검색
layout: doc
sidebar: true
search: true
access: public
---

# 검색

RustPress는 빌드 시 로컬 검색 인덱스를 작성합니다.

## 출력 파일

- `dist/assets/search-index.json`
- `dist/assets/search-index.json.br`
- `dist/assets/rustpress_search_bg.wasm`

현재 runtime은 JavaScript fallback으로 JSON 인덱스를 조회합니다. WASM 파일은 MVP 출력 계약을 유지하기 위해 존재합니다.

## English

영어 token은 대소문자를 구분하지 않고 매칭됩니다. `BUILD`, `build`, `search`를 검색해 보세요.

## 중국어 검색

중국어 콘텐츠는 문자 token으로 인덱스에 들어갑니다. `搜索`, `中文`, `访问遮罩`를 검색해 로컬 검색 결과를 확인할 수 있습니다.
