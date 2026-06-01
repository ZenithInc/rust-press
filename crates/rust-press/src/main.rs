use std::path::PathBuf;

use anyhow::Result;
use clap::{Parser, Subcommand};
use rustpress_core::{build_site, init_project, BuildOptions};
use rustpress_dev::{serve_dev, serve_preview, ServeOptions};

#[derive(Debug, Parser)]
#[command(
    name = "rust-press",
    version,
    about = "Rust-first static documentation generator CLI"
)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    /// Create a new RustPress documentation project.
    Init {
        /// Target directory. Defaults to the current directory.
        #[arg(default_value = ".")]
        dir: PathBuf,
    },
    /// Build the site, serve it, and rebuild when Markdown or config files change.
    Dev {
        #[arg(long, default_value = "rustpress.toml")]
        config: PathBuf,
        #[arg(long, default_value = "127.0.0.1")]
        host: String,
        #[arg(long, default_value_t = 5177)]
        port: u16,
    },
    /// Build the static site.
    Build {
        #[arg(long, default_value = "rustpress.toml")]
        config: PathBuf,
    },
    /// Serve the built static site.
    Preview {
        #[arg(long, default_value = "rustpress.toml")]
        config: PathBuf,
        #[arg(long, default_value = "127.0.0.1")]
        host: String,
        #[arg(long, default_value_t = 4177)]
        port: u16,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Command::Init { dir } => init_project(&dir),
        Command::Build { config } => {
            let result = build_site(BuildOptions::new(config))?;
            println!(
                "Built {} page(s) into {}",
                result.page_count,
                result.out_dir.display()
            );
            Ok(())
        }
        Command::Dev { config, host, port } => serve_dev(ServeOptions {
            config_path: config,
            host,
            port,
        }),
        Command::Preview { config, host, port } => serve_preview(ServeOptions {
            config_path: config,
            host,
            port,
        }),
    }
}
