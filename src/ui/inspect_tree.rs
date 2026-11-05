use ratatui::{
    layout::{Constraint, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, List, ListItem, Paragraph},
    Frame,
};

use crate::app::App;
use crate::inspect_tree::{InspectTree, Kind};

pub fn render(f: &mut Frame, area: Rect, app: &App, name: &str, tree: Option<&InspectTree>) {
    // Leave the title and status bars visible: the search prompt lives in the latter.
    let popup = Rect::new(
        area.x,
        area.y + 1,
        area.width,
        area.height.saturating_sub(2),
    );
    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(app.theme.info))
        .title(format!(" Inspect · {name} "));
    let inner = block.inner(popup);

    f.render_widget(Clear, popup);
    f.render_widget(block.style(app.theme.base()), popup);

    let Some(tree) = tree else {
        f.render_widget(Paragraph::new("  loading…"), inner);
        return;
    };

    let detail = detail_lines(app, tree, inner);
    let [rows_area, detail_area] = Layout::vertical([
        Constraint::Min(1),
        Constraint::Length(detail.len() as u16 + 1),
    ])
    .areas(inner);

    render_rows(f, rows_area, app, tree);
    f.render_widget(
        Paragraph::new(detail).block(
            Block::default()
                .borders(Borders::TOP)
                .border_style(Style::default().fg(app.theme.border)),
        ),
        detail_area,
    );
}

fn render_rows(f: &mut Frame, area: Rect, app: &App, tree: &InspectTree) {
    let t = &app.theme;
    let visible = tree.visible();
    let height = area.height as usize;
    let pos = visible.iter().position(|&i| i == tree.cursor).unwrap_or(0);
    let offset = pos
        .saturating_sub(height / 2)
        .min(visible.len().saturating_sub(height));

    let items: Vec<ListItem> = visible
        .iter()
        .skip(offset)
        .take(height)
        .map(|&i| {
            let node = &tree.nodes[i];
            let marker = if !tree.has_children(i) {
                "  "
            } else if node.expanded {
                "▾ "
            } else {
                "▸ "
            };
            let (key_style, value_style) = if tree.is_match(i) {
                let hit = Style::default()
                    .fg(t.highlight)
                    .add_modifier(Modifier::BOLD);
                (hit, hit)
            } else {
                (
                    Style::default().fg(t.muted).add_modifier(Modifier::BOLD),
                    match node.kind {
                        Kind::Branch => Style::default().fg(t.muted),
                        Kind::Text => Style::default(),
                        Kind::Literal => Style::default().fg(t.info),
                    },
                )
            };
            let row = Line::from(vec![
                Span::raw(format!(" {}{marker}", "  ".repeat(node.depth))),
                Span::styled(node.key.as_str(), key_style),
                Span::raw("  "),
                Span::styled(node.text.as_str(), value_style),
            ]);
            if i == tree.cursor {
                ListItem::new(row).style(
                    Style::default()
                        .bg(t.selected_bg)
                        .add_modifier(Modifier::BOLD),
                )
            } else {
                ListItem::new(row)
            }
        })
        .collect();

    f.render_widget(List::new(items), area);
}

/// The cursor's path and untruncated value, capped at half of the popup.
fn detail_lines(app: &App, tree: &InspectTree, inner: Rect) -> Vec<Line<'static>> {
    let Some(node) = tree.nodes.get(tree.cursor) else {
        return vec![Line::from("")];
    };
    let mut lines = vec![Line::styled(
        format!(" {}", tree.path(tree.cursor)),
        Style::default().fg(app.theme.info),
    )];
    if node.kind != Kind::Branch {
        // shortcut: wraps by char count, so wide chars can clip; use unicode-width if that shows up
        let width = (inner.width as usize).saturating_sub(2).max(1);
        let chars: Vec<char> = node.text.chars().collect();
        let max = (inner.height as usize / 2).saturating_sub(2).max(1);
        lines.extend(
            chars
                .chunks(width)
                .take(max)
                .map(|chunk| Line::from(format!(" {}", chunk.iter().collect::<String>()))),
        );
    }
    lines
}
