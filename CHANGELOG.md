# Changelog

All notable changes to rocker are documented here. This file is generated
from the commit history by git-cliff on every release; don't edit it by hand.

## [0.1.0] - 2026-10-14

### Features

- Initial commit & phase 1 done
- Implement phase 2
- Phase3 draft, needs bugfixing and testing
- Add basic yanking
- Improve exec command with history and interactive/one-shot mode
- Improve filtering
- Basic phase 5 implemented
- Rename project to rocker
- **theme:** Add built-in color themes with per-role overrides

### Bug Fixes

- Stable lists, no jumping
- Make commands async
- Container grouping, indent grouped containers
- Better inspect
- Container sorting to keep yanking working
- **app:** Keep toast messages visible across list refreshes

### Documentation

- Add README.md and update PLAN.md
- Add AGENTS.md with architecture and gotchas for AI agents
- Replace PLAN.md with OpenSpec and GitHub issues
- Plan color themes as OpenSpec change add-color-themes
- Document color themes and roles
- **openspec:** Plan Homebrew and cargo-binstall releases
- **openspec:** Plan commit-driven releases with git-cliff
- Document installing and releasing

### Miscellaneous

- Add github build action
- Re-format with cargo fmt
- Clippy --fix
- Crossplatform builds and artefacts
- **openspec:** Mark add-color-themes tasks done
- **openspec:** Archive add-color-themes, add ui-theming spec
- License under MIT OR Apache-2.0, rename crate to rocker-tui
- **release:** Build and publish releases with dist
- **release:** Cut a release on every push to main
- **openspec:** Mark release-distribution tasks done
