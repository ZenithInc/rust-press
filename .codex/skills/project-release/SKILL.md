---
name: project-release
description: Release workflow for this RustPress repository. Use when the user says "发布", "发版", "release", "publish", "发布 crates.io", "触发 release/pages", or asks Codex to ship a new RustPress version through GitHub Actions, GitHub Pages, GitHub Releases, and crates.io.
---

# Project Release

## Overview

Execute the RustPress release process end to end. Do not stop at a plan when the user asks to publish; perform the checks, versioning, crates.io publishing, GitHub pushes, and CI monitoring unless a required credential or repository state blocks progress.

## Release Workflow

1. Inspect repository state.
   - Run `git status --short --branch`, `git log -5 --oneline --decorate`, and read `.github/workflows/release.yml`, `.github/workflows/pages.yml`, `Cargo.toml`, and crate manifests as needed.
   - Confirm `main` is the release branch and note the current tag/version.
   - If there are unrelated uncommitted changes, do not discard them. Either work with them if they are intended for the release, or ask before proceeding.

2. Decide the next version.
   - Check crates.io for the current published versions before publishing. Use `cargo search <crate> --limit 3` or the crates.io API with a User-Agent if needed.
   - If the workspace version is already published, bump to the next patch version unless the user specified a different version.
   - Update:
     - `[workspace.package].version` in `Cargo.toml`
     - internal dependency versions in `crates/rust-press/Cargo.toml`, `crates/rustpress-core/Cargo.toml`, `crates/rustpress-dev/Cargo.toml`, and `crates/rustpress-theme/Cargo.toml`
     - release tag example in `README.md` if it names the previous tag
   - Run `cargo check` to refresh `Cargo.lock` and validate dependency resolution.

3. Verify locally before publishing.
   - Run `cargo fmt`.
   - Run `cargo test`.
   - Run package-specific tests if the release includes targeted changes and they add useful signal.

4. Commit the release state.
   - `cargo publish --dry-run` requires a clean git working tree. Do not use `--allow-dirty` for release publishing.
   - Commit the version bump and release changes with a message like `Prepare vX.Y.Z release`.

5. Publish crates.io in dependency order.
   - Dry-run and publish each crate in this exact order:

```bash
cargo publish --dry-run -p rustpress-md
cargo publish -p rustpress-md
cargo publish --dry-run -p rustpress-search
cargo publish -p rustpress-search
cargo publish --dry-run -p rustpress-theme
cargo publish -p rustpress-theme
cargo publish --dry-run -p rustpress-core
cargo publish -p rustpress-core
cargo publish --dry-run -p rustpress-dev
cargo publish -p rustpress-dev
cargo publish --dry-run -p rust-press
cargo publish -p rust-press
```

   - If a dry-run fails because an internal dependency version is not on crates.io yet, publish the earlier dependency crate first, wait for it to become available, then retry.
   - After publishing, confirm the public index shows the new version with `cargo search` for all six crates.

6. Trigger Pages and Release CI.
   - Push `main` to trigger `.github/workflows/pages.yml`.
   - Create and push tag `vX.Y.Z` to trigger `.github/workflows/release.yml`.

```bash
git push origin main
git tag vX.Y.Z
git push origin vX.Y.Z
```

   - If the tag already exists locally or remotely, stop and inspect. Do not force-update release tags unless the user explicitly approves.

7. Monitor CI to completion.
   - Confirm Pages and Release runs started:

```bash
gh run list --repo ZenithInc/rust-press --workflow "Deploy GitHub Pages" --limit 3
gh run list --repo ZenithInc/rust-press --workflow Release --limit 5
```

   - Watch both relevant runs with `gh run watch <run-id> --repo ZenithInc/rust-press --exit-status --interval 10`.
   - If a run fails, inspect logs before reporting. Fix, commit, push, and rerun or retag only when safe.

8. Final confirmation.
   - Confirm the GitHub Release exists and has 8 assets:

```bash
gh api repos/ZenithInc/rust-press/releases/tags/vX.Y.Z --jq '.html_url, (.assets | length), .assets[].name'
```

   - Confirm Pages URL/status:

```bash
gh api repos/ZenithInc/rust-press/pages --jq '.html_url, .status'
```

   - Confirm `git status --short --branch` is clean and synced.
   - Report the tag, commit, Pages URL, GitHub Release URL, crates.io versions, and any non-blocking warnings.

## Project Facts

- GitHub repository: `ZenithInc/rust-press`
- Pages workflow: `.github/workflows/pages.yml`, triggered by push to `main` or manual dispatch
- Release workflow: `.github/workflows/release.yml`, triggered by push tags matching `v*`
- crates.io install package: `rust-press`
- crates.io crate order: `rustpress-md`, `rustpress-search`, `rustpress-theme`, `rustpress-core`, `rustpress-dev`, `rust-press`
