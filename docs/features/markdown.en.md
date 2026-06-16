---
title: Markdown
layout: doc
sidebar: true
search: true
access: public
---

# Markdown

Markdown support lives in `rustpress-md`. It parses frontmatter, renders HTML, collects headings, extracts search text, and wraps code blocks for theme enhancements.

## Supported Syntax

Enabled `pulldown-cmark` extensions:

- Tables
- Footnotes
- Strikethrough
- Task lists
- Heading attributes

Standard Markdown features such as paragraphs, headings, links, images, quotes, lists, inline code, and fenced code blocks work as expected.

## Frontmatter Defaults

```yaml
---
title: Page Title
layout: doc
sidebar: true
search: true
access: public
---
```

If `title` is missing, RustPress uses the first heading. If there is no heading, the title becomes `Untitled`.

## Heading Anchors

```toml
[markdown]
heading_anchors = true
```

Rules:

- English text is lowercased.
- Spaces and `_` become `-`.
- Punctuation is removed.
- Duplicates get `-2`, `-3`, and so on.
- CJK characters are preserved.

## Code Blocks

Fenced code blocks are rendered with:

- syntect highlighting
- line numbers by default
- language labels
- copy buttons that copy code without line numbers

```toml
[markdown]
code_highlight = true
code_line_numbers = true
```

Language tags such as `language-rust` or `rust,{...}` are normalized to `rust`.

## Mermaid

```toml
[markdown]
mermaid = true
```

`mermaid` code fences become `<pre class="mermaid">` blocks. The theme JavaScript renders them in the browser and rerenders when the color mode changes.

## Search Text

RustPress extracts plain text from Markdown events. Fenced code blocks are excluded by default:

```toml
[search]
index_code = false
```

Set `index_code = true` for API references or code-heavy docs.

## Markdown Source Files

Every page gets an `index.md.txt` copy in the same route directory. For example, `/guide/cli/` emits `/guide/cli/index.md.txt`. The theme provides Copy Markdown and Copy Markdown URL actions, which are useful for review, AI tools, and external automation.
