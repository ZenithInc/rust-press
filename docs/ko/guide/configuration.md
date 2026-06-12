---
title: 설정
layout: doc
sidebar: true
search: true
access: public
---

# 설정

RustPress는 `rustpress.toml`로 사이트, 내비게이션, 테마, 검색, 다국어, 접근 마스크를 설정합니다. 설정 파일이 있는 디렉터리가 프로젝트 루트입니다.

## 최소 설정

```toml
title = "My Docs"
src_dir = "docs"
out_dir = "dist"
base = "/"
```

| 항목 | 설명 |
| --- | --- |
| `title` | 사이트 제목 |
| `src_dir` | Markdown 소스 디렉터리 |
| `out_dir` | 정적 출력 디렉터리. 빌드 전 삭제됨 |
| `base` | 배포 경로 접두사 |

## 상단 내비게이션

`top_nav`는 상단 메뉴만 제어합니다.

```toml
[[top_nav]]
text = "가이드"
link = "/guide/cli/"
sidebar = "guide"

[[top_nav.items]]
text = "빠른 시작"
link = "/guide/installation/"
```

`sidebar = "guide"`는 상단 섹션을 `sidebars.guide`에 연결할 뿐입니다. `top_nav.items`는 사이드바에 재사용되지 않습니다. 이전 `nav` 설정은 유효하지 않습니다.

## 사이드바

```toml
[[sidebars.guide]]
text = "가이드"
link = "/guide/cli/"

[[sidebars.guide.items]]
text = "CLI"
link = "/guide/cli/"

[[sidebars.guide.items]]
text = "설치"
link = "/guide/installation/"
```

`sidebars.<id>.items`가 왼쪽 사이드바를 결정합니다. 명시적 `sidebars`가 없으면 Markdown 경로에서 자동 생성됩니다.

## Markdown

```toml
[markdown]
mermaid = true
code_highlight = true
code_line_numbers = true
heading_anchors = true
```

Mermaid, 코드 하이라이트, 줄 번호, 제목 앵커를 제어합니다.

## 검색

```toml
[search]
enabled = true
languages = ["zh", "en", "ja", "ko"]
index_code = false
```

페이지 frontmatter에 `search: false`를 쓰면 그 페이지만 검색에서 제외됩니다.

## 테마

```toml
[theme]
name = "default"
skin = "light"
allow_switch = true
github_url = "https://github.com/your-org/your-repo"
```

`skin`은 `light` 또는 `dark`입니다. `allow_switch`는 전환 버튼을 표시합니다.

## 접근 마스크

```toml
[access]
enabled = true
mode = "mask"
password = "rustpress"
password_hint = "Enter password"
```

페이지에서 `access: masked`를 지정하면 프런트엔드 마스크가 표시됩니다. 보안 기능은 아닙니다.

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

| 항목 | 기본값 | 설명 |
| --- | --- | --- |
| `title` | 첫 제목 또는 `Untitled` | 페이지 제목 |
| `layout` | `doc` | 문서 레이아웃 |
| `sidebar` | `true` | 자동 사이드바 포함 여부 |
| `search` | `true` | 검색 포함 여부 |
| `access` | `public` | `public` 또는 `masked` |

## 다국어 문서

`locales`를 설정하면 `locales.root`가 필요합니다. root는 `docs/`, 다른 언어는 `docs/<locale>/`에 둡니다.

```toml
[locales.root]
label = "简体中文"
lang = "zh-CN"

[locales.en]
label = "English"
lang = "en-US"
link = "/en/"

[[locales.en.top_nav]]
text = "Guide"
link = "guide/cli/"
sidebar = "guide"
```

locale 안의 상대 링크는 locale 접두사 아래로 해석됩니다.

## 정적 자산

`public/`의 내용은 `out_dir`로 복사됩니다.
