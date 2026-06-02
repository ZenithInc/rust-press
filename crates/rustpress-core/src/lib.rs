use std::collections::BTreeMap;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use walkdir::WalkDir;

use rustpress_md::{parse_markdown, Document, MarkdownOptions};
use rustpress_search::{build_search_index, SearchConfig, SearchPage};
use rustpress_theme::{
    render_page, write_theme_assets, LanguageOption, NavItem, PageRender, SiteRender, ThemeConfig,
    TopNavItem, TopNavLink,
};

#[derive(Debug, Clone)]
pub struct BuildOptions {
    pub config_path: PathBuf,
}

impl BuildOptions {
    pub fn new(config_path: impl Into<PathBuf>) -> Self {
        Self {
            config_path: config_path.into(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BuildResult {
    pub out_dir: PathBuf,
    pub page_count: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct Config {
    pub title: String,
    pub src_dir: PathBuf,
    pub out_dir: PathBuf,
    pub base: String,
    pub theme: ThemeSection,
    pub markdown: MarkdownSection,
    pub search: SearchSection,
    pub access: AccessSection,
    pub nav: Vec<NavSection>,
    pub sidebars: BTreeMap<String, Vec<SidebarSection>>,
    pub locales: BTreeMap<String, LocaleSection>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct ThemeSection {
    pub name: String,
    pub skin: String,
    pub allow_switch: bool,
    pub github_url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct MarkdownSection {
    pub mermaid: bool,
    pub code_highlight: bool,
    pub code_line_numbers: bool,
    pub heading_anchors: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct SearchSection {
    pub enabled: bool,
    pub languages: Vec<String>,
    pub index_code: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct AccessSection {
    pub enabled: bool,
    pub mode: String,
    pub password: String,
    pub password_hint: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct NavSection {
    pub text: String,
    pub link: Option<String>,
    pub sidebar: Option<String>,
    pub items: Vec<NavLinkSection>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct NavLinkSection {
    pub text: String,
    pub link: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct SidebarSection {
    pub text: String,
    pub link: String,
    pub items: Vec<SidebarLinkSection>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct SidebarLinkSection {
    pub text: String,
    pub link: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct LocaleSection {
    pub label: String,
    pub lang: String,
    pub link: String,
    pub title: Option<String>,
    pub nav: Vec<NavSection>,
    pub sidebars: BTreeMap<String, Vec<SidebarSection>>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            title: "My Docs".to_string(),
            src_dir: "docs".into(),
            out_dir: "dist".into(),
            base: "/".to_string(),
            theme: ThemeSection::default(),
            markdown: MarkdownSection::default(),
            search: SearchSection::default(),
            access: AccessSection::default(),
            nav: Vec::new(),
            sidebars: BTreeMap::new(),
            locales: BTreeMap::new(),
        }
    }
}

impl Default for ThemeSection {
    fn default() -> Self {
        Self {
            name: "default".to_string(),
            skin: "light".to_string(),
            allow_switch: true,
            github_url: String::new(),
        }
    }
}

impl Default for MarkdownSection {
    fn default() -> Self {
        Self {
            mermaid: true,
            code_highlight: true,
            code_line_numbers: true,
            heading_anchors: true,
        }
    }
}

impl Default for SearchSection {
    fn default() -> Self {
        Self {
            enabled: true,
            languages: vec!["zh".to_string(), "en".to_string()],
            index_code: false,
        }
    }
}

impl Default for AccessSection {
    fn default() -> Self {
        Self {
            enabled: true,
            mode: "mask".to_string(),
            password: String::new(),
            password_hint: "Enter password".to_string(),
        }
    }
}

impl Default for NavSection {
    fn default() -> Self {
        Self {
            text: String::new(),
            link: None,
            sidebar: None,
            items: Vec::new(),
        }
    }
}

impl Default for NavLinkSection {
    fn default() -> Self {
        Self {
            text: String::new(),
            link: String::new(),
        }
    }
}

impl Default for LocaleSection {
    fn default() -> Self {
        Self {
            label: String::new(),
            lang: String::new(),
            link: String::new(),
            title: None,
            nav: Vec::new(),
            sidebars: BTreeMap::new(),
        }
    }
}

impl Default for SidebarSection {
    fn default() -> Self {
        Self {
            text: String::new(),
            link: String::new(),
            items: Vec::new(),
        }
    }
}

impl Default for SidebarLinkSection {
    fn default() -> Self {
        Self {
            text: String::new(),
            link: String::new(),
        }
    }
}

impl Config {
    pub fn load(path: &Path) -> Result<Self> {
        let raw = fs::read_to_string(path)
            .with_context(|| format!("failed to read config {}", path.display()))?;
        let mut config: Config = toml::from_str(&raw)
            .with_context(|| format!("failed to parse config {}", path.display()))?;
        config.normalize()?;
        Ok(config)
    }

    fn normalize(&mut self) -> Result<()> {
        if self.base.is_empty() {
            self.base = "/".to_string();
        }
        if !self.base.starts_with('/') {
            self.base.insert(0, '/');
        }
        if !self.base.ends_with('/') {
            self.base.push('/');
        }
        self.theme.skin = normalize_theme_skin(&self.theme.skin);
        self.theme.github_url = self.theme.github_url.trim().to_string();
        self.access.password = self.access.password.trim().to_string();
        normalize_nav(&mut self.nav, None);
        normalize_sidebars(&mut self.sidebars, None);

        if !self.locales.is_empty() {
            if !self.locales.contains_key("root") {
                anyhow::bail!("locales.root is required when locales are configured");
            }

            let keys = self.locales.keys().cloned().collect::<Vec<_>>();
            for key in keys {
                let locale = self
                    .locales
                    .get_mut(&key)
                    .expect("locale key collected from map");
                locale.label = locale.label.trim().to_string();
                if locale.label.is_empty() {
                    locale.label = key.clone();
                }
                locale.lang = locale.lang.trim().to_string();
                if locale.lang.is_empty() {
                    locale.lang = if key == "root" {
                        "en".to_string()
                    } else {
                        key.clone()
                    };
                }
                locale.title = locale
                    .title
                    .take()
                    .map(|title| title.trim().to_string())
                    .filter(|title| !title.is_empty());
                locale.link = if key == "root" {
                    "/".to_string()
                } else {
                    normalize_locale_prefix(&key, &locale.link)?
                };
            }

            for locale in self.locales.values_mut() {
                let locale_prefix = locale.link.clone();
                normalize_nav(&mut locale.nav, Some(&locale_prefix));
                normalize_sidebars(&mut locale.sidebars, Some(&locale_prefix));
            }
        }
        Ok(())
    }
}

pub fn init_project(dir: &Path) -> Result<()> {
    fs::create_dir_all(dir).with_context(|| format!("failed to create {}", dir.display()))?;
    let docs_dir = dir.join("docs");
    let public_dir = dir.join("public");
    fs::create_dir_all(&docs_dir)
        .with_context(|| format!("failed to create {}", docs_dir.display()))?;
    fs::create_dir_all(&public_dir)
        .with_context(|| format!("failed to create {}", public_dir.display()))?;

    write_new(
        &dir.join("rustpress.toml"),
        r#"title = "My Docs"
src_dir = "docs"
out_dir = "dist"
base = "/"

[[nav]]
text = "Guide"
link = "/"
sidebar = "guide"

[[nav.items]]
text = "Home"
link = "/"

[[nav.items]]
text = "Masked Page"
link = "/private/"

[[sidebars.guide]]
text = "Guide"
link = "/"

[[sidebars.guide.items]]
text = "Home"
link = "/"

[[sidebars.guide.items]]
text = "Masked Page"
link = "/private/"

[theme]
name = "default"
skin = "light"
allow_switch = true
github_url = ""

[markdown]
mermaid = true
code_highlight = true
code_line_numbers = true
heading_anchors = true

[search]
enabled = true
languages = ["zh", "en"]
index_code = false

[access]
enabled = true
mode = "mask"
password = "rustpress"
password_hint = "Enter password"
"#,
    )?;

    write_new(
        &docs_dir.join("index.md"),
        r#"---
title: Welcome
layout: doc
sidebar: true
search: true
access: public
---

# Welcome

RustPress turns Markdown into a static documentation site.

## Mermaid

```mermaid
flowchart LR
    A[Markdown] --> B[RustPress]
    B --> C[Static HTML]
```

## Search

Local search indexes English and 中文 content by default.
"#,
    )?;

    write_new(
        &docs_dir.join("private.md"),
        r#"---
title: Masked Page
layout: doc
sidebar: true
search: true
access: masked
---

# Masked Page

This page demonstrates the front-end password mask. The HTML content is still present in the static output.
"#,
    )?;

    write_new(&public_dir.join(".gitkeep"), "")?;
    Ok(())
}

pub fn build_site(options: BuildOptions) -> Result<BuildResult> {
    let config_path = normalize_config_path(&options.config_path)?;
    let project_root = config_path
        .parent()
        .map(Path::to_path_buf)
        .unwrap_or_else(|| PathBuf::from("."));
    let config = Config::load(&config_path)?;
    let src_dir = absolutize(&project_root, &config.src_dir);
    let out_dir = absolutize(&project_root, &config.out_dir);
    let public_dir = project_root.join("public");

    if out_dir.exists() {
        fs::remove_dir_all(&out_dir)
            .with_context(|| format!("failed to clean {}", out_dir.display()))?;
    }
    fs::create_dir_all(&out_dir)
        .with_context(|| format!("failed to create {}", out_dir.display()))?;

    let pages = read_pages(&src_dir, &config)?;
    let translations = build_translation_map(&pages);
    let site = base_site_render(&config);

    write_theme_assets(&out_dir, &site)?;
    copy_public_assets(&public_dir, &out_dir)?;

    let mut search_pages = Vec::new();
    for page in &pages {
        let page_site = site_render_for_page(&config, &pages, &translations, page);
        let rendered = render_page(
            &page_site,
            &PageRender {
                title: page.document.title.clone(),
                route: page.route.clone(),
                html: page.document.html.clone(),
                markdown_source: page.markdown_source.clone(),
                markdown_source_url: markdown_source_url(&config.base, &page.route),
                headings: page.document.headings.clone(),
                masked: page.document.frontmatter.access == "masked",
                search: page.document.frontmatter.search,
            },
        );
        write_page(&out_dir, &page.route, &rendered)?;
        write_markdown_source(&out_dir, &page.route, &page.markdown_source)?;

        if page.document.frontmatter.search {
            search_pages.push(SearchPage {
                title: page.document.title.clone(),
                url: site_url(&config.base, &page.route),
                headings: page
                    .document
                    .headings
                    .iter()
                    .map(|heading| heading.text.clone())
                    .collect(),
                body: page.document.search_text.clone(),
            });
        }
    }

    if config.search.enabled {
        write_search_index(&out_dir, &config.search, &search_pages)?;
    }

    Ok(BuildResult {
        out_dir,
        page_count: pages.len(),
    })
}

#[derive(Debug, Clone)]
struct Page {
    route: String,
    locale_key: String,
    translation_key: String,
    markdown_source: String,
    document: Document,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct PageMetadata {
    route: String,
    locale_key: String,
    translation_key: String,
}

fn read_pages(src_dir: &Path, config: &Config) -> Result<Vec<Page>> {
    let mut pages = Vec::new();
    for entry in WalkDir::new(src_dir).sort_by_file_name() {
        let entry = entry.with_context(|| format!("failed to scan {}", src_dir.display()))?;
        if !entry.file_type().is_file() {
            continue;
        }
        if entry.path().extension().and_then(|value| value.to_str()) != Some("md") {
            continue;
        }

        let markdown = fs::read_to_string(entry.path())
            .with_context(|| format!("failed to read {}", entry.path().display()))?;
        let document = parse_markdown(
            &markdown,
            MarkdownOptions {
                mermaid: config.markdown.mermaid,
                code_highlight: config.markdown.code_highlight,
                code_line_numbers: config.markdown.code_line_numbers,
                heading_anchors: config.markdown.heading_anchors,
                index_code: config.search.index_code,
            },
        )
        .with_context(|| format!("failed to parse {}", entry.path().display()))?;
        let metadata = page_metadata_for(src_dir, entry.path(), config)?;
        pages.push(Page {
            route: metadata.route,
            locale_key: metadata.locale_key,
            translation_key: metadata.translation_key,
            markdown_source: markdown,
            document,
        });
    }
    Ok(pages)
}

fn build_nav(pages: &[Page], config: &Config, page: &Page) -> Vec<NavItem> {
    if !sidebars_for_locale(config, &page.locale_key).is_empty() {
        return build_explicit_nav(config, &page.locale_key, &page.route);
    }

    build_legacy_nav(pages, config, &page.locale_key)
}

fn build_legacy_nav(pages: &[Page], config: &Config, locale_key: &str) -> Vec<NavItem> {
    let locale_prefix = home_for_locale(config, locale_key);
    let group_meta =
        sidebar_group_meta(nav_sections_for_locale(config, locale_key), &locale_prefix);
    let mut roots = Vec::new();
    let mut groups = Vec::<SidebarGroup>::new();

    for page in pages
        .iter()
        .filter(|page| page.locale_key == locale_key && page.document.frontmatter.sidebar)
    {
        let segments = route_segments(&page.translation_key);
        let leaf = NavItem {
            title: page.document.title.clone(),
            href: page.route.clone(),
            active_prefix: page.route.clone(),
            items: Vec::new(),
        };

        if segments.len() < 2 {
            roots.push(leaf);
            continue;
        }

        let segment = segments[0].to_string();
        let meta = group_meta.iter().find(|meta| meta.segment == segment);
        let group_index =
            if let Some(index) = groups.iter().position(|group| group.segment == segment) {
                index
            } else {
                groups.push(SidebarGroup {
                    segment: segment.clone(),
                    title: meta
                        .map(|meta| meta.title.clone())
                        .unwrap_or_else(|| titleize_segment(&segment)),
                    href: meta
                        .map(|meta| meta.href.clone())
                        .unwrap_or_else(|| page.route.clone()),
                    active_prefix: route_with_prefix(&locale_prefix, &format!("/{segment}/")),
                    order: meta.map(|meta| meta.order).unwrap_or(usize::MAX),
                    item_order: meta.map(|meta| meta.item_order.clone()).unwrap_or_default(),
                    items: Vec::new(),
                });
                groups.len() - 1
            };
        groups[group_index].items.push(leaf);
    }

    roots.sort_by(|a, b| {
        let a_home = a.href == locale_prefix;
        let b_home = b.href == locale_prefix;
        b_home.cmp(&a_home).then_with(|| a.href.cmp(&b.href))
    });
    groups.sort_by(|a, b| {
        a.order
            .cmp(&b.order)
            .then_with(|| a.title.cmp(&b.title))
            .then_with(|| a.href.cmp(&b.href))
    });
    for group in &mut groups {
        group.items.sort_by(|a, b| {
            nav_item_order(&group.item_order, &a.href)
                .cmp(&nav_item_order(&group.item_order, &b.href))
                .then_with(|| a.href.cmp(&b.href))
        });
    }

    roots.extend(groups.into_iter().map(|group| NavItem {
        title: group.title,
        href: group.href,
        active_prefix: group.active_prefix,
        items: group.items,
    }));
    roots
}

fn build_explicit_nav(config: &Config, locale_key: &str, route: &str) -> Vec<NavItem> {
    let nav = nav_sections_for_locale(config, locale_key);
    let sidebars = sidebars_for_locale(config, locale_key);
    let Some(sidebar_id) = active_sidebar_id(nav, sidebars, route) else {
        return Vec::new();
    };

    sidebars
        .get(sidebar_id)
        .map(|sections| sidebar_sections_to_nav_items(sections))
        .unwrap_or_default()
}

fn active_sidebar_id<'a>(
    nav: &'a [NavSection],
    sidebars: &'a BTreeMap<String, Vec<SidebarSection>>,
    route: &str,
) -> Option<&'a str> {
    nav.iter()
        .filter_map(|item| item.sidebar.as_deref().map(|sidebar| (item, sidebar)))
        .find_map(|(item, sidebar)| {
            let sections = sidebars.get(sidebar)?;
            if nav_section_matches_route(item, route)
                || sidebar_sections_match_route(sections, route)
            {
                Some(sidebar)
            } else {
                None
            }
        })
}

fn nav_section_matches_route(item: &NavSection, route: &str) -> bool {
    item.link
        .as_deref()
        .is_some_and(|href| route_matches_link(route, href))
        || item
            .items
            .iter()
            .any(|child| route_matches_link(route, &child.link))
}

fn sidebar_sections_match_route(items: &[SidebarSection], route: &str) -> bool {
    items.iter().any(|item| {
        route_matches_link(route, &item.link)
            || item
                .items
                .iter()
                .any(|child| route_matches_link(route, &child.link))
    })
}

fn sidebar_sections_to_nav_items(items: &[SidebarSection]) -> Vec<NavItem> {
    items
        .iter()
        .map(|item| NavItem {
            title: item.text.clone(),
            href: item.link.clone(),
            active_prefix: item.link.clone(),
            items: item
                .items
                .iter()
                .map(|child| NavItem {
                    title: child.text.clone(),
                    href: child.link.clone(),
                    active_prefix: child.link.clone(),
                    items: Vec::new(),
                })
                .collect(),
        })
        .collect()
}

fn route_matches_link(route: &str, href: &str) -> bool {
    href.starts_with('/') && (route == href || (href != "/" && route.starts_with(href)))
}

fn build_top_nav(config: &Config, locale_key: &str) -> Vec<TopNavItem> {
    nav_sections_for_locale(config, locale_key)
        .iter()
        .map(|item| TopNavItem {
            title: item.text.clone(),
            href: item.link.clone(),
            items: item
                .items
                .iter()
                .map(|child| TopNavLink {
                    title: child.text.clone(),
                    href: child.link.clone(),
                })
                .collect(),
        })
        .collect()
}

#[derive(Debug, Clone)]
struct SidebarGroup {
    segment: String,
    title: String,
    href: String,
    active_prefix: String,
    order: usize,
    item_order: Vec<String>,
    items: Vec<NavItem>,
}

#[derive(Debug, Clone)]
struct SidebarGroupMeta {
    segment: String,
    title: String,
    href: String,
    order: usize,
    item_order: Vec<String>,
}

fn nav_sections_for_locale<'a>(config: &'a Config, locale_key: &str) -> &'a [NavSection] {
    config
        .locales
        .get(locale_key)
        .filter(|locale| !locale.nav.is_empty())
        .map(|locale| locale.nav.as_slice())
        .unwrap_or(config.nav.as_slice())
}

fn sidebars_for_locale<'a>(
    config: &'a Config,
    locale_key: &str,
) -> &'a BTreeMap<String, Vec<SidebarSection>> {
    config
        .locales
        .get(locale_key)
        .filter(|locale| !locale.sidebars.is_empty())
        .map(|locale| &locale.sidebars)
        .unwrap_or(&config.sidebars)
}

fn sidebar_group_meta(nav: &[NavSection], locale_prefix: &str) -> Vec<SidebarGroupMeta> {
    let mut metas = Vec::new();
    for item in nav {
        let href = item
            .link
            .as_deref()
            .or_else(|| item.items.first().map(|child| child.link.as_str()));
        let Some(href) = href else { continue };
        let Some(segment) = first_route_segment(href, locale_prefix) else {
            continue;
        };
        if metas
            .iter()
            .any(|meta: &SidebarGroupMeta| meta.segment == segment)
        {
            continue;
        }
        metas.push(SidebarGroupMeta {
            segment,
            title: item.text.clone(),
            href: href.to_string(),
            order: metas.len(),
            item_order: item.items.iter().map(|child| child.link.clone()).collect(),
        });
    }
    metas
}

fn nav_item_order(order: &[String], href: &str) -> usize {
    order
        .iter()
        .position(|item| item == href)
        .unwrap_or(usize::MAX)
}

fn build_translation_map(pages: &[Page]) -> BTreeMap<(String, String), String> {
    pages
        .iter()
        .map(|page| {
            (
                (page.locale_key.clone(), page.translation_key.clone()),
                page.route.clone(),
            )
        })
        .collect()
}

fn base_site_render(config: &Config) -> SiteRender {
    SiteRender {
        title: config.title.clone(),
        lang: default_lang(config),
        base: config.base.clone(),
        home_href: "/".to_string(),
        theme: theme_config(config),
        search_enabled: config.search.enabled,
        access_enabled: access_mask_enabled(config),
        access_password: config.access.password.clone(),
        password_hint: config.access.password_hint.clone(),
        top_nav: build_top_nav(config, "root"),
        nav: Vec::new(),
        languages: Vec::new(),
    }
}

fn site_render_for_page(
    config: &Config,
    pages: &[Page],
    translations: &BTreeMap<(String, String), String>,
    page: &Page,
) -> SiteRender {
    SiteRender {
        title: title_for_locale(config, &page.locale_key),
        lang: lang_for_locale(config, &page.locale_key),
        base: config.base.clone(),
        home_href: home_for_locale(config, &page.locale_key),
        theme: theme_config(config),
        search_enabled: config.search.enabled,
        access_enabled: access_mask_enabled(config),
        access_password: config.access.password.clone(),
        password_hint: config.access.password_hint.clone(),
        top_nav: build_top_nav(config, &page.locale_key),
        nav: build_nav(pages, config, page),
        languages: build_language_options(config, page, translations),
    }
}

fn access_mask_enabled(config: &Config) -> bool {
    config.access.enabled && config.access.mode == "mask" && !config.access.password.is_empty()
}

fn theme_config(config: &Config) -> ThemeConfig {
    ThemeConfig {
        skin: config.theme.skin.clone(),
        allow_switch: config.theme.allow_switch,
        github_url: config.theme.github_url.clone(),
    }
}

fn title_for_locale(config: &Config, locale_key: &str) -> String {
    config
        .locales
        .get(locale_key)
        .and_then(|locale| locale.title.as_ref())
        .cloned()
        .unwrap_or_else(|| config.title.clone())
}

fn default_lang(config: &Config) -> String {
    if config.locales.is_empty() {
        "en".to_string()
    } else {
        lang_for_locale(config, "root")
    }
}

fn lang_for_locale(config: &Config, locale_key: &str) -> String {
    config
        .locales
        .get(locale_key)
        .map(|locale| locale.lang.clone())
        .unwrap_or_else(|| "en".to_string())
}

fn home_for_locale(config: &Config, locale_key: &str) -> String {
    config
        .locales
        .get(locale_key)
        .map(|locale| locale.link.clone())
        .unwrap_or_else(|| "/".to_string())
}

fn build_language_options(
    config: &Config,
    page: &Page,
    translations: &BTreeMap<(String, String), String>,
) -> Vec<LanguageOption> {
    if config.locales.is_empty() {
        return Vec::new();
    }

    locale_keys(config)
        .into_iter()
        .filter_map(|locale_key| {
            let locale = config.locales.get(&locale_key)?;
            let href = translations
                .get(&(locale_key.clone(), page.translation_key.clone()))
                .cloned()
                .unwrap_or_else(|| locale.link.clone());
            Some(LanguageOption {
                label: locale.label.clone(),
                href,
                current: locale_key == page.locale_key,
            })
        })
        .collect()
}

fn locale_keys(config: &Config) -> Vec<String> {
    let mut keys = Vec::new();
    if config.locales.contains_key("root") {
        keys.push("root".to_string());
    }
    keys.extend(config.locales.keys().filter(|key| *key != "root").cloned());
    keys
}

fn write_search_index(out_dir: &Path, config: &SearchSection, pages: &[SearchPage]) -> Result<()> {
    let assets_dir = out_dir.join("assets");
    fs::create_dir_all(&assets_dir)
        .with_context(|| format!("failed to create {}", assets_dir.display()))?;
    let index = build_search_index(
        SearchConfig {
            languages: config.languages.clone(),
        },
        pages,
    );
    let json = serde_json::to_vec_pretty(&index)?;
    fs::write(assets_dir.join("search-index.json"), &json)?;

    let mut compressed = Vec::new();
    {
        let mut writer = brotli::CompressorWriter::new(&mut compressed, 4096, 5, 22);
        writer.write_all(&json)?;
    }
    fs::write(assets_dir.join("search-index.json.br"), compressed)?;
    fs::write(
        assets_dir.join("rustpress_search_bg.wasm"),
        rustpress_search::wasm_placeholder(),
    )?;
    Ok(())
}

fn copy_public_assets(public_dir: &Path, out_dir: &Path) -> Result<()> {
    if !public_dir.exists() {
        return Ok(());
    }

    for entry in WalkDir::new(public_dir) {
        let entry = entry.with_context(|| format!("failed to scan {}", public_dir.display()))?;
        if !entry.file_type().is_file() {
            continue;
        }
        let relative = entry.path().strip_prefix(public_dir)?;
        if relative.file_name().and_then(|value| value.to_str()) == Some(".gitkeep") {
            continue;
        }
        let target = out_dir.join(relative);
        if let Some(parent) = target.parent() {
            fs::create_dir_all(parent)
                .with_context(|| format!("failed to create {}", parent.display()))?;
        }
        fs::copy(entry.path(), &target).with_context(|| {
            format!(
                "failed to copy {} to {}",
                entry.path().display(),
                target.display()
            )
        })?;
    }
    Ok(())
}

fn write_page(out_dir: &Path, route: &str, html: &str) -> Result<()> {
    let target = page_html_target(out_dir, route);
    if let Some(parent) = target.parent() {
        fs::create_dir_all(parent)
            .with_context(|| format!("failed to create {}", parent.display()))?;
    }
    fs::write(&target, html).with_context(|| format!("failed to write {}", target.display()))
}

fn write_markdown_source(out_dir: &Path, route: &str, markdown_source: &str) -> Result<()> {
    let target = page_markdown_target(out_dir, route);
    if let Some(parent) = target.parent() {
        fs::create_dir_all(parent)
            .with_context(|| format!("failed to create {}", parent.display()))?;
    }
    fs::write(&target, markdown_source)
        .with_context(|| format!("failed to write {}", target.display()))
}

fn page_html_target(out_dir: &Path, route: &str) -> PathBuf {
    let path = out_dir.join(route.trim_start_matches('/'));
    if route.ends_with('/') {
        path.join("index.html")
    } else {
        path
    }
}

fn page_markdown_target(out_dir: &Path, route: &str) -> PathBuf {
    let mut target = page_html_target(out_dir, route);
    target.set_file_name("index.md.txt");
    target
}

fn page_metadata_for(src_dir: &Path, path: &Path, config: &Config) -> Result<PageMetadata> {
    if config.locales.is_empty() {
        let route = route_for(src_dir, path)?;
        return Ok(PageMetadata {
            route: route.clone(),
            locale_key: "root".to_string(),
            translation_key: route,
        });
    }

    let relative = path.strip_prefix(src_dir)?;
    if let Some((locale_key, locale_relative)) = locale_relative_path(relative, config) {
        let translation_key = route_for_relative(&locale_relative);
        let route = route_with_prefix(&config.locales[&locale_key].link, &translation_key);
        return Ok(PageMetadata {
            route,
            locale_key,
            translation_key,
        });
    }

    let route = route_for_relative(relative);
    Ok(PageMetadata {
        route: route.clone(),
        locale_key: "root".to_string(),
        translation_key: route,
    })
}

fn locale_relative_path(relative: &Path, config: &Config) -> Option<(String, PathBuf)> {
    let mut components = relative.components();
    let first = components.next()?.as_os_str().to_str()?;
    if first == "root" || !config.locales.contains_key(first) {
        return None;
    }

    let mut locale_relative = PathBuf::new();
    for component in components {
        locale_relative.push(component.as_os_str());
    }
    Some((first.to_string(), locale_relative))
}

fn route_for(src_dir: &Path, path: &Path) -> Result<String> {
    let relative = path.strip_prefix(src_dir)?;
    Ok(route_for_relative(relative))
}

fn route_for_relative(relative: &Path) -> String {
    let without_ext = relative.with_extension("");
    if without_ext == Path::new("index") {
        return "/".to_string();
    }

    if without_ext.file_name().and_then(|value| value.to_str()) == Some("index") {
        without_ext
            .parent()
            .map(path_to_route)
            .unwrap_or_else(|| "/".to_string())
    } else {
        path_to_route(&without_ext)
    }
}

fn path_to_route(path: &Path) -> String {
    if path.as_os_str().is_empty() {
        return "/".to_string();
    }

    let route = path
        .components()
        .map(|component| component.as_os_str().to_string_lossy())
        .collect::<Vec<_>>()
        .join("/");
    format!("/{route}/")
}

fn route_with_prefix(prefix: &str, route: &str) -> String {
    if route == "/" {
        return prefix.to_string();
    }
    if prefix == "/" {
        route.to_string()
    } else {
        format!("{}{}", prefix, route.trim_start_matches('/'))
    }
}

fn route_segments(route: &str) -> Vec<&str> {
    route
        .trim_matches('/')
        .split('/')
        .filter(|segment| !segment.is_empty())
        .collect()
}

fn first_route_segment(route: &str, locale_prefix: &str) -> Option<String> {
    if route.starts_with("http://")
        || route.starts_with("https://")
        || route.starts_with("mailto:")
        || route.starts_with('#')
    {
        return None;
    }

    let local_route = if locale_prefix != "/" && route.starts_with(locale_prefix) {
        let rest = &route[locale_prefix.len()..];
        if rest.is_empty() {
            "/".to_string()
        } else {
            format!("/{rest}")
        }
    } else {
        route.to_string()
    };
    route_segments(&local_route)
        .first()
        .map(|segment| (*segment).to_string())
}

fn titleize_segment(segment: &str) -> String {
    segment
        .split(['-', '_'])
        .filter(|part| !part.is_empty())
        .map(|part| {
            let mut chars = part.chars();
            match chars.next() {
                Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
                None => String::new(),
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}

fn site_url(base: &str, route: &str) -> String {
    if route == "/" {
        base.to_string()
    } else {
        format!("{}{}", base, route.trim_start_matches('/'))
    }
}

fn markdown_source_url(base: &str, route: &str) -> String {
    format!("{}index.md.txt", site_url(base, route))
}

fn normalize_nav(nav: &mut Vec<NavSection>, locale_prefix: Option<&str>) {
    nav.retain(|item| !item.text.trim().is_empty());
    for item in nav {
        item.text = item.text.trim().to_string();
        item.sidebar = item
            .sidebar
            .take()
            .map(|sidebar| sidebar.trim().to_string())
            .filter(|sidebar| !sidebar.is_empty());
        if item
            .link
            .as_deref()
            .is_some_and(|link| link.trim().is_empty())
        {
            item.link = None;
        }
        item.items
            .retain(|child| !child.text.trim().is_empty() && !child.link.trim().is_empty());
        for child in &mut item.items {
            child.text = child.text.trim().to_string();
            child.link = normalize_nav_link(&child.link, locale_prefix);
        }
        if let Some(link) = &mut item.link {
            *link = normalize_nav_link(link, locale_prefix);
        }
    }
}

fn normalize_sidebars(
    sidebars: &mut BTreeMap<String, Vec<SidebarSection>>,
    locale_prefix: Option<&str>,
) {
    sidebars.retain(|id, items| {
        if id.trim().is_empty() {
            return false;
        }

        for item in items.iter_mut() {
            item.text = item.text.trim().to_string();
            item.items
                .retain(|child| !child.text.trim().is_empty() && !child.link.trim().is_empty());
            for child in &mut item.items {
                child.text = child.text.trim().to_string();
                child.link = normalize_nav_link(&child.link, locale_prefix);
            }

            item.link = item.link.trim().to_string();
            if item.link.is_empty() {
                if let Some(first_child) = item.items.first() {
                    item.link = first_child.link.clone();
                }
            } else {
                item.link = normalize_nav_link(&item.link, locale_prefix);
            }
        }

        items.retain(|item| !item.text.is_empty() && !item.link.is_empty());

        !items.is_empty()
    });
}

fn normalize_nav_link(link: &str, locale_prefix: Option<&str>) -> String {
    match locale_prefix {
        Some(prefix) => normalize_locale_nav_link(link, prefix),
        None => normalize_link(link),
    }
}

fn normalize_locale_nav_link(link: &str, locale_prefix: &str) -> String {
    let link = link.trim();
    if link.is_empty()
        || link.starts_with('/')
        || link.starts_with('#')
        || link.starts_with("http://")
        || link.starts_with("https://")
        || link.starts_with("mailto:")
    {
        link.to_string()
    } else {
        route_with_prefix(locale_prefix, &normalize_link(link))
    }
}

fn normalize_locale_prefix(key: &str, link: &str) -> Result<String> {
    let mut link = if link.trim().is_empty() {
        format!("/{key}/")
    } else {
        normalize_link(link)
    };
    if !link.starts_with('/') {
        anyhow::bail!("locale `{key}` link must be a path");
    }
    if link != "/" && !link.ends_with('/') {
        link.push('/');
    }
    Ok(link)
}

fn normalize_theme_skin(skin: &str) -> String {
    match skin.trim().to_ascii_lowercase().as_str() {
        "dark" => "dark".to_string(),
        _ => "light".to_string(),
    }
}

fn normalize_link(link: &str) -> String {
    let link = link.trim();
    if link.is_empty()
        || link.starts_with('/')
        || link.starts_with('#')
        || link.starts_with("http://")
        || link.starts_with("https://")
        || link.starts_with("mailto:")
    {
        link.to_string()
    } else {
        format!("/{link}")
    }
}

fn normalize_config_path(path: &Path) -> Result<PathBuf> {
    if path.exists() {
        return Ok(path.to_path_buf());
    }
    anyhow::bail!("config file does not exist: {}", path.display());
}

fn absolutize(root: &Path, path: &Path) -> PathBuf {
    if path.is_absolute() {
        path.to_path_buf()
    } else {
        root.join(path)
    }
}

fn write_new(path: &Path, contents: &str) -> Result<()> {
    if path.exists() {
        anyhow::bail!("refusing to overwrite existing file {}", path.display());
    }
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)
            .with_context(|| format!("failed to create {}", parent.display()))?;
    }
    fs::write(path, contents).with_context(|| format!("failed to write {}", path.display()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn init_and_build_generates_index() {
        let dir = tempfile::tempdir().unwrap();
        init_project(dir.path()).unwrap();

        let result = build_site(BuildOptions::new(dir.path().join("rustpress.toml"))).unwrap();

        assert_eq!(result.page_count, 2);
        assert!(dir.path().join("dist/index.html").exists());
        assert!(dir.path().join("dist/index.md.txt").exists());
        assert!(dir.path().join("dist/private/index.html").exists());
        assert!(dir.path().join("dist/private/index.md.txt").exists());
        assert!(dir.path().join("dist/assets/search-index.json").exists());
        assert!(dir.path().join("dist/assets/search-index.json.br").exists());
        assert!(dir
            .path()
            .join("dist/assets/rustpress_search_bg.wasm")
            .exists());

        let public_html = fs::read_to_string(dir.path().join("dist/index.html")).unwrap();
        let masked_html = fs::read_to_string(dir.path().join("dist/private/index.html")).unwrap();
        let public_markdown = fs::read_to_string(dir.path().join("dist/index.md.txt")).unwrap();
        let source_markdown = fs::read_to_string(dir.path().join("docs/index.md")).unwrap();
        let theme_js = fs::read_to_string(dir.path().join("dist/assets/rustpress.js")).unwrap();
        assert!(public_html.contains("rp-topnav-group"));
        assert!(public_html.contains("Masked Page"));
        assert_eq!(public_markdown, source_markdown);
        assert!(!public_html.contains("data-rp-language-select"));
        assert!(!public_html.contains("data-rp-access-mask"));
        assert!(masked_html.contains("data-rp-access-mask"));
        assert!(theme_js.contains(r#"const accessPassword = "rustpress";"#));
    }

    #[test]
    fn markdown_code_line_numbers_default_to_true() {
        let raw = r#"
title = "Docs"
src_dir = "docs"
out_dir = "dist"
base = "/"

[markdown]
mermaid = true
"#;
        let config: Config = toml::from_str(raw).unwrap();

        assert!(config.markdown.code_line_numbers);
    }

    #[test]
    fn markdown_code_line_numbers_false_reaches_rendered_pages() {
        let dir = tempfile::tempdir().unwrap();
        fs::create_dir_all(dir.path().join("docs")).unwrap();
        fs::write(
            dir.path().join("rustpress.toml"),
            r#"title = "Docs"
src_dir = "docs"
out_dir = "dist"
base = "/"

[markdown]
code_highlight = false
code_line_numbers = false
"#,
        )
        .unwrap();
        fs::write(
            dir.path().join("docs/index.md"),
            "# Home\n\n```rust\nfn main() {}\n```",
        )
        .unwrap();

        build_site(BuildOptions::new(dir.path().join("rustpress.toml"))).unwrap();

        let html = fs::read_to_string(dir.path().join("dist/index.html")).unwrap();
        assert!(html.contains("class=\"rp-code-content language-rust\""));
        assert!(!html.contains("rp-code-line-numbers"));
        assert!(!html.contains("rp-code-lines"));
    }

    #[test]
    fn access_mask_requires_configured_password() {
        let mut config = Config::default();
        config.access.enabled = true;
        config.access.mode = "mask".to_string();
        config.access.password.clear();
        assert!(!access_mask_enabled(&config));

        config.access.password = "secret".to_string();
        assert!(access_mask_enabled(&config));

        config.access.enabled = false;
        assert!(!access_mask_enabled(&config));
    }

    #[test]
    fn base_url_is_normalized() {
        let mut config = Config {
            base: "docs".to_string(),
            ..Config::default()
        };
        config.normalize().unwrap();
        assert_eq!(config.base, "/docs/");
    }

    #[test]
    fn markdown_source_urls_use_page_directory() {
        assert_eq!(markdown_source_url("/", "/"), "/index.md.txt");
        assert_eq!(
            markdown_source_url("/docs/", "/guide/cli/"),
            "/docs/guide/cli/index.md.txt"
        );
    }

    #[test]
    fn theme_skin_is_limited_to_light_and_dark() {
        let mut dark_config = Config {
            theme: ThemeSection {
                skin: "dark".to_string(),
                ..ThemeSection::default()
            },
            ..Config::default()
        };
        dark_config.normalize().unwrap();
        assert_eq!(dark_config.theme.skin, "dark");

        let mut old_skin_config = Config {
            theme: ThemeSection {
                skin: "modern".to_string(),
                ..ThemeSection::default()
            },
            ..Config::default()
        };
        old_skin_config.normalize().unwrap();
        assert_eq!(old_skin_config.theme.skin, "light");
    }

    #[test]
    fn theme_github_url_is_rendered_when_configured() {
        let raw = r#"
title = "Docs"
src_dir = "docs"
out_dir = "dist"
base = "/"

[theme]
github_url = " https://github.com/example/docs "
"#;
        let mut config: Config = toml::from_str(raw).unwrap();
        config.normalize().unwrap();

        assert_eq!(config.theme.github_url, "https://github.com/example/docs");

        let site = base_site_render(&config);
        let html = render_page(
            &site,
            &PageRender {
                title: "Home".to_string(),
                route: "/".to_string(),
                html: "<h1>Home</h1>".to_string(),
                markdown_source: "---\ntitle: Home\n---\n# Home\n".to_string(),
                markdown_source_url: "/index.md.txt".to_string(),
                headings: vec![],
                masked: false,
                search: true,
            },
        );

        assert!(html.contains("rp-github-link"));
        assert!(html.contains(r#"href="https://github.com/example/docs""#));
    }

    #[test]
    fn nav_links_are_normalized() {
        let mut config = Config {
            nav: vec![NavSection {
                text: " Guide ".to_string(),
                link: Some("guide/cli/".to_string()),
                sidebar: None,
                items: vec![
                    NavLinkSection {
                        text: " CLI ".to_string(),
                        link: "guide/cli/".to_string(),
                    },
                    NavLinkSection {
                        text: String::new(),
                        link: "/bad/".to_string(),
                    },
                ],
            }],
            ..Config::default()
        };

        config.normalize().unwrap();

        assert_eq!(config.nav[0].text, "Guide");
        assert_eq!(config.nav[0].link.as_deref(), Some("/guide/cli/"));
        assert_eq!(config.nav[0].items.len(), 1);
        assert_eq!(config.nav[0].items[0].text, "CLI");
        assert_eq!(config.nav[0].items[0].link, "/guide/cli/");
    }

    #[test]
    fn sidebar_links_are_normalized() {
        let mut sidebars = BTreeMap::new();
        sidebars.insert(
            "docs".to_string(),
            vec![SidebarSection {
                text: " Guide ".to_string(),
                link: String::new(),
                items: vec![
                    SidebarLinkSection {
                        text: " CLI ".to_string(),
                        link: "guide/cli/".to_string(),
                    },
                    SidebarLinkSection {
                        text: String::new(),
                        link: "/bad/".to_string(),
                    },
                ],
            }],
        );
        let mut config = Config {
            nav: vec![NavSection {
                text: "Guide".to_string(),
                link: Some("/guide/".to_string()),
                sidebar: Some(" docs ".to_string()),
                items: Vec::new(),
            }],
            sidebars,
            ..Config::default()
        };

        config.normalize().unwrap();

        assert_eq!(config.nav[0].sidebar.as_deref(), Some("docs"));
        let sidebar = &config.sidebars["docs"][0];
        assert_eq!(sidebar.text, "Guide");
        assert_eq!(sidebar.link, "/guide/cli/");
        assert_eq!(sidebar.items.len(), 1);
        assert_eq!(sidebar.items[0].text, "CLI");
        assert_eq!(sidebar.items[0].link, "/guide/cli/");
    }

    #[test]
    fn locales_require_root() {
        let mut config = Config::default();
        config.locales.insert(
            "en".to_string(),
            LocaleSection {
                label: "English".to_string(),
                lang: "en-US".to_string(),
                ..LocaleSection::default()
            },
        );

        let err = config.normalize().unwrap_err();

        assert!(err.to_string().contains("locales.root"));
    }

    #[test]
    fn locale_links_and_relative_nav_are_normalized() {
        let mut config = localized_config();
        let locale = config.locales.get_mut("en").unwrap();
        locale.nav[0].sidebar = Some(" guide ".to_string());
        locale.sidebars.insert(
            "guide".to_string(),
            vec![SidebarSection {
                text: " Guide ".to_string(),
                link: "guide/cli/".to_string(),
                items: vec![SidebarLinkSection {
                    text: "CLI".to_string(),
                    link: "guide/cli/".to_string(),
                }],
            }],
        );

        config.normalize().unwrap();

        let root = &config.locales["root"];
        let en = &config.locales["en"];
        assert_eq!(root.label, "简体中文");
        assert_eq!(root.lang, "zh-CN");
        assert_eq!(root.link, "/");
        assert_eq!(en.link, "/en/");
        assert_eq!(en.nav[0].link.as_deref(), Some("/en/guide/cli/"));
        assert_eq!(en.nav[0].sidebar.as_deref(), Some("guide"));
        assert_eq!(en.nav[0].items[0].link, "/en/guide/cli/");
        assert_eq!(en.sidebars["guide"][0].link, "/en/guide/cli/");
        assert_eq!(en.sidebars["guide"][0].items[0].link, "/en/guide/cli/");
    }

    #[test]
    fn explicit_sidebars_are_selected_by_top_nav_section() {
        let dir = tempfile::tempdir().unwrap();
        fs::create_dir_all(dir.path().join("docs")).unwrap();
        fs::write(
            dir.path().join("rustpress.toml"),
            r#"title = "Docs"
src_dir = "docs"
out_dir = "dist"
base = "/"

[[nav]]
text = "Guide"
link = "/guide/"
sidebar = "guide"

[[nav]]
text = "Reference"
link = "/reference/"
sidebar = "reference"

[[sidebars.guide]]
text = "Guide"
link = "/guide/"

[[sidebars.guide.items]]
text = "CLI"
link = "/guide/cli/"

[[sidebars.reference]]
text = "Reference"
link = "/reference/"

[[sidebars.reference.items]]
text = "API"
link = "/reference/api/"
"#,
        )
        .unwrap();
        write_doc(dir.path(), "docs/guide/cli.md", "Guide CLI", "Guide CLI").unwrap();
        write_doc(
            dir.path(),
            "docs/reference/api.md",
            "Reference API",
            "Reference API",
        )
        .unwrap();

        build_site(BuildOptions::new(dir.path().join("rustpress.toml"))).unwrap();

        let guide_html = fs::read_to_string(dir.path().join("dist/guide/cli/index.html")).unwrap();
        let reference_html =
            fs::read_to_string(dir.path().join("dist/reference/api/index.html")).unwrap();
        assert!(guide_html.contains("CLI"));
        assert!(!guide_html.contains("rp-nav-level-1\">API"));
        assert!(reference_html.contains("API"));
        assert!(!reference_html.contains("rp-nav-level-1\">CLI"));
    }

    #[test]
    fn routes_markdown_pages_without_double_slashes() {
        let src = Path::new("/site/docs");

        assert_eq!(
            route_for(src, Path::new("/site/docs/guide.md")).unwrap(),
            "/guide/"
        );
        assert_eq!(
            route_for(src, Path::new("/site/docs/guide/index.md")).unwrap(),
            "/guide/"
        );
        assert_eq!(
            route_for(src, Path::new("/site/docs/index.md")).unwrap(),
            "/"
        );
    }

    #[test]
    fn localized_routes_use_locale_prefixes() {
        let mut config = localized_config();
        config.normalize().unwrap();
        let src = Path::new("/site/docs");

        assert_eq!(
            page_metadata_for(src, Path::new("/site/docs/index.md"), &config)
                .unwrap()
                .route,
            "/"
        );
        assert_eq!(
            page_metadata_for(src, Path::new("/site/docs/guide.md"), &config)
                .unwrap()
                .route,
            "/guide/"
        );
        let en_home = page_metadata_for(src, Path::new("/site/docs/en/index.md"), &config).unwrap();
        assert_eq!(en_home.route, "/en/");
        assert_eq!(en_home.locale_key, "en");
        assert_eq!(en_home.translation_key, "/");
        let en_guide =
            page_metadata_for(src, Path::new("/site/docs/en/guide.md"), &config).unwrap();
        assert_eq!(en_guide.route, "/en/guide/");
        assert_eq!(en_guide.locale_key, "en");
        assert_eq!(en_guide.translation_key, "/guide/");
    }

    #[test]
    fn builds_multilingual_pages_and_language_switcher() {
        let dir = tempfile::tempdir().unwrap();
        write_multilingual_config(dir.path()).unwrap();
        write_doc(dir.path(), "docs/index.md", "Root Home", "Root Home").unwrap();
        write_doc(dir.path(), "docs/guide.md", "Root Guide", "Root Guide").unwrap();
        write_doc(dir.path(), "docs/guide/cli.md", "Root CLI", "Root CLI").unwrap();
        write_doc(dir.path(), "docs/root-only.md", "Root Only", "Root Only").unwrap();
        write_doc(
            dir.path(),
            "docs/en/index.md",
            "English Home",
            "English Home",
        )
        .unwrap();
        write_doc(
            dir.path(),
            "docs/en/guide.md",
            "English Guide",
            "English Guide",
        )
        .unwrap();
        write_doc(
            dir.path(),
            "docs/en/guide/cli.md",
            "English CLI",
            "English CLI",
        )
        .unwrap();

        let result = build_site(BuildOptions::new(dir.path().join("rustpress.toml"))).unwrap();

        assert_eq!(result.page_count, 7);
        assert!(dir.path().join("dist/index.html").exists());
        assert!(dir.path().join("dist/en/index.html").exists());
        assert!(dir.path().join("dist/en/guide/index.html").exists());
        assert!(dir.path().join("dist/en/guide/cli/index.html").exists());

        let root_guide = fs::read_to_string(dir.path().join("dist/guide/index.html")).unwrap();
        let en_guide = fs::read_to_string(dir.path().join("dist/en/guide/index.html")).unwrap();
        let root_cli = fs::read_to_string(dir.path().join("dist/guide/cli/index.html")).unwrap();
        assert!(root_guide.contains(r#"<html lang="zh-CN""#));
        assert!(root_guide.contains("data-rp-language-select"));
        assert!(root_guide.contains(r#"data-rp-language-href="/guide/">简体中文</button>"#));
        assert!(root_guide.contains(r#"data-rp-language-href="/en/guide/">English</button>"#));
        assert!(en_guide.contains(r#"<html lang="en-US""#));
        assert!(en_guide.contains(r#"data-rp-language-href="/guide/">简体中文</button>"#));
        assert!(en_guide.contains(r#"data-rp-language-href="/en/guide/">English</button>"#));
        assert!(root_cli.contains("rp-nav-group"));
        assert!(root_cli.contains("rp-nav-group-title"));
        assert!(root_cli.contains("Root Guide"));
        assert!(root_cli.contains("Root CLI"));
        assert!(!en_guide.contains("Root Only"));
    }

    #[test]
    fn language_switcher_falls_back_to_locale_home_when_translation_is_missing() {
        let dir = tempfile::tempdir().unwrap();
        write_multilingual_config(dir.path()).unwrap();
        write_doc(dir.path(), "docs/index.md", "Root Home", "Root Home").unwrap();
        write_doc(dir.path(), "docs/guide.md", "Root Guide", "Root Guide").unwrap();
        write_doc(
            dir.path(),
            "docs/en/index.md",
            "English Home",
            "English Home",
        )
        .unwrap();

        build_site(BuildOptions::new(dir.path().join("rustpress.toml"))).unwrap();

        let root_guide = fs::read_to_string(dir.path().join("dist/guide/index.html")).unwrap();
        assert!(root_guide.contains(r#"data-rp-language-href="/en/">English</button>"#));
    }

    #[test]
    fn search_false_pages_are_excluded_from_index() {
        let dir = tempfile::tempdir().unwrap();
        init_project(dir.path()).unwrap();
        fs::write(
            dir.path().join("docs/hidden.md"),
            "---\ntitle: Hidden\nsearch: false\n---\n# Hidden\nUniqueSecret",
        )
        .unwrap();

        build_site(BuildOptions::new(dir.path().join("rustpress.toml"))).unwrap();

        let index = fs::read_to_string(dir.path().join("dist/assets/search-index.json")).unwrap();
        assert!(!index.contains("UniqueSecret"));
        assert!(!index.contains("\"Hidden\""));
    }

    #[test]
    fn rendered_pages_include_copyable_markdown_source() {
        let dir = tempfile::tempdir().unwrap();
        init_project(dir.path()).unwrap();
        fs::write(
            dir.path().join("docs/agent.md"),
            "---\ntitle: Agent Copy\naccess: public\n---\n# Agent Copy\n\nUse <agent> context.\n",
        )
        .unwrap();

        build_site(BuildOptions::new(dir.path().join("rustpress.toml"))).unwrap();

        let html = fs::read_to_string(dir.path().join("dist/agent/index.html")).unwrap();
        let markdown = fs::read_to_string(dir.path().join("dist/agent/index.md.txt")).unwrap();
        let source = fs::read_to_string(dir.path().join("docs/agent.md")).unwrap();
        assert!(html.contains("data-rp-copy-markdown"));
        assert!(html.contains("data-rp-copy-markdown-url"));
        assert!(html.contains("data-rp-markdown-source"));
        assert!(html.contains(r#"data-rp-markdown-source-url="/agent/index.md.txt""#));
        assert!(html.contains("---\ntitle: Agent Copy\naccess: public\n---"));
        assert!(html.contains("Use &lt;agent&gt; context."));
        assert_eq!(markdown, source);
    }

    fn localized_config() -> Config {
        let mut locales = BTreeMap::new();
        locales.insert(
            "root".to_string(),
            LocaleSection {
                label: " 简体中文 ".to_string(),
                lang: " zh-CN ".to_string(),
                ..LocaleSection::default()
            },
        );
        locales.insert(
            "en".to_string(),
            LocaleSection {
                label: "English".to_string(),
                lang: "en-US".to_string(),
                nav: vec![NavSection {
                    text: "Guide".to_string(),
                    link: Some("guide/cli/".to_string()),
                    sidebar: None,
                    items: vec![NavLinkSection {
                        text: "CLI".to_string(),
                        link: "guide/cli/".to_string(),
                    }],
                }],
                ..LocaleSection::default()
            },
        );
        Config {
            locales,
            ..Config::default()
        }
    }

    fn write_multilingual_config(root: &Path) -> Result<()> {
        fs::write(
            root.join("rustpress.toml"),
            r#"title = "Docs"
src_dir = "docs"
out_dir = "dist"
base = "/"

[[nav]]
text = "Root Guide"
link = "/guide/"

[locales.root]
label = "简体中文"
lang = "zh-CN"
title = "中文文档"

[locales.en]
label = "English"
lang = "en-US"
link = "/en/"
title = "English Docs"

[[locales.en.nav]]
text = "Guide"
link = "guide/"
"#,
        )?;
        Ok(())
    }

    fn write_doc(root: &Path, relative: &str, title: &str, body: &str) -> Result<()> {
        let path = root.join(relative);
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::write(
            path,
            format!(
                "---\ntitle: {title}\nlayout: doc\nsidebar: true\nsearch: true\naccess: public\n---\n\n# {body}\n"
            ),
        )?;
        Ok(())
    }
}
