use regex::Regex;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SearchConfig {
    pub languages: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SearchPage {
    pub title: String,
    pub url: String,
    pub headings: Vec<String>,
    pub body: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SearchIndex {
    pub version: u8,
    pub languages: Vec<String>,
    pub pages: Vec<SearchIndexPage>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SearchIndexPage {
    pub id: String,
    pub title: String,
    pub url: String,
    pub headings: Vec<String>,
    pub body: String,
    pub tokens: Vec<String>,
}

pub fn build_search_index(config: SearchConfig, pages: &[SearchPage]) -> SearchIndex {
    SearchIndex {
        version: 1,
        languages: config.languages,
        pages: pages
            .iter()
            .map(|page| {
                let mut tokens = tokenize(&format!(
                    "{} {} {}",
                    page.title,
                    page.headings.join(" "),
                    page.body
                ));
                tokens.sort();
                tokens.dedup();
                SearchIndexPage {
                    id: stable_id(&page.url),
                    title: page.title.clone(),
                    url: page.url.clone(),
                    headings: page.headings.clone(),
                    body: page.body.clone(),
                    tokens,
                }
            })
            .collect(),
    }
}

pub fn tokenize(input: &str) -> Vec<String> {
    let mut tokens = Vec::new();
    let latin = Regex::new(r"[A-Za-z0-9]+").unwrap();
    for capture in latin.find_iter(input) {
        let token = stem_english(&capture.as_str().to_lowercase());
        if !token.is_empty() {
            tokens.push(token);
        }
    }

    for ch in input.chars() {
        if is_cjk(ch) {
            tokens.push(ch.to_string());
        }
    }

    tokens
}

pub fn wasm_placeholder() -> &'static [u8] {
    // Empty WASM module: "\0asm" + version 1. It lets the static output keep
    // the MVP asset contract while JS provides the runtime fallback.
    b"\0asm\x01\0\0\0"
}

fn stem_english(token: &str) -> String {
    for suffix in ["ing", "ed", "es"] {
        if token.len() > suffix.len() + 2 && token.ends_with(suffix) {
            return token[..token.len() - suffix.len()].to_string();
        }
    }
    token.to_string()
}

fn is_cjk(ch: char) -> bool {
    matches!(ch as u32, 0x3400..=0x9fff | 0xf900..=0xfaff)
}

fn stable_id(input: &str) -> String {
    let hash = Sha256::digest(input.as_bytes());
    hash.iter()
        .take(8)
        .map(|byte| format!("{byte:02x}"))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tokenizes_english_case_insensitive_and_chinese() {
        let tokens = tokenize("Running DOCS 中文搜索");

        assert!(tokens.contains(&"runn".to_string()));
        assert!(tokens.contains(&"docs".to_string()));
        assert!(tokens.contains(&"中".to_string()));
        assert!(tokens.contains(&"文".to_string()));
    }

    #[test]
    fn builds_page_index() {
        let index = build_search_index(
            SearchConfig {
                languages: vec!["zh".to_string(), "en".to_string()],
            },
            &[SearchPage {
                title: "Hello".to_string(),
                url: "/".to_string(),
                headings: vec!["Intro".to_string()],
                body: "Body".to_string(),
            }],
        );

        assert_eq!(index.pages.len(), 1);
        assert_eq!(index.pages[0].url, "/");
        assert!(index.pages[0].tokens.contains(&"hello".to_string()));
    }
}
