//! Problem table widget — owns table scroll state and navigation.
use ratatui::widgets::TableState;

/// Manages the table cursor position and scroll state for the problem list.
pub struct ProblemTable {
    pub state: TableState,
    pub len: usize,
}

impl ProblemTable {
    pub fn new(len: usize) -> Self {
        let mut state = TableState::default();
        if len > 0 {
            state.select(Some(0));
        }
        Self { state, len }
    }

    pub fn update_len(&mut self, len: usize) {
        self.len = len;
        if len == 0 {
            self.state.select(None);
        } else if self.state.selected().is_none_or(|i| i >= len) {
            self.state.select(Some(0));
        }
    }

    pub fn next(&mut self) {
        if self.len == 0 {
            return;
        }
        let i = match self.state.selected() {
            Some(i) if i >= self.len - 1 => 0,
            Some(i) => i + 1,
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn previous(&mut self) {
        if self.len == 0 {
            return;
        }
        let i = match self.state.selected() {
            Some(0) | None => self.len - 1,
            Some(i) => i - 1,
        };
        self.state.select(Some(i));
    }

    pub fn select_first(&mut self) {
        if self.len > 0 {
            self.state.select(Some(0));
        }
    }

    pub fn select_last(&mut self) {
        self.state.select_last();
    }

    pub fn scroll_down(&mut self, n: u16) {
        self.state.scroll_down_by(n);
    }

    pub fn scroll_up(&mut self, n: u16) {
        self.state.scroll_up_by(n);
    }
}
