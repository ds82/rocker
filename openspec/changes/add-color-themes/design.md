# Design

## Context

- `src/theme.rs` has two types:
  - `ThemeConfig`: 4 string fields, read by serde.
  - `Theme`: 4 `Color`s.

  `parse_color` maps unknown strings to `Color::Reset` silently.
- `Config::load` (`src/config.rs`) deserializes the whole file with `#[serde(default)]`. Unknown keys are dropped. Any parse error discards the whole file without telling the user.
- `App` holds the resolved `Theme`. UI modules read `app.theme`, except `help::render` and `menu::render`, which don't receive `App`.
- Every successful `refresh_*` in `src/app.rs` sets `self.status = None`, so a message set at startup would disappear on the first refresh. Toasts with `status_expires` currently get cleared by refresh too.
- The project has no tests yet.
- Theming vocabulary (theme, built-in theme, role, override) is defined in `CONTEXT.md`.

## Goals / Non-Goals

**Goals:**
- One `Theme` value is the only source of colors in `src/ui/`.
- Adding a role means editing one place, and a test catches a role that is missing from the config key list.
- The first unit tests in the repo, covering theme resolution.

**Non-Goals:**
- Exposing text modifiers (bold etc.) in themes. `Modifier::BOLD` stays hard-coded.
- Changing which container state uses which status role. `restarting` keeps the paused color, and `created`/`dead`/unknown keep the exited color. Tracked separately in ds82/rocker#10.
- A `selected_fg` role. Each built-in theme picks a `selected_bg` that contrasts with its own `fg`, which task 4.3 checks. The role can be added later without breaking configs.
- A setting to turn off the truecolor warning. The fix is `export COLORTERM=truecolor`, documented in the README.
- Converting hex colors for terminals without truecolor. We only warn.
- Reporting errors in non-theme parts of the config. That's a separate problem.

## Decisions

### Roles

Roles are a flat set of semantic names. Each currently hard-coded color maps to exactly one role. The `default` column is today's color, so the `default` theme reproduces the current look.

| Role | default | Used for |
|---|---|---|
| `bg` | reset | main area, popups, log view background |
| `fg` | reset | normal text |
| `title` | White | section name in title bar |
| `muted` | DarkGray | secondary columns, hints like `Esc`/`q`, compose group headers, "no … found", inspect keys, the "rocker" label |
| `border` | DarkGray | list and log borders, table header |
| `bar_bg` | DarkGray | title bar and status bar background |
| `accent` | Yellow | popup borders (help, menu), menu highlight, help section titles, `:` prompt, Tab hint, visual-mode cursor row text |
| `info` | Cyan | filter prompt and text, spinner, logs hint, inspect border, help keys, `[log]` label in log view |
| `success` | Green | `[normal]`/`[exec]` labels, exec prompt, `[follow]`, start/restart/exec hints, active menu entry |
| `warning` | Yellow | status messages, `[paused]`, one-shot exec flag |
| `danger` | Red | `[confirm]`, `!` error marker, delete hints |
| `highlight` | Magenta | `[visual]`/`[log]` labels, visual count and hints |
| `secondary` | Blue | `[yank]` label and yank key hints |
| `selected_bg` | DarkGray | selected row |
| `visual_bg` | Blue | visual-mode range and cursor row background |
| `status_running` | Green | container state |
| `status_exited` | Red | container state |
| `status_paused` | Yellow | container state |

Mode labels map to these roles:

| Mode | Role |
|---|---|
| normal | `success` |
| command | `accent` |
| filter | `info` |
| log | `highlight` |
| confirm | `danger` |
| menu | `accent` |
| visual | `highlight` |
| yank | `secondary` |
| exec | `success` |
| help | `accent` |
| inspect | `info` |

*Alternative considered:* one role per mode (`mode_normal`, …). That gives finer control but adds 11 keys that built-in themes would mostly set to the same few colors. Per-mode roles can be added later without breaking any config.

### Theme struct and role registry

`Theme` stays a plain struct with one `pub Color` field per role, read as `app.theme.danger`. It gets:

