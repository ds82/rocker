# Proposal

## Why

rocker's look is mostly fixed. The `[theme]` config table controls only four colors (selection background, running/exited/paused status). Around 90 other colors are hard-coded across `src/ui/`: mode labels, bars, borders, key hints, popups. On light terminals, or for users who prefer a particular dark colorscheme, most of the UI can't be adjusted. Users of Neovim-style tools expect to pick a colorscheme by name.

## What Changes

- **Built-in themes**, selected with `name = "<theme>"` in `[theme]`. First set: `default` (today's look in ANSI colors), `gruvbox-dark`, `catppuccin-mocha` and `solarized-light`. Names follow `<scheme>-<variant>`, so light or dark counterparts can be added later without clashing.
- **Roles**: a fixed set of semantic color slots, such as `bg`, `fg`, `muted`, `bar_bg`, `accent`, `success`, `warning`, `danger`, the selection and visual-mode backgrounds, and the container status colors. Mode labels and key hints use these semantic roles; there is no role per mode. Every UI color comes from a role of the active theme, and no `Color::*` literals remain in `src/ui/`.
- **Overrides**: any role can be set in `[theme]` next to `name`, replacing the active theme's color for that role only. The four existing keys (`selected_bg`, `status_running`, `status_exited`, `status_paused`) are roles, so existing configs keep working unchanged.
- **Theme backgrounds**: themes with a hex background paint it. Setting `bg = "reset"` keeps a transparent terminal background.
- **Hex colors**: color values accept `#rrggbb` in addition to the existing ANSI color names.
- **Visible config errors**: these problems fall back instead of failing, and show a warning in the status bar at startup:
  - an unknown theme name falls back to `default`
  - an invalid color value keeps the theme's color for that role
  - an unknown key is ignored

  Today an invalid color silently becomes the terminal default.
- **Truecolor warning**: if the active theme uses hex colors and `$COLORTERM` doesn't advertise truecolor, a startup warning says so. Colors are not converted.
- **Docs**: `README.md` documents the themes and roles.

Not in scope:
- switching themes at runtime (`:theme`)
- user theme files under `~/.config/rocker/themes/`
- converting hex colors for terminals without truecolor

## Capabilities

### New Capabilities
- `ui-theming`: how rocker chooses and applies colors. Covers built-in themes, roles, overrides, the color value syntax, background painting, and how config problems are reported.

### Modified Capabilities
<!-- none: no existing specs -->

## Impact

- `src/theme.rs`: rewritten to hold the theme struct, the built-in theme definitions, color parsing (names and hex), override merging and warning collection. This is the bulk of the change.
- `src/ui/*.rs` (`command.rs`, `list.rs`, `logs.rs`, `help.rs`, `menu.rs`, `inspect.rs`): every hard-coded color is replaced with a role.
- `src/main.rs` and `src/app.rs`: theme warnings are shown in the status bar at startup, and refreshes no longer clear an active warning.
- `README.md`: the config section documents `name`, the built-in themes, the roles and `bg = "reset"`.
- `CONTEXT.md`: theming vocabulary (theme, built-in theme, role, override).
- No new crate dependencies. Existing config files stay valid.
