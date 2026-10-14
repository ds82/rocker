# Spec Delta

## Purpose

Defines how rocker versions are released and how users install them: automatic, commit-driven versioning on every push to `main`, a generated changelog, versioned GitHub Releases with prebuilt binaries, a Homebrew tap, and crates.io-backed installs via cargo-binstall.

## ADDED Requirements

### Requirement: Releases are cut automatically from main
Each push to `main` that adds commits since the latest `vX.Y.Z` tag SHALL release a new version without manual steps. The version SHALL be derived from the Conventional Commit messages since that tag:
- While the major version is 0: a breaking change (`!` after the type, or a `BREAKING CHANGE:` footer) or a `feat` bumps the minor version; any other commit, including one that doesn't follow Conventional Commits, bumps the patch version.
- From 1.0.0 on: a breaking change bumps the major version, `feat` the minor, anything else the patch.

The release SHALL commit the new version to `Cargo.toml`, `Cargo.lock` and `CHANGELOG.md` on `main` as `chore(release): vX.Y.Z`, tag that commit `vX.Y.Z`, and publish the release from the tag.

#### Scenario: Feature
- **WHEN** the latest tag is `v0.2.3` and a push adds `feat: add volume prune`
- **THEN** version 0.3.0 is released, and `Cargo.toml` on `main` has `version = "0.3.0"`

#### Scenario: Fix
- **WHEN** the latest tag is `v0.2.3` and a push adds `fix: keep cursor after refresh`
- **THEN** version 0.2.4 is released

#### Scenario: Breaking change while 0.x
- **WHEN** the latest tag is `v0.2.3` and a push adds `feat!: rename config keys`
- **THEN** version 0.3.0 is released, not 1.0.0

#### Scenario: Housekeeping only
- **WHEN** the latest tag is `v0.2.3` and a push adds only `docs: fix typo`
- **THEN** version 0.2.4 is released

#### Scenario: Release commit
- **WHEN** the release automation pushes `chore(release): v0.2.4` and its tag to `main`
- **THEN** no further release is started

#### Scenario: First release
- **WHEN** no `vX.Y.Z` tag exists and `Cargo.toml` has `version = "0.1.0"`
- **THEN** version 0.1.0 is released

#### Scenario: Manual version override
- **WHEN** the maintainer manually runs the release automation with version `1.0.0`
- **THEN** version 1.0.0 is released, whatever the commits since the last tag

#### Scenario: Pull request
- **WHEN** a pull request against `main` is opened or updated
- **THEN** the next version and a changelog preview are shown in the workflow run, and nothing is committed, tagged or published

### Requirement: Release version matches the package version
A release SHALL fail without publishing anything when the tag's version differs from the package version in `Cargo.toml` at that tag.

#### Scenario: Mismatched tag
- **WHEN** a release is started for the tag `v0.3.0` and `Cargo.toml` at that tag has `version = "0.2.0"`
- **THEN** the release fails before publishing, and no GitHub Release, formula update or crates.io version is created

### Requirement: Changelog and release notes
`CHANGELOG.md` SHALL be generated from the commit history. It SHALL have one section per released version, with entries grouped by commit type, and SHALL NOT list `chore(release)` commits. Each GitHub Release SHALL use its version's changelog section as its release notes.

#### Scenario: Release notes
- **WHEN** version 0.3.0 is released with one `feat` and one `fix` commit since `v0.2.3`
- **THEN** `CHANGELOG.md` has a `0.3.0` section with both entries under separate feature and bug-fix groups, and the GitHub Release `v0.3.0` shows the same entries

### Requirement: GitHub Release with prebuilt binaries
Each release SHALL publish a GitHub Release named after the tag. It SHALL contain one archive per supported target, and each archive SHALL contain the `rocker` binary, the README and both license files. The Release SHALL include a SHA-256 checksum for every archive.

#### Scenario: Release assets
- **WHEN** the release for `v0.2.0` completes
- **THEN** the GitHub Release `v0.2.0` has archives for `x86_64-apple-darwin`, `aarch64-apple-darwin`, `x86_64-unknown-linux-musl` and `aarch64-unknown-linux-musl`, each with a checksum

#### Scenario: Linux binary runs without system libraries
- **WHEN** the Linux x86_64 binary is run on a distribution with a different libc version than the build machine, or on Alpine
- **THEN** it starts, because it is statically linked

### Requirement: Install via Homebrew
Each release SHALL update the formula `rocker` in the tap `ds82/homebrew-tap` to the new version and checksums. The formula SHALL install the prebuilt binary as `rocker`, on macOS (Intel and Apple Silicon) and on Linux (x86_64 and arm64).

#### Scenario: Fresh install
- **WHEN** a user runs `brew install ds82/tap/rocker`
- **THEN** the latest released `rocker` binary is installed without compiling, and `rocker` is on the user's PATH

#### Scenario: Upgrade
- **WHEN** a new version is released and the user runs `brew upgrade rocker`
- **THEN** the new version is installed

### Requirement: Install via cargo-binstall
Each release SHALL publish the crate `rocker-tui` to crates.io at the released version. `cargo binstall rocker-tui` SHALL install that version's prebuilt binary from the GitHub Release as `rocker`.

#### Scenario: Prebuilt install
- **WHEN** a user on a supported target runs `cargo binstall rocker-tui`
- **THEN** cargo-binstall downloads the matching GitHub Release archive and installs `rocker` without compiling

#### Scenario: Source install fallback
- **WHEN** a user runs `cargo install rocker-tui`
- **THEN** cargo compiles and installs the binary `rocker`

### Requirement: Install paths agree on the version
For a given release, the GitHub Release, the Homebrew formula and the crates.io version SHALL all carry the same version number. A release SHALL NOT be considered complete until all three are published.

#### Scenario: Release completes
- **WHEN** the release for `v0.2.0` finishes successfully
- **THEN** the GitHub Release, the `rocker` formula and the crate `rocker-tui` all report version 0.2.0

#### Scenario: Publishing step fails
- **WHEN** publishing to Homebrew or crates.io fails during a release
- **THEN** the release workflow fails visibly, so the maintainer can re-run the failed job without rebuilding the binaries

### Requirement: License declared
rocker SHALL be distributed under `MIT OR Apache-2.0`. Both license texts SHALL be in the repository and in every release archive, and the license SHALL be declared in the crate metadata and the Homebrew formula.

#### Scenario: License visible to package managers
- **WHEN** a user views the crate on crates.io or runs `brew info ds82/tap/rocker`
- **THEN** the license is shown as `MIT OR Apache-2.0`
