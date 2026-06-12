---
title: 검색
layout: doc
sidebar: true
search: true
access: public
---

# 검색

RustPress 검색은 완전히 로컬입니다. 빌드 시 인덱스를 만들고 브라우저가 JSON을 읽어 검색합니다.

## 활성화

```toml
[search]
enabled = true
languages = ["zh", "en", "ja", "ko"]
index_code = false
```

`enabled = false`면 검색 입구와 인덱스 생성을 비활성화합니다.

## 출력 파일

```text
dist/assets/search-index.json
dist/assets/search-index.json.br
dist/assets/rustpress_search_bg.wasm
```

현재 실행 스크립트는 JSON 인덱스를 검색합니다.

## 페이지 단위 제어

```yaml
---
title: Internal Note
search: false
---
```

페이지는 렌더링되지만 검색 인덱스에는 포함되지 않습니다.

## 코드 인덱싱

기본적으로 코드 블록은 검색 본문에 포함되지 않습니다.

```toml
[search]
index_code = false
```

API 문서나 코드 예제가 많은 문서라면 `true`로 바꿀 수 있습니다.

## 토큰화

영어는 대소문자를 무시하고 가벼운 어간 처리를 합니다. CJK는 문자 단위 token으로 중국어, 일본어, 한국어 검색을 처리합니다.

## 사용

검색 버튼 또는 `Shift` 두 번으로 검색 대화상자를 엽니다. 검색어는 외부 서비스로 전송되지 않습니다.
