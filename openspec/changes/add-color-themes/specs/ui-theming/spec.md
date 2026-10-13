# Spec Delta

## Purpose

Defines how rocker chooses the colors it draws with: built-in themes, the roles every UI element draws from, user overrides in the config file, and how invalid theme config is reported.

## ADDED Requirements

### Requirement: Built-in themes selectable by name
The system SHALL provide the built-in themes `default`, `gruvbox-dark`, `catppuccin-mocha` and `solarized-light`, and SHALL apply the theme named by `name` in the `[theme]` table of `config.toml` at startup. Built-in theme names SHALL be kebab-case `<scheme>-<variant>`, except `default`.

#### Scenario: Theme selected in config
- **WHEN** `config.toml` contains `[theme]` with `name = "gruvbox-dark"`
- **THEN** rocker starts with the gruvbox-dark colors applied to the whole UI

#### Scenario: No theme configured
- **WHEN** `config.toml` is missing, has no `[theme]` table, or `[theme]` has no `name`
- **THEN** rocker uses the `default` theme

#### Scenario: Theme name is case-insensitive
- **WHEN** `[theme]` contains `name = "Gruvbox-Dark"`
- **THEN** rocker applies the gruvbox-dark theme

### Requirement: Default theme preserves current appearance
The `default` theme SHALL use only the 16 ANSI colors and the terminal's own default foreground and background, and SHALL look the same as rocker did before themes were introduced.

#### Scenario: Upgrade without config changes
- **WHEN** a user upgrades and their config sets no theme name and no new roles
- **THEN** every UI element has the same colors as in the previous release

### Requirement: Every UI color comes from a role
Every color rocker draws SHALL come from a role of the active theme. This includes the title bar, status bar, mode labels, key hints, borders, popups, list rows, selection, the visual-mode range, the log view and container status.

#### Scenario: Theme change recolors all elements
- **WHEN** the user switches `name` from `default` to `solarized-light` and restarts rocker
- **THEN** no UI element keeps a color from the `default` theme unless `solarized-light` assigns that role the same color

### Requirement: Theme background is painted
When the active theme's `bg` role is a color other than `reset`, rocker SHALL paint the main list area, popups and log view with it. When `bg` is `reset`, rocker SHALL leave the terminal's own background visible, including any transparency.

#### Scenario: Built-in theme with a background
- **WHEN** the active theme is `gruvbox-dark` and `bg` is not overridden
- **THEN** the list area, popups and log view show the gruvbox-dark background instead of the terminal's background

#### Scenario: Transparent terminal kept
- **WHEN** `[theme]` contains `name = "gruvbox-dark"` and `bg = "reset"`
- **THEN** rocker uses gruvbox-dark colors for everything except the background, which stays the terminal's own

### Requirement: Roles can be overridden in config
The system SHALL accept any role as a key in the `[theme]` table, next to `name`. An override SHALL replace the active theme's color for that role only. All other roles keep the theme's colors.

#### Scenario: Single override on a built-in theme
- **WHEN** `[theme]` contains `name = "gruvbox-dark"` and `danger = "#ff0000"`
- **THEN** rocker uses `#ff0000` for danger-colored elements and gruvbox-dark colors for everything else

#### Scenario: Existing theme keys keep working
- **WHEN** a config written for an earlier release sets `selected_bg`, `status_running`, `status_exited` or `status_paused` under `[theme]`
- **THEN** those values are applied exactly as before, on top of the `default` theme

### Requirement: Color values accept ANSI names and hex
A color value SHALL be one of the following:
- one of the 16 ANSI color names, case-insensitive, with `_` or no separator (e.g. `darkgray`, `dark_gray`, `light_blue`)
- `reset`, meaning the terminal default
- a 24-bit hex value written `#rrggbb`

#### Scenario: Hex color
- **WHEN** a role is set to `"#282828"`
- **THEN** rocker draws that role with RGB (40, 40, 40)

#### Scenario: ANSI name
- **WHEN** a role is set to `"LightCyan"`
- **THEN** rocker draws that role with the terminal's light cyan

### Requirement: Invalid theme config is reported and falls back
When the theme config is invalid, rocker SHALL still start, SHALL fall back as described in the scenarios, and SHALL show a warning in the status bar at startup that names the problem. The warning SHALL remain visible for about 10 seconds, even if the lists refresh in the meantime.

#### Scenario: Unknown theme name
- **WHEN** `[theme]` contains `name = "dracula"` and no built-in theme has that name
- **THEN** rocker uses the `default` theme and the status bar shows a warning naming `dracula` and listing the available themes

#### Scenario: Invalid color value
- **WHEN** a role is set to a value that is neither a known color name nor `#rrggbb` (e.g. `"#12345"` or `"purpleish"`)
- **THEN** that role keeps the active theme's color and the status bar shows a warning naming the key and the bad value

#### Scenario: Unknown key
- **WHEN** `[theme]` contains a key that is neither a role nor `name`
- **THEN** rocker ignores the key and the status bar shows a warning naming it

#### Scenario: Several problems at once
- **WHEN** the theme config has more than one problem
- **THEN** the startup warning lists all of them

### Requirement: Warn when hex colors may not render
When the active theme, with overrides applied, uses at least one hex color and the `COLORTERM` environment variable is neither `truecolor` nor `24bit`, rocker SHALL show a startup warning that the theme needs a truecolor terminal. rocker SHALL NOT convert or approximate the colors.

#### Scenario: Hex theme without truecolor
- **WHEN** the active theme is `gruvbox-dark` and `COLORTERM` is unset
- **THEN** the startup warning says that `gruvbox-dark` needs a truecolor terminal

#### Scenario: Truecolor terminal
- **WHEN** the active theme is `gruvbox-dark` and `COLORTERM=truecolor`
- **THEN** no truecolor warning is shown

#### Scenario: ANSI-only theme
- **WHEN** the active theme is `default` with no hex overrides
- **THEN** no truecolor warning is shown, whatever `COLORTERM` is set to
