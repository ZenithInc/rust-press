---
title: Markdown
layout: doc
sidebar: true
search: true
access: public
---

# Markdown

Markdown 처리는 `rustpress-md`가 담당합니다. frontmatter, HTML 렌더링, 제목 수집, 검색 텍스트 추출, 코드 블록 확장을 처리합니다.

## 지원 문법

- 표
- 각주
- 취소선
- 작업 목록
- 제목 속성
- 일반 Markdown 문법

## Frontmatter

```yaml
---
title: Page Title
layout: doc
sidebar: true
search: true
access: public
---
```

`title`이 없으면 첫 제목을 사용하고, 제목도 없으면 `Untitled`가 됩니다.

## 제목 앵커

```toml
[markdown]
heading_anchors = true
```

영어는 소문자로 바뀌고 공백은 `-`가 되며 중복 제목에는 번호가 붙습니다. CJK 문자는 유지됩니다.

## 코드 블록

```toml
[markdown]
code_highlight = true
code_line_numbers = true
```

코드 블록에는 syntect 하이라이트, 줄 번호, 언어 라벨, 복사 버튼이 표시됩니다. 복사 내용에는 줄 번호가 포함되지 않습니다.

## Mermaid

```toml
[markdown]
mermaid = true
```

`mermaid` fenced block은 브라우저에서 다이어그램으로 렌더링됩니다. 색상 모드를 바꾸면 다시 렌더링됩니다.

## 검색 텍스트

기본적으로 코드 블록은 검색 본문에 포함되지 않습니다.

```toml
[search]
index_code = false
```

API 문서라면 `true`로 바꿀 수 있습니다.

## Markdown 원본

각 페이지는 같은 라우트 디렉터리에 `index.md.txt`를 출력합니다. 예를 들어 `/guide/cli/`는 `/guide/cli/index.md.txt`를 생성합니다. 테마에서 Markdown 본문과 Markdown URL을 복사할 수 있습니다.
