# Design

## Context

- `.github/workflows/ci.yml` has two jobs:
  - `check`: `cargo fmt --check` and `cargo clippy`.
  - `build`: a 3-target matrix (Linux x86_64, Linux i686 via `cross`, macOS arm64) that uploads workflow artifacts. It sets `OPENSSL_DIR` on macOS, but `Cargo.lock` contains no OpenSSL, native-tls, ring or aws-lc. The dependency tree is pure Rust over a Unix socket, so that step is dead and musl builds need no C TLS stack.
- `Cargo.toml` has `name = "rocker"` and `version = "0.1.0"`, a `[[bin]] name = "rocker"`, and no metadata. The code never reads `CARGO_PKG_*`, so renaming the package doesn't change behaviour.
- No GitHub Releases, tags or license exist. `ds82/homebrew-tap` doesn't exist. `rocker-tui` is unclaimed on crates.io (`rocker` is taken).
- `dist` (cargo-dist) v0.33.0 is the current stable release (2026-09-11).

## Goals / Non-Goals

**Goals:**
- One tag push produces a GitHub Release, a formula update and a crates.io version, with no manual steps after setup.
- Release config is declarative, and the workflow is regenerated from it rather than edited by hand.

**Non-Goals:**
- Signing or notarising the macOS binaries. Homebrew installs don't hit Gatekeeper's quarantine; direct downloads may need `xattr -d com.apple.quarantine`, which `RELEASING.md` notes.
- Shell or PowerShell installer scripts from dist. They're easy to add later.
- Building on PRs. The release workflow only runs dist's `plan` step on PRs.

## Decisions

### dist generates the release workflow

The config lives in `dist-workspace.toml` and the generated `.github/workflows/release.yml` is committed. The version is pinned with `cargo-dist-version = "0.33.0"`, so `dist generate` is reproducible and CI checks that the workflow matches the config.

Planned config:

```toml
[workspace]
members = ["cargo:."]

[dist]
cargo-dist-version = "0.33.0"
ci = "github"
installers = ["homebrew"]
tap = "ds82/homebrew-tap"
formula = "rocker"
publish-jobs = ["homebrew", "./publish-crates"]
targets = [
  "aarch64-apple-darwin",
  "x86_64-apple-darwin",
  "aarch64-unknown-linux-musl",
  "x86_64-unknown-linux-musl",
]
include = ["LICENSE-MIT", "LICENSE-APACHE"]
pr-run-mode = "plan"
```

dist already ensures that the pushed tag matches the package version: its `plan` job fails on a mismatch before anything is built or published, which covers the spec's mismatched-tag scenario. dist also lays out the jobs as plan → build → host (upload to the GitHub Release) → publish jobs → announce. Publish jobs only run after the binaries exist, and a failed publish job can be re-run on its own.

*Alternative considered:* extending `ci.yml` by hand. It's full control, but means maintaining matrix, checksum, upload and formula-templating YAML ourselves. dist also names archives so that binstall needs no extra metadata (see below).

### crates.io publish as a custom dist publish job

dist doesn't publish to crates.io itself. A small reusable workflow, `.github/workflows/publish-crates.yml` (`on: workflow_call`), checks out the tag and runs `cargo publish --locked` with `CARGO_REGISTRY_TOKEN`. dist calls it through `publish-jobs = ["./publish-crates"]`, after the GitHub Release assets are uploaded. So when binstall sees a new crate version, the binaries for it already exist.

*Alternative considered:* running `cargo publish` by hand after each release. It's easy to forget, and then binstall users and Homebrew users end up on different versions, which breaks the "install paths agree" requirement.

### Crate renamed, binary name kept

The package becomes `rocker-tui`, and `[[bin]] name = "rocker"` stays. dist names archives after the package (`rocker-tui-<target>.tar.xz`, containing `rocker-tui-<target>/rocker`), and `formula = "rocker"` overrides the Homebrew formula name. cargo-binstall's default URL template, `{repo}/releases/download/v{version}/{name}-{target}.tar.xz`, and its default binary path, `{name}-{target}/{bin}`, match this layout exactly. No `[package.metadata.binstall]` is needed, only `repository`.

### Targets and musl

Linux uses `*-unknown-linux-musl`, so a single static binary runs on glibc and musl distributions alike. That's possible because nothing links C libraries. dist builds `aarch64-unknown-linux-musl` on GitHub's arm64 Linux runners, or with cargo-zigbuild when it selects that. Either way, `dist plan` shows the chosen runner before the first release.

### CI cleanup

The `build` job is removed from `ci.yml`, along with its dead OpenSSL step and the `cross` setup. The release workflow covers building, and `check` keeps linting every push.

### One-time setup lives outside the repo

These steps are done by the repo owner, not by code, and `RELEASING.md` lists them as a checklist:
- create the `ds82/homebrew-tap` repo
- create a fine-grained token with Contents write access to that repo and store it as `HOMEBREW_TAP_TOKEN` in `ds82/rocker`
- create a crates.io token with publish scope and store it as `CARGO_REGISTRY_TOKEN`

## Risks / Trade-offs

- [The first `cargo publish` permanently claims `rocker-tui` and its version. crates.io versions can be yanked but never deleted.] → The first release is only tagged after `dist plan` and a PR plan run pass, and with explicit go-ahead from the maintainer.
- [dist's arm64 musl build may need a different runner or zigbuild than expected] → Verify with `dist plan` locally and on a PR before tagging; fall back to dropping `aarch64-unknown-linux-musl` only with the maintainer's agreement.
- [The CI artifacts `rocker-linux-*` and `rocker-macos-*` disappear] → Nobody downloads them outside the maintainer, and releases replace them. This is noted as BREAKING in the proposal.
- [The `formula = "rocker"` name might clash with a homebrew-core formula of the same name for unqualified `brew install rocker`] → The docs always use the qualified `ds82/tap/rocker`.
- [The macOS quarantine flag on directly downloaded binaries] → Documented; Homebrew and binstall installs are unaffected.

## Migration Plan

1. Merge the change; nothing is released yet.
2. The maintainer completes the one-time setup in `RELEASING.md`.
3. Tag `v0.1.0`, the current version, as the first release.

Rollback: delete the GitHub Release and tag, revert the formula commit in the tap, and yank the crate version. A yanked version stays visible but won't be resolved by new installs.
