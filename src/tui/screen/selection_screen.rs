//! The main problem-selection screen shown when the TUI starts.
//!
//! Implements fuzzy search, difficulty filtering, topic filtering, Vim-style
//! navigation, and premium-problem gating via the [`Screen`] trait.
use std::collections::HashSet;
use std::rc::Rc;

use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers};
use fuzzy_matcher::{FuzzyMatcher, skim::SkimMatcherV2};
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    text::Span,
    widgets::{
        Block, Borders, Cell, Clear, List, ListItem, ListState, Paragraph, Row, Table, TableState,
    },
};
use tui_input::{Input, backend::crossterm::EventHandler};

use crate::{
    models::{ProblemSummary, UserDetail},
    tui::{Action, screen::Screen},
};

/// Keyboard input mode for the selection screen.
pub enum InputMode {
    Editing,
    Normal,
    TopicFilter,
}

/// State for the topic-filter overlay.
pub struct TopicFilterState {
    /// Sorted list of all unique topic names derived from the loaded problem set.
    pub all_topics: Vec<String>,
    /// Topics currently selected by the user (OR semantics: problem must have at least one).
    pub selected_topics: HashSet<String>,
    /// Drives the scroll position in the overlay list widget.
    pub list_state: ListState,
}

impl TopicFilterState {
    pub fn new(problems: &[ProblemSummary]) -> Self {
        let mut set = HashSet::new();
        for p in problems.iter() {
            for t in &p.topics {
                set.insert(t.clone());
            }
        }
        let mut all_topics: Vec<String> = set.into_iter().collect();
        all_topics.sort();

        let mut list_state = ListState::default();
        if !all_topics.is_empty() {
            list_state.select(Some(0));
        }

        Self {
            all_topics,
            selected_topics: HashSet::new(),
            list_state,
        }
    }

    fn cursor(&self) -> usize {
        self.list_state.selected().unwrap_or(0)
    }

    pub fn next(&mut self) {
        if self.all_topics.is_empty() {
            return;
        }
        let i = self.cursor();
        let next = if i >= self.all_topics.len() - 1 {
            0
        } else {
            i + 1
        };
        self.list_state.select(Some(next));
    }

    pub fn previous(&mut self) {
        if self.all_topics.is_empty() {
            return;
        }
        let i = self.cursor();
        let prev = if i == 0 {
            self.all_topics.len() - 1
        } else {
            i - 1
        };
        self.list_state.select(Some(prev));
    }

    pub fn toggle_current(&mut self) {
        let cursor = self.cursor();
        if let Some(topic) = self.all_topics.get(cursor).cloned() {
            if self.selected_topics.contains(&topic) {
                self.selected_topics.remove(&topic);
            } else {
                self.selected_topics.insert(topic);
            }
        }
    }

    pub fn clear(&mut self) {
        self.selected_topics.clear();
    }
}

/// The problem-list screen: a searchable, filterable table of all problems.
pub struct SelectionScreen {
    /// The full, immutable problem list shared with [`App`].
    pub all_problems: Rc<[ProblemSummary]>,
    /// Indices into `all_problems` that survive the current search/filter.
    pub filtered_problems: Vec<usize>,
    pub table_state: TableState,
    pub selected_problem: Option<String>,
    /// Current contents of the search input box.
    pub input: Input,
    pub input_mode: InputMode,
    /// Active difficulty filter (`1`/`2`/`3`), or `None` for all.
    pub difficulty_filter: Option<u8>,
    /// Tracks the previous key for `gg` (jump-to-top) detection.
    pub previous_key: Option<KeyCode>,
    pub user_detail: Option<UserDetail>,
    /// State for the topic-filter overlay.
    pub topic_filter: TopicFilterState,
}

impl Screen for SelectionScreen {
    fn render(&mut self, frame: &mut Frame) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints([
                Constraint::Length(3),
                Constraint::Min(1),
                Constraint::Length(3),
            ])
            .split(frame.area());

