---
title: Theme
layout: doc
sidebar: true
search: true
access: public
---

# Theme

RustPress currently ships one built-in theme. It is static HTML, CSS, and a small JavaScript runtime; no front-end build step is required.

## Layout

The default page includes:

- sticky top bar
- top dropdown navigation
- left sidebar
- document body
- H2/H3 table of contents
- language switcher
- search dialog
- Light/Dark switch
- GitHub link
- Markdown copy menu

The mobile layout keeps the page readable while collapsing dense navigation.

## Top Nav and Sidebar

The theme renders two independent navigation surfaces:

- `top_nav` for the top bar and dropdowns
- Markdown paths for the left document navigation

Top navigation does not define sidebar content. The current page's first directory segment selects the generated sidebar section.

## Color Mode

```toml
[theme]
skin = "light"
allow_switch = true
```

Supported skins:

- `light`
- `dark`

When `allow_switch = true`, the user choice is saved in `localStorage`.

## GitHub Link

```toml
[theme]
github_url = "https://github.com/your-org/your-repo"
```

An empty string hides the icon.

## Search UI

When `[search].enabled` is true, the theme shows a search button. Pressing `Shift` twice also opens the search dialog.

Search reads `assets/search-index.json` in the browser and does not need a server.

## Copy Actions

The theme exposes:

- copy buttons for every code block
- a Markdown copy menu for every page, with actions for the page source and its `index.md.txt` URL

Successful copy actions briefly show a completed state.

## Access Mask UI

Pages with `access: masked` show the front-end mask panel. The copy explicitly says the mask is a viewing layer, not security.
