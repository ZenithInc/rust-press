use std::fs;
use std::path::Path;

use anyhow::{Context, Result};
use rustpress_md::Heading;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ThemeConfig {
    pub skin: String,
    pub allow_switch: bool,
    pub github_url: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SiteRender {
    pub title: String,
    pub lang: String,
    pub base: String,
    pub home_href: String,
    pub theme: ThemeConfig,
    pub search_enabled: bool,
    pub access_enabled: bool,
    pub access_password: String,
    pub password_hint: String,
    pub top_nav: Vec<TopNavItem>,
    pub nav: Vec<NavItem>,
    pub languages: Vec<LanguageOption>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TopNavItem {
    pub title: String,
    pub href: Option<String>,
    pub items: Vec<TopNavLink>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TopNavLink {
    pub title: String,
    pub href: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NavItem {
    pub title: String,
    pub href: String,
    pub active_prefix: String,
    pub items: Vec<NavItem>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LanguageOption {
    pub label: String,
    pub href: String,
    pub current: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PageRender {
    pub title: String,
    pub route: String,
    pub html: String,
    pub markdown_source: String,
    pub markdown_source_url: String,
    pub headings: Vec<Heading>,
    pub masked: bool,
    pub search: bool,
}

pub fn write_theme_assets(out_dir: &Path, site: &SiteRender) -> Result<()> {
    let assets = out_dir.join("assets");
    fs::create_dir_all(&assets)
        .with_context(|| format!("failed to create {}", assets.display()))?;
    fs::write(assets.join("rustpress.css"), css())?;
    fs::write(assets.join("rustpress.js"), js(site))?;
    Ok(())
}

pub fn render_page(site: &SiteRender, page: &PageRender) -> String {
    let title = if page.title == site.title {
        site.title.clone()
    } else {
        format!("{} | {}", page.title, site.title)
    };
    let base = site.base.trim_end_matches('/');
    let asset_base = if base.is_empty() { "" } else { base };
    let search_markup = if site.search_enabled {
        r#"<button class="rp-icon-button" data-rp-search-open aria-label="Open search" title="Search">
<svg viewBox="0 0 24 24" aria-hidden="true"><circle cx="11" cy="11" r="7"></circle><path d="m20 20-3.5-3.5"></path></svg>
</button>"#
    } else {
        ""
    };
    let skin_switcher = if site.theme.allow_switch {
        render_skin_switcher(site)
    } else {
        String::new()
    };
    let github_link = render_github_link(site);
    let language_switcher = render_language_switcher(site);
    let access_mask = if site.access_enabled && page.masked {
        render_access_mask(site)
    } else {
        String::new()
    };
    let markdown_copy = render_markdown_copy(page);

    format!(
        r#"<!doctype html>
<html lang="{lang}" data-rp-skin="{skin}">
<head>
<meta charset="utf-8">
<meta name="viewport" content="width=device-width, initial-scale=1">
<title>{title}</title>
<link rel="stylesheet" href="{asset_base}/assets/rustpress.css">
</head>
<body data-rp-route="{route}" data-rp-masked="{masked}">
<header class="rp-topbar">
  <a class="rp-brand" href="{base_href}">{site_title}</a>
  {top_nav}
  <div class="rp-topbar-actions">
    {search_markup}
    {language_switcher}
    {skin_switcher}
    <button class="rp-icon-button rp-menu-button" data-rp-menu aria-label="Toggle navigation" title="Navigation">
      <svg viewBox="0 0 24 24" aria-hidden="true"><path d="M4 7h16M4 12h16M4 17h16"></path></svg>
    </button>
    {github_link}
  </div>
</header>
<div class="rp-shell">
  <aside class="rp-sidebar" data-rp-sidebar>
    <nav aria-label="Main navigation">
      {nav}
    </nav>
  </aside>
  <main class="rp-main" data-rp-content>
    <article class="rp-doc">
      {content}
    </article>
    {toc}
  </main>
</div>
{markdown_copy}
{search_dialog}
{access_mask}
<script type="module" src="{asset_base}/assets/rustpress.js"></script>
{mermaid_script}
</body>
</html>
"#,
        lang = escape_attr(&site.lang),
        skin = escape_attr(&site.theme.skin),
        title = escape_html(&title),
        asset_base = asset_base,
        route = escape_attr(&page.route),
        masked = page.masked,
        base_href = escape_attr(&href_for(site, &site.home_href)),
        site_title = escape_html(&site.title),
        top_nav = render_top_nav(site, page),
        search_markup = search_markup,
        language_switcher = language_switcher,
        skin_switcher = skin_switcher,
        github_link = github_link,
        nav = render_nav(site, page),
        content = page.html,
        toc = render_toc(page),
        markdown_copy = markdown_copy,
        search_dialog = render_search_dialog(site),
        access_mask = access_mask,
        mermaid_script = mermaid_script(),
    )
}

fn mermaid_script() -> &'static str {
    r##"<script type="module">
import mermaid from "https://cdn.jsdelivr.net/npm/mermaid@11/dist/mermaid.esm.min.mjs";

const mermaidBlocks = Array.from(document.querySelectorAll("pre.mermaid"));
const mermaidSources = new Map(mermaidBlocks.map(block => [block, block.textContent || ""]));

function mermaidColor(name, fallback) {
  const value = getComputedStyle(document.documentElement).getPropertyValue(name).trim();
  return value || fallback;
}

function mermaidConfig() {
  const isDark = document.documentElement.dataset.rpSkin === "dark";
  const background = mermaidColor("--rp-mermaid-bg", isDark ? "#151a20" : "#ffffff");
  const text = mermaidColor("--rp-mermaid-text", isDark ? "#edf2f4" : "#1d2528");
  const line = mermaidColor("--rp-mermaid-line", isDark ? "#9fb4bd" : "#6c7a80");
  const node = mermaidColor("--rp-mermaid-node-bg", isDark ? "#17382f" : "#e8f5f1");
  const nodeBorder = mermaidColor("--rp-mermaid-node-border", isDark ? "#66c2a5" : "#176b5b");
  const cluster = mermaidColor("--rp-mermaid-cluster-bg", isDark ? "#101820" : "#f2f8f6");
  const clusterBorder = mermaidColor("--rp-mermaid-cluster-border", isDark ? "#3d5c54" : "#b7ccc5");
  const label = mermaidColor("--rp-mermaid-label-bg", isDark ? "#181d23" : "#ffffff");

  return {
    startOnLoad: false,
    theme: "base",
    themeVariables: {
      darkMode: isDark,
      background,
      primaryColor: node,
      primaryTextColor: text,
      primaryBorderColor: nodeBorder,
      secondaryColor: cluster,
      secondaryTextColor: text,
      secondaryBorderColor: clusterBorder,
      tertiaryColor: label,
      tertiaryTextColor: text,
      tertiaryBorderColor: clusterBorder,
      mainBkg: node,
      nodeBorder,
      nodeTextColor: text,
      textColor: text,
      titleColor: text,
      lineColor: line,
      defaultLinkColor: line,
      clusterBkg: cluster,
      clusterBorder,
      edgeLabelBackground: label,
      labelTextColor: text,
      actorBkg: node,
      actorBorder: nodeBorder,
      actorTextColor: text,
      actorLineColor: line,
      signalColor: line,
      signalTextColor: text,
      noteBkg: label,
      noteTextColor: text,
      noteBorderColor: clusterBorder
    }
  };
}

async function renderMermaid() {
  if (mermaidBlocks.length === 0) return;
  for (const block of mermaidBlocks) {
    block.removeAttribute("data-processed");
    block.textContent = mermaidSources.get(block) || "";
  }
  mermaid.initialize(mermaidConfig());
  try {
    await mermaid.run({ nodes: mermaidBlocks });
  } catch (error) {
    console.warn("RustPress Mermaid render failed", error);
  }
}

renderMermaid();
document.addEventListener("rustpress:skinchange", renderMermaid);
</script>"##
}

fn render_top_nav(site: &SiteRender, page: &PageRender) -> String {
    if site.top_nav.is_empty() {
        return String::new();
    }

    let items = site
        .top_nav
        .iter()
        .map(|item| {
            if item.items.is_empty() {
                let Some(href) = &item.href else {
                    return String::new();
                };
                let active = link_is_active(&page.route, href);
                return format!(
                    r#"<a class="rp-topnav-link{active}" href="{href}">{title}</a>"#,
                    active = if active { " is-active" } else { "" },
                    href = href_for(site, href),
                    title = escape_html(&item.title)
                );
            }

            let active = item
                .href
                .as_ref()
                .is_some_and(|href| link_is_active(&page.route, href))
                || item
                    .items
                    .iter()
                    .any(|child| link_is_active(&page.route, &child.href));
            let trigger = if let Some(href) = &item.href {
                format!(
                    r#"<a class="rp-topnav-trigger" href="{href}" aria-haspopup="true">{title}</a>"#,
                    href = href_for(site, href),
                    title = escape_html(&item.title)
                )
            } else {
                format!(
                    r#"<span class="rp-topnav-trigger" tabindex="0" aria-haspopup="true">{title}</span>"#,
                    title = escape_html(&item.title)
                )
            };
            let links = item
                .items
                .iter()
                .map(|child| {
                    let child_active = link_is_active(&page.route, &child.href);
                    format!(
                        r#"<a class="rp-topnav-menu-link{active}" href="{href}">{title}</a>"#,
                        active = if child_active { " is-active" } else { "" },
                        href = href_for(site, &child.href),
                        title = escape_html(&child.title)
                    )
                })
                .collect::<Vec<_>>()
                .join("\n");

            format!(
                r#"<div class="rp-topnav-group{active}">{trigger}<div class="rp-topnav-menu">{links}</div></div>"#,
                active = if active { " is-active" } else { "" },
                trigger = trigger,
                links = links
            )
        })
        .filter(|item| !item.is_empty())
        .collect::<Vec<_>>()
        .join("\n");

    format!(r#"<nav class="rp-topnav" aria-label="Top navigation">{items}</nav>"#)
}

fn render_nav(site: &SiteRender, page: &PageRender) -> String {
    render_nav_items(site, page, &site.nav, 0)
}

fn render_nav_items(
    site: &SiteRender,
    page: &PageRender,
    items: &[NavItem],
    level: usize,
) -> String {
    items
        .iter()
        .map(|item| {
            let active = nav_item_is_active(item, &page.route);
            if item.items.is_empty() {
                return format!(
                    r#"<a class="rp-nav-link rp-nav-level-{level}{active}" href="{href}">{title}</a>"#,
                    level = level,
                    active = if active { " is-active" } else { "" },
                    href = href_for(site, &item.href),
                    title = escape_html(&item.title)
                );
            }

            let children = render_nav_items(site, page, &item.items, level + 1);
            format!(
                r#"<div class="rp-nav-group{active}"><a class="rp-nav-group-title" href="{href}">{title}</a><div class="rp-nav-children">{children}</div></div>"#,
                active = if active { " is-active" } else { "" },
                href = href_for(site, &item.href),
                title = escape_html(&item.title),
                children = children
            )
        })
        .collect::<Vec<_>>()
        .join("\n")
}

fn nav_item_is_active(item: &NavItem, route: &str) -> bool {
    route == item.href
        || (item.href != "/" && route.starts_with(&item.active_prefix))
        || item
            .items
            .iter()
            .any(|child| nav_item_is_active(child, route))
}

fn render_toc(page: &PageRender) -> String {
    let links = page
        .headings
        .iter()
        .filter(|heading| heading.level > 1 && heading.level < 4)
        .map(|heading| {
            format!(
                r##"<a class="rp-toc-link rp-toc-level-{level}" href="#{anchor}">{title}</a>"##,
                level = heading.level,
                anchor = escape_attr(&heading.anchor),
                title = escape_html(&heading.text)
            )
        })
        .collect::<Vec<_>>()
        .join("\n");

    if links.is_empty() {
        String::new()
    } else {
        format!(r#"<aside class="rp-toc" aria-label="On this page">{links}</aside>"#)
    }
}

fn render_skin_switcher(site: &SiteRender) -> String {
    let options = ["light", "dark"]
        .iter()
        .map(|skin| {
            let selected = skin == &site.theme.skin.as_str();
            format!(
                r#"<button class="rp-select-option{selected}" type="button" role="option" aria-selected="{aria_selected}" data-rp-skin-option data-rp-skin-value="{value}">{label}</button>"#,
                value = escape_attr(skin),
                selected = if selected { " is-selected" } else { "" },
                aria_selected = selected,
                label = skin_label(skin)
            )
        })
        .collect::<Vec<_>>()
        .join("");
    format!(
        r#"<div class="rp-select rp-skin-select" data-rp-select data-rp-skin-select title="Color theme"><button class="rp-select-button" type="button" data-rp-select-trigger data-rp-skin-trigger aria-haspopup="listbox" aria-expanded="false"><span class="rp-select-label">Theme</span><span class="rp-select-value" data-rp-skin-current>{current}</span></button><div class="rp-select-menu" role="listbox">{options}</div></div>"#,
        current = skin_label(&site.theme.skin)
    )
}

fn render_github_link(site: &SiteRender) -> String {
    let href = site.theme.github_url.trim();
    if href.is_empty() {
        return String::new();
    }

    format!(
        r#"<a class="rp-icon-button rp-github-link" href="{href}" target="_blank" rel="noopener noreferrer" aria-label="GitHub repository" title="GitHub">
<svg viewBox="0 0 24 24" aria-hidden="true"><path d="M12 2C6.48 2 2 6.59 2 12.25c0 4.53 2.87 8.37 6.84 9.72.5.09.68-.22.68-.49 0-.24-.01-.88-.01-1.72-2.78.62-3.37-1.37-3.37-1.37-.45-1.18-1.11-1.49-1.11-1.49-.91-.64.07-.63.07-.63 1 .07 1.53 1.06 1.53 1.06.89 1.56 2.34 1.11 2.91.85.09-.66.35-1.11.63-1.37-2.22-.26-4.55-1.14-4.55-5.06 0-1.12.39-2.03 1.03-2.75-.1-.26-.45-1.3.1-2.71 0 0 .84-.28 2.75 1.05A9.32 9.32 0 0 1 12 7.01c.85 0 1.71.12 2.51.35 1.91-1.33 2.75-1.05 2.75-1.05.55 1.41.2 2.45.1 2.71.64.72 1.03 1.63 1.03 2.75 0 3.93-2.34 4.8-4.57 5.05.36.32.68.95.68 1.91 0 1.38-.01 2.49-.01 2.83 0 .27.18.59.69.49A10.22 10.22 0 0 0 22 12.25C22 6.59 17.52 2 12 2Z"></path></svg>
</a>"#,
        href = escape_attr(href)
    )
}

fn skin_label(skin: &str) -> &'static str {
    match skin {
        "dark" => "Dark",
        _ => "Light",
    }
}

fn render_language_switcher(site: &SiteRender) -> String {
    if site.languages.is_empty() {
        return String::new();
    }

    let options = site
        .languages
        .iter()
        .map(|language| {
            let selected = language.current;
            format!(
                r#"<button class="rp-select-option{selected}" type="button" role="option" aria-selected="{aria_selected}" data-rp-language-option data-rp-language-href="{value}">{label}</button>"#,
                value = escape_attr(&href_for(site, &language.href)),
                selected = if selected { " is-selected" } else { "" },
                aria_selected = selected,
                label = escape_html(&language.label)
            )
        })
        .collect::<Vec<_>>()
        .join("");
    let current = site
        .languages
        .iter()
        .find(|language| language.current)
        .map(|language| language.label.as_str())
        .unwrap_or("Language");
    format!(
        r#"<div class="rp-select rp-language-select" data-rp-select data-rp-language-select title="Language"><button class="rp-select-button" type="button" data-rp-select-trigger data-rp-language-trigger aria-haspopup="listbox" aria-expanded="false"><span class="rp-select-label">Language</span><span class="rp-select-value" data-rp-language-current>{current}</span></button><div class="rp-select-menu" role="listbox">{options}</div></div>"#,
        current = escape_html(current)
    )
}

fn render_search_dialog(site: &SiteRender) -> String {
    if !site.search_enabled {
        return String::new();
    }

    r#"<dialog class="rp-search" data-rp-search>
  <form method="dialog" class="rp-search-box">
    <input data-rp-search-input type="search" autocomplete="off" placeholder="Search docs">
    <button class="rp-icon-button" value="close" aria-label="Close search" title="Close">
      <svg viewBox="0 0 24 24" aria-hidden="true"><path d="M18 6 6 18M6 6l12 12"></path></svg>
    </button>
  </form>
  <div class="rp-search-results" data-rp-search-results></div>
</dialog>"#
        .to_string()
}

fn render_markdown_copy(page: &PageRender) -> String {
    if page.markdown_source.is_empty() {
        return String::new();
    }

    format!(
        r#"<div class="rp-markdown-copy" data-rp-markdown-copy>
  <button class="rp-markdown-copy-trigger" type="button" data-rp-markdown-copy-trigger aria-label="Copy Markdown" title="Copy Markdown" aria-haspopup="menu" aria-expanded="false">
    <svg class="rp-code-copy-icon" viewBox="0 0 24 24" aria-hidden="true"><rect x="9" y="9" width="11" height="11" rx="2"></rect><path d="M5 15H4a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h9a2 2 0 0 1 2 2v1"></path></svg>
    <svg class="rp-code-copy-check" viewBox="0 0 24 24" aria-hidden="true"><path d="M20 6 9 17l-5-5"></path></svg>
  </button>
  <div class="rp-markdown-copy-menu" data-rp-markdown-copy-menu role="menu" aria-label="Copy Markdown options">
    <button class="rp-markdown-copy-option" type="button" role="menuitem" data-rp-copy-markdown>Copy Markdown</button>
    <button class="rp-markdown-copy-option" type="button" role="menuitem" data-rp-copy-markdown-url data-rp-markdown-source-url="{}">Copy Markdown URL</button>
  </div>
</div>
<textarea class="rp-markdown-source" data-rp-markdown-source readonly hidden>{}</textarea>"#,
        escape_attr(&page.markdown_source_url),
        escape_html(&page.markdown_source)
    )
}

fn render_access_mask(site: &SiteRender) -> String {
    format!(
        r#"<div class="rp-access-mask" data-rp-access-mask>
  <form class="rp-access-panel" data-rp-access-form>
    <h2>Masked content</h2>
    <p>This is a front-end viewing mask. Static files still contain the page content.</p>
    <input data-rp-access-input type="password" placeholder="{hint}" aria-label="{hint}" autocomplete="current-password" required>
    <p class="rp-access-error" data-rp-access-error hidden>Incorrect password.</p>
    <button type="submit">View page</button>
  </form>
</div>"#,
        hint = escape_attr(&site.password_hint)
    )
}

fn href_for(site: &SiteRender, href: &str) -> String {
    if href.starts_with("http://")
        || href.starts_with("https://")
        || href.starts_with("mailto:")
        || href.starts_with('#')
    {
        href.to_string()
    } else if href == "/" {
        site.base.clone()
    } else if href.starts_with('/') {
        format!("{}{}", site.base, href.trim_start_matches('/'))
    } else {
        format!("{}{}", site.base, href)
    }
}

fn link_is_active(route: &str, href: &str) -> bool {
    href.starts_with('/') && (route == href || (href != "/" && route.starts_with(href)))
}

fn css() -> &'static str {
    r#":root {
  color-scheme: light;
  --rp-bg: #f7f7f4;
  --rp-panel: #ffffff;
  --rp-text: #1d2528;
  --rp-muted: #607179;
  --rp-line: #dbe1de;
  --rp-accent: #176b5b;
  --rp-accent-soft: #dff0eb;
  --rp-danger: #b42318;
  --rp-code-bg: #172026;
  --rp-code-text: #edf7f6;
  --rp-shadow: 0 12px 30px rgb(27 40 42 / 12%);
  --rp-grid-line: rgb(23 107 91 / 5%);
  --rp-mermaid-bg: #ffffff;
  --rp-mermaid-text: #1d2528;
  --rp-mermaid-line: #6c7a80;
  --rp-mermaid-node-bg: #e8f5f1;
  --rp-mermaid-node-border: #176b5b;
  --rp-mermaid-cluster-bg: #f2f8f6;
  --rp-mermaid-cluster-border: #b7ccc5;
  --rp-mermaid-label-bg: #ffffff;
  font-family: Inter, ui-sans-serif, system-ui, -apple-system, BlinkMacSystemFont, "Segoe UI", sans-serif;
}

[data-rp-skin="dark"] {
  color-scheme: dark;
  --rp-bg: #111418;
  --rp-panel: #181d23;
  --rp-text: #edf2f4;
  --rp-muted: #a0abb3;
  --rp-line: #2d3741;
  --rp-accent: #66c2a5;
  --rp-accent-soft: #12392f;
  --rp-danger: #fca5a5;
  --rp-code-bg: #0b1117;
  --rp-code-text: #f2fbff;
  --rp-shadow: 0 14px 34px rgb(0 0 0 / 34%);
  --rp-grid-line: rgb(255 255 255 / 5%);
  --rp-mermaid-bg: #151a20;
  --rp-mermaid-text: #edf2f4;
  --rp-mermaid-line: #9fb4bd;
  --rp-mermaid-node-bg: #17382f;
  --rp-mermaid-node-border: #66c2a5;
  --rp-mermaid-cluster-bg: #101820;
  --rp-mermaid-cluster-border: #3d5c54;
  --rp-mermaid-label-bg: #181d23;
}

* { box-sizing: border-box; }
html { scroll-padding-top: 88px; }
body {
  margin: 0;
  background:
    linear-gradient(var(--rp-grid-line) 1px, transparent 1px),
    linear-gradient(90deg, var(--rp-grid-line) 1px, transparent 1px),
    var(--rp-bg);
  background-size: 32px 32px;
  color: var(--rp-text);
  line-height: 1.65;
  font-size: 16px;
}
a { color: var(--rp-accent); text-decoration: none; }
a:hover { text-decoration: underline; }

.rp-topbar {
  position: sticky;
  top: 0;
  z-index: 20;
  height: 64px;
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 0 24px;
  border-bottom: 1px solid var(--rp-line);
  background: color-mix(in srgb, var(--rp-panel) 92%, transparent);
  backdrop-filter: blur(16px);
}
.rp-brand {
  min-width: 0;
  color: var(--rp-text);
  font-weight: 750;
  font-size: 18px;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}
.rp-topnav {
  flex: 0 1 auto;
  min-width: 0;
  display: flex;
  align-items: center;
  justify-content: flex-end;
  gap: 4px;
  margin: 0 12px 0 auto;
}
.rp-topnav-link,
.rp-topnav-trigger {
  height: 36px;
  display: flex;
  align-items: center;
  padding: 0 10px;
  border-radius: 8px;
  color: var(--rp-muted);
  font-size: 14px;
  line-height: 1;
  white-space: nowrap;
  cursor: pointer;
}
.rp-topnav-link:hover,
.rp-topnav-link.is-active,
.rp-topnav-group.is-active > .rp-topnav-trigger,
.rp-topnav-group:hover > .rp-topnav-trigger,
.rp-topnav-group:focus-within > .rp-topnav-trigger {
  background: var(--rp-accent-soft);
  color: var(--rp-accent);
  text-decoration: none;
}
.rp-topnav-group {
  position: relative;
  height: 64px;
  display: flex;
  align-items: center;
}
.rp-topnav-trigger::after {
  content: "";
  width: 6px;
  height: 6px;
  margin-left: 8px;
  border-right: 1.5px solid currentColor;
  border-bottom: 1.5px solid currentColor;
  transform: translateY(-2px) rotate(45deg);
}
.rp-topnav-menu {
  position: absolute;
  top: 100%;
  right: 0;
  z-index: 30;
  min-width: 190px;
  display: none;
  gap: 2px;
  padding: 8px;
  border: 1px solid var(--rp-line);
  border-radius: 8px;
  background: var(--rp-panel);
  box-shadow: var(--rp-shadow);
}
.rp-topnav-group:hover > .rp-topnav-menu,
.rp-topnav-group:focus-within > .rp-topnav-menu {
  display: grid;
}
.rp-topnav-menu-link {
  display: block;
  padding: 8px 10px;
  border-radius: 8px;
  color: var(--rp-muted);
  font-size: 14px;
  line-height: 1.35;
}
.rp-topnav-menu-link:hover,
.rp-topnav-menu-link.is-active {
  background: var(--rp-accent-soft);
  color: var(--rp-accent);
  text-decoration: none;
}
.rp-topbar-actions {
  display: flex;
  gap: 8px;
  align-items: center;
  min-width: 0;
}
.rp-icon-button {
  width: 36px;
  height: 36px;
  display: inline-grid;
  place-items: center;
  border: 1px solid var(--rp-line);
  background: var(--rp-panel);
  color: var(--rp-text);
  border-radius: 8px;
  cursor: pointer;
}
.rp-icon-button svg {
  width: 18px;
  height: 18px;
  fill: none;
  stroke: currentColor;
  stroke-width: 2;
  stroke-linecap: round;
}
.rp-icon-button:hover {
  border-color: var(--rp-accent);
  color: var(--rp-accent);
  text-decoration: none;
}
.rp-github-link svg {
  fill: currentColor;
  stroke: none;
}
.rp-select {
  position: relative;
  flex: 0 0 auto;
}
.rp-select-button {
  appearance: none;
  -webkit-appearance: none;
  min-width: 124px;
  height: 36px;
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 8px;
  padding: 0 32px 0 10px;
  border: 1px solid color-mix(in srgb, var(--rp-line) 82%, var(--rp-muted));
  background: linear-gradient(180deg, var(--rp-panel), color-mix(in srgb, var(--rp-panel) 88%, var(--rp-bg)));
  border-radius: 8px;
  color: var(--rp-muted);
  font-size: 13px;
  line-height: 1;
  box-shadow: inset 0 -1px 0 rgb(0 0 0 / 4%);
  cursor: pointer;
  transition: border-color 140ms ease, box-shadow 140ms ease, color 140ms ease;
}
.rp-language-select .rp-select-button { min-width: 152px; }
.rp-select-button:hover,
.rp-select.is-open .rp-select-button,
.rp-select-button:focus-visible {
  border-color: var(--rp-accent);
  color: var(--rp-accent);
  box-shadow: 0 0 0 3px color-mix(in srgb, var(--rp-accent-soft) 72%, transparent);
  outline: 0;
}
.rp-select-button::after {
  content: "";
  position: absolute;
  right: 12px;
  top: 50%;
  width: 7px;
  height: 7px;
  border-right: 1.5px solid currentColor;
  border-bottom: 1.5px solid currentColor;
  transform: translateY(-65%) rotate(45deg);
  pointer-events: none;
}
.rp-select-label {
  color: currentColor;
  font-size: 12px;
  font-weight: 650;
}
.rp-select-value {
  min-width: 0;
  overflow: hidden;
  color: var(--rp-text);
  text-overflow: ellipsis;
  white-space: nowrap;
}
.rp-select-menu {
  position: absolute;
  top: calc(100% + 8px);
  right: 0;
  z-index: 35;
  min-width: 100%;
  display: none;
  gap: 2px;
  padding: 6px;
  border: 1px solid var(--rp-line);
  border-radius: 8px;
  background: var(--rp-panel);
  box-shadow: var(--rp-shadow);
}
.rp-select.is-open .rp-select-menu { display: grid; }
.rp-select-option {
  width: 100%;
  min-width: 132px;
  min-height: 32px;
  display: block;
  padding: 7px 10px;
  border: 0;
  border-radius: 7px;
  background: transparent;
  color: var(--rp-muted);
  font: inherit;
  font-size: 13px;
  line-height: 1.35;
  text-align: left;
  white-space: nowrap;
  cursor: pointer;
}
.rp-select-option:hover,
.rp-select-option:focus-visible,
.rp-select-option.is-selected {
  background: var(--rp-accent-soft);
  color: var(--rp-accent);
  outline: 0;
}
.rp-menu-button { display: none; }

.rp-shell {
  display: grid;
  grid-template-columns: minmax(220px, 280px) minmax(0, 1fr);
  max-width: 1440px;
  margin: 0 auto;
}
.rp-sidebar {
  position: sticky;
  top: 64px;
  height: calc(100vh - 64px);
  padding: 24px 16px 24px 24px;
  border-right: 1px solid var(--rp-line);
  overflow: auto;
}
.rp-nav-group {
  display: grid;
  gap: 4px;
  margin: 4px 0 12px;
}
.rp-nav-group-title {
  display: block;
  padding: 7px 10px;
  color: var(--rp-text);
  border-radius: 8px;
  font-size: 13px;
  font-weight: 750;
  line-height: 1.35;
}
.rp-nav-group-title:hover,
.rp-nav-group.is-active > .rp-nav-group-title {
  background: var(--rp-accent-soft);
  color: var(--rp-accent);
  text-decoration: none;
}
.rp-nav-children {
  display: grid;
  gap: 2px;
  margin-left: 10px;
  padding-left: 10px;
  border-left: 1px solid var(--rp-line);
}
.rp-nav-link {
  display: block;
  padding: 8px 10px;
  color: var(--rp-muted);
  border-radius: 8px;
  font-size: 14px;
  line-height: 1.35;
}
.rp-nav-level-1 {
  font-size: 13px;
}
.rp-nav-link:hover,
.rp-nav-link.is-active {
  background: var(--rp-accent-soft);
  color: var(--rp-accent);
  text-decoration: none;
}

.rp-main {
  display: grid;
  grid-template-columns: minmax(0, 820px) minmax(180px, 240px);
  gap: 42px;
  min-width: 0;
  padding: 48px 32px 96px;
}
.rp-doc {
  min-width: 0;
}
.rp-doc > :first-child { margin-top: 0; }
.rp-doc h1,
.rp-doc h2,
.rp-doc h3,
.rp-doc h4 {
  line-height: 1.2;
  margin: 2em 0 0.65em;
  overflow-wrap: anywhere;
}
.rp-doc h1 { font-size: 42px; margin-top: 0; }
.rp-doc h2 { font-size: 28px; padding-top: 12px; border-top: 1px solid var(--rp-line); }
.rp-doc h3 { font-size: 22px; }
.rp-doc p,
.rp-doc li { color: var(--rp-text); }
.rp-doc ul,
.rp-doc ol {
  margin: 1.1em 0 1.25em;
  padding-left: 1.55em;
}
.rp-doc li {
  margin: 0.45em 0;
  padding-left: 0.1em;
}
.rp-doc li > ul,
.rp-doc li > ol {
  margin: 0.55em 0;
}
.rp-doc blockquote {
  margin: 20px 0;
  padding: 4px 18px;
  border-left: 4px solid var(--rp-accent);
  background: var(--rp-accent-soft);
}
.rp-doc table {
  width: 100%;
  border-collapse: collapse;
  display: block;
  overflow-x: auto;
}
.rp-doc th,
.rp-doc td {
  border: 1px solid var(--rp-line);
  padding: 8px 10px;
}
.rp-doc code {
  padding: 2px 5px;
  background: var(--rp-accent-soft);
  border-radius: 5px;
  font-size: 0.9em;
}
.rp-doc pre {
  overflow: auto;
  padding: 16px;
  border-radius: 8px;
  background: var(--rp-code-bg);
  color: var(--rp-code-text);
  box-shadow: var(--rp-shadow);
}
.rp-doc pre code {
  padding: 0;
  background: transparent;
  color: inherit;
  border-radius: 0;
}
.rp-code {
  position: relative;
  margin: 20px 0;
  overflow: hidden;
  border: 1px solid color-mix(in srgb, var(--rp-code-bg) 78%, var(--rp-line));
  border-radius: 8px;
  background: var(--rp-code-bg);
  box-shadow: var(--rp-shadow);
}
.rp-code-header {
  display: flex;
  align-items: center;
  min-height: 34px;
  padding: 0 54px 0 14px;
  border-bottom: 1px solid rgb(255 255 255 / 8%);
  background: color-mix(in srgb, var(--rp-code-bg) 82%, black);
  color: rgb(237 247 246 / 72%);
  font-size: 12px;
  font-weight: 700;
  line-height: 1;
}
.rp-code-copy {
  appearance: none;
  -webkit-appearance: none;
  position: absolute;
  top: 8px;
  right: 8px;
  z-index: 2;
  width: 30px;
  height: 30px;
  display: inline-grid;
  place-items: center;
  padding: 0;
  border: 1px solid rgb(255 255 255 / 16%);
  border-radius: 7px;
  background: color-mix(in srgb, var(--rp-code-bg) 78%, white);
  color: rgb(237 247 246 / 76%);
  cursor: pointer;
  transition: border-color 140ms ease, background 140ms ease, color 140ms ease, opacity 140ms ease;
}
.rp-code-copy svg {
  width: 16px;
  height: 16px;
  fill: none;
  stroke: currentColor;
  stroke-width: 2;
  stroke-linecap: round;
  stroke-linejoin: round;
}
.rp-code-copy-check {
  display: none;
}
.rp-code-copy:hover,
.rp-code-copy:focus-visible {
  border-color: var(--rp-accent);
  background: color-mix(in srgb, var(--rp-code-bg) 64%, var(--rp-accent));
  color: var(--rp-code-text);
}
.rp-code-copy:focus-visible {
  outline: 2px solid var(--rp-accent);
  outline-offset: 2px;
}
.rp-code-copy:disabled {
  cursor: default;
  opacity: 0.9;
}
.rp-code-copy[data-rp-copied="true"] {
  border-color: var(--rp-accent);
  background: var(--rp-accent-soft);
  color: var(--rp-accent);
}
.rp-code-copy[data-rp-copied="true"] .rp-code-copy-icon {
  display: none;
}
.rp-code-copy[data-rp-copied="true"] .rp-code-copy-check {
  display: block;
}
.rp-markdown-copy {
  position: fixed;
  right: 28px;
  bottom: 28px;
  z-index: 24;
  display: grid;
  justify-items: end;
  gap: 8px;
}
.rp-markdown-copy-trigger {
  appearance: none;
  -webkit-appearance: none;
  width: 44px;
  height: 44px;
  display: inline-grid;
  place-items: center;
  padding: 0;
  border: 1px solid var(--rp-line);
  border-radius: 8px;
  background: var(--rp-panel);
  color: var(--rp-muted);
  box-shadow: var(--rp-shadow);
  cursor: pointer;
  transition: border-color 140ms ease, background 140ms ease, color 140ms ease, opacity 140ms ease;
}
.rp-markdown-copy-trigger svg {
  width: 19px;
  height: 19px;
  fill: none;
  stroke: currentColor;
  stroke-width: 2;
  stroke-linecap: round;
  stroke-linejoin: round;
}
.rp-markdown-copy-trigger:hover,
.rp-markdown-copy-trigger:focus-visible {
  border-color: var(--rp-accent);
  background: var(--rp-accent-soft);
  color: var(--rp-accent);
}
.rp-markdown-copy-trigger:focus-visible {
  outline: 2px solid var(--rp-accent);
  outline-offset: 2px;
}
.rp-markdown-copy-trigger:disabled {
  cursor: default;
  opacity: 0.95;
}
.rp-markdown-copy-trigger[data-rp-copied="true"] {
  border-color: var(--rp-accent);
  background: var(--rp-accent-soft);
  color: var(--rp-accent);
}
.rp-markdown-copy-trigger[data-rp-copied="true"] .rp-code-copy-icon {
  display: none;
}
.rp-markdown-copy-trigger[data-rp-copied="true"] .rp-code-copy-check {
  display: block;
}
.rp-markdown-copy-menu {
  position: absolute;
  right: 0;
  bottom: calc(100% + 8px);
  z-index: 25;
  min-width: 182px;
  display: none;
  gap: 2px;
  padding: 6px;
  border: 1px solid var(--rp-line);
  border-radius: 8px;
  background: var(--rp-panel);
  box-shadow: var(--rp-shadow);
}
.rp-markdown-copy.is-open .rp-markdown-copy-menu {
  display: grid;
}
.rp-markdown-copy-option {
  width: 100%;
  min-height: 34px;
  display: block;
  padding: 8px 10px;
  border: 0;
  border-radius: 7px;
  background: transparent;
  color: var(--rp-muted);
  font: inherit;
  font-size: 13px;
  line-height: 1.35;
  text-align: left;
  white-space: nowrap;
  cursor: pointer;
}
.rp-markdown-copy-option:hover,
.rp-markdown-copy-option:focus-visible {
  background: var(--rp-accent-soft);
  color: var(--rp-accent);
  outline: 0;
}
.rp-markdown-source {
  display: none;
}
.rp-code pre {
  margin: 0;
  padding-right: 56px;
  border-radius: 0;
  box-shadow: none;
  font-family: ui-monospace, SFMono-Regular, "SF Mono", Consolas, "Liberation Mono", Menlo, monospace;
  font-size: 0.9em;
  line-height: 1.65;
}
.rp-code code {
  display: block;
  min-width: max-content;
  font: inherit;
}
.rp-code-line-numbers pre {
  display: grid;
  grid-template-columns: 72px minmax(0, max-content);
  align-items: start;
  padding-left: 0;
}
.rp-code-lines {
  display: block;
  box-sizing: border-box;
  width: 72px;
  padding: 0 12px 0 16px;
  border-right: 1px solid rgb(255 255 255 / 12%);
  color: rgb(237 247 246 / 42%);
  font: inherit;
  line-height: inherit;
  text-align: right;
  user-select: none;
  white-space: pre;
}
.rp-code-line-numbers .rp-code-content {
  padding-left: 16px;
}
.rp-doc pre.mermaid {
  background: var(--rp-mermaid-bg);
  color: var(--rp-mermaid-text);
  border: 1px solid var(--rp-mermaid-cluster-border);
}
.rp-doc pre.mermaid svg {
  max-width: 100%;
  height: auto;
}
.heading-anchor {
  display: inline-block;
  width: 0.85em;
  margin-left: -0.85em;
  opacity: 0;
  color: var(--rp-accent);
}
h1:hover .heading-anchor,
h2:hover .heading-anchor,
h3:hover .heading-anchor,
h4:hover .heading-anchor,
h5:hover .heading-anchor,
h6:hover .heading-anchor { opacity: 1; text-decoration: none; }
.rp-toc {
  position: sticky;
  top: 88px;
  height: max-content;
  max-height: calc(100vh - 112px);
  overflow: auto;
  padding-left: 16px;
  border-left: 1px solid var(--rp-line);
}
.rp-toc-link {
  display: block;
  padding: 4px 0;
  color: var(--rp-muted);
  font-size: 13px;
  line-height: 1.35;
}
.rp-toc-level-3 { padding-left: 14px; }

.rp-search::backdrop { background: rgb(20 28 31 / 45%); }
.rp-search {
  width: min(720px, calc(100vw - 32px));
  max-height: min(720px, calc(100vh - 48px));
  border: 1px solid var(--rp-line);
  border-radius: 8px;
  padding: 0;
  background: var(--rp-panel);
  color: var(--rp-text);
  box-shadow: var(--rp-shadow);
}
.rp-search-box {
  display: flex;
  gap: 8px;
  padding: 12px;
  border-bottom: 1px solid var(--rp-line);
}
.rp-search-box input {
  flex: 1;
  min-width: 0;
  border: 1px solid var(--rp-line);
  border-radius: 8px;
  padding: 0 12px;
  font: inherit;
}
.rp-search-results {
  display: grid;
  gap: 4px;
  padding: 8px;
}
.rp-search-result {
  display: block;
  padding: 10px 12px;
  border-radius: 8px;
  color: var(--rp-text);
}
.rp-search-result:hover {
  background: var(--rp-accent-soft);
  text-decoration: none;
}
.rp-search-result span {
  display: block;
  color: var(--rp-muted);
  font-size: 13px;
}
.rp-access-mask {
  position: fixed;
  inset: 0;
  z-index: 40;
  display: grid;
  place-items: center;
  padding: 24px;
  background: color-mix(in srgb, var(--rp-bg) 86%, transparent);
  backdrop-filter: blur(14px);
}
.rp-access-mask.is-unlocked { display: none; }
.rp-access-panel {
  width: min(420px, 100%);
  padding: 24px;
  background: var(--rp-panel);
  border: 1px solid var(--rp-line);
  border-radius: 8px;
  box-shadow: var(--rp-shadow);
}
.rp-access-panel h2 {
  margin: 0 0 8px;
  font-size: 22px;
}
.rp-access-panel p {
  margin: 0 0 18px;
  color: var(--rp-muted);
}
.rp-access-panel input {
  width: 100%;
  height: 40px;
  border: 1px solid var(--rp-line);
  border-radius: 8px;
  padding: 0 10px;
  font: inherit;
}
.rp-access-panel input[aria-invalid="true"] {
  border-color: var(--rp-danger);
}
.rp-access-error {
  margin: 8px 0 0;
  color: var(--rp-danger);
  font-size: 13px;
  line-height: 1.4;
}
.rp-access-panel button {
  margin-top: 12px;
  width: 100%;
  height: 40px;
  border: 0;
  border-radius: 8px;
  background: var(--rp-accent);
  color: white;
  font-weight: 700;
  cursor: pointer;
}

@media (max-width: 1080px) {
  .rp-main { grid-template-columns: minmax(0, 1fr); }
  .rp-toc { display: none; }
}

@media (max-width: 760px) {
  .rp-topbar { padding: 0 12px; }
  .rp-topnav { display: none; }
  .rp-select-button { min-width: 74px; max-width: 96px; padding: 0 28px 0 8px; }
  .rp-language-select .rp-select-button { min-width: 92px; max-width: 112px; }
  .rp-select-label { display: none; }
  .rp-select-menu { right: 0; }
  .rp-select-option { min-width: 118px; }
  .rp-menu-button { display: inline-grid; }
  .rp-shell { display: block; }
  .rp-sidebar {
    display: none;
    position: fixed;
    inset: 64px 0 auto 0;
    z-index: 18;
    height: calc(100vh - 64px);
    background: var(--rp-panel);
    border-right: 0;
    border-bottom: 1px solid var(--rp-line);
    padding: 16px;
  }
  .rp-sidebar.is-open { display: block; }
  .rp-main { padding: 32px 18px 72px; }
  .rp-doc h1 { font-size: 32px; }
  .rp-doc h2 { font-size: 24px; }
  .heading-anchor { margin-left: 0; width: auto; opacity: 1; margin-right: 6px; }
  .rp-markdown-copy { right: 16px; bottom: 16px; }
}
"#
}

fn js(site: &SiteRender) -> String {
    format!(
        r##"const base = {base:?};
const defaultSkin = {skin:?};
const accessPassword = {access_password:?};
const supportedSkins = ["light", "dark"];

const root = document.documentElement;
const savedSkin = localStorage.getItem("rustpress:skin");
root.dataset.rpSkin = supportedSkins.includes(savedSkin) ? savedSkin : defaultSkin;
if (!supportedSkins.includes(root.dataset.rpSkin)) root.dataset.rpSkin = "light";

const selectMenus = Array.from(document.querySelectorAll("[data-rp-select]"));
for (const selectMenu of selectMenus) {{
  const trigger = selectMenu.querySelector("[data-rp-select-trigger]");
  if (!trigger) continue;
  trigger.addEventListener("click", event => {{
    event.stopPropagation();
    const wasOpen = selectMenu.classList.contains("is-open");
    closeSelectMenus();
    if (!wasOpen) openSelectMenu(selectMenu);
  }});
}}
document.addEventListener("click", closeSelectMenus);
document.addEventListener("keydown", event => {{
  if (event.key === "Escape") closeSelectMenus();
}});

const skinSelect = document.querySelector("[data-rp-skin-select]");
if (skinSelect) {{
  const skinCurrent = skinSelect.querySelector("[data-rp-skin-current]");
  const skinOptions = Array.from(skinSelect.querySelectorAll("[data-rp-skin-option]"));
  setSkin(root.dataset.rpSkin, false);
  for (const option of skinOptions) {{
    option.addEventListener("click", event => {{
      event.stopPropagation();
      setSkin(option.dataset.rpSkinValue, true);
      closeSelectMenus();
    }});
  }}

  function setSkin(skin, persist) {{
    if (!supportedSkins.includes(skin)) skin = "light";
    const previousSkin = root.dataset.rpSkin;
    root.dataset.rpSkin = skin;
    if (skinCurrent) skinCurrent.textContent = skin === "dark" ? "Dark" : "Light";
    for (const option of skinOptions) {{
      const selected = option.dataset.rpSkinValue === skin;
      option.classList.toggle("is-selected", selected);
      option.setAttribute("aria-selected", selected ? "true" : "false");
    }}
    if (persist) localStorage.setItem("rustpress:skin", skin);
    if (previousSkin !== skin) {{
      document.dispatchEvent(new CustomEvent("rustpress:skinchange", {{ detail: {{ skin }} }}));
    }}
  }}
}}

const languageSelect = document.querySelector("[data-rp-language-select]");
if (languageSelect) {{
  for (const option of languageSelect.querySelectorAll("[data-rp-language-option]")) {{
    option.addEventListener("click", event => {{
      event.stopPropagation();
      if (option.dataset.rpLanguageHref) window.location.href = option.dataset.rpLanguageHref;
    }});
  }}
}}

function openSelectMenu(selectMenu) {{
  selectMenu.classList.add("is-open");
  const trigger = selectMenu.querySelector("[data-rp-select-trigger]");
  if (trigger) trigger.setAttribute("aria-expanded", "true");
}}

function closeSelectMenus() {{
  for (const selectMenu of selectMenus) {{
    selectMenu.classList.remove("is-open");
    const trigger = selectMenu.querySelector("[data-rp-select-trigger]");
    if (trigger) trigger.setAttribute("aria-expanded", "false");
  }}
}}

const menu = document.querySelector("[data-rp-menu]");
const sidebar = document.querySelector("[data-rp-sidebar]");
if (menu && sidebar) {{
  menu.addEventListener("click", () => sidebar.classList.toggle("is-open"));
}}

const codeCopyButtons = Array.from(document.querySelectorAll("[data-rp-copy-code]"));
for (const button of codeCopyButtons) {{
  button.addEventListener("click", async () => {{
    const codeBlock = button.closest(".rp-code");
    const codeContent = codeBlock ? codeBlock.querySelector(".rp-code-content") : null;
    if (!codeContent) return;
    try {{
      await copyCodeText(codeContent.textContent || "");
      showCodeCopied(button);
    }} catch (error) {{
      console.warn("RustPress copy code failed", error);
    }}
  }});
}}

const markdownCopyRoot = document.querySelector("[data-rp-markdown-copy]");
const markdownCopyTrigger = markdownCopyRoot ? markdownCopyRoot.querySelector("[data-rp-markdown-copy-trigger]") : null;
const markdownCopyMenu = markdownCopyRoot ? markdownCopyRoot.querySelector("[data-rp-markdown-copy-menu]") : null;
const markdownCopyButton = markdownCopyRoot ? markdownCopyRoot.querySelector("[data-rp-copy-markdown]") : null;
const markdownCopyUrlButton = markdownCopyRoot ? markdownCopyRoot.querySelector("[data-rp-copy-markdown-url]") : null;
const markdownSource = document.querySelector("[data-rp-markdown-source]");

if (markdownCopyRoot && markdownCopyTrigger && markdownCopyMenu) {{
  markdownCopyTrigger.addEventListener("click", event => {{
    event.stopPropagation();
    const wasOpen = markdownCopyRoot.classList.contains("is-open");
    if (wasOpen) closeMarkdownCopyMenu();
    else openMarkdownCopyMenu();
  }});
  document.addEventListener("click", event => {{
    if (!markdownCopyRoot.contains(event.target)) closeMarkdownCopyMenu();
  }});
  document.addEventListener("keydown", event => {{
    if (event.key === "Escape") closeMarkdownCopyMenu();
  }});
}}

if (markdownCopyButton && markdownSource) {{
  markdownCopyButton.addEventListener("click", async event => {{
    event.stopPropagation();
    try {{
      await copyCodeText(markdownSource.value || "");
      showMarkdownCopied();
      closeMarkdownCopyMenu();
    }} catch (error) {{
      console.warn("RustPress copy Markdown failed", error);
    }}
  }});
}}

if (markdownCopyUrlButton) {{
  markdownCopyUrlButton.addEventListener("click", async event => {{
    event.stopPropagation();
    try {{
      await copyCodeText(markdownCopyHref(markdownCopyUrlButton));
      showMarkdownCopied();
      closeMarkdownCopyMenu();
    }} catch (error) {{
      console.warn("RustPress copy Markdown URL failed", error);
    }}
  }});
}}

function openMarkdownCopyMenu() {{
  if (!markdownCopyRoot || !markdownCopyTrigger) return;
  markdownCopyRoot.classList.add("is-open");
  markdownCopyTrigger.setAttribute("aria-expanded", "true");
}}

function closeMarkdownCopyMenu() {{
  if (!markdownCopyRoot || !markdownCopyTrigger) return;
  markdownCopyRoot.classList.remove("is-open");
  markdownCopyTrigger.setAttribute("aria-expanded", "false");
}}

function markdownCopyHref(button) {{
  const sourceUrl = button.dataset.rpMarkdownSourceUrl || "";
  try {{
    return new URL(sourceUrl, location.href).href;
  }} catch (error) {{
    return sourceUrl;
  }}
}}

function showMarkdownCopied() {{
  const button = markdownCopyTrigger || markdownCopyButton || markdownCopyUrlButton;
  if (button) showCopied(button, "Copy Markdown", "Copy Markdown");
}}

async function copyCodeText(text) {{
  if (window.navigator && navigator.clipboard && typeof navigator.clipboard.writeText === "function") {{
    try {{
      await navigator.clipboard.writeText(text);
      return;
    }} catch (error) {{
      fallbackCopyText(text);
      return;
    }}
  }}
  fallbackCopyText(text);
}}

function fallbackCopyText(text) {{
  const textarea = document.createElement("textarea");
  textarea.value = text;
  textarea.setAttribute("readonly", "");
  textarea.style.position = "fixed";
  textarea.style.top = "-1000px";
  textarea.style.left = "-1000px";
  textarea.style.opacity = "0";
  document.body.appendChild(textarea);
  textarea.focus();
  textarea.select();
  textarea.setSelectionRange(0, textarea.value.length);
  let copied = false;
  try {{
    copied = typeof document.execCommand === "function" && document.execCommand("copy");
  }} finally {{
    textarea.remove();
  }}
  if (!copied) throw new Error("copy command failed");
}}

function showCodeCopied(button) {{
  showCopied(button, "Copy code", "Copy code");
}}

function showCopied(button, resetLabel, resetTitle) {{
  button.dataset.rpCopied = "true";
  button.disabled = true;
  button.setAttribute("aria-label", "Copied");
  button.setAttribute("title", "Copied");
  window.clearTimeout(button.rpCopyReset);
  button.rpCopyReset = window.setTimeout(() => {{
    delete button.dataset.rpCopied;
    button.disabled = false;
    button.setAttribute("aria-label", resetLabel);
    button.setAttribute("title", resetTitle);
  }}, 1500);
}}

const mask = document.querySelector("[data-rp-access-mask]");
const accessForm = document.querySelector("[data-rp-access-form]");
const accessInput = document.querySelector("[data-rp-access-input]");
const accessError = document.querySelector("[data-rp-access-error]");
if (mask && accessForm) {{
  const key = "rustpress:access:" + location.pathname;
  if (sessionStorage.getItem(key) === "unlocked") mask.classList.add("is-unlocked");
  accessForm.addEventListener("submit", event => {{
    event.preventDefault();
    if (!accessInput || accessInput.value !== accessPassword) {{
      if (accessError) accessError.hidden = false;
      if (accessInput) {{
        accessInput.setAttribute("aria-invalid", "true");
        accessInput.focus();
      }}
      return;
    }}
    sessionStorage.setItem(key, "unlocked");
    mask.classList.add("is-unlocked");
  }});
  if (accessInput) {{
    accessInput.addEventListener("input", () => {{
      accessInput.removeAttribute("aria-invalid");
      if (accessError) accessError.hidden = true;
    }});
  }}
}}

const searchDialog = document.querySelector("[data-rp-search]");
const searchOpen = document.querySelector("[data-rp-search-open]");
const searchInput = document.querySelector("[data-rp-search-input]");
const searchResults = document.querySelector("[data-rp-search-results]");
let searchIndexPromise;

if (searchDialog && searchOpen && searchInput && searchResults) {{
  let lastShiftPress = 0;

  searchOpen.addEventListener("click", () => {{
    openSearch();
  }});

  document.addEventListener("keydown", event => {{
    if (event.key !== "Shift" || event.repeat) return;
    const now = Date.now();
    if (now - lastShiftPress <= 500) {{
      event.preventDefault();
      openSearch();
      lastShiftPress = 0;
    }} else {{
      lastShiftPress = now;
    }}
  }});

  function openSearch() {{
    if (typeof searchDialog.showModal === "function") searchDialog.showModal();
    else searchDialog.setAttribute("open", "");
    searchInput.focus();
    loadSearchIndex();
  }}

  searchInput.addEventListener("input", () => runSearch(searchInput.value));
}}

function loadSearchIndex() {{
  if (!searchIndexPromise) {{
    searchIndexPromise = fetch(joinBase("assets/search-index.json"))
      .then(response => response.ok ? response.json() : Promise.reject(new Error("search index missing")))
      .catch(() => ({{ pages: [] }}));
  }}
  return searchIndexPromise;
}}

function runSearch(query) {{
  const normalized = query.trim().toLowerCase();
  if (!normalized) {{
    searchResults.innerHTML = "";
    return;
  }}
  loadSearchIndex().then(index => {{
    const tokens = tokenize(normalized);
    const results = index.pages
      .map(page => ({{ page, score: scorePage(page, tokens), snippet: snippet(page, tokens) }}))
      .filter(result => result.score > 0)
      .sort((a, b) => b.score - a.score)
      .slice(0, 12);
    searchResults.innerHTML = results.length
      ? results.map(renderResult).join("")
      : "<p class=\"rp-search-empty\">No results</p>";
  }});
}}

function scorePage(page, tokens) {{
  const title = (page.title || "").toLowerCase();
  const body = (page.body || "").toLowerCase();
  let score = 0;
  for (const token of tokens) {{
    if (title.includes(token)) score += 8;
    if (body.includes(token)) score += 2;
  }}
  return score;
}}

function snippet(page, tokens) {{
  const body = page.body || "";
  const lower = body.toLowerCase();
  const first = tokens.map(token => lower.indexOf(token)).filter(index => index >= 0).sort((a, b) => a - b)[0] || 0;
  const start = Math.max(0, first - 56);
  const text = body.slice(start, start + 150);
  return (start > 0 ? "..." : "") + text;
}}

function renderResult(result) {{
  const page = result.page;
  return `<a class="rp-search-result" href="${{escapeHtml(page.url || "#")}}">${{escapeHtml(page.title || "Untitled")}}<span>${{escapeHtml(result.snippet || page.url || "")}}</span></a>`;
}}

function tokenize(input) {{
  const latin = input.match(/[a-z0-9]+/g) || [];
  const cjk = Array.from(input.matchAll(/[\u3400-\u9fff]/g)).map(match => match[0]);
  return [...latin, ...cjk].filter(Boolean);
}}

function joinBase(path) {{
  return base.replace(/\/$/, "") + "/" + path.replace(/^\//, "");
}}

function escapeHtml(value) {{
  return String(value).replace(/[&<>"']/g, char => ({{
    "&": "&amp;",
    "<": "&lt;",
    ">": "&gt;",
    "\"": "&quot;",
    "'": "&#39;"
  }}[char]));
}}
"##,
        base = site.base,
        skin = site.theme.skin,
        access_password = site.access_password
    )
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

    fn site() -> SiteRender {
        SiteRender {
            title: "Docs".to_string(),
            lang: "en-US".to_string(),
            base: "/".to_string(),
            home_href: "/".to_string(),
            theme: ThemeConfig {
                skin: "light".to_string(),
                allow_switch: true,
                github_url: "https://github.com/ZenithInc/rust-press".to_string(),
            },
            search_enabled: true,
            access_enabled: true,
            access_password: "rustpress".to_string(),
            password_hint: "Password".to_string(),
            top_nav: vec![
                TopNavItem {
                    title: "Guide".to_string(),
                    href: Some("/guide/".to_string()),
                    items: vec![TopNavLink {
                        title: "CLI".to_string(),
                        href: "/guide/cli/".to_string(),
                    }],
                },
                TopNavItem {
                    title: "Reference".to_string(),
                    href: Some("/reference/".to_string()),
                    items: vec![],
                },
            ],
            nav: vec![NavItem {
                title: "Home".to_string(),
                href: "/".to_string(),
                active_prefix: "/".to_string(),
                items: Vec::new(),
            }],
            languages: vec![
                LanguageOption {
                    label: "English".to_string(),
                    href: "/".to_string(),
                    current: true,
                },
                LanguageOption {
                    label: "Deutsch".to_string(),
                    href: "/de/".to_string(),
                    current: false,
                },
            ],
        }
    }

    #[test]
    fn renders_theme_switcher_and_mask() {
        let html = render_page(
            &site(),
            &PageRender {
                title: "Home".to_string(),
                route: "/".to_string(),
                html: "<h1>Home</h1>".to_string(),
                markdown_source: "---\ntitle: Home\n---\n# Home\n".to_string(),
                markdown_source_url: "/index.md.txt".to_string(),
                headings: vec![],
                masked: true,
                search: true,
            },
        );

        assert!(html.contains("data-rp-skin-select"));
        assert!(html.contains(r#"data-rp-skin-value="light">Light</button>"#));
        assert!(html.contains(r#"data-rp-skin-value="dark">Dark</button>"#));
        assert!(!html.contains("classic"));
        assert!(!html.contains("dense"));
        assert!(html.contains(r#"<html lang="en-US""#));
        assert!(html.contains("data-rp-language-select"));
        assert!(!html.contains("<select"));
        assert!(!html.contains("<option"));
        assert!(html.contains(r#"data-rp-language-href="/">English</button>"#));
        assert!(html.contains(r#"class="rp-icon-button rp-github-link""#));
        assert!(html.contains(r#"href="https://github.com/ZenithInc/rust-press""#));
        assert!(html.contains(r#"aria-label="GitHub repository""#));
        assert!(html.contains("data-rp-access-mask"));
        assert!(html.contains("data-rp-access-error"));
        assert!(html.contains("autocomplete=\"current-password\""));
        assert!(html.contains("front-end viewing mask"));
        assert!(html.contains("data-rp-copy-markdown"));
        assert!(html.contains("data-rp-copy-markdown-url"));
        assert!(html.contains("data-rp-markdown-copy-trigger"));
        assert!(html.contains("data-rp-markdown-copy-menu"));
        assert!(html.contains(r#"data-rp-markdown-source-url="/index.md.txt""#));
        assert!(html.contains("Copy Markdown URL"));
        assert!(html.contains("data-rp-markdown-source"));
        assert!(html.contains("---\ntitle: Home\n---\n# Home\n"));
        assert!(html.contains("rp-topnav-group"));
        assert!(html.contains("rp-topnav-trigger"));
        assert!(html.contains("Reference"));
        assert!(!html.contains("<details"));
        assert!(html.contains("theme: \"base\""));
        assert!(html.contains("themeVariables"));
        assert!(html.contains("mermaid.run({ nodes: mermaidBlocks })"));
        assert!(!html.contains("startOnLoad: true"));

        let js = js(&site());
        assert!(js.contains("lastShiftPress"));
        assert!(js.contains(r#"const accessPassword = "rustpress";"#));
        assert!(js.contains("accessInput.value !== accessPassword"));
        assert!(js.contains(r#"accessInput.setAttribute("aria-invalid", "true")"#));
        assert!(js.contains(r#"event.key !== "Shift""#));
        assert!(js.contains("rustpress:skinchange"));
        assert!(js.contains(r#"new CustomEvent("rustpress:skinchange""#));
    }

    #[test]
    fn css_includes_mermaid_theme_colors() {
        let styles = css();
        assert!(styles.contains("--rp-mermaid-bg: #ffffff;"));
        assert!(styles.contains("--rp-mermaid-text: #1d2528;"));
        assert!(styles.contains("--rp-mermaid-line: #6c7a80;"));
        assert!(styles.contains("--rp-mermaid-bg: #151a20;"));
        assert!(styles.contains("--rp-mermaid-text: #edf2f4;"));
        assert!(styles.contains("--rp-mermaid-line: #9fb4bd;"));
        assert!(styles.contains(".rp-doc pre.mermaid svg"));
        assert!(styles.contains(".rp-github-link svg"));
        assert!(styles.contains("max-width: 100%;"));
    }

    #[test]
    fn css_and_js_include_code_copy_support() {
        let styles = css();
        assert!(styles.contains(".rp-code-copy"));
        assert!(styles.contains(".rp-code-copy[data-rp-copied=\"true\"]"));
        assert!(styles.contains(".rp-code-copy:disabled"));
        assert!(styles.contains("padding-right: 56px;"));
        assert!(styles.contains(".rp-code-line-numbers pre"));
        assert!(styles.contains("grid-template-columns: 72px minmax(0, max-content);"));
        assert!(styles.contains(".rp-code-lines"));
        assert!(styles.contains("font-family: ui-monospace"));
        assert!(styles.contains("font-size: 0.9em;"));
        assert!(styles.contains("line-height: 1.65;"));
        assert!(styles.contains("font: inherit;"));
        assert!(styles.contains("width: 72px;"));
        assert!(!styles.contains("grid-template-columns: minmax(42px, auto)"));
        assert!(styles.contains("user-select: none;"));
        assert!(styles.contains(".rp-code-line-numbers .rp-code-content"));
        assert!(styles.contains(".rp-markdown-copy"));
        assert!(styles.contains("position: fixed;"));
        assert!(styles.contains(".rp-markdown-copy-trigger"));
        assert!(styles.contains(".rp-markdown-copy-trigger[data-rp-copied=\"true\"]"));
        assert!(styles.contains(".rp-markdown-copy-menu"));
        assert!(styles.contains("bottom: calc(100% + 8px);"));
        assert!(styles.contains(".rp-markdown-copy.is-open .rp-markdown-copy-menu"));
        assert!(styles.contains(".rp-markdown-copy-option:focus-visible"));
        assert!(styles.contains(".rp-markdown-source"));

        let script = js(&site());
        assert!(script.contains("[data-rp-copy-code]"));
        assert!(script.contains("[data-rp-copy-markdown]"));
        assert!(script.contains("[data-rp-copy-markdown-url]"));
        assert!(script.contains("[data-rp-markdown-source]"));
        assert!(script.contains("[data-rp-markdown-copy-trigger]"));
        assert!(script.contains("[data-rp-markdown-copy-menu]"));
        assert!(script.contains("RustPress copy Markdown failed"));
        assert!(script.contains("RustPress copy Markdown URL failed"));
        assert!(script.contains(".rp-code-content"));
        assert!(script.contains("openMarkdownCopyMenu"));
        assert!(script.contains("closeMarkdownCopyMenu"));
        assert!(script.contains(r#"event.key === "Escape""#));
        assert!(script.contains("new URL(sourceUrl, location.href).href"));
        assert!(script.contains("navigator.clipboard.writeText"));
        assert!(script.contains("fallbackCopyText"));
        assert!(script.contains("document.execCommand(\"copy\")"));
        assert!(script.contains("showMarkdownCopied"));
        assert!(script.contains("1500"));
    }

    #[test]
    fn omits_github_link_without_theme_url() {
        let mut site = site();
        site.theme.github_url.clear();

        let html = render_page(
            &site,
            &PageRender {
                title: "Home".to_string(),
                route: "/".to_string(),
                html: "<h1>Home</h1>".to_string(),
                markdown_source: "# Home\n".to_string(),
                markdown_source_url: "/index.md.txt".to_string(),
                headings: vec![],
                masked: false,
                search: true,
            },
        );

        assert!(!html.contains("rp-github-link"));
        assert!(!html.contains("GitHub repository"));
    }
}
