use ratatui::{
    layout::Rect,
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, Paragraph},
    Frame,
};

use crate::theme::Theme;

pub fn render(f: &mut Frame, area: Rect, theme: &Theme, scroll: usize) {
    let lines = help_lines(theme);
    let popup = centered_rect(
        62,
        (lines.len() + 2).min(area.height as usize - 2) as u16,
        area,
    );

    f.render_widget(Clear, popup);
    f.render_widget(
        Paragraph::new(lines)
            .style(theme.base())
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(theme.accent))
                    .title(" Help  j/k scroll  ? or Esc close "),
            )
            .scroll((scroll as u16, 0)),
        popup,
    );
}

fn key(theme: &Theme, k: &'static str) -> Span<'static> {
    Span::styled(k, Style::default().fg(theme.info))
}

fn desc(d: &'static str) -> Span<'static> {
    Span::raw(d)
}

fn section(theme: &Theme, title: &'static str) -> Line<'static> {
    Line::from(Span::styled(
        title,
        Style::default()
            .fg(theme.accent)
            .add_modifier(Modifier::BOLD),
    ))
}

fn help_lines(theme: &Theme) -> Vec<Line<'static>> {
    vec![
        section(theme, "  Normal mode"),
        Line::from(vec![
            key(theme, "  j / k"),
            desc("           move down / up"),
        ]),
        Line::from(vec![key(theme, "  g / G"), desc("           top / bottom")]),
        Line::from(vec![
            key(theme, "  Ctrl-d / Ctrl-u"),
            desc("   half page down / up"),
        ]),
        Line::from(vec![
            key(theme, "  Tab"),
            desc("              cycle sections"),
        ]),
        Line::from(vec![
            key(theme, "  s"),
            desc("                start / stop"),
        ]),
        Line::from(vec![key(theme, "  r"), desc("                restart")]),
        Line::from(vec![
            key(theme, "  d"),
            desc("                delete (confirm)"),
        ]),
        Line::from(vec![key(theme, "  l  Enter"), desc("         logs")]),
        Line::from(vec![
            key(theme, "  e"),
            desc("                exec into container"),
        ]),
        Line::from(vec![
            key(theme, "  v"),
            desc("                visual (multi-select)"),
        ]),
        Line::from(vec![
            key(theme, "  y"),
            desc("                yank mode (y/i/d)"),
        ]),
        Line::from(vec![
            key(theme, "  K"),
            desc("                inspect selected"),
        ]),
        Line::from(vec![key(theme, "  /"), desc("                filter list")]),
        Line::from(vec![
            key(theme, "  :"),
            desc("                command mode"),
        ]),
        Line::from(vec![
            key(theme, "  R"),
            desc("                force refresh"),
        ]),
        Line::from(vec![
            key(theme, "  Esc"),
            desc("             clear active filter"),
        ]),
        Line::from(vec![key(theme, "  q"), desc("                quit")]),
        Line::from(vec![key(theme, "  ?"), desc("                this help")]),
        Line::from(""),
        section(theme, "  Visual mode  [v]"),
        Line::from(vec![
            key(theme, "  j / k"),
            desc("           extend selection"),
        ]),
        Line::from(vec![
            key(theme, "  d"),
            desc("                delete all selected"),
        ]),
        Line::from(vec![
            key(theme, "  s"),
            desc("                start/stop all selected"),
        ]),
        Line::from(vec![key(theme, "  Esc"), desc("             cancel")]),
        Line::from(""),
        section(theme, "  Log mode  [l / Enter]"),
        Line::from(vec![key(theme, "  j / k"), desc("           scroll")]),
        Line::from(vec![
            key(theme, "  G"),
            desc("                jump to latest"),
        ]),
        Line::from(vec![
            key(theme, "  f"),
            desc("                toggle follow mode"),
        ]),
        Line::from(vec![key(theme, "  q  Esc"), desc("          close")]),
        Line::from(""),
        section(theme, "  Exec mode  [e]"),
        Line::from(vec![key(theme, "  Enter"), desc("           run command")]),
        Line::from(vec![
            key(theme, "  Tab"),
            desc("              toggle interactive / one-shot"),
        ]),
        Line::from(vec![
            key(theme, "  ↑ / ↓"),
            desc("           command history"),
        ]),
        Line::from(vec![key(theme, "  Esc"), desc("             cancel")]),
        Line::from(""),
        section(theme, "  Yank mode  [y …]"),
        Line::from(vec![key(theme, "  y"), desc("                copy name")]),
        Line::from(vec![
            key(theme, "  i"),
            desc("                copy secondary (image / ID / mountpoint / subnet)"),
        ]),
        Line::from(vec![key(theme, "  d"), desc("                copy ID")]),
        Line::from(vec![key(theme, "  Esc"), desc("             cancel")]),
        Line::from(""),
        section(theme, "  Filter mode  [/]"),
        Line::from(vec![
            key(theme, "  Enter"),
            desc("           lock filter, return to Normal"),
        ]),
        Line::from(vec![key(theme, "  Esc"), desc("             cancel edit")]),
        Line::from(""),
        section(theme, "  Command mode  [:]"),
        Line::from(vec![key(theme, "  Enter"), desc("           execute")]),
        Line::from(vec![
            key(theme, "  ↑ / ↓"),
            desc("           command history"),
        ]),
        Line::from(vec![key(theme, "  Esc"), desc("             cancel")]),
        Line::from(""),
        section(theme, "  Clipboard  (OSC 52 — works over SSH)"),
        Line::from(vec![
            desc("  For tmux: "),
            key(theme, "set -g set-clipboard on"),
            desc(" in ~/.tmux.conf"),
        ]),
    ]
}

fn centered_rect(width: u16, height: u16, area: Rect) -> Rect {
    let x = area.x + area.width.saturating_sub(width) / 2;
    let y = area.y + area.height.saturating_sub(height) / 2;
    Rect::new(x, y, width.min(area.width), height.min(area.height))
}
