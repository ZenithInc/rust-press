---
title: Search
layout: doc
sidebar: true
search: true
access: public
---

# Search

RustPress search is fully local. The build creates an index, and the browser reads JSON at runtime.

## Enable Search

```toml
[search]
enabled = true
languages = ["zh", "en", "ja", "ko"]
index_code = false
```

`enabled = false` hides the search entry and skips index generation.

## Output Files

```text
dist/assets/search-index.json
dist/assets/search-index.json.br
dist/assets/rustpress_search_bg.wasm
```

The current runtime searches the JSON index. The `.br` and wasm files remain part of the output contract and leave room for later optimization.

## Page-Level Control

```yaml
---
title: Internal Note
search: false
---
```

The page still renders, but it is excluded from the index.

## Code Indexing

Default:

```toml
[search]
index_code = false
```

This prevents identifiers and long code blocks from dominating search results. Set it to `true` for API manuals or snippet libraries.

## Tokenization

The index stores title, URL, headings, and body tokens.

- English matching is case-insensitive with light stemming.
- CJK text is tokenized by character for Chinese, Japanese, and Korean queries.
- The front-end creates snippets from titles and body text.

## User Experience

Users can:

- click the search button
- press `Shift` twice
- type a query and select a result

No search query is sent to an external service.
