//! Shared rendering helpers for TUI screens.
use ratatui::{
    style::{Color, Style},
    text::{Line, Span},
    widgets::ListItem,
};

/// Creates a [`ListItem`] with a left-aligned label and a right-aligned label,
/// separated by enough padding to fill `area_width` columns.
///
/// Used by [`super::screen::help_screen::HelpScreen`] to render the keybinding
/// table with consistent alignment regardless of terminal width.
pub fn create_split_item(
    left: &str,
    right: &str,
    color: Color,
    area_width: u16,
) -> ListItem<'static> {
    let left_len = left.chars().count();
    let right_len = right.chars().count();

    // The available width. We subtract 2 if your List has a Block with borders.
    let available_width = area_width.saturating_sub(2) as usize;

    let space_count = available_width.saturating_sub(left_len + right_len);
    let padding = " ".repeat(space_count.saturating_sub(4));

    let line = Line::from(vec![
        Span::raw(left.to_string()).style(Style::default().fg(color)),
        Span::raw(padding),
        Span::raw(right.to_string()),
    ]);

    ListItem::new(line)
}
