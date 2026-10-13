# AGENTS.md

rocker is a vim-style terminal UI for Docker. It's written in Rust (ratatui + crossterm + bollard on tokio) and builds to a single binary. User-facing behaviour (keybindings, config, `DOCKER_HOST` handling) is documented in `README.md`.

## Verify

Run all of these before calling work done. CI (`.github/workflows/ci.yml`) runs the first three; `cargo test` is not in CI yet.

```sh
cargo fmt --check
cargo clippy -- -D warnings
cargo build
cargo test
```

Unit tests exist only for theme resolution (`src/theme.rs`). To check UI behaviour, run `cargo run` against a live Docker daemon.

## Architecture

`App` in `src/app.rs` holds all state. Everything else is a thin layer around it:

- **Event loop**: `src/main.rs` `run()`. A `tokio::select!` waits on four things: key events, the Docker refresh tick, the spinner tick and the `AppMsg` channel. The loop redraws once per iteration.
- **Input**: `src/input/mod.rs` maps `(Mode, KeyEvent)` to an `Action`. The mapping is pure: no state, no side effects.
- **Dispatch**: `App::dispatch` in `src/app.rs` routes each `Action` to a `dispatch_<mode>` method. Mode transitions happen here.
- **Render**: `src/ui/` is read-only over `&App`. `ui::render` draws the title bar, the list and the status bar, then lays overlays (menu, help, inspect) on top. Log mode replaces the whole screen.
- **Docker**: `src/docker/client.rs` connects to the daemon. `src/docker/types.rs` converts bollard summaries into the domain types `Container`, `Image`, `Volume` and `Network`.

### Mode FSM

`Mode` (in `app.rs`) is an enum whose variants carry their own state, e.g. `Visual { anchor, cursor }` or `Log { scroll, follow }`. When a mode needs extra state, add a field to its variant, not to `App`.

Adding a key or mode touches these places, all of which must stay in sync:

1. Mapping: the `map_*` fn in `src/input/mod.rs`. Add a new `Action` variant if needed.
2. Handler: the `dispatch_*` method in `src/app.rs`.
3. Rendering, if visible: `src/ui/`.
4. Help overlay: `help_lines()` in `src/ui/help.rs`, which is written by hand.
5. Keybinding tables in `README.md`.

### Async Docker operations

The UI never awaits a mutating Docker call. Each mutation goes through `App::spawn_cmd(label, refresh_section, closure)`, which:

- increments `pending_count`, which drives the spinner
- runs the closure on a `tokio::spawn`
- sends back `AppMsg::Cmd(CmdResult)`

`handle_cmd_result` then refreshes the named section, or shows an error toast that clears after 5 s. Follow this pattern for new actions; `spawn_toggle` and `spawn_remove_*` are examples. Log streaming (`start_log_stream`) and container inspect (`spawn_inspect`) send their results over the same channel.

The list refreshes (`refresh_*`) are awaited inline on the periodic tick. The tick only runs in Normal, Filter and Visual modes.

## Gotchas

- **Container order is load-bearing.** `refresh_containers` sorts containers in this order: compose project first, ungrouped containers last, then by name. `ui/list.rs` draws the compose groups in that same order. Because `selected` is an index into `visible_containers()`, changing the sort or the grouping without changing the other side makes the selection point at the wrong row.
- **Exec leaves the TUI.** The `e` key sets `app.pending_exec`. The event loop then exits raw mode and the alternate screen, runs the `docker` CLI (`docker exec [-it] …`) as a subprocess, and restores the terminal. This means exec relies on the `docker` CLI and the inherited `DOCKER_HOST`, not on bollard or rocker's own SSH tunnel.
- **SSH remotes** use `SshTunnel` in `docker/client.rs`. It spawns `ssh -N -L /tmp/rocker-<pid>.sock:/var/run/docker.sock` and points bollard at the local socket. The tunnel stays alive for as long as `App._tunnel` holds it, and the ssh child is killed on drop. Any `DOCKER_HOST` that doesn't start with `unix://` or `tcp://` is treated as SSH.
- **Terminal safety**: `main.rs` installs a panic hook that restores the terminal. Keep it around any new code path that leaves raw mode.
- **Clipboard** goes through OSC 52 (`src/clipboard.rs`, with its own base64 encoder) and has no system clipboard dependency. Copying works over SSH and in tmux.
- **Config** (`src/config.rs`) is entirely `#[serde(default)]`. A missing or unparseable file silently falls back to the defaults, so every new field needs a `Default` value. The exception is `[theme]`: it's read as a raw table and resolved by `Theme::resolve`, which turns bad entries into startup warnings shown in the status bar.
- **Colors come from the theme.** UI code uses `app.theme.<role>`, never a `Color::*` literal. Adding a role means touching each of:
  - the field on `Theme`
  - `ROLES`
  - `role_mut`
  - every built-in theme constructor in `src/theme.rs`
  - the roles table in `README.md`

  The `default` theme must keep the original ANSI look. Theming vocabulary is defined in `CONTEXT.md`.
- Command history (`src/history.rs`) is kept in `~/.local/share/rocker/history` and `exec_history`. The `:` command bar currently implements only `:q`/`:quit` (`execute_command`).

## Planning

Plan features as OpenSpec changes in `openspec/` (use the `openspec-*` skills). The feature backlog lives in GitHub issues labelled `enhancement`.
