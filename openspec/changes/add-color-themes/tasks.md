# Tasks

## 1. Theme model and color parsing

- [ ] 1.1 In `src/theme.rs`, replace `Theme` with one `pub Color` field per role (the table in design.md), add `ROLES` and `role_mut`, and add a unit test that every entry in `ROLES` resolves through `role_mut`. Verify with `cargo test`.
- [ ] 1.2 Change `parse_color` to return `Option<Color>`. It accepts the existing ANSI names, `reset` and `#rrggbb`. Add unit tests for a name, `dark_gray`/`darkgrey` variants, a hex value, and the rejected inputs `#12345`, `#gggggg` and `purpleish`. Verify with `cargo test`.
- [ ] 1.3 Add `Theme::default_ansi()` with the `default` column from design.md. Add a unit test asserting its values equal the colors that were hard-coded before. Verify with `cargo test`.

## 2. Built-in themes and config resolution

- [ ] 2.1 Add the `gruvbox-dark`, `catppuccin-mocha` and `solarized-light` built-in themes, using each scheme's official palette and setting `bg`/`fg`. Add `Theme::builtin(name)` with case-insensitive lookup. Verify with unit tests for `Gruvbox-Dark`, `gruvbox-dark` and an unknown name.
- [ ] 2.2 Add `Theme::resolve(&toml::Table) -> (Theme, Vec<String>)` following the steps in design.md. Write a unit test for each scenario in the spec's "Roles can be overridden" and "Invalid theme config" requirements, plus one where the value isn't a string. Verify with `cargo test`.
- [ ] 2.3 Add `Theme::uses_rgb()` and a pure helper `truecolor_warning(theme_name, &Theme, colorterm: Option<&str>) -> Option<String>`. Write unit tests for the three truecolor scenarios in the spec, plus `default` with one hex override. Verify with `cargo test`.
- [ ] 2.4 In `src/config.rs`, change `Config.theme` to a raw `toml::Table`, delete `ThemeConfig`, and resolve the theme (including the truecolor check) in `main.rs`. Verify that `cargo build` succeeds and that an existing config with only the legacy keys still applies them.

## 3. Startup warnings in the status bar

- [ ] 3.1 Pass the collected warnings into `App::new` and show them as a 10 s toast (`status` plus `status_expires`). Verify by running rocker with `name = "dracula"` and `COLORTERM` unset: the warning names `dracula` and lists the available themes. Then run it with `name = "gruvbox-dark"`: the truecolor warning appears.
- [ ] 3.2 Make every `refresh_*` in `src/app.rs` clear `status` only when `status_expires` is `None`. Verify that the startup warning survives several refresh ticks and disappears after about 10 s, and that a failed Docker command's error toast also stays for its full 5 s.

## 4. Route every UI color through the theme

- [ ] 4.1 Fill the frame with `bg`/`fg` in `ui::render`, and apply the same style to the log view and to the `help`, `menu` and `inspect` popups. Pass `&Theme` to `help::render` and `menu::render`. Verify visually:
  - with `gruvbox-dark`, no area shows the terminal's background
  - with `gruvbox-dark` and `bg = "reset"` in a transparent terminal, the terminal's background shows through
- [ ] 4.2 Replace every `Color::*` in `src/ui/command.rs`, `list.rs`, `logs.rs`, `help.rs`, `menu.rs` and `inspect.rs` with the role from the design.md mapping. Verify that `grep -rn "Color::" src/ui` returns nothing, and that with no theme configured the UI looks the same as before in Normal, Visual, Yank, Exec, Log, Menu, Help, Inspect and Confirm modes.
- [ ] 4.3 Check each hex theme (`gruvbox-dark`, `catppuccin-mocha`, `solarized-light`) in a truecolor terminal across the modes listed in 4.2. Fix any unreadable role or unpainted gap by adjusting that theme's colors.

## 5. Documentation

- [ ] 5.1 Update the config section of `README.md` to cover:
  - the `name` key and the list of built-in themes
  - the roles and what each one colors
  - the color value syntax
  - `bg = "reset"` for transparent terminals
  - the truecolor requirement and how to set `COLORTERM`
  - the warning behaviour

  Verify that every role in `ROLES` appears in the README and that the example config in the README produces no startup warning.
- [ ] 5.2 In `AGENTS.md`, add a gotcha that UI code uses `app.theme.<role>` instead of `Color::*` literals, and that a new role needs an entry in `ROLES`, `role_mut`, every built-in theme and the README. Verify by reading the file.

## 6. Final checks

- [ ] 6.1 Run `cargo fmt --check`, `cargo clippy -- -D warnings`, `cargo build` and `cargo test`, and verify all of them pass.
