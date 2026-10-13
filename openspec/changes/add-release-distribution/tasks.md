# Tasks

## 1. License and package metadata

- [ ] 1.1 Add `LICENSE-MIT` (copyright holder from the git author, year 2026) and `LICENSE-APACHE` (the unmodified Apache-2.0 text). Verify that both files exist and that `LICENSE-APACHE` matches the official text, e.g. by diffing it against `https://www.apache.org/licenses/LICENSE-2.0.txt`.
- [ ] 1.2 In `Cargo.toml`, rename the package to `rocker-tui`, keep `[[bin]] name = "rocker"`, and add `description`, `license = "MIT OR Apache-2.0"`, `repository = "https://github.com/ds82/rocker"`, `homepage`, `readme = "README.md"`, `keywords` and `categories` (`command-line-utilities`). Verify that `cargo build` produces `target/debug/rocker`, and that `cargo package --list --allow-dirty` includes `LICENSE-*`, `README.md` and `src/` but not `openspec/` or `.github/`. Add an `exclude` if needed.
- [ ] 1.3 Run `cargo publish --dry-run --allow-dirty` and verify it succeeds, with no metadata warnings.

## 2. dist release workflow

- [ ] 2.1 Install `dist` 0.33.0 locally (`cargo binstall cargo-dist@0.33.0` or the official installer) and run `dist init` with the config from design.md. Commit `dist-workspace.toml`, the `[profile.dist]` it adds to `Cargo.toml`, and the generated `.github/workflows/release.yml`. Verify that `dist generate --check` passes.
- [ ] 2.2 Add `.github/workflows/publish-crates.yml` (`on: workflow_call`, `secrets: inherit`): check out the tag, install stable Rust, and run `cargo publish --locked` with `CARGO_REGISTRY_TOKEN`. Reference it as `./publish-crates` in `publish-jobs`, then rerun `dist generate`. Verify that `dist generate --check` passes and that `release.yml` calls `publish-crates.yml` after the host job.
- [ ] 2.3 Run `dist plan` and verify that it lists exactly the four targets, `rocker-tui-<target>.tar.xz` archives with checksums, the `rocker` Homebrew formula for `ds82/homebrew-tap`, and the custom publish job. Run `dist build` for the local target and verify that the archive contains `rocker`, `README.md`, `LICENSE-MIT` and `LICENSE-APACHE`, and that the extracted `rocker` runs.
- [ ] 2.4 Check how dist builds `aarch64-unknown-linux-musl` (runner or zigbuild) from `dist plan` and `release.yml`. If it can't, pause and ask the maintainer whether to drop the target rather than changing the spec silently.

## 3. CI cleanup

- [ ] 3.1 Remove the `build` job (matrix, `cross`, OpenSSL step, artifact upload) from `.github/workflows/ci.yml` and keep `check`. Verify that the YAML parses (`python3 -c "import yaml;yaml.safe_load(open('.github/workflows/ci.yml'))"`) and that `check` still runs fmt and clippy.
- [ ] 3.2 Push the branch and open a PR. Verify that CI `check` passes and that the release workflow's `plan` job runs green on the PR, with no build or publish.

## 4. Documentation

- [ ] 4.1 Replace the README install section with Homebrew (`brew install ds82/tap/rocker`), cargo-binstall (`cargo binstall rocker-tui`), `cargo install rocker-tui` and download from GitHub Releases (including the macOS `xattr` note). Add a License section. Verify that every command in the section matches the names in `dist-workspace.toml` and `Cargo.toml`.
- [ ] 4.2 Write `RELEASING.md`: the one-time setup checklist (tap repo, `HOMEBREW_TAP_TOKEN`, `CARGO_REGISTRY_TOKEN`, claiming `rocker-tui`), the release steps (bump version, commit, tag `vX.Y.Z`, push tag), how to re-run a failed publish job, and the rollback steps from design.md. Verify that each secret name matches what `release.yml` and `publish-crates.yml` reference.
- [ ] 4.3 Update `AGENTS.md`:
  - Verify section: CI runs `check` only.
  - Add a pointer to `RELEASING.md`.
  - Note that `release.yml` is generated: edit `dist-workspace.toml` and run `dist generate`; never edit the YAML by hand.

  Verify by reading the file.

## 5. First release (needs the maintainer)

- [ ] 5.1 Pause and ask the maintainer to complete the one-time setup in `RELEASING.md`. Optionally, offer a guided setup wizard. Verify that `gh secret list -R ds82/rocker` shows both secrets and that `gh repo view ds82/homebrew-tap` succeeds.
- [ ] 5.2 With the maintainer's explicit go-ahead (publishing to crates.io can't be undone), merge the PR, tag `v0.1.0` and push the tag. Verify that the release workflow completes and that the GitHub Release `v0.1.0` has four archives with checksums.
- [ ] 5.3 Verify the install paths against the real release:
  - `brew install ds82/tap/rocker` installs a `rocker` that starts, and `brew info` shows the license
  - `cargo binstall --no-confirm rocker-tui` reports downloading the prebuilt archive, not compiling
  - crates.io shows `rocker-tui` 0.1.0 with license `MIT OR Apache-2.0`
