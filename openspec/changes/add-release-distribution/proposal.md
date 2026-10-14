# Proposal

## Why

The only way to install rocker today is to clone the repo and run `cargo install --path .`, which needs a Rust toolchain and a full compile. CI builds binaries, but only as short-lived workflow artifacts; there are no versioned releases to download. Homebrew and `cargo binstall` are the two install paths the target audience (terminal and Neovim users on macOS and Linux) expects, and both need versioned, prebuilt release assets.

## What Changes

- **Automatic releases from `main`**: every push to `main` that adds commits since the last release cuts a new version. git-cliff derives the version from the Conventional Commit messages (`feat` and breaking changes bump the minor while rocker is 0.x, everything else bumps the patch). A workflow commits the version to `Cargo.toml` and `CHANGELOG.md`, tags it `vX.Y.Z`, and starts the release. Nobody bumps or tags by hand; a manual run can override the version, e.g. for 1.0.0.
- **Versioned GitHub Releases**: each release builds rocker for four targets and publishes a GitHub Release with an archive per target and SHA-256 checksums. Releases are built by `dist` (cargo-dist), which generates the release workflow.
- **Changelog**: git-cliff maintains `CHANGELOG.md`, grouped by commit type, and each GitHub Release uses that version's section as its notes.
- **Targets**: macOS x86_64 and arm64, and Linux x86_64 and arm64 as static musl binaries. **BREAKING** for anyone using the CI artifacts: Linux i686 is dropped, and the build job in `ci.yml` is removed in favour of the release workflow.
- **Homebrew**: each release updates a formula named `rocker` in a new tap repo `ds82/homebrew-tap`, so `brew install ds82/tap/rocker` works on macOS and Linux.
- **cargo-binstall**: the crate is renamed to `rocker-tui`, because `rocker` is taken on crates.io. The binary stays `rocker`. Each release is published to crates.io, so `cargo binstall rocker-tui` downloads the prebuilt binary, and `cargo install rocker-tui` builds from source as a fallback.
- **License**: rocker is licensed `MIT OR Apache-2.0`, with `LICENSE-MIT` and `LICENSE-APACHE` files. crates.io requires a license and Homebrew expects one.
- **Package metadata**: `Cargo.toml` gains `description`, `repository`, `license`, `readme`, `keywords` and `categories`. binstall needs `repository` to find the release assets.
- **Docs**: the README install section lists Homebrew, cargo-binstall, `cargo install` and direct download. A `RELEASING.md` describes how to cut a release.

Not in scope:
- getting into homebrew-core
- Windows builds
- other package managers (AUR, Nix, apt)
- adding `cargo test` to CI

## Capabilities

### New Capabilities
- `release-distribution`: how rocker versions are published and installed. Covers automatic versioning and release triggering, the changelog, release assets, the Homebrew and cargo-binstall install paths, and version consistency between them.

### Modified Capabilities
<!-- none -->

## Impact

- `Cargo.toml`: package renamed to `rocker-tui`, metadata added, `dist` profile and config added. From now on its version is bumped by the release workflow, not by hand.
- New `dist-workspace.toml` and a generated `.github/workflows/release.yml`.
- New `cliff.toml`, `.github/workflows/auto-release.yml`, and a generated `CHANGELOG.md`.
- `main` receives bot commits `chore(release): vX.Y.Z`. Every push to `main` publishes a crates.io version, which can be yanked but never deleted.
- Commit messages now decide version numbers, so they must follow Conventional Commits.
- `.github/workflows/ci.yml`: the `build` job is removed; `check` (fmt and clippy) stays.
- New files: `LICENSE-MIT`, `LICENSE-APACHE`, `RELEASING.md`. `README.md` gets a new install section.
- **Outside this repo** (manual, by the repo owner):
  - create the `ds82/homebrew-tap` repo
  - add the `HOMEBREW_TAP_TOKEN` and `CARGO_REGISTRY_TOKEN` secrets
  - own the `rocker-tui` crate on crates.io
- The `AGENTS.md` lines about CI change.
- The `rocker-<target>` CI artifact names go away.
