---
title: 설정
layout: doc
sidebar: true
search: true
access: public
---

# 설정

RustPress는 `rustpress.toml`로 사이트 기본 정보, 상단 내비게이션, 테마, 검색, 다국어, 접근 마스크를 설정합니다. 설정 파일이 있는 디렉터리가 프로젝트 루트입니다.

## 최소 설정

```toml
title = "My Docs"
src_dir = "docs"
out_dir = "dist"
base = "/"
```

| 필드 | 설명 |
| --- | --- |
| `title` | 사이트 제목이며 페이지 제목 suffix에도 사용됩니다 |
| `src_dir` | Markdown 소스 디렉터리 |
| `out_dir` | 정적 출력 디렉터리. 빌드 전에 정리됩니다 |
| `base` | 배포 경로 prefix. 예: `/rust-press/` |

`base`는 앞뒤 `/`가 자동으로 보정됩니다.

`build`는 HTML 링크, 검색 URL, Markdown 소스 URL에 `base`를 반영합니다. `dev`는 로컬 서빙을 위해 일시적으로 `/`를 사용하므로 하위 경로 배포 설정이 있어도 로컬 미리보기가 동작합니다.

## 상단 내비게이션

`top_nav`는 상단 바의 진입점만 설정합니다. 직접 링크 또는 드롭다운 그룹으로 사용할 수 있습니다.

```toml
[[top_nav]]
text = "가이드"
link = "/guide/cli/"

[[top_nav.items]]
text = "빠른 시작"
link = "/guide/installation/"

[[top_nav.items]]
text = "사이트 설정"
link = "/guide/configuration/"
```

`http://`, `https://`, `mailto:`, `#`로 시작하는 링크는 그대로 유지됩니다. 로컬 링크는 사이트 경로로 정규화되고, 다국어 페이지에서는 현재 locale prefix 아래로 렌더링됩니다. 예를 들어 `/guide/cli/`는 `/ko/guide/cli/`가 됩니다.

이전 `nav`, `top_nav.sidebar`, `sidebars`, `locales.*.top_nav`, `locales.*.sidebars`는 제거되었습니다. 설정에 남아 있으면 로딩 중 에러가 발생합니다.

## 사이드바

사이드바는 `docs/` 경로에서 자동 생성되며 TOML에 작성하지 않습니다. 현재 페이지의 첫 번째 경로 세그먼트에 속한 페이지만 표시하고, 홈페이지에는 디렉터리 사이드바를 표시하지 않습니다.

```text
docs/
  index.md                  -> /
  guide.md                  -> /guide/
  guide/cli.md              -> /guide/cli/
  guide/configuration.md    -> /guide/configuration/
  features/search.md        -> /features/search/
```

`/guide/cli/`에서는 `guide` 아래 페이지만, `/features/search/`에서는 `features` 아래 페이지만 표시합니다. 사이드바는 최대 2단계입니다. 더 깊은 페이지도 라우트는 생성되지만 사이드바에서는 두 번째 단계로 접힙니다.

페이지 제목은 frontmatter `title`을 우선 사용하고, 없으면 첫 번째 Markdown 제목을 사용합니다. `sidebar: false`로 자동 사이드바에서 제외할 수 있습니다.

## Markdown

```toml
[markdown]
mermaid = true
code_highlight = true
code_line_numbers = true
heading_anchors = true
```

## 검색

```toml
[search]
enabled = true
languages = ["zh", "en", "ja", "ko"]
index_code = false
```

페이지 frontmatter의 `search: false`는 해당 페이지를 검색 인덱스에서 제외합니다.

## 테마

```toml
[theme]
name = "default"
skin = "light"
allow_switch = true
github_url = "https://github.com/your-org/your-repo"
```

`skin`은 `light`와 `dark`를 지원합니다. `github_url`을 설정하면 상단 바에 GitHub 아이콘이 표시됩니다.

## 접근 마스크

```toml
[access]
enabled = true
mode = "mask"
password = "rustpress"
password_hint = "Enter password"
```

페이지 frontmatter에 `access: masked`를 설정하면 프런트엔드 비밀번호 마스크가 표시됩니다. 정적 HTML은 출력에 남기 때문에 보안 경계는 아닙니다.

## 페이지 frontmatter

```yaml
---
title: Page Title
layout: doc
sidebar: true
search: true
access: public
---
```

| 필드 | 기본값 | 설명 |
| --- | --- | --- |
| `title` | 첫 제목 또는 `Untitled` | 페이지 제목 |
| `layout` | `doc` | 현재 테마는 doc layout을 사용합니다 |
| `sidebar` | `true` | 자동 사이드바 포함 여부 |
| `search` | `true` | 검색 인덱스 포함 여부 |
| `access` | `public` | `public` 또는 `masked` |

## 다국어 문서

`locales`를 설정하면 `locales.root`가 필요합니다. root 언어 파일은 suffix를 쓰지 않고, 다른 언어는 `.<locale>.md` 파일명 suffix를 사용합니다.

```toml
[locales.root]
label = "简体中文"
lang = "zh-CN"
title = "Chinese Docs"

[locales.en]
label = "English"
lang = "en-US"
link = "/en/"
title = "English Docs"
```

```text
docs/index.md          -> /
docs/index.ko.md       -> /ko/
docs/guide/cli.md      -> /guide/cli/
docs/guide/cli.ko.md   -> /ko/guide/cli/
```

번역 키는 locale suffix를 제거한 뒤 계산됩니다. 대응 번역이 없으면 언어 전환은 대상 locale 홈으로 돌아갑니다. 상단 내비게이션은 root에 한 번만 설정합니다.

## 정적 리소스

프로젝트 루트의 `public/`은 `out_dir`로 복사됩니다.

```text
public/logo.png -> dist/logo.png
```
