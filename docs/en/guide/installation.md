---
title: Installation
layout: doc
sidebar: true
search: true
access: public
---

# Installation

The RustPress CLI crate is named `rust-press`, and it installs a `rust-press` binary.

The repository is prepared for crates.io publication, but `cargo install rust-press` works only after the crate has been published. Until then, install from a source checkout or Git.

## Source Checkout

Install the CLI from a local checkout:

```bash
git clone https://github.com/ZenithInc/rust-press.git
cd rust-press
cargo install --path crates/rust-press
```

Rust 1.93 or newer is required.

## Git

Install the latest commit from GitHub:

```bash
cargo install --git https://github.com/ZenithInc/rust-press rust-press
```

Install a specific release tag by replacing the example version with a tag that exists:

```bash
cargo install --git https://github.com/ZenithInc/rust-press --tag v0.1.2 rust-press
```

## crates.io

After the crate is published to crates.io, install it with Cargo:

```bash
cargo install rust-press
rust-press --version
```

## Prebuilt Binaries

After release tags are published, GitHub Releases provide prebuilt archives for Linux, macOS, and Windows. Download the archive for your platform, extract it, and place `rust-press` on `PATH`, or run it from the extracted directory:

```bash
rust-press --help
```

Each release archive is published with a SHA256 checksum file.

## Updating

After the crate is published, update to the latest Cargo release:

```bash
cargo install rust-press --force
```
