use ratatui::style::{Color, Style};

/// Names of all built-in themes, in the order shown to the user.
pub const BUILTIN_NAMES: &[&str] = &[
    "default",
    "gruvbox-dark",
    "catppuccin-mocha",
    "solarized-light",
];

/// Every role name accepted as a key in the `[theme]` config table.
/// Must stay in sync with the fields of `Theme` and with `Theme::role_mut`.
pub const ROLES: &[&str] = &[
    "bg",
    "fg",
    "title",
    "muted",
    "border",
    "bar_bg",
    "accent",
    "info",
    "success",
    "warning",
    "danger",
    "highlight",
    "secondary",
    "selected_bg",
    "visual_bg",
    "status_running",
    "status_exited",
    "status_paused",
];

/// A complete set of colors, one per role. See CONTEXT.md for the vocabulary.
#[derive(Debug, Clone, PartialEq)]
pub struct Theme {
    pub name: &'static str,
    pub bg: Color,
    pub fg: Color,
    pub title: Color,
    pub muted: Color,
    pub border: Color,
    pub bar_bg: Color,
    pub accent: Color,
    pub info: Color,
    pub success: Color,
    pub warning: Color,
    pub danger: Color,
    pub highlight: Color,
    pub secondary: Color,
    pub selected_bg: Color,
    pub visual_bg: Color,
    pub status_running: Color,
    pub status_exited: Color,
    pub status_paused: Color,
}

impl Default for Theme {
    fn default() -> Self {
        Self::default_ansi()
    }
}

impl Theme {
    /// The original rocker look: ANSI colors on the terminal's own background.
    pub fn default_ansi() -> Self {
        Self {
            name: "default",
            bg: Color::Reset,
            fg: Color::Reset,
            title: Color::White,
            muted: Color::DarkGray,
            border: Color::DarkGray,
            bar_bg: Color::DarkGray,
            accent: Color::Yellow,
            info: Color::Cyan,
            success: Color::Green,
            warning: Color::Yellow,
            danger: Color::Red,
            highlight: Color::Magenta,
            secondary: Color::Blue,
            selected_bg: Color::DarkGray,
            visual_bg: Color::Blue,
            status_running: Color::Green,
            status_exited: Color::Red,
            status_paused: Color::Yellow,
        }
    }

    /// https://github.com/morhetz/gruvbox (dark, medium contrast)
    pub fn gruvbox_dark() -> Self {
        Self {
            name: "gruvbox-dark",
            bg: rgb(0x282828),
            fg: rgb(0xebdbb2),
            title: rgb(0xfbf1c7),
            muted: rgb(0x928374),
            border: rgb(0x665c54),
            bar_bg: rgb(0x3c3836),
            accent: rgb(0xfabd2f),
            info: rgb(0x8ec07c),
            success: rgb(0xb8bb26),
            warning: rgb(0xfe8019),
            danger: rgb(0xfb4934),
            highlight: rgb(0xd3869b),
            secondary: rgb(0x83a598),
            selected_bg: rgb(0x3c3836),
            visual_bg: rgb(0x076678),
            status_running: rgb(0xb8bb26),
            status_exited: rgb(0xfb4934),
            status_paused: rgb(0xfabd2f),
        }
    }

    /// https://catppuccin.com/palette (Mocha flavor)
    pub fn catppuccin_mocha() -> Self {
        Self {
            name: "catppuccin-mocha",
            bg: rgb(0x1e1e2e),
            fg: rgb(0xcdd6f4),
            title: rgb(0xb4befe),
            muted: rgb(0x7f849c),
            border: rgb(0x585b70),
            bar_bg: rgb(0x313244),
            accent: rgb(0xf9e2af),
            info: rgb(0x89dceb),
            success: rgb(0xa6e3a1),
            warning: rgb(0xfab387),
            danger: rgb(0xf38ba8),
            highlight: rgb(0xcba6f7),
            secondary: rgb(0x89b4fa),
            selected_bg: rgb(0x45475a),
            visual_bg: rgb(0x3e4a6b),
            status_running: rgb(0xa6e3a1),
            status_exited: rgb(0xf38ba8),
            status_paused: rgb(0xf9e2af),
        }
    }

