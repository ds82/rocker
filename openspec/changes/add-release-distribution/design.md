# Design

## Context

- `.github/workflows/ci.yml` has two jobs:
  - `check`: `cargo fmt --check` and `cargo clippy`.
  - `build`: a 3-target matrix (Linux x86_64, Linux i686 via `cross`, macOS arm64) that uploads workflow artifacts. It sets `OPENSSL_DIR` on macOS, but `Cargo.lock` contains no OpenSSL, native-tls, ring or aws-lc. The dependency tree is pure Rust over a Unix socket, so that step is dead and musl builds need no C TLS stack.
- `Cargo.toml` has `name = "rocker"` and `version = "0.1.0"`, a `[[bin]] name = "rocker"`, and no metadata. The code never reads `CARGO_PKG_*`, so renaming the package doesn't change behaviour.
- No GitHub Releases, tags or license exist. `ds82/homebrew-tap` doesn't exist. `rocker-tui` is unclaimed on crates.io (`rocker` is taken).
- `dist` (cargo-dist) v0.33.0 is the current stable release (2026-09-11).
- Commit messages already follow Conventional Commits (`feat`, `fix`, `docs`, `chore`), with an occasional slip such as `doc:`. There's no `cliff.toml` or `CHANGELOG.md`.
- `main` has no branch protection, and the `merge-to-main` ruleset is disabled, so the workflow's `GITHUB_TOKEN` can push to it.

## Goals / Non-Goals

**Goals:**
- Every push to `main` produces a new version, a GitHub Release, a formula update and a crates.io version, with no manual steps after setup.
- The version number follows from the commit messages; nobody bumps `Cargo.toml` or tags by hand.
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
pr-run-mode = "plan"
dispatch-releases = true
```

dist bundles `README.md` and the `LICENSE-*` files into every archive by default, so no `include` is needed (listing them again would put them in twice).

`dispatch-releases = true` makes `release.yml` start on `workflow_dispatch` with a `tag` input, instead of on a tag push (see "Starting the release" below). dist still ensures that the tag matches the package version: its `plan` job fails on a mismatch before anything is built or published, which covers the spec's mismatched-tag scenario. dist also lays out the jobs as plan → build → host (upload to the GitHub Release) → publish jobs → announce. Publish jobs only run after the binaries exist, and a failed publish job can be re-run on its own.

*Alternative considered:* extending `ci.yml` by hand. It's full control, but means maintaining matrix, checksum, upload and formula-templating YAML ourselves. dist also names archives so that binstall needs no extra metadata (see below).

### git-cliff computes the version and the changelog

A `cliff.toml` at the repo root configures git-cliff for Conventional Commits:
- `[bump]`: `features_always_bump_minor = true`, `breaking_always_bump_major = false`, `initial_tag = "0.1.0"`. While rocker is 0.x, `feat` and breaking changes bump the minor version and everything else the patch version. Once the version is 1.0.0 or higher, git-cliff's normal semver rules apply.
- `tag_pattern = "v[0-9]+\\.[0-9]+\\.[0-9]+"`, so only release tags count.
- `filter_unconventional = false`: a commit that doesn't follow the convention still appears in the changelog and still counts as a patch. Every commit since the last tag leads to a release.
- `commit_parsers` group entries into Features, Bug Fixes, Performance, Documentation and Miscellaneous, and skip `^chore\(release\)`.
- The changelog headings are `## [X.Y.Z] - YYYY-MM-DD` (keep-a-changelog style), which dist's changelog parser recognises.

`git cliff --bumped-version` prints the next tag, and `git cliff --bump -o CHANGELOG.md` writes the changelog with that version.

*Alternative considered:* release-plz. It's Rust-native, uses git-cliff internally and also publishes to crates.io, but it releases by opening a release PR and waiting for it to be merged, not on every push to `main`. It would also duplicate what dist does for GitHub Releases.

*Alternative considered:* setting the version only inside the build, without committing it. dist checks the tag against `Cargo.toml` at the tag, `cargo publish` needs the real version, and the repo would report a version that was never released. So the version is committed.

### The auto-release workflow

`.github/workflows/auto-release.yml` runs on `push` to `main`, on `pull_request` against `main`, and on `workflow_dispatch` with an optional `version` input. It runs under `concurrency: { group: auto-release, cancel-in-progress: false }`, with `permissions: { contents: write, actions: write }`.

