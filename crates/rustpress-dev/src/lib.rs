use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::sync::mpsc;
use std::sync::{
    atomic::{AtomicU64, Ordering},
    Arc,
};
use std::thread;
use std::time::{Duration, Instant};

use anyhow::Result;
use notify::{Config as NotifyConfig, EventKind, RecommendedWatcher, RecursiveMode, Watcher};
use rustpress_core::{build_site, BuildOptions, Config};
use tiny_http::{Header, Response, Server, StatusCode};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ServeOptions {
    pub config_path: PathBuf,
    pub host: String,
    pub port: u16,
}

pub fn serve_preview(options: ServeOptions) -> Result<()> {
    let config = Config::load(&options.config_path)?;
    let root = config_root(&options.config_path).join(config.out_dir);
    serve_dir(&root, &options.host, options.port, None)
}

pub fn serve_dev(options: ServeOptions) -> Result<()> {
    let build_options = BuildOptions::new(options.config_path.clone());
    let result = build_site(build_options.clone())?;
    let root = result.out_dir;
    let config_path = options.config_path.clone();
    let refresh_version = Arc::new(AtomicU64::new(1));

    let (tx, rx) = mpsc::channel();
    let mut watcher = RecommendedWatcher::new(tx, NotifyConfig::default())?;
    let config = Config::load(&config_path)?;
    let project_root = config_root(&config_path);
    watcher.watch(
        &project_root.join(&config.src_dir),
        RecursiveMode::Recursive,
    )?;
    watcher.watch(&config_path, RecursiveMode::NonRecursive)?;

    let rebuild_config_path = config_path.clone();
    let rebuild_refresh_version = Arc::clone(&refresh_version);
    thread::spawn(move || {
        let mut last = Instant::now() - Duration::from_secs(2);
        while let Ok(event) = rx.recv() {
            let Ok(event) = event else { continue };
            if !matches!(
                event.kind,
                EventKind::Create(_) | EventKind::Modify(_) | EventKind::Remove(_)
            ) {
                continue;
            }
            if last.elapsed() < Duration::from_millis(250) {
                continue;
            }
            last = Instant::now();
            match build_site(BuildOptions::new(rebuild_config_path.clone())) {
                Ok(result) => {
                    rebuild_refresh_version.fetch_add(1, Ordering::SeqCst);
                    eprintln!("rebuilt {} page(s)", result.page_count);
                }
                Err(err) => eprintln!("rebuild failed: {err:?}"),
            }
        }
    });

    println!(
        "RustPress dev server: http://{}:{}/",
        options.host, options.port
    );
    serve_dir(&root, &options.host, options.port, Some(refresh_version))
}

fn serve_dir(
    root: &Path,
    host: &str,
    port: u16,
    refresh_version: Option<Arc<AtomicU64>>,
) -> Result<()> {
    let address = format!("{host}:{port}");
    let server = Server::http(&address).map_err(|err| anyhow::anyhow!("{err}"))?;
    println!("Serving {} at http://{address}/", root.display());

    for request in server.incoming_requests() {
        let url = request.url().split('?').next().unwrap_or("/");
        if url == "/__rustpress/version" {
            let version = refresh_version
                .as_ref()
                .map(|version| version.load(Ordering::SeqCst))
                .unwrap_or(0);
            let mut response = Response::from_string(version.to_string());
            if let Some(header) = header("Cache-Control", "no-store") {
                response = response.with_header(header);
            }
            if let Some(header) = header("Content-Type", "text/plain; charset=utf-8") {
                response = response.with_header(header);
            }
            request.respond(response)?;
            continue;
        }

        let path = resolve_path(root, url);
        let response = match File::open(&path) {
            Ok(mut file) => {
                let mut bytes = Vec::new();
                file.read_to_end(&mut bytes)?;
                if refresh_version.is_some()
                    && path.extension().and_then(|value| value.to_str()) == Some("html")
                {
                    bytes = inject_live_reload(bytes);
                }
                let mut response = Response::from_data(bytes);
                if let Some(header) = content_type_header(&path) {
                    response = response.with_header(header);
                }
                response
            }
            Err(_) => Response::from_string("Not Found").with_status_code(StatusCode(404)),
        };
        request.respond(response)?;
    }
    Ok(())
}

const LIVE_RELOAD_SCRIPT: &str = r#"<script>
(() => {
  let current = null;
  async function check() {
    try {
      const response = await fetch("/__rustpress/version", { cache: "no-store" });
      const next = await response.text();
      if (current === null) current = next;
      else if (next !== current) {
        window.dispatchEvent(new CustomEvent("rustpress:refresh"));
        location.reload();
      }
    } catch (_) {}
  }
  setInterval(check, 700);
  check();
})();
</script>"#;

fn inject_live_reload(bytes: Vec<u8>) -> Vec<u8> {
    let mut html = match String::from_utf8(bytes) {
        Ok(html) => html,
        Err(err) => return err.into_bytes(),
    };

    if let Some(index) = html.rfind("</body>") {
        html.insert_str(index, LIVE_RELOAD_SCRIPT);
    } else {
        html.push_str(LIVE_RELOAD_SCRIPT);
    }
    html.into_bytes()
}

fn resolve_path(root: &Path, url: &str) -> PathBuf {
    let clean = url.trim_start_matches('/');
    let candidate = root.join(clean);
    if url.ends_with('/') || clean.is_empty() {
        return candidate.join("index.html");
    }
    if candidate.is_dir() {
        candidate.join("index.html")
    } else {
        candidate
    }
}

fn content_type_header(path: &Path) -> Option<Header> {
    let content_type = match path.extension().and_then(|value| value.to_str()) {
        Some("html") => "text/html; charset=utf-8",
        Some("css") => "text/css; charset=utf-8",
        Some("js") => "text/javascript; charset=utf-8",
        Some("json") => "application/json; charset=utf-8",
        Some("txt") => "text/plain; charset=utf-8",
        Some("wasm") => "application/wasm",
        Some("br") => "application/octet-stream",
        Some("svg") => "image/svg+xml",
        Some("png") => "image/png",
        Some("jpg") | Some("jpeg") => "image/jpeg",
        Some("webp") => "image/webp",
        _ => return None,
    };
    header("Content-Type", content_type)
}

fn header(name: &str, value: &str) -> Option<Header> {
    Header::from_bytes(name, value).ok()
}

fn config_root(config_path: &Path) -> PathBuf {
    config_path
        .parent()
        .map(Path::to_path_buf)
        .unwrap_or_else(|| PathBuf::from("."))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn resolves_directory_routes() {
        let root = Path::new("/tmp/site");

        assert_eq!(resolve_path(root, "/"), Path::new("/tmp/site/index.html"));
        assert_eq!(
            resolve_path(root, "/guide/"),
            Path::new("/tmp/site/guide/index.html")
        );
        assert_eq!(
            resolve_path(root, "/assets/rustpress.css"),
            Path::new("/tmp/site/assets/rustpress.css")
        );
    }

    #[test]
    fn injects_live_reload_before_body_close() {
        let html = inject_live_reload(b"<html><body>Docs</body></html>".to_vec());
        let html = String::from_utf8(html).unwrap();

        assert!(html.contains("/__rustpress/version"));
        assert!(html.contains("rustpress:refresh"));
        assert!(html.contains("</script></body>"));
    }

    #[test]
    fn markdown_text_files_are_served_as_plain_text() {
        let header = content_type_header(Path::new("/tmp/site/guide/index.md.txt")).unwrap();

        assert_eq!(header.field.as_str(), "Content-Type");
        assert_eq!(header.value.as_str(), "text/plain; charset=utf-8");
    }
}