- `fn role_mut(&mut self, key: &str) -> Option<&mut Color>`: a `match` over the role names.
- `const ROLES: &[&str]`: the list of role names.

A unit test walks `ROLES` and asserts `role_mut` returns `Some` for each. This keeps the key list and the struct in sync without a macro.

*Alternative considered:* a `HashMap<&str, Color>`. That's simpler to merge, but every lookup becomes a string lookup, and a typo in UI code would only fail at runtime.

### Built-in themes

Built-in themes are `fn` constructors in `src/theme.rs`: `Theme::default_ansi()`, `Theme::gruvbox_dark()`, etc. `Theme::builtin(name)` does a case-insensitive lookup and returns `Option<Theme>`.

The hex themes take their colors from each scheme's official palette and set `bg`/`fg` explicitly, so they look right regardless of the terminal's own colors. `solarized-light` is the light-background theme. Names follow `<scheme>-<variant>` (see spec).

### Config parsing

`Config.theme` changes from `ThemeConfig` to a raw `toml::Table`, still `#[serde(default)]`. A new `Theme::resolve(&toml::Table) -> (Theme, Vec<String>)` applies these steps in order:

1. Read `name`. If it's unknown, use `default` and add a warning.
2. Start from that built-in theme.
3. For every other key, look up `role_mut`:
   - an unknown key gets a warning
   - a value that isn't a string, or fails `parse_color`, gets a warning and the role keeps the theme's color
   - otherwise the override replaces the role's color

Reading the table raw is what makes unknown-key warnings possible; serde would drop those keys silently. `ThemeConfig` is deleted. The 4 old keys are simply roles, so old configs keep working.

`parse_color` returns `Option<Color>`. It accepts today's name set plus `reset` and `#rrggbb` (exactly 6 hex digits).

*Alternative considered:* ratatui's `Color::from_str`. It also accepts indexed colors (`"42"`), `"light-blue"` and other forms, which would widen the documented syntax and silently accept typos as indexed colors. A small own parser keeps the syntax exactly as specified.

### Truecolor check

After resolving, `Theme::uses_rgb()` checks whether any role is `Color::Rgb`. `main.rs` adds a warning when it returns true and `COLORTERM` is neither `truecolor` nor `24bit`. The check is a one-off read of an environment variable. There is no terminal probing, since that would require escape-sequence round trips before the TUI starts.

### Startup warnings

`main.rs` collects the warnings from `resolve` and the truecolor check, then passes them to `App::new`. If there are any, `App` sets `status` to them joined with `; ` and sets a `status_expires` of 10 s.

To keep refreshes from wiping it, every `refresh_*` clears `status` only when `status_expires` is `None`, i.e. when it isn't a toast. This also fixes command-failure toasts being cleared early by the next refresh.

### Applying colors

- `ui::render` first fills the whole frame with `Style::default().bg(theme.bg).fg(theme.fg)`. With `bg = reset` this is a no-op, which keeps transparent terminals transparent.
- Popups (`help`, `menu`, `inspect`) set the same style on their `Block` after `Clear`.
- `help::render` and `menu::render` take `&Theme` as an extra parameter.
- The log view sets the style on its `Paragraph`.

## Risks / Trade-offs

- [`COLORTERM` is often missing over SSH or inside tmux even when the terminal does support truecolor, giving a false warning] → The warning is a 10 s toast and doesn't block anything. The README explains how to set `COLORTERM`.
- [Hex themes in a terminal without truecolor show wrong colors] → The startup warning says so. `default` works everywhere.
- [Painting `bg` everywhere could leave unpainted gaps, e.g. table column spacing] → `default` uses `reset`, so nothing changes there. Check each hex theme visually for gaps.
- [Changing refresh behaviour around `status` affects error display] → It only changes when a toast is active, and toasts already expire on their own.
- [A typo in a non-theme config key still discards the whole config silently] → This is already the case today and is out of scope. The theme table can no longer cause it, because it's a generic table.

## Migration Plan

None needed. Old `[theme]` keys are valid roles, and the `default` theme equals today's colors. Rolling back is a plain revert. An older binary ignores `name` and the new keys.
