---
title: 설정
layout: doc
sidebar: true
search: true
access: public
---

# 설정

RustPress는 VitePress 프로젝트 관례 대신 `rustpress.toml`을 사용합니다.

## 예시

```toml
title = "My Docs"
src_dir = "docs"
out_dir = "dist"
base = "/"

[[nav]]
text = "가이드"
link = "/guide/cli/"

[[nav.items]]
text = "CLI"
link = "/guide/cli/"

[[nav.items]]
text = "설정"
link = "/guide/configuration/"

[theme]
name = "default"
skin = "light"
allow_switch = true
github_url = "https://github.com/your-org/your-repo"

[markdown]
mermaid = true
code_highlight = true
heading_anchors = true

[search]
enabled = true
languages = ["zh", "en", "ja", "ko"]
index_code = false

[access]
enabled = true
mode = "mask"
password = "demo123"
password_hint = "Enter password"
```

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

`access`는 `public` 또는 `masked`가 될 수 있습니다. `[access].password`가 설정된 경우에만 `masked` 페이지에 접근 마스크가 표시됩니다. `search: false`는 해당 페이지를 생성된 검색 인덱스에서 제외합니다.

## 상단 내비게이션

`[[nav]]`를 사용해 상단 내비게이션 링크 또는 그룹 메뉴를 렌더링합니다.

```toml
[[nav]]
text = "가이드"
link = "/guide/cli/"

[[nav.items]]
text = "CLI"
link = "/guide/cli/"

[[nav.items]]
text = "설정"
link = "/guide/configuration/"

[[nav]]
text = "참조"
link = "/internals/crates/"
```

`items`가 있으면 테마는 드롭다운 메뉴를 렌더링합니다. `items`가 없으면 해당 항목은 직접적인 상위 링크로 렌더링됩니다.

## 다국어 문서

RustPress는 기본적으로 단일 언어입니다. `locales`를 추가하면 URL 기반 다국어 문서가 활성화됩니다. `locales`가 설정된 경우 `/`의 기본 언어를 나타내는 `locales.root`가 필요합니다.

```toml
[locales.root]
label = "简体中文"
lang = "zh-CN"

[locales.en]
label = "English"
lang = "en-US"
link = "/en/"

[locales.ja]
label = "日本語"
lang = "ja-JP"
link = "/ja/"

[locales.ko]
label = "한국어"
lang = "ko-KR"
link = "/ko/"

[[locales.en.nav]]
text = "Guide"
link = "guide/cli/"
```

루트 언어는 계속 `docs/` 바로 아래의 파일을 사용합니다. 다른 언어 파일은 `docs/<locale>/`에 둡니다.

```text
docs/index.md              -> /
docs/guide/cli.md          -> /guide/cli/
docs/en/index.md           -> /en/
docs/en/guide/cli.md       -> /en/guide/cli/
docs/ja/index.md           -> /ja/
docs/ko/index.md           -> /ko/
```

root가 아닌 언어 링크는 기본적으로 `/<locale>/`입니다. `link`로 이 prefix를 덮어쓸 수 있습니다. Locale의 `nav`와 `title`은 전역 값을 덮어쓰며, 생략하면 전역 설정으로 fallback됩니다. Locale 내비게이션의 상대 링크는 해당 언어 prefix 아래로 해석됩니다. 예를 들어 `locales.en.nav`의 `guide/cli/`는 `/en/guide/cli/`가 됩니다.

언어 선택기는 `locales`가 설정된 경우에만 상단 바에 표시됩니다. 언어를 전환하면 해당 번역 페이지로 이동합니다. 대상 언어에 해당 페이지가 없으면 그 언어의 홈 페이지로 이동합니다.
