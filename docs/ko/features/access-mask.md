---
title: 접근 마스크
layout: doc
sidebar: true
search: true
access: masked
---

# 접근 마스크

이 페이지는 `access: masked`로 표시되어 있습니다.

오버레이는 프런트엔드 보기 마스크일 뿐입니다. 콘텐츠를 암호화하지 않고, 정적 HTML에서 콘텐츠를 제거하지 않으며, 서버 측 인증을 제공하지 않습니다.

## 동작 확인

예제 사이트는 `[access].password`에 데모 비밀번호를 설정합니다. 올바른 비밀번호를 입력하면 이 브라우저 세션에서 오버레이가 숨겨집니다.

```toml
[access]
enabled = true
mode = "mask"
password = "rustpress"
password_hint = "Enter password"
```

`password`가 설정되지 않은 경우 `access: masked` 페이지에는 오버레이가 표시되지 않습니다.

## 참고

접근 마스크는 프런트엔드 오버레이일 뿐 보안 보호가 아닙니다. 생성된 HTML과 프런트엔드 스크립트는 직접 볼 수 있습니다.