        let title = format!(" Search ({} matches) ", self.filtered_problems.len());
        let input_widget = Paragraph::new(self.input.value())
            .style(match self.input_mode {
                InputMode::Editing => Style::default().fg(Color::Yellow),
                _ => Style::default(),
            })
            .block(Block::default().borders(Borders::ALL).title(title));

        frame.render_widget(input_widget, chunks[0]);

        // Handle blinking cursor in Editing mode
        if let InputMode::Editing = self.input_mode {
            // We set the cursor position right after the text
            frame.set_cursor_position((
                chunks[0].x + self.input.visual_cursor() as u16 + 1,
                chunks[0].y + 1,
            ));
        }

        let table_title = self.build_table_title();

        let header_cells = ["ID", "Name", "Acceptance", "Topics", "Premium?", "Done"]
            .into_iter()
            .map(|h| Cell::from(h).style(Style::default().fg(Color::Yellow)));
        let header = Row::new(header_cells).style(Style::default());

        let rows = self.filtered_problems.iter().map(|&p| {
            let p = &self.all_problems[p];
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
                p.title.as_str(),
                Style::default().fg(diff_color),
            ));
            let acceptance_text = format!("{:.1}%", p.acceptance * 100.0);
            let acceptance_cell = Cell::from(acceptance_text);
            let done_text = if let Some(status) = &p.status {
                match status.as_str() {
                    "ac" => "\u{f00c}",
                    "notac" => "\u{eabc}",
                    _ => "",
                }
            } else {
                ""
            };

            let done_cell = match done_text {
                "\u{f00c}" => Cell::from(done_text).style(Style::default().fg(Color::Green)),
                _ => Cell::from(done_text).style(Style::default().fg(Color::White)),
            };

            let premium_text = match &p.is_paid {
                true => "󰌾",
                false => "",
            };

            let premium_cell = Cell::from(premium_text).style(Style::default().fg(Color::Red));

            let topics_text = match p.topics.first() {
                Some(topic) => topic.as_str(),
                None => "",
            };
            let topics_cell = Cell::from(topics_text);