    /// https://ethanschoonover.com/solarized (light background)
    pub fn solarized_light() -> Self {
        Self {
            name: "solarized-light",
            bg: rgb(0xfdf6e3),
            fg: rgb(0x586e75),
            title: rgb(0x073642),
            muted: rgb(0x839496),
            border: rgb(0x93a1a1),
            bar_bg: rgb(0xeee8d5),
            accent: rgb(0x6c71c4),
            info: rgb(0x2aa198),
            success: rgb(0x859900),
            warning: rgb(0xcb4b16),
            danger: rgb(0xdc322f),
            highlight: rgb(0xd33682),
            secondary: rgb(0x268bd2),
            selected_bg: rgb(0xeee8d5),
            visual_bg: rgb(0xd5e5ef),
            status_running: rgb(0x859900),
            status_exited: rgb(0xdc322f),
            status_paused: rgb(0xb58900),
        }
    }

    /// Background and text color for areas that show the theme background.
    pub fn base(&self) -> Style {
        Style::default().bg(self.bg).fg(self.fg)
    }

    /// Look up a built-in theme by name, case-insensitively.
    pub fn builtin(name: &str) -> Option<Self> {
        match name.to_lowercase().as_str() {
            "default" => Some(Self::default_ansi()),
            "gruvbox-dark" => Some(Self::gruvbox_dark()),
            "catppuccin-mocha" => Some(Self::catppuccin_mocha()),
            "solarized-light" => Some(Self::solarized_light()),
            _ => None,
        }
    }

    pub fn role_mut(&mut self, key: &str) -> Option<&mut Color> {
        Some(match key {
            "bg" => &mut self.bg,
            "fg" => &mut self.fg,
            "title" => &mut self.title,
            "muted" => &mut self.muted,
            "border" => &mut self.border,
            "bar_bg" => &mut self.bar_bg,
            "accent" => &mut self.accent,
            "info" => &mut self.info,
            "success" => &mut self.success,
            "warning" => &mut self.warning,
            "danger" => &mut self.danger,
            "highlight" => &mut self.highlight,
            "secondary" => &mut self.secondary,
            "selected_bg" => &mut self.selected_bg,
            "visual_bg" => &mut self.visual_bg,
            "status_running" => &mut self.status_running,
            "status_exited" => &mut self.status_exited,
            "status_paused" => &mut self.status_paused,
            _ => return None,
        })
    }

    /// True if any role uses a 24-bit color, which needs a truecolor terminal.
    pub fn uses_rgb(&self) -> bool {
        let mut probe = self.clone();
        ROLES
            .iter()
            .any(|key| matches!(probe.role_mut(key), Some(Color::Rgb(..))))
    }

    /// Build the active theme from the `[theme]` config table: start from the
    /// built-in theme named by `name`, then apply every other key as an override.
    /// Problems never fail; they fall back and are returned as warnings.
    pub fn resolve(table: &toml::Table) -> (Self, Vec<String>) {
        let mut warnings = Vec::new();
        let mut theme = match table.get("name") {
            None => Self::default_ansi(),
            Some(toml::Value::String(name)) => Self::builtin(name).unwrap_or_else(|| {
                warnings.push(format!(
                    "unknown theme \"{name}\" (available: {})",
                    BUILTIN_NAMES.join(", ")
                ));
                Self::default_ansi()
            }),
            Some(other) => {
                warnings.push(format!("theme name must be a string, got {other}"));
                Self::default_ansi()
            }
        };

        for (key, value) in table {
            if key == "name" {
                continue;
            }
            let Some(slot) = theme.role_mut(key) else {
                warnings.push(format!("unknown theme key \"{key}\""));
                continue;
            };
            match value.as_str().and_then(parse_color) {
                Some(color) => *slot = color,
                None => warnings.push(format!("invalid color for {key}: {value}")),
            }
        }

        (theme, warnings)
    }
}

/// Warning for a theme that uses hex colors in a terminal that does not
/// advertise truecolor via `COLORTERM`.
pub fn truecolor_warning(theme: &Theme, colorterm: Option<&str>) -> Option<String> {
    let truecolor = matches!(colorterm, Some("truecolor" | "24bit"));
    if truecolor || !theme.uses_rgb() {
        return None;
    }
    Some(format!(
        "theme \"{}\" needs a truecolor terminal (set COLORTERM=truecolor)",
        theme.name
    ))
}

