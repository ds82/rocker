# Proposal

## Why

The `K` overlay shows only a handful of container fields as a flat list. Anything else (env, labels, restart policy, health, full network settings) still needs `docker inspect` in another terminal, and that output is too long to scan without structure or search.

## What Changes

- **Tree view**: inspecting a container shows the complete `docker inspect` data as a collapsible tree. Top-level keys start visible with their branches collapsed.
- **Keybinding**: `i` opens it. `K` keeps working as an alias.
- **Whole-branch folding**: `L` and `H` expand or collapse the current branch including everything nested in it.
- **Search**: `/` searches keys and scalar values incrementally and case-insensitively, reveals the match by expanding its ancestors, and `n` / `N` step through matches. Vim-style search rather than narrowing the tree, so a match keeps its surrounding context.
- **Detail pane**: shows the full path and the untruncated value of the row under the cursor, because long values don't fit on a row.
- **Docs**: help overlay and `README.md`.

Not in scope:
- images, volumes and networks, which keep the existing key-value overlay
- live refresh while the tree is open (inspect data is a snapshot)
- yanking values from the tree
- redacting secrets in `Env`: the view shows what `docker inspect` shows

## Capabilities

### New Capabilities
- `container-inspect`: browsing and searching a container's inspect data.

### Modified Capabilities
<!-- none -->

## Impact

- `src/inspect_tree.rs` (new): pure tree model with navigation and search, unit tested.
- `src/app.rs`: new `Mode::ContainerInspect`, its dispatch, and `spawn_inspect` now sends the whole response. `ContainerInspectExtra` and friends are removed.
- `src/input/mod.rs`, `src/ui/inspect_tree.rs` (new), `src/ui/command.rs`, `src/ui/help.rs`, `README.md`.
- `Cargo.toml`: `serde_json` becomes a direct dependency. It's already in the tree through bollard.