            Row::new(vec![
                id_cell,
                name_cell,
                acceptance_cell,
                topics_cell,
                premium_cell,
                done_cell,
            ])
        });

        let table = Table::new(
            rows,
            [
                Constraint::Length(6),
                Constraint::Percentage(45),
                Constraint::Min(10),
                Constraint::Fill(10),
                Constraint::Min(8),
                Constraint::Length(6),
            ],
        )
        .header(header)
        .block(
            Block::default()
                .title(table_title.as_str())
                .borders(Borders::ALL),
        )
        .row_highlight_style(Style::default().bg(Color::DarkGray).fg(Color::White))
        .highlight_symbol(">> ");

        frame.render_stateful_widget(table, chunks[1], &mut self.table_state);

        // Bottom status bar (3 single-line rows)
        let bottom_bar = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(1),
                Constraint::Length(1),
                Constraint::Length(1),
            ])
            .split(chunks[2]);

        let (instruction_text, instruction_style) = match self.input_mode {
            InputMode::Normal => (
                "Press '/' to search, 'j'/'k' to scroll, 'Enter' to select, 'o' to open in \
                browser, 'q' to quit.",
                Style::default().fg(Color::DarkGray),
            ),
            InputMode::Editing => (
                "Type to filter, press 'Esc' to return to list, press 'Enter' to select.",
                Style::default().fg(Color::Yellow),
            ),
            InputMode::TopicFilter => (
                "j/k: navigate   Space: toggle   c: clear all   Esc/Enter: close",
                Style::default().fg(Color::Cyan),
            ),
        };
        frame.render_widget(
            Paragraph::new(instruction_text).style(instruction_style),
            bottom_bar[0],
        );

        if let InputMode::Normal = self.input_mode {
            frame.render_widget(
                Paragraph::new("1: Easy  2: Medium  3: Hard  4: All  |  t: Topic filter")
                    .style(Style::default().fg(Color::DarkGray)),
                bottom_bar[1],
            );
        }

        // Active topic status line (row 2)
        let topic_status_widget = if self.topic_filter.selected_topics.is_empty() {
            Paragraph::new("Press ? to view help.").style(Style::default().fg(Color::DarkGray))
        } else {
            let mut names: Vec<&str> = self
                .topic_filter
                .selected_topics
                .iter()
                .map(|s| s.as_str())
                .collect();
            names.sort();
            let display = if names.len() <= 3 {
                format!("Topics: {}", names.join(", "))
            } else {
                format!(
                    "Topics: {}, ... (+{} more)",
                    names[..2].join(", "),
                    names.len() - 2
                )
            };
            Paragraph::new(display).style(Style::default().fg(Color::Cyan))
        };
        frame.render_widget(topic_status_widget, bottom_bar[2]);

        // Render topic-filter overlay on top of everything else
        if let InputMode::TopicFilter = self.input_mode {
            self.render_topic_overlay(frame);
        }
    }

    fn event_loop(&mut self, key_event: &KeyEvent) -> Option<Action> {
        // Topic filter overlay intercepts all keys while active
        if let InputMode::TopicFilter = self.input_mode {
            return self.handle_topic_filter_key(key_event);
        }

        if let KeyCode::Enter = key_event.code {
            if let Some(i) = self.table_state.selected()
                && !self.filtered_problems.is_empty()
            {
                let index = self.filtered_problems[i];
                let selected_problem = &self.all_problems[index];
                if let Some(user) = &self.user_detail {
                    if let Some(is_premium) = user.is_premium
                        && selected_problem.is_paid
                        && !is_premium
                    {
                        return Some(Action::ShowMessage(
                            "This problem is premium. please subscribe to access it.".to_string(),
                        ));
                    }
                } else if selected_problem.is_paid {
                    return Some(Action::ShowMessage(
                        "This problem is premium. please login to access it. (use `leetrs auth`"
                            .to_string(),
                    ));
                }

                return Some(Action::Select(self.all_problems[index].slug.clone()));
            }
        }

        match self.input_mode {
            InputMode::Normal => match key_event.code {
                KeyCode::Char('q') | KeyCode::Esc => return Some(Action::Quit),
                KeyCode::Down | KeyCode::Char('j') => self.next(),
                KeyCode::Up | KeyCode::Char('k') => self.previous(),
                KeyCode::Left | KeyCode::Char('h') => self.table_state.select_next_column(),
                KeyCode::Right | KeyCode::Char('l') => self.table_state.select_previous_column(),
                KeyCode::Char('/') => self.input_mode = InputMode::Editing,
                KeyCode::Char('t') => self.input_mode = InputMode::TopicFilter,
                KeyCode::Char('o') => {
                    if let Some(i) = self.table_state.selected()
                        && !self.filtered_problems.is_empty()
                    {
                        let index = self.filtered_problems[i];
                        let selected = &self.all_problems[index];
                        let url = format!("https://leetcode.com/problems/{}", selected.slug);
                        self.input_mode = InputMode::Normal;
                        return Some(Action::Open(url));
                    }
                }
                KeyCode::Char('g') => {
                    if let Some(prev_key) = self.previous_key
                        && prev_key == KeyCode::Char('g')
                    {
                        self.table_state.select(Some(0));
                    }
                }
                KeyCode::Char('G') => {
                    self.table_state.select_last();
                }
                KeyCode::Char('d') => {
                    self.table_state.scroll_down_by(10);
                }
                KeyCode::Char('u') => {
                    self.table_state.scroll_up_by(10);
                }
                KeyCode::Char(c) => {
                    if let Some(number) = c.to_digit(10) {
                        self.switch_difficulty(number as u8);
                    }
                }
                _ => {}
            },

            InputMode::Editing => match key_event.code {
                KeyCode::Esc => {
                    self.input_mode = InputMode::Normal;
                }
                KeyCode::Char('j') if key_event.modifiers.contains(KeyModifiers::CONTROL) => {
                    self.next();
                }
                KeyCode::Char('k') if key_event.modifiers.contains(KeyModifiers::CONTROL) => {
                    self.previous();
                }
                _ => {
                    self.input.handle_event(&Event::Key(*key_event));
                    self.apply_filters();
                }
            },

            InputMode::TopicFilter => unreachable!(),
        }
        self.previous_key = Some(key_event.code);
        None
    }
}

