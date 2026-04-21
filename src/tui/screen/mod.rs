//! Screen trait and sub-module declarations.
pub mod help_screen;
pub mod selection_screen;

use crossterm::event::KeyEvent;
use ratatui::Frame;

use crate::tui::Action;

/// Contract that every TUI screen must implement.
///
/// [`crate::tui::run_app`] dispatches to the active screen through this trait,
/// keeping the event loop decoupled from concrete screen types.
pub trait Screen {
    /// Draws the screen into the current `Frame`.
    fn render(&mut self, frame: &mut Frame);
    /// Processes a single key event and optionally returns an [`Action`] to
    /// be handled by the top-level event loop.
    fn event_loop(&mut self, event: &KeyEvent) -> Option<Action>;
}
