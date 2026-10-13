# rocker

A vim-style terminal UI for managing Docker containers, images, volumes and networks on a local or remote daemon.

## Language

### Theming

**Theme**:
A named, complete set of colors for every role, e.g. `gruvbox-dark`.
_Avoid_: Preset, colorscheme, skin

**Built-in theme**:
A theme that ships inside rocker and is selected by its name.
_Avoid_: Preset, bundled theme

**Role**:
A named color slot the UI draws with, such as `danger` or `bar_bg`; each theme assigns every role a color.
_Avoid_: Palette entry, color key, slot

**Override**:
A role set in the user's `[theme]` config that replaces the active theme's color for that role only.
_Avoid_: Custom color, theme setting