impl SelectionScreen {
    pub fn new(problems: Rc<[ProblemSummary]>, user_detail: Option<UserDetail>) -> Self {
        let mut list_state = TableState::default();
        if !problems.is_empty() {
            list_state.select(Some(0)); // Start by highlighting the first item
        }

        let topic_filter = TopicFilterState::new(&problems);

        Self {
            selected_problem: None,
            filtered_problems: (0..problems.len()).collect(),
            topic_filter,
            all_problems: problems,
            table_state: list_state,
            input: Input::default(),
            input_mode: InputMode::Normal,
            difficulty_filter: None,
            previous_key: None,
            user_detail,
        }
    }

    pub fn next(&mut self) {
        let len = self.filtered_problems.len();
        if len == 0 {
            return;
        }
        let i = match self.table_state.selected() {
            Some(i) => {
                if i >= len - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.table_state.select(Some(i));
    }

    // Move cursor up
    pub fn previous(&mut self) {
        let len = self.filtered_problems.len();
        if len == 0 {
            return;
        }
        let i = match self.table_state.selected() {
            Some(i) => {
                if i == 0 {
                    len - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.table_state.select(Some(i));
    }

    /// Sets the difficulty filter and re-applies all active filters.
    /// Passing `4` (or any value > 3) clears the difficulty filter.
    pub fn switch_difficulty(&mut self, difficulty: u8) {
        if difficulty > 0 && difficulty < 4 {
            self.difficulty_filter = Some(difficulty);
        } else {
            self.difficulty_filter = None;
        }
        self.apply_filters();
    }

    /// Rebuilds `filtered_problems` by applying difficulty, topic, and fuzzy-search
    /// filters in a single pass. Replaces the old separate `filter_problems` /
    /// `update_search` pair and fixes the bug where clearing search ignored the
    /// active difficulty filter.
    pub fn apply_filters(&mut self) {
        self.filtered_problems.clear();

        let query = self.input.value();
        let has_topics = !self.topic_filter.selected_topics.is_empty();

        let candidates = self.all_problems.iter().enumerate().filter(|(_, p)| {
            if let Some(diff) = self.difficulty_filter {
                if p.difficulty != diff {
                    return false;
                }
            }

            if has_topics {
                let matches_topic = p
                    .topics
                    .iter()
                    .any(|t| self.topic_filter.selected_topics.contains(t));
                if !matches_topic {
                    return false;
                }
            }

            true
        });

        if query.is_empty() {
            self.filtered_problems
                .extend(candidates.map(|(idx, _)| idx));
        } else {
            let matcher = SkimMatcherV2::default();
            let mut scored: Vec<(i64, usize)> = Vec::with_capacity(self.all_problems.len());

            for (idx, p) in candidates {
                if let Some(score) = matcher
                    .fuzzy_match(&p.title, query)
                    .or_else(|| matcher.fuzzy_match(&p.id.to_string(), query))
                {
                    scored.push((score, idx));
                }
            }

            scored.sort_unstable_by(|a, b| b.0.cmp(&a.0));
            self.filtered_problems
                .extend(scored.into_iter().map(|(_, idx)| idx));
        }

        if !self.filtered_problems.is_empty() {
            self.table_state.select(Some(0));
        } else {
            self.table_state.select(None);
        }
    }

    /// Thin wrapper kept for compatibility — delegates to [`apply_filters`].
    pub fn filter_problems(&mut self) {
        self.apply_filters();
    }

    /// Thin wrapper kept for compatibility — delegates to [`apply_filters`].
    pub fn update_search(&mut self) {
        self.apply_filters();
    }

    fn handle_topic_filter_key(&mut self, key_event: &KeyEvent) -> Option<Action> {
        match key_event.code {
            KeyCode::Esc | KeyCode::Enter => {
                self.input_mode = InputMode::Normal;
            }
            KeyCode::Down | KeyCode::Char('j') => {
                self.topic_filter.next();
            }
            KeyCode::Up | KeyCode::Char('k') => {
                self.topic_filter.previous();
            }
            KeyCode::Char(' ') => {
                self.topic_filter.toggle_current();
                self.apply_filters();
            }
            KeyCode::Char('c') => {
                self.topic_filter.clear();
                self.apply_filters();
            }
            _ => {}
        }
        None
    }

    fn render_topic_overlay(&mut self, frame: &mut Frame) {
        let overlay_area = frame
            .area()
            .centered(Constraint::Percentage(70), Constraint::Percentage(80));

        frame.render_widget(Clear, overlay_area);

        let selected_count = self.topic_filter.selected_topics.len();
        let title = if selected_count == 0 {
            " Topic Filter — Space: toggle  c: clear  Esc: close ".to_string()
        } else {
            format!(
                " Topic Filter ({} selected) — Space: toggle  c: clear  Esc: close ",
                selected_count
            )
        };

        let block = Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Cyan))
            .title(title.as_str());

        let inner = block.inner(overlay_area);
        frame.render_widget(block, overlay_area);

        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(1), Constraint::Length(1)])
            .split(inner);

        if self.topic_filter.all_topics.is_empty() {
            frame.render_widget(
                Paragraph::new("No topics available — ensure the problem list is fully loaded.")
                    .style(Style::default().fg(Color::DarkGray)),
                layout[0],
            );
        } else {
            let items: Vec<ListItem> = self
                .topic_filter
                .all_topics
                .iter()
                .map(|t| {
                    let (prefix, color) = if self.topic_filter.selected_topics.contains(t) {
                        ("[x] ", Color::Green)
                    } else {
                        ("[ ] ", Color::White)
                    };
                    ListItem::new(format!("{}{}", prefix, t)).style(Style::default().fg(color))
                })
                .collect();

            let list = List::new(items)
                .highlight_style(Style::default().bg(Color::DarkGray).fg(Color::White))
                .highlight_symbol(">> ");

            frame.render_stateful_widget(list, layout[0], &mut self.topic_filter.list_state);
        }

        let hint = if self.filtered_problems.is_empty() {
            Paragraph::new("No problems match current filters")
                .style(Style::default().fg(Color::Red))
        } else {
            Paragraph::new(format!("{} problems match", self.filtered_problems.len()))
                .style(Style::default().fg(Color::DarkGray))
        };
        frame.render_widget(hint, layout[1]);
    }

    fn build_table_title(&self) -> String {
        let diff_part = match self.difficulty_filter {
            Some(1) => " (Easy)",
            Some(2) => " (Medium)",
            Some(3) => " (Hard)",
            _ => "",
        };

        let topic_part = match self.topic_filter.selected_topics.len() {
            0 => String::new(),
            n => {
                let mut names: Vec<&str> = self
                    .topic_filter
                    .selected_topics
                    .iter()
                    .map(|s| s.as_str())
                    .collect();
                names.sort();
                if n <= 2 {
                    format!(" [{}]", names.join(", "))
                } else {
                    format!(" [{}, +{}]", names[..2].join(", "), n - 2)
                }
            }
        };

        format!(" Problems{}{} ", diff_part, topic_part)
    }
}
