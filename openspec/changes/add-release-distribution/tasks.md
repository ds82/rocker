# Tasks

## 1. License and package metadata

- [x] 1.1 Add `LICENSE-MIT` (copyright holder from the git author, year 2026) and `LICENSE-APACHE` (the unmodified Apache-2.0 text). Verify that both files exist and that `LICENSE-APACHE` matches the official text, e.g. by diffing it against `https://www.apache.org/licenses/LICENSE-2.0.txt`.
- [x] 1.2 In `Cargo.toml`, rename the package to `rocker-tui`, keep `[[bin]] name = "rocker"`, and add `description`, `license = "MIT OR Apache-2.0"`, `repository = "https://github.com/ds82/rocker"`, `homepage`, `readme = "README.md"`, `keywords` and `categories` (`command-line-utilities`). Verify that `cargo build` produces `target/debug/rocker`, and that `cargo package --list --allow-dirty` includes `LICENSE-*`, `README.md` and `src/` but not `openspec/` or `.github/`. Add an `exclude` if needed.
- [x] 1.3 Run `cargo publish --dry-run --allow-dirty` and verify it succeeds, with no metadata warnings.

## 2. Versioning and changelog

- [x] 2.1 Add `cliff.toml` with the config from design.md: Conventional Commits, `filter_unconventional = false`, the commit groups, skipping `chore(release)`, `tag_pattern`, keep-a-changelog headings, and `[bump]` with `features_always_bump_minor = true`, `breaking_always_bump_major = false` and `initial_tag = "0.1.0"`. Verify:
  - in this repo (no tags), `git cliff --bumped-version` prints `v0.1.0`, and `git cliff --bump --unreleased` renders a `## [0.1.0]` section
  - in a scratch repo tagged `v0.2.3`, adding `feat:` gives `v0.3.0`, `fix:` gives `v0.2.4`, `feat!:` gives `v0.3.0`, `docs:` only gives `v0.2.4`, and a non-conventional message gives `v0.2.4`
  - in a scratch repo tagged `v1.2.3`, `feat!:` gives `v2.0.0`
  - a `chore(release): v0.2.4` commit doesn't appear in the rendered changelog
- [x] 2.2 Add `.github/workflows/auto-release.yml` as described in design.md: triggered by push to `main`, pull requests and `workflow_dispatch` with an optional `version` input. Set `concurrency` and `permissions` (`contents: write`, `actions: write`). Install git-cliff and cargo-edit with `taiki-e/install-action`. On push or dispatch, it checks out `main` with full history, stops when there are no commits since the latest tag, computes or takes the version, runs `cargo set-version` and `cargo update --workspace`, regenerates `CHANGELOG.md`, commits `chore(release): vX.Y.Z`, creates an annotated tag, runs `git push --atomic origin main vX.Y.Z` and then `gh workflow run release.yml -f tag=vX.Y.Z`. On pull requests it only writes the next version and a changelog preview to the job summary. Verify that the YAML parses, and that `actionlint` passes if it's available.
- [x] 2.3 Dry-run the push path locally in a scratch clone: run the workflow's shell steps up to `git push`, against a scratch bare remote. Verify that the commit touches only `Cargo.toml`, `Cargo.lock` and `CHANGELOG.md`, that the tag points at it, that `cargo publish --dry-run --locked` succeeds on it, and that running the steps again stops with "no commits since last tag".

## 3. dist release workflow

- [x] 3.1 Install `dist` 0.33.0 locally (`cargo binstall cargo-dist@0.33.0` or the official installer) and run `dist init` with the config from design.md, including `dispatch-releases = true`. Commit `dist-workspace.toml`, the `[profile.dist]` it adds to `Cargo.toml`, and the generated `.github/workflows/release.yml`. Verify that `dist generate --check` passes, and that `release.yml` triggers on `workflow_dispatch` with a `tag` input, not on a tag push.
- [x] 3.2 Add `.github/workflows/publish-crates.yml` (`on: workflow_call`, `secrets: inherit`): check out the tag, install stable Rust, and run `cargo publish --locked` with `CARGO_REGISTRY_TOKEN`. Reference it as `./publish-crates` in `publish-jobs`, then rerun `dist generate`. Verify that `dist generate --check` passes and that `release.yml` calls `publish-crates.yml` after the host job.
- [x] 3.3 Run `dist plan` and verify that it lists exactly the four targets, `rocker-tui-<target>.tar.xz` archives with checksums, the `rocker` Homebrew formula for `ds82/homebrew-tap`, and the custom publish job. Run `dist build` for the local target and verify that the archive contains `rocker`, `README.md`, `LICENSE-MIT` and `LICENSE-APACHE`, and that the extracted `rocker` runs.
- [x] 3.4 With the `CHANGELOG.md` from the 2.3 dry run in place, run `dist plan --tag v0.1.0 --output-format=json`. Verify that the announcement body contains the `0.1.0` changelog entries. Don't commit that `CHANGELOG.md`; the first release generates it.
- [x] 3.5 Check how dist builds `aarch64-unknown-linux-musl` (runner or zigbuild) from `dist plan` and `release.yml`. If it can't, pause and ask the maintainer whether to drop the target rather than changing the spec silently.