On push and dispatch:
1. Check out `main`, not the event SHA, with `fetch-depth: 0`. If two pushes land close together, the second run then starts from whatever the first one pushed.
2. Stop if there are no commits since the latest `vX.Y.Z` tag. This also covers the release commit itself, which is tagged.
3. Take the version from the `version` input, or else from `git cliff --bumped-version`.
4. Set it in `Cargo.toml` with `cargo set-version` (cargo-edit), and refresh `Cargo.lock` with `cargo update --workspace` so `cargo publish --locked` sees a consistent lockfile.
5. `git cliff --bump -o CHANGELOG.md` (or `--tag vX.Y.Z` when the version was overridden).
6. Commit `chore(release): vX.Y.Z` as `github-actions[bot]`, create an annotated tag `vX.Y.Z`, and push the commit and the tag atomically (`git push --atomic origin main vX.Y.Z`).
7. Start the release: `gh workflow run release.yml -f tag=vX.Y.Z`.

On pull requests, it only runs steps 2 and 3 and `git cliff --bump --unreleased`, and writes the next version and changelog preview to the job summary.

Pushes made with `GITHUB_TOKEN` don't start other workflows, so the release commit doesn't re-trigger `auto-release.yml` or `ci.yml`. Step 2 makes that safe even if a PAT is used later.

### Starting the release

A tag pushed with `GITHUB_TOKEN` doesn't trigger `on: push: tags`, but `workflow_dispatch` events are an exception to that rule. So the release workflow is started by dispatch (`dispatch-releases = true`), not by the tag push. This needs no PAT or GitHub App. It also allows a release for an existing tag to be restarted by hand, with `gh workflow run release.yml -f tag=vX.Y.Z`.

*Alternative considered:* pushing with a PAT so the tag push triggers dist's default tag workflow. That's one more long-lived secret with write access to the repo, just to get a trigger that dispatch already provides.

dist reads `CHANGELOG.md` and uses the section for the released version as the GitHub Release body. That's why the changelog is committed before tagging.

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

- [The first `cargo publish` permanently claims `rocker-tui` and its version. crates.io versions can be yanked but never deleted.] → The PR is only merged after `dist plan`, the PR plan run and the auto-release preview pass, and with explicit go-ahead from the maintainer. Merging is what triggers the first release.
- [Every push to `main` publishes a crates.io version, so many small pushes produce many versions] → Accepted, as the maintainer's choice. Batch work on branches and merge it in one push when a release per commit isn't wanted.
- [Commit messages now decide version numbers: a mistyped `feat` becomes a patch release, and a missing `!` hides a breaking change] → `AGENTS.md` makes Conventional Commits a rule. The manual `version` input can correct the next release.
- [If the `merge-to-main` ruleset or branch protection is enabled later, the bot's push to `main` is rejected] → `RELEASING.md` says to add `github-actions` as a bypass actor, or to switch to a GitHub App token.
- [A push lands between the workflow's checkout and its push] → The atomic push is rejected and the run fails without tagging. The next push to `main` releases both changes.
- [The tag is pushed but `release.yml` fails] → The version is used up. The maintainer re-runs the failed jobs. If the build itself is broken, a fix commit releases the next version, and the orphaned tag stays without a GitHub Release.
- [dist's arm64 musl build may need a different runner or zigbuild than expected] → Verify with `dist plan` locally and on a PR before tagging; fall back to dropping `aarch64-unknown-linux-musl` only with the maintainer's agreement.
- [The CI artifacts `rocker-linux-*` and `rocker-macos-*` disappear] → Nobody downloads them outside the maintainer, and releases replace them. This is noted as BREAKING in the proposal.
- [The `formula = "rocker"` name might clash with a homebrew-core formula of the same name for unqualified `brew install rocker`] → The docs always use the qualified `ds82/tap/rocker`.
- [The macOS quarantine flag on directly downloaded binaries] → Documented; Homebrew and binstall installs are unaffected.

## Migration Plan

1. Before merging, the maintainer completes the one-time setup in `RELEASING.md`. Merging starts a release, so the secrets and the tap must already exist.
2. Merge the PR. With no tags yet, `auto-release.yml` releases `v0.1.0`, the current version: it commits the initial `CHANGELOG.md` as `chore(release): v0.1.0`, tags it, and starts the release.

Pause: `gh workflow disable auto-release.yml` stops new releases until it's re-enabled; pushes in between are released together by the next run.

Rollback of one release: delete the GitHub Release, revert the formula commit in the tap, and yank the crate version. A yanked version stays visible but won't be resolved by new installs. Keep the tag: versioning counts from it, and without it the next run would compute the yanked version again, which crates.io refuses. Then push the fix, which releases the next version.
