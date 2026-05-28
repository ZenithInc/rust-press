---
title: Crates
layout: doc
sidebar: true
search: true
access: public
---

# Crates

The workspace is split into small crates.

## rustpress-cli

Parses command-line arguments and dispatches commands.

## rustpress-core

Loads config, scans source Markdown, orchestrates rendering, copies public assets, and writes search assets.

## rustpress-md

Parses frontmatter, renders Markdown, generates heading anchors, and extracts search text.

## rustpress-theme

Renders default HTML and writes CSS and JavaScript runtime assets.

## rustpress-search

Builds the local search index and exposes tokenization helpers.

## rustpress-dev

Serves static files, watches source files, rebuilds on change, and injects a refresh script in dev mode.