## 4. CI cleanup

- [x] 4.1 Remove the `build` job (matrix, `cross`, OpenSSL step, artifact upload) from `.github/workflows/ci.yml` and keep `check`. Verify that the YAML parses (`python3 -c "import yaml;yaml.safe_load(open('.github/workflows/ci.yml'))"`) and that `check` still runs fmt and clippy.
- [ ] 4.2 Push the branch and open a PR. Verify all of these on the PR:
  - CI `check` passes
  - the release workflow's `plan` job runs green, with no build or publish
  - the `auto-release` job summary shows `v0.1.0` and a changelog preview
  - nothing was committed, tagged or published

## 5. Documentation

- [x] 5.1 Replace the README install section with Homebrew (`brew install ds82/tap/rocker`), cargo-binstall (`cargo binstall rocker-tui`), `cargo install rocker-tui` and download from GitHub Releases (including the macOS `xattr` note). Add a License section and link `CHANGELOG.md`. Verify that every command in the section matches the names in `dist-workspace.toml` and `Cargo.toml`.
- [x] 5.2 Write `RELEASING.md` with:
  - the one-time setup checklist: tap repo, `HOMEBREW_TAP_TOKEN`, `CARGO_REGISTRY_TOKEN`, claiming `rocker-tui`
  - how releases happen: every push to `main` releases; the commit types decide the version; no manual bump or tag
  - overriding the version: `gh workflow run auto-release.yml -f version=X.Y.Z`
  - pausing and resuming: `gh workflow disable` / `gh workflow enable auto-release.yml`
  - re-running a failed release or publish job: `gh workflow run release.yml -f tag=vX.Y.Z`, or re-run the failed jobs
  - what to do if branch protection or a ruleset blocks the bot's push
  - the rollback steps from design.md

  Verify that each secret, workflow and input name matches what `auto-release.yml`, `release.yml` and `publish-crates.yml` reference.
- [x] 5.3 Update `AGENTS.md`:
  - Verify section: CI runs `check` only.
  - Commit messages must follow Conventional Commits, because they decide the next version. Mark breaking changes with `!`. Never bump `Cargo.toml`'s version or edit `CHANGELOG.md` by hand.
  - Every push to `main` publishes a release.
  - Add a pointer to `RELEASING.md`.
  - Note that `release.yml` is generated: edit `dist-workspace.toml` and run `dist generate`; never edit the YAML by hand.

  Verify by reading the file.

## 6. First release (needs the maintainer)

- [ ] 6.1 Pause and ask the maintainer to complete the one-time setup in `RELEASING.md` **before merging**. Optionally, offer a guided setup wizard. Verify that `gh secret list -R ds82/rocker` shows both secrets and that `gh repo view ds82/homebrew-tap` succeeds.
- [ ] 6.2 With the maintainer's explicit go-ahead (merging releases immediately, and publishing to crates.io can't be undone), merge the PR. Verify all of these:
  - `auto-release` pushed `chore(release): v0.1.0` with `CHANGELOG.md` and the tag `v0.1.0`
  - `release.yml` was dispatched for `v0.1.0` and completed
  - the GitHub Release `v0.1.0` has four archives with checksums, and its notes match the changelog section
- [ ] 6.3 Verify the install paths against the real release:
  - `brew install ds82/tap/rocker` installs a `rocker` that starts, and `brew info` shows the license
  - `cargo binstall --no-confirm rocker-tui` reports downloading the prebuilt archive, not compiling
  - crates.io shows `rocker-tui` 0.1.0 with license `MIT OR Apache-2.0`
- [ ] 6.4 Verify that the release commit didn't start another `auto-release` run, e.g. with `gh run list -w auto-release.yml`. The next real push to `main` will release `v0.1.1` or `v0.2.0`.
