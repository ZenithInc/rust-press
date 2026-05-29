---
title: Theme
layout: doc
sidebar: true
search: true
access: public
---

# Theme

The default theme is static HTML, CSS, and a small JavaScript runtime.

## Color Modes

The theme exposes two built-in color modes:

- `light`
- `dark`

When `allow_switch = true`, the top bar shows a Light/Dark switcher and saves the selected mode to `localStorage`.

## GitHub Link

Configure `github_url` in `[theme]` to show a GitHub icon on the right side of the top bar. The icon opens the configured repository.

```toml
[theme]
github_url = "https://github.com/your-org/your-repo"
```

## Layout

The generated pages include:

- sticky top navigation
- sidebar navigation
- responsive mobile menu
- table of contents for level 2 and level 3 headings
- local search dialog

The theme avoids claiming that front-end masking is security.
