---
title: 테마
layout: doc
sidebar: true
search: true
access: public
---

# 테마

RustPress는 내장 기본 테마를 제공합니다. HTML, CSS, 작은 JavaScript 실행 스크립트만 사용하며 프런트엔드 빌드 단계가 필요 없습니다.

## 레이아웃

- sticky 상단 바
- 상단 드롭다운
- 왼쪽 사이드바
- 본문
- H2/H3 목차
- 언어 전환
- 검색 대화상자
- Light/Dark 전환
- GitHub 링크
- Markdown 복사 메뉴

## 내비게이션

`top_nav`는 상단 바를 제어하고 왼쪽 사이드바는 Markdown 경로에서 자동 생성됩니다. 현재 페이지의 첫 번째 디렉터리 세그먼트가 표시할 사이드바 섹션을 결정합니다.

## 색상 모드

```toml
[theme]
skin = "light"
allow_switch = true
```

`light`와 `dark`를 지원합니다. 선택 값은 `localStorage`에 저장됩니다.

## GitHub 링크

```toml
[theme]
github_url = "https://github.com/your-org/your-repo"
```

빈 문자열이면 아이콘이 표시되지 않습니다.

## 검색과 복사

검색이 활성화되면 상단 바에 검색 버튼이 표시됩니다. `Shift`를 두 번 눌러도 열 수 있습니다. 테마는 코드 복사와 함께 페이지 원본 또는 `index.md.txt` URL을 복사하는 Markdown 복사 메뉴도 제공합니다.

## 접근 마스크

`access: masked` 페이지에는 프런트엔드 마스크 패널이 표시됩니다. 이것은 보기용 마스크이며 보안 기능이 아닙니다.
