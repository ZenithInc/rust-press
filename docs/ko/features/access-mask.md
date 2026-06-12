---
title: 접근 마스크
layout: doc
sidebar: true
search: true
access: masked
---

# 접근 마스크

이 페이지는 `access: masked`를 사용하므로 프런트엔드 접근 마스크가 표시됩니다.

## 할 수 있는 것

데모, 가벼운 내부 미리보기, 가벼운 열람 억제에 사용할 수 있습니다. 올바른 비밀번호를 입력하면 현재 브라우저 세션에서 잠금 해제 상태가 저장됩니다.

```toml
[access]
enabled = true
mode = "mask"
password = "rustpress"
password_hint = "Enter password"
```

```yaml
---
title: Private Preview
access: masked
---
```

## 할 수 없는 것

접근 마스크는 인증이 아닙니다.

- HTML을 암호화하지 않습니다.
- `dist/`에서 내용을 제거하지 않습니다.
- 소스나 네트워크 응답 열람을 막지 않습니다.
- 서버 로그인, VPN, reverse proxy 인증을 대체하지 않습니다.

민감한 정보에는 호스팅 계층의 접근 제어를 사용하세요.

## 표시 조건

1. `[access].enabled = true`
2. `[access].mode = "mask"`
3. `[access].password`가 비어 있지 않음
4. 페이지가 `access: masked`

## 검색

`access: masked`는 검색 제외가 아닙니다. 필요하면 `search: false`도 지정하세요.
