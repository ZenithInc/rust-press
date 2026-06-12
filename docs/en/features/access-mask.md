---
title: Access Mask
layout: doc
sidebar: true
search: true
access: masked
---

# Access Mask

This page uses `access: masked`, so RustPress shows the front-end access mask.

## What It Does

The mask is useful for demos, lightweight internal previews, or avoiding casual browsing. After the user enters the configured password, the current browser session remembers the unlock state.

```toml
[access]
enabled = true
mode = "mask"
password = "rustpress"
password_hint = "Enter password"
```

Page frontmatter:

```yaml
---
title: Private Preview
access: masked
---
```

## What It Does Not Do

The access mask is not authentication:

- It does not encrypt HTML.
- It does not remove content from `dist/`.
- It does not prevent reading source files or network responses.
- It does not replace server login, VPN, reverse-proxy auth, or storage permissions.

Use real hosting-level access control for sensitive content.

## Display Conditions

The mask appears only when all conditions are true:

1. `[access].enabled = true`
2. `[access].mode = "mask"`
3. `[access].password` is not empty
4. The page has `access: masked`

Otherwise the page renders normally.

## Unlock Behavior

The runtime stores the unlock state for the current path in `sessionStorage`.

- Refreshing the same page stays unlocked.
- Closing the browser session requires unlocking again.
- Different paths are tracked separately.

## Search

`access: masked` does not exclude a page from search. Add this if needed:

```yaml
search: false
```