const fn rgb(hex: u32) -> Color {
    Color::Rgb((hex >> 16) as u8, (hex >> 8) as u8, hex as u8)
}

/// Parse a color value: one of the 16 ANSI names (case-insensitive, `_` optional),
/// `reset`, or `#rrggbb`.
pub fn parse_color(s: &str) -> Option<Color> {
    if let Some(hex) = s.strip_prefix('#') {
        if hex.len() != 6 || !hex.chars().all(|c| c.is_ascii_hexdigit()) {
            return None;
        }
        return u32::from_str_radix(hex, 16).ok().map(rgb);
    }
    Some(match s.to_lowercase().replace('_', "").as_str() {
        "reset" => Color::Reset,
        "black" => Color::Black,
        "red" => Color::Red,
        "green" => Color::Green,
        "yellow" => Color::Yellow,
        "blue" => Color::Blue,
        "magenta" => Color::Magenta,
        "cyan" => Color::Cyan,
        "white" => Color::White,
        "gray" | "grey" => Color::Gray,
        "darkgray" | "darkgrey" => Color::DarkGray,
        "lightred" => Color::LightRed,
        "lightgreen" => Color::LightGreen,
        "lightyellow" => Color::LightYellow,
        "lightblue" => Color::LightBlue,
        "lightmagenta" => Color::LightMagenta,
        "lightcyan" => Color::LightCyan,
        _ => return None,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn table(src: &str) -> toml::Table {
        toml::from_str(src).unwrap()
    }

    #[test]
    fn every_role_is_settable() {
        let mut theme = Theme::default();
        for key in ROLES {
            assert!(
                theme.role_mut(key).is_some(),
                "role {key} missing from role_mut"
            );
        }
    }

    #[test]
    fn parses_ansi_names() {
        assert_eq!(parse_color("LightCyan"), Some(Color::LightCyan));
        assert_eq!(parse_color("red"), Some(Color::Red));
        assert_eq!(parse_color("reset"), Some(Color::Reset));
        for s in ["darkgray", "dark_gray", "darkgrey", "Dark_Grey"] {
            assert_eq!(parse_color(s), Some(Color::DarkGray), "{s}");
        }
        assert_eq!(parse_color("light_blue"), Some(Color::LightBlue));
    }

    #[test]
    fn parses_hex() {
        assert_eq!(parse_color("#282828"), Some(Color::Rgb(40, 40, 40)));
        assert_eq!(parse_color("#FfA0b1"), Some(Color::Rgb(0xff, 0xa0, 0xb1)));
    }

    #[test]
    fn rejects_invalid_colors() {
        for s in ["#12345", "#1234567", "#gggggg", "purpleish", "", "42"] {
            assert_eq!(parse_color(s), None, "{s}");
        }
    }

    #[test]
    fn default_matches_previous_hard_coded_colors() {
        let t = Theme::default_ansi();
        assert_eq!((t.bg, t.fg), (Color::Reset, Color::Reset));
        assert_eq!(t.title, Color::White);
        assert_eq!(
            (t.muted, t.border, t.bar_bg),
            (Color::DarkGray, Color::DarkGray, Color::DarkGray)
        );
        assert_eq!((t.accent, t.warning), (Color::Yellow, Color::Yellow));
        assert_eq!(t.info, Color::Cyan);
        assert_eq!(t.success, Color::Green);
        assert_eq!(t.danger, Color::Red);
        assert_eq!(t.highlight, Color::Magenta);
        assert_eq!(t.secondary, Color::Blue);
        assert_eq!((t.selected_bg, t.visual_bg), (Color::DarkGray, Color::Blue));
        assert_eq!(
            (t.status_running, t.status_exited, t.status_paused),
            (Color::Green, Color::Red, Color::Yellow)
        );
        assert!(!t.uses_rgb());
    }

    #[test]
    fn builtin_lookup_is_case_insensitive() {
        assert_eq!(Theme::builtin("gruvbox-dark"), Some(Theme::gruvbox_dark()));
        assert_eq!(Theme::builtin("Gruvbox-Dark"), Some(Theme::gruvbox_dark()));
        assert_eq!(Theme::builtin("dracula"), None);
    }

    #[test]
    fn every_builtin_name_resolves() {
        for name in BUILTIN_NAMES {
            let theme = Theme::builtin(name).unwrap_or_else(|| panic!("{name} not found"));
            assert_eq!(theme.name, *name);
        }
    }

    #[test]
    fn no_name_gives_default() {
        let (theme, warnings) = Theme::resolve(&table(""));
        assert_eq!(theme, Theme::default_ansi());
        assert!(warnings.is_empty());
    }

    #[test]
    fn override_on_builtin() {
        let (theme, warnings) =
            Theme::resolve(&table("name = \"gruvbox-dark\"\ndanger = \"#ff0000\""));
        assert!(warnings.is_empty());
        assert_eq!(theme.danger, Color::Rgb(255, 0, 0));
        let expected = Theme {
            danger: Color::Rgb(255, 0, 0),
            ..Theme::gruvbox_dark()
        };
        assert_eq!(theme, expected);
    }

    #[test]
    fn legacy_keys_apply_on_default() {
        let (theme, warnings) = Theme::resolve(&table(
            "selected_bg = \"Blue\"\nstatus_running = \"LightGreen\"\nstatus_exited = \"magenta\"\nstatus_paused = \"cyan\"",
        ));
        assert!(warnings.is_empty());
        let expected = Theme {
            selected_bg: Color::Blue,
            status_running: Color::LightGreen,
            status_exited: Color::Magenta,
            status_paused: Color::Cyan,
            ..Theme::default_ansi()
        };
        assert_eq!(theme, expected);
    }

    #[test]
    fn unknown_name_falls_back_with_warning() {
        let (theme, warnings) = Theme::resolve(&table("name = \"dracula\""));
        assert_eq!(theme, Theme::default_ansi());
        assert_eq!(warnings.len(), 1);
        assert!(warnings[0].contains("dracula"));
        for name in BUILTIN_NAMES {
            assert!(warnings[0].contains(name), "warning should list {name}");
        }
    }

    #[test]
    fn invalid_value_keeps_theme_color() {
        let (theme, warnings) =
            Theme::resolve(&table("name = \"gruvbox-dark\"\naccent = \"#12345\""));
        assert_eq!(theme, Theme::gruvbox_dark());
        assert_eq!(warnings.len(), 1);
        assert!(warnings[0].contains("accent") && warnings[0].contains("#12345"));
    }

    #[test]
    fn non_string_value_is_invalid() {
        let (theme, warnings) = Theme::resolve(&table("accent = 3"));
        assert_eq!(theme, Theme::default_ansi());
        assert_eq!(warnings.len(), 1);
        assert!(warnings[0].contains("accent"));
    }

    #[test]
    fn unknown_key_is_ignored_with_warning() {
        let (theme, warnings) = Theme::resolve(&table("selected_fg = \"red\""));
        assert_eq!(theme, Theme::default_ansi());
        assert_eq!(warnings.len(), 1);
        assert!(warnings[0].contains("selected_fg"));
    }

    #[test]
    fn several_problems_are_all_reported() {
        let (_, warnings) = Theme::resolve(&table(
            "name = \"dracula\"\nfoo = \"red\"\naccent = \"purpleish\"",
        ));
        assert_eq!(warnings.len(), 3);
    }

    #[test]
    fn truecolor_warning_for_hex_theme_without_colorterm() {
        let w = truecolor_warning(&Theme::gruvbox_dark(), None).expect("warning");
        assert!(w.contains("gruvbox-dark") && w.contains("truecolor"));
    }

    #[test]
    fn no_truecolor_warning_in_truecolor_terminal() {
        assert_eq!(
            truecolor_warning(&Theme::gruvbox_dark(), Some("truecolor")),
            None
        );
        assert_eq!(
            truecolor_warning(&Theme::gruvbox_dark(), Some("24bit")),
            None
        );
    }

    #[test]
    fn no_truecolor_warning_for_ansi_theme() {
        assert_eq!(truecolor_warning(&Theme::default_ansi(), None), None);
    }

    #[test]
    fn hex_override_on_default_triggers_truecolor_warning() {
        let (theme, _) = Theme::resolve(&table("accent = \"#ff8800\""));
        assert!(truecolor_warning(&theme, None).is_some());
    }
}
