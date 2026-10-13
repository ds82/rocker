# rocker

A vim-inspired terminal UI for Docker, built with Rust and [Ratatui](https://github.com/ratatui-org/ratatui).

```
 [normal] containers                                          rocker
 NAME              IMAGE              STATUS    PORTS
 nginx             nginx:latest       running   0.0.0.0:80->80/tcp
 postgres          postgres:15        exited    5432/tcp
 redis             redis:7-alpine     running   6379/tcp

  s start/stop  r restart  d delete  l logs  e exec  / filter  Tab sections  q quit
```

## Features

- Modal navigation (Normal / Command / Filter / Visual / Log / Yank)
- Containers, images, volumes, and networks views
- Async Docker operations with spinner — the UI never blocks
- Log streaming with follow mode
- Exec into a container shell (`e`)
- Visual mode multi-select for bulk start/stop/delete
- Vim-style yank to clipboard (`yy`, `yi`, `yd`) via OSC 52
- Command history with up/down arrow navigation
- Filter with `/` — substring match, clears on Esc
- Remote Docker daemon via SSH (`DOCKER_HOST`)
- Color themes (gruvbox, catppuccin, solarized) with per-color overrides
- Config file at `~/.config/rocker/config.toml`
- Command history persisted at `~/.local/share/rocker/history`

## Install

```sh
cargo install --path .
```

Requires Rust 1.75+ and a running Docker daemon.

## Usage

```sh
# Local Docker socket
rocker

# Remote host over SSH
DOCKER_HOST=user@myserver rocker
DOCKER_HOST=ssh://user@myserver:2222 rocker
```

## Keybindings

### Normal mode

| Key | Action |
|-----|--------|
| `j` / `k` or `↓` / `↑` | Move down / up |
| `g` / `G` | Jump to top / bottom |
| `Ctrl-d` / `Ctrl-u` | Half-page down / up |
| `Tab` | Cycle sections (containers → images → volumes → networks) |
| `s` | Start / stop (toggle) |
| `r` | Restart |
| `d` | Delete (confirmation required) |
| `l` or `Enter` | View logs |
| `e` | Exec into container shell |
| `v` | Enter visual (multi-select) mode |
| `y` | Enter yank mode |
| `/` | Filter list |
| `:` | Command mode |
| `R` | Force refresh |
| `q` | Quit |

### Visual mode

| Key | Action |
|-----|--------|
| `j` / `k` | Extend selection |
| `g` / `G` | Extend to top / bottom |
| `s` | Start / stop all selected |
| `d` | Delete all selected |
| `Esc` | Return to Normal |

### Yank mode (`y` → …)

| Key | Containers | Images | Volumes | Networks |
|-----|-----------|--------|---------|---------|
| `y` | name | `repo:tag` | name | name |
| `i` | image | short ID | mountpoint | subnet |
| `d` | full container ID | short ID | — | — |

Copied via OSC 52 — works locally and over SSH. For tmux, add `set -g set-clipboard on` to `~/.tmux.conf`.

### Log mode

| Key | Action |
|-----|--------|
| `j` / `k` | Scroll |
| `G` | Jump to latest |
| `f` | Toggle follow mode |
| `q` / `Esc` | Close |

### Command mode

| Key | Action |
|-----|--------|
| `Enter` | Execute |
| `↑` / `↓` | Navigate history |
| `Esc` | Cancel |

Built-in commands: `:q` / `:quit`.

## Config

`~/.config/rocker/config.toml` (all fields optional):

```toml
[general]
refresh_interval_ms = 2000                 # default: 2000
default_section = "containers"             # containers | images | volumes | networks

[theme]
name = "gruvbox-dark"                      # default: "default"
danger = "#ff5555"                         # override any role, see below
```

### Themes

Pick a built-in theme with `name` (case-insensitive):

| Theme | Colors |
|-------|--------|
| `default` | 16 ANSI colors on your terminal's background (the original look) |
| `gruvbox-dark` | [Gruvbox](https://github.com/morhetz/gruvbox) dark |
| `catppuccin-mocha` | [Catppuccin](https://catppuccin.com/palette) Mocha |
| `solarized-light` | [Solarized](https://ethanschoonover.com/solarized) light |

Every other key under `[theme]` overrides one **role**, a named color slot the UI draws with:

| Role | Colors |
|------|--------|
| `bg` / `fg` | background and text of the main area, popups and log view |
| `title` | section name in the title bar |
| `muted` | secondary columns, compose group headers, dim key hints |
| `border` | list and log borders, table headers |
| `bar_bg` | title bar and status bar background |
| `accent` | `[command]`/`[menu]`/`[help]` labels, popup borders, `:` prompt, visual cursor row |
| `info` | `[filter]`/`[inspect]` labels, filter text, spinner, help keys |
| `success` | `[normal]`/`[exec]` labels, start/restart hints, `[follow]` |
| `warning` | status messages, `[paused]` in logs, one-shot exec |
| `danger` | `[confirm]`, errors, delete hints |
| `highlight` | `[visual]`/`[log]` labels, visual-mode hints |
| `secondary` | `[yank]` label and yank hints |
| `selected_bg` | selected row |
| `visual_bg` | rows selected in visual mode |
| `status_running` / `status_exited` / `status_paused` | container state |

Color values are one of the 16 ANSI names (`red`, `light_blue`, `darkgray`, … — case and `_` don't matter), `reset` (the terminal's default) or hex `#rrggbb`.

- **Transparent terminal?** The hex themes paint their own background. Add `bg = "reset"` to keep your terminal's background.
- **Truecolor:** hex colors need a truecolor terminal. rocker warns at startup when `COLORTERM` isn't `truecolor` or `24bit`. If your terminal supports truecolor but the variable gets lost (common in tmux or over SSH), add `export COLORTERM=truecolor` to your shell profile.
- **Mistakes in `[theme]`** (unknown theme name, invalid color, unknown key) never stop rocker. It falls back to the theme's colors and shows a warning in the status bar for 10 seconds.

## Docker host resolution

Priority order (first match wins):

1. `DOCKER_HOST` environment variable
2. Local Unix socket at `/var/run/docker.sock`

| `DOCKER_HOST` value | Interpreted as |
|--------------------|---------------|
| `hostname` | SSH to hostname |
| `user@hostname` | SSH as user |
| `ssh://user@hostname:port` | SSH, explicit port |
| `unix:///path/to/sock` | Custom local socket |

SSH connections reuse `~/.ssh/config` and the system SSH agent — no credentials need to be configured in rocker.

## Tech stack

| Concern | Crate |
|---------|-------|
| TUI rendering | `ratatui 0.30` |
| Terminal backend | `crossterm 0.29` |
| Docker API | `bollard 0.21` |
| Async runtime | `tokio` (full) |
| Config | `toml` + `serde` |
| Error handling | `anyhow` |
