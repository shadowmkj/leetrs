//! Shared rendering utilities for the problem-selection TUI.
use ratatui::{
    style::{Color, Style},
    text::Span,
    widgets::{Cell, Row},
};

use crate::models::ProblemSummary;

/// Converts a [`ProblemSummary`] into a [`Row`] for the problems table.
///
/// Extracted from the inline lambda in `SelectionScreen::render()` so it can
/// be unit-tested and reused across views.
pub fn render_problem_row(p: &ProblemSummary) -> Row<'static> {
    let diff_color = match p.difficulty {
        1 => Color::Green,
        2 => Color::Yellow,
        _ => Color::Red,
    };

    let id_cell = Cell::from(Span::styled(
        format!("[{}]", p.id),
        Style::default().fg(diff_color),
    ));
    let name_cell = Cell::from(Span::styled(
        p.title.clone(),
        Style::default().fg(diff_color),
    ));
    let acceptance_text = format!("{:.1}%", p.acceptance * 100.0);
    let acceptance_cell = Cell::from(acceptance_text);

    let (done_text, done_color) = match p.status.as_deref() {
        Some("ac") => ("\u{f00c}", Color::Green),
        Some("notac") => ("\u{eabc}", Color::White),
        _ => ("", Color::White),
    };
    let done_cell = Cell::from(done_text).style(Style::default().fg(done_color));

    let premium_text = if p.is_paid { "󰌾" } else { "" };
    let premium_cell = Cell::from(premium_text).style(Style::default().fg(Color::Red));

    let topics_text = p.topics.first().map(|s| s.as_str()).unwrap_or("");
    let topics_cell = Cell::from(topics_text.to_string());

    Row::new(vec![
        id_cell,
        name_cell,
        acceptance_cell,
        topics_cell,
        premium_cell,
        done_cell,
    ])
}
