use std::collections::HashMap;

use anyhow::{Context, Result};
use once_cell::sync::Lazy;
use pulldown_cmark::{
    html, CodeBlockKind, CowStr, Event, HeadingLevel, Options, Parser, Tag, TagEnd,
};
use regex::Regex;
use serde::{Deserialize, Serialize};
use syntect::easy::HighlightLines;
use syntect::highlighting::{Theme, ThemeSet};
use syntect::html::{styled_line_to_highlighted_html, IncludeBackground};
use syntect::parsing::SyntaxSet;
use syntect::util::LinesWithEndings;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MarkdownOptions {
    pub mermaid: bool,
    pub code_highlight: bool,
    pub code_line_numbers: bool,
    pub heading_anchors: bool,
    pub index_code: bool,
}

impl Default for MarkdownOptions {
    fn default() -> Self {
        Self {
            mermaid: true,
            code_highlight: true,
            code_line_numbers: true,
            heading_anchors: true,
            index_code: false,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(default)]
pub struct Frontmatter {
    pub title: Option<String>,
    pub layout: String,
    pub sidebar: bool,
    pub search: bool,
    pub access: String,
}

impl Default for Frontmatter {
    fn default() -> Self {
        Self {
            title: None,
            layout: "doc".to_string(),
            sidebar: true,
            search: true,
            access: "public".to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Heading {
    pub level: u8,
    pub text: String,
    pub anchor: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Document {
    pub frontmatter: Frontmatter,
    pub title: String,
    pub html: String,
    pub headings: Vec<Heading>,
    pub search_text: String,
}

pub fn parse_markdown(input: &str, options: MarkdownOptions) -> Result<Document> {
    let (frontmatter, markdown) = split_frontmatter(input)?;
    let mut frontmatter = frontmatter;
    normalize_frontmatter(&mut frontmatter);

    let mut parser_options = Options::empty();
    parser_options.insert(Options::ENABLE_TABLES);
    parser_options.insert(Options::ENABLE_FOOTNOTES);
    parser_options.insert(Options::ENABLE_STRIKETHROUGH);
    parser_options.insert(Options::ENABLE_TASKLISTS);
    parser_options.insert(Options::ENABLE_HEADING_ATTRIBUTES);

    let events: Vec<Event<'_>> = Parser::new_ext(markdown, parser_options).collect();
    let headings = collect_headings(&events);
    let html = render_html(events, &headings, &options);
    let search_text = collect_search_text(markdown, options.index_code);
    let title = frontmatter
        .title
        .clone()
        .or_else(|| headings.first().map(|heading| heading.text.clone()))
        .unwrap_or_else(|| "Untitled".to_string());

    Ok(Document {
        frontmatter,
        title,
        html,
        headings,
        search_text,
    })
}

fn split_frontmatter(input: &str) -> Result<(Frontmatter, &str)> {
    let trimmed = input.strip_prefix('\u{feff}').unwrap_or(input);
    if !trimmed.starts_with("---\n") && !trimmed.starts_with("---\r\n") {
        return Ok((Frontmatter::default(), trimmed));
    }

    let body_start = if trimmed.starts_with("---\r\n") { 5 } else { 4 };
    let rest = &trimmed[body_start..];
    for marker in ["\n---\n", "\r\n---\r\n", "\n---\r\n", "\r\n---\n"] {
        if let Some(index) = rest.find(marker) {
            let yaml = &rest[..index];
            let after_marker = &rest[index + marker.len()..];
            let frontmatter = serde_yaml::from_str(yaml).context("failed to parse frontmatter")?;
            return Ok((frontmatter, after_marker));
        }
    }

    anyhow::bail!("frontmatter starts with --- but has no closing marker")
}

fn normalize_frontmatter(frontmatter: &mut Frontmatter) {
    if frontmatter.layout.is_empty() {
        frontmatter.layout = "doc".to_string();
    }
    if frontmatter.access != "public" && frontmatter.access != "masked" {
        frontmatter.access = "public".to_string();
    }
}

fn collect_headings(events: &[Event<'_>]) -> Vec<Heading> {
    let mut headings = Vec::new();
    let mut current: Option<(u8, String)> = None;
    let mut used = HashMap::<String, usize>::new();

    for event in events {
        match event {
            Event::Start(Tag::Heading { level, .. }) => {
                current = Some((heading_level(*level), String::new()));
            }
            Event::Text(text) | Event::Code(text) => {
                if let Some((_, current_text)) = &mut current {
                    current_text.push_str(text);
                }
            }
            Event::End(TagEnd::Heading(_)) => {
                if let Some((level, text)) = current.take() {
                    let anchor = unique_slug(&slugify(&text), &mut used);
                    headings.push(Heading {
                        level,
                        text: text.trim().to_string(),
                        anchor,
                    });
                }
            }
            _ => {}
        }
    }

    headings
}

fn render_html(events: Vec<Event<'_>>, headings: &[Heading], options: &MarkdownOptions) -> String {
    let mut out_events = Vec::with_capacity(events.len() + headings.len());
    let mut heading_index = 0usize;
    let mut in_code_block = false;
    let mut code_lang: Option<String> = None;
    let mut code_text = String::new();

    for event in events {
        match event {
            Event::Start(Tag::Heading { level, .. }) if options.heading_anchors => {
                let anchor = headings
                    .get(heading_index)
                    .map(|heading| heading.anchor.as_str())
                    .unwrap_or_default();
                heading_index += 1;
                out_events.push(Event::Html(CowStr::from(format!(
                    "<{} id=\"{}\"><a class=\"heading-anchor\" href=\"#{}\" aria-label=\"Link to section\">#</a>",
                    heading_tag(level),
                    escape_attr(anchor),
                    escape_attr(anchor)
                ))));
            }
            Event::End(TagEnd::Heading(level)) if options.heading_anchors => {
                out_events.push(Event::Html(CowStr::from(format!(
                    "</{}>",
                    heading_tag(level)
                ))));
            }
            Event::Start(Tag::CodeBlock(kind)) => {
                let lang = match &kind {
                    CodeBlockKind::Fenced(value) => value
                        .split_whitespace()
                        .next()
                        .filter(|value| !value.is_empty())
                        .map(str::to_string),
                    CodeBlockKind::Indented => None,
                };

                if options.mermaid && lang.as_deref() == Some("mermaid") {
                    in_code_block = true;
                    code_lang = lang;
                    out_events.push(Event::Html(CowStr::from("<pre class=\"mermaid\">")));
                } else {
                    in_code_block = true;
                    code_lang = lang;
                    code_text.clear();
                }
            }
            Event::Text(text) if in_code_block && code_lang.as_deref() == Some("mermaid") => {
                out_events.push(Event::Html(CowStr::from(escape_html(&text))));
            }
            Event::Text(text) | Event::Code(text) if in_code_block => {
                code_text.push_str(&text);
            }
            Event::End(TagEnd::CodeBlock) if in_code_block => {
                if code_lang.as_deref() == Some("mermaid") {
                    out_events.push(Event::Html(CowStr::from("</pre>")));
                } else {
                    out_events.push(Event::Html(CowStr::from(render_code_block(
                        &code_text,
                        code_lang.as_deref(),
                        options.code_highlight,
                        options.code_line_numbers,
                    ))));
                    code_text.clear();
                }
                in_code_block = false;
                code_lang = None;
            }
            _ => out_events.push(event),
        }
    }

    let mut rendered = String::new();
    html::push_html(&mut rendered, out_events.into_iter());
    rendered
}

fn render_code_block(
    code: &str,
    lang: Option<&str>,
    highlight: bool,
    code_line_numbers: bool,
) -> String {
    let code = trim_trailing_blank_lines(code);
    let normalized_lang = lang
        .map(normalize_code_lang)
        .filter(|lang| !lang.is_empty());
    let content = if highlight {
        highlight_code(code, normalized_lang)
    } else {
        escape_html(code)
    };
    let lang_class = normalized_lang
        .map(|lang| format!(" language-{}", escape_attr(lang)))
        .unwrap_or_default();
    let header = normalized_lang
        .map(|lang| {
            format!(
                r#"<div class="rp-code-header"><span>{}</span></div>"#,
                escape_html(lang)
            )
        })
        .unwrap_or_default();

    if code_line_numbers {
        let line_count = LinesWithEndings::from(code).count().max(1);
        let lines = (1..=line_count)
            .map(|line| line.to_string())
            .collect::<Vec<_>>()
            .join("\n");
        return format!(
            r#"<div class="rp-code rp-code-line-numbers">{header}<button class="rp-code-copy" type="button" data-rp-copy-code aria-label="Copy code" title="Copy code"><svg class="rp-code-copy-icon" viewBox="0 0 24 24" aria-hidden="true"><rect x="9" y="9" width="11" height="11" rx="2"></rect><path d="M5 15H4a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h9a2 2 0 0 1 2 2v1"></path></svg><svg class="rp-code-copy-check" viewBox="0 0 24 24" aria-hidden="true"><path d="M20 6 9 17l-5-5"></path></svg></button><pre><span class="rp-code-lines" aria-hidden="true">{lines}</span><code class="rp-code-content{lang_class}">{content}</code></pre></div>"#
        );
    }

    format!(
        r#"<div class="rp-code">{header}<button class="rp-code-copy" type="button" data-rp-copy-code aria-label="Copy code" title="Copy code"><svg class="rp-code-copy-icon" viewBox="0 0 24 24" aria-hidden="true"><rect x="9" y="9" width="11" height="11" rx="2"></rect><path d="M5 15H4a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h9a2 2 0 0 1 2 2v1"></path></svg><svg class="rp-code-copy-check" viewBox="0 0 24 24" aria-hidden="true"><path d="M20 6 9 17l-5-5"></path></svg></button><pre><code class="rp-code-content{lang_class}">{content}</code></pre></div>"#
    )
}

fn trim_trailing_blank_lines(code: &str) -> &str {
    let Some((last_non_whitespace, ch)) = code.char_indices().rfind(|(_, ch)| !ch.is_whitespace())
    else {
        return "";
    };
    let last_non_whitespace_end = last_non_whitespace + ch.len_utf8();
    let trailing = &code[last_non_whitespace_end..];
    let line_break = match (trailing.find('\n'), trailing.find('\r')) {
        (Some(newline), Some(carriage_return)) => Some(newline.min(carriage_return)),
        (Some(newline), None) => Some(newline),
        (None, Some(carriage_return)) => Some(carriage_return),
        (None, None) => None,
    };

    line_break
        .map(|line_break| &code[..last_non_whitespace_end + line_break])
        .unwrap_or(code)
}

fn normalize_code_lang(lang: &str) -> &str {
    lang.trim()
        .trim_start_matches("language-")
        .split([',', '{'])
        .next()
        .unwrap_or("")
        .trim()
}

fn highlight_code(code: &str, lang: Option<&str>) -> String {
    let syntax = lang
        .and_then(|lang| SYNTAX_SET.find_syntax_by_token(lang))
        .unwrap_or_else(|| SYNTAX_SET.find_syntax_plain_text());
    let mut highlighter = HighlightLines::new(syntax, highlight_theme());
    let mut html = String::new();

    for line in LinesWithEndings::from(code) {
        match highlighter
            .highlight_line(line, &SYNTAX_SET)
            .and_then(|regions| styled_line_to_highlighted_html(&regions, IncludeBackground::No))
        {
            Ok(line_html) => html.push_str(&line_html),
            Err(_) => html.push_str(&escape_html(line)),
        }
    }

    html
}

static SYNTAX_SET: Lazy<SyntaxSet> = Lazy::new(SyntaxSet::load_defaults_newlines);
static THEME_SET: Lazy<ThemeSet> = Lazy::new(ThemeSet::load_defaults);

fn highlight_theme() -> &'static Theme {
    THEME_SET
        .themes
        .get("base16-ocean.dark")
        .or_else(|| THEME_SET.themes.values().next())
        .expect("syntect ships with default themes")
}

fn collect_search_text(markdown: &str, index_code: bool) -> String {
    let mut parser_options = Options::empty();
    parser_options.insert(Options::ENABLE_TABLES);
    parser_options.insert(Options::ENABLE_STRIKETHROUGH);
    parser_options.insert(Options::ENABLE_TASKLISTS);

    let mut text = String::new();
    let mut in_code_block = false;

    for event in Parser::new_ext(markdown, parser_options) {
        match event {
            Event::Start(Tag::CodeBlock(_)) => in_code_block = true,
            Event::End(TagEnd::CodeBlock) => in_code_block = false,
            Event::Text(value) | Event::Code(value) => {
                if index_code || !in_code_block {
                    if !text.is_empty() {
                        text.push(' ');
                    }
                    text.push_str(&value);
                }
            }
            _ => {}
        }
    }

    normalize_space(&text)
}

fn heading_level(level: HeadingLevel) -> u8 {
    match level {
        HeadingLevel::H1 => 1,
        HeadingLevel::H2 => 2,
        HeadingLevel::H3 => 3,
        HeadingLevel::H4 => 4,
        HeadingLevel::H5 => 5,
        HeadingLevel::H6 => 6,
    }
}

fn heading_tag(level: HeadingLevel) -> &'static str {
    match level {
        HeadingLevel::H1 => "h1",
        HeadingLevel::H2 => "h2",
        HeadingLevel::H3 => "h3",
        HeadingLevel::H4 => "h4",
        HeadingLevel::H5 => "h5",
        HeadingLevel::H6 => "h6",
    }
}

fn slugify(text: &str) -> String {
    static PUNCT: Lazy<Regex> =
        Lazy::new(|| Regex::new(r"[^\p{Alphabetic}\p{Number}_\-\s]+").unwrap());
    static SPACE: Lazy<Regex> = Lazy::new(|| Regex::new(r"[\s_]+").unwrap());

    let lower = text.trim().to_lowercase();
    let without_punct = PUNCT.replace_all(&lower, "");
    let slug = SPACE.replace_all(without_punct.trim(), "-");
    if slug.is_empty() {
        "section".to_string()
    } else {
        slug.to_string()
    }
}

fn unique_slug(slug: &str, used: &mut HashMap<String, usize>) -> String {
    let count = used.entry(slug.to_string()).or_insert(0);
    *count += 1;
    if *count == 1 {
        slug.to_string()
    } else {
        format!("{slug}-{}", *count)
    }
}

fn normalize_space(input: &str) -> String {
    input.split_whitespace().collect::<Vec<_>>().join(" ")
}

fn escape_html(input: &str) -> String {
    input
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
}

fn escape_attr(input: &str) -> String {
    escape_html(input).replace('"', "&quot;")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_frontmatter_and_title() {
        let doc = parse_markdown(
            "---\ntitle: Page Title\naccess: masked\n---\n# Ignored\nBody",
            MarkdownOptions::default(),
        )
        .unwrap();

        assert_eq!(doc.title, "Page Title");
        assert_eq!(doc.frontmatter.access, "masked");
        assert!(doc.html.contains("id=\"ignored\""));
    }

    #[test]
    fn chinese_heading_anchor_is_preserved() {
        let doc = parse_markdown("# 中文 标题\n\ntext", MarkdownOptions::default()).unwrap();

        assert_eq!(doc.headings[0].anchor, "中文-标题");
        assert!(doc.html.contains("id=\"中文-标题\""));
    }

    #[test]
    fn mermaid_code_block_becomes_mermaid_pre() {
        let doc = parse_markdown(
            "```mermaid\nflowchart LR\nA-->B\n```",
            MarkdownOptions::default(),
        )
        .unwrap();

        assert!(doc.html.contains("<pre class=\"mermaid\">"));
        assert!(doc.html.contains("A--&gt;B"));
        assert!(!doc.html.contains("data-rp-copy-code"));
        assert!(!doc.html.contains("rp-code-line-numbers"));
        assert!(!doc.html.contains("rp-code-lines"));
    }

    #[test]
    fn fenced_code_has_copy_button_and_is_highlighted_with_syntect() {
        let doc = parse_markdown(
            "```rust\nfn main() {\n    println!(\"hi\");\n}\n```",
            MarkdownOptions::default(),
        )
        .unwrap();

        assert!(doc.html.contains("class=\"rp-code rp-code-line-numbers\""));
        assert!(doc.html.contains("class=\"rp-code-copy\""));
        assert!(doc.html.contains("data-rp-copy-code"));
        assert!(doc.html.contains("aria-label=\"Copy code\""));
        assert!(doc.html.contains("rp-code-line-numbers"));
        assert!(doc
            .html
            .contains("class=\"rp-code-lines\" aria-hidden=\"true\""));
        assert!(doc.html.contains("language-rust"));
        assert!(doc.html.contains("<span style="));
        assert!(doc.html.contains("println"));
    }

    #[test]
    fn code_line_numbers_can_be_disabled() {
        let doc = parse_markdown(
            "```rust\nfn main() { println!(\"hi\"); }\n```",
            MarkdownOptions {
                code_line_numbers: false,
                ..MarkdownOptions::default()
            },
        )
        .unwrap();

        assert!(doc.html.contains("class=\"rp-code\""));
        assert!(doc.html.contains("data-rp-copy-code"));
        assert!(!doc.html.contains("rp-code-line-numbers"));
        assert!(!doc.html.contains("rp-code-lines"));
    }

    #[test]
    fn code_highlight_can_be_disabled_without_removing_copy_button() {
        let doc = parse_markdown(
            "```rust\nfn main() { println!(\"<hi>\"); }\n```",
            MarkdownOptions {
                code_highlight: false,
                ..MarkdownOptions::default()
            },
        )
        .unwrap();

        assert!(doc.html.contains("class=\"rp-code rp-code-line-numbers\""));
        assert!(doc.html.contains("data-rp-copy-code"));
        assert!(doc.html.contains("class=\"rp-code-content language-rust\""));
        assert!(doc.html.contains("println!(\"&lt;hi&gt;\")"));
        assert!(!doc.html.contains("<span style="));
    }

    #[test]
    fn code_line_numbers_match_multiline_trailing_and_empty_blocks() {
        let multiline = render_code_block("one\ntwo\n\n", None, false, true);
        assert!(
            multiline.contains("<span class=\"rp-code-lines\" aria-hidden=\"true\">1\n2</span>")
        );
        assert!(multiline.contains("<code class=\"rp-code-content\">one\ntwo</code>"));

        let empty = render_code_block("", None, false, true);
        assert!(empty.contains("<span class=\"rp-code-lines\" aria-hidden=\"true\">1</span>"));
    }

    #[test]
    fn code_block_trims_trailing_whitespace_only_lines() {
        let html = render_code_block("one\n  \n\t\n", None, false, true);

        assert!(html.contains("<span class=\"rp-code-lines\" aria-hidden=\"true\">1</span>"));
        assert!(html.contains("<code class=\"rp-code-content\">one</code>"));
    }

    #[test]
    fn code_block_preserves_trailing_spaces_on_last_content_line() {
        let html = render_code_block("one  \n\n", None, false, true);

        assert!(html.contains("<span class=\"rp-code-lines\" aria-hidden=\"true\">1</span>"));
        assert!(html.contains("<code class=\"rp-code-content\">one  </code>"));
    }

    #[test]
    fn code_content_does_not_include_line_numbers() {
        let doc = parse_markdown(
            "```\nalpha\nbeta\n```",
            MarkdownOptions {
                code_highlight: false,
                ..MarkdownOptions::default()
            },
        )
        .unwrap();

        assert!(doc
            .html
            .contains("class=\"rp-code-lines\" aria-hidden=\"true\">1\n2</span>"));
        assert_eq!(code_content(&doc.html), "alpha\nbeta");
    }

    #[test]
    fn code_is_excluded_from_search_by_default() {
        let doc = parse_markdown(
            "Body\n\n```rust\nlet hidden = true;\n```",
            MarkdownOptions::default(),
        )
        .unwrap();

        assert!(doc.search_text.contains("Body"));
        assert!(!doc.search_text.contains("hidden"));
    }

    fn code_content(html: &str) -> &str {
        let class_start = html.find("class=\"rp-code-content").unwrap();
        let content_start = class_start + html[class_start..].find('>').unwrap() + 1;
        let content_end = content_start + html[content_start..].find("</code>").unwrap();
        &html[content_start..content_end]
    }
}
