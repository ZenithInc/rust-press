---
title: Markdown
layout: doc
sidebar: true
search: true
access: public
---

# Markdown

Markdown은 `pulldown-cmark`로 파싱됩니다. MVP는 표, 작업 목록, 취소선, 각주, 제목 속성, 제목 앵커, Mermaid fenced blocks를 활성화합니다.

## 제목 앵커

모든 제목에는 안정적인 앵커가 부여됩니다. ASCII가 아닌 제목은 보존되므로 `中文 标题` 같은 제목은 `#中文-标题`가 됩니다.

## Mermaid

`mermaid` 언어의 fenced code block은 Mermaid block으로 출력되고 클라이언트 측 Mermaid 스크립트로 렌더링됩니다.

```mermaid
sequenceDiagram
    participant User
    participant CLI
    participant Builder
    User->>CLI: rust-press build
    CLI->>Builder: BuildOptions
    Builder-->>User: dist/
```

## 검색 텍스트

`index_code = false`이면 코드 블록은 검색 인덱스에서 제외됩니다.
