---
title: Access Mask
layout: doc
sidebar: true
search: true
access: masked
---

# Access Mask

This page is marked with `access: masked`.

The overlay is only a front-end viewing mask. It does not encrypt content, does not remove content from static HTML, and does not provide server-side authentication.

## Test Behavior

The example site configures a demo password in `[access].password`. Enter the correct password to hide the overlay for this browser session.

```toml
[access]
enabled = true
mode = "mask"
password = "rustpress"
password_hint = "Enter password"
```

If `password` is not configured, `access: masked` pages do not show the overlay.

## Note

Access masking is only a front-end overlay, not security protection. The generated HTML and front-end script remain directly viewable.
