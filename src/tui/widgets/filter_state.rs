//! Filter state — difficulty, topic, and fuzzy-search filter logic.
use std::collections::HashSet;

use fuzzy_matcher::{FuzzyMatcher, skim::SkimMatcherV2};
use ratatui::widgets::ListState;

use crossterm::event::KeyEvent;
use tui_input::{Input, backend::crossterm::EventHandler};

use crate::models::ProblemSummary;

// ==============================================================================
// Topic Catalog Definition
// ==============================================================================

// Embed the complete topic catalog at compile time from `topics.txt` at the root
// of the repository. This avoids looping through the entire problem set at runtime
// just to build the list of filterable topics.
const TOPICS_TXT: &str = include_str!("../../../topics.txt");

static STATIC_TOPICS: std::sync::LazyLock<Vec<String>> = std::sync::LazyLock::new(|| {
    let mut topics: Vec<String> = TOPICS_TXT
        .lines()
        .map(|line| line.trim().trim_matches('"').to_string())
        .filter(|s| !s.is_empty())
        .collect();
    topics.sort();
    topics
});

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TopicInputMode {
    #[default]
    Normal,
    Editing,
}

/// Topic filter overlay state — a sorted list of all known topics and the
/// currently selected subset.
pub struct TopicFilterState {
    pub all_topics: Vec<String>,
    pub selected_topics: HashSet<String>,
    pub list_state: ListState,
    pub search_input: Input,
    pub filtered_topics: Vec<String>,
    pub mode: TopicInputMode,
}

impl TopicFilterState {
    /// Constructs a new [`TopicFilterState`] populated with topics loaded
    /// directly from `topics.txt`.
    pub fn new() -> Self {
        let all_topics = STATIC_TOPICS.clone();

        let mut list_state = ListState::default();
        if !all_topics.is_empty() {
            list_state.select(Some(0));
        }

        let filtered_topics = all_topics.clone();

        Self {
            all_topics,
            selected_topics: HashSet::new(),
            list_state,
            search_input: Input::default(),
            filtered_topics,
            mode: TopicInputMode::Normal,
        }
    }

    pub fn cursor(&self) -> usize {
        self.list_state.selected().unwrap_or(0)
    }

    pub fn update_filter(&mut self) {
        let query = self.search_input.value().to_lowercase();
        if query.is_empty() {
            self.filtered_topics = self.all_topics.clone();
        } else {
            self.filtered_topics = self
                .all_topics
                .iter()
                .filter(|t| t.to_lowercase().contains(&query))
                .cloned()
                .collect();
        }

        if self.filtered_topics.is_empty() {
            self.list_state.select(None);
        } else {
            let curr = self.list_state.selected().unwrap_or(0);
            if curr >= self.filtered_topics.len() || self.list_state.selected().is_none() {
                self.list_state.select(Some(0));
            }
        }
    }

    pub fn handle_key(&mut self, key_event: &KeyEvent) {
        self.search_input
            .handle_event(&crossterm::event::Event::Key(*key_event));
        self.update_filter();
    }

    pub fn clear_search(&mut self) {
        self.search_input = Input::default();
        self.update_filter();
    }

    pub fn next(&mut self) {
        if self.filtered_topics.is_empty() {
            return;
        }
        let i = self.cursor();
        let next = if i >= self.filtered_topics.len() - 1 {
            0
        } else {
            i + 1
        };
        self.list_state.select(Some(next));
    }

    pub fn previous(&mut self) {
        if self.filtered_topics.is_empty() {
            return;
        }
        let i = self.cursor();
        let prev = if i == 0 {
            self.filtered_topics.len() - 1
        } else {
            i - 1
        };
        self.list_state.select(Some(prev));
    }

    pub fn scroll_down(&mut self, n: usize) {
        if self.filtered_topics.is_empty() {
            return;
        }
        let max_idx = self.filtered_topics.len() - 1;
        let next = (self.cursor() + n).min(max_idx);
        self.list_state.select(Some(next));
    }

    pub fn scroll_up(&mut self, n: usize) {
        if self.filtered_topics.is_empty() {
            return;
        }
        let prev = self.cursor().saturating_sub(n);
        self.list_state.select(Some(prev));
    }

    pub fn toggle_current(&mut self) {
        let cursor = self.cursor();
        if let Some(topic) = self.filtered_topics.get(cursor).cloned() {
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

impl Default for TopicFilterState {
    fn default() -> Self {
        Self::new()
    }
}

/// Encapsulates difficulty, topic, and search filters.
///
/// Call [`FilterState::apply`] to get the set of problem indices that pass all
/// active filters in one efficient pass.
pub struct FilterState {
    pub difficulty: Option<u8>,
    pub topics: TopicFilterState,
}

impl FilterState {
    pub fn new() -> Self {
        Self {
            difficulty: None,
            topics: TopicFilterState::new(),
        }
    }
    /// Returns the sorted list of indices into `problems` that survive all
    /// active filters and the given search query.
    pub fn apply(&self, problems: &[ProblemSummary], query: &str) -> Vec<usize> {
        let has_topics = !self.topics.selected_topics.is_empty();

        let candidates = problems.iter().enumerate().filter(|(_, p)| {
            if let Some(diff) = self.difficulty
                && p.difficulty != diff
            {
                return false;
            }
            if has_topics {
                let matches_topic = p
                    .topics
                    .iter()
                    .any(|t| self.topics.selected_topics.contains(t));
                if !matches_topic {
                    return false;
                }
            }
            true
        });

        if query.is_empty() {
            candidates.map(|(idx, _)| idx).collect()
        } else {
            let matcher = SkimMatcherV2::default();
            let mut scored: Vec<(i64, usize)> = Vec::with_capacity(problems.len());
            for (idx, p) in candidates {
                if let Some(score) = matcher
                    .fuzzy_match(&p.title, query)
                    .or_else(|| matcher.fuzzy_match(&p.id.to_string(), query))
                {
                    scored.push((score, idx));
                }
            }
            scored.sort_unstable_by_key(|b| std::cmp::Reverse(b.0));
            scored.into_iter().map(|(_, idx)| idx).collect()
        }
    }

    pub fn set_difficulty(&mut self, difficulty: u8) {
        if difficulty > 0 && difficulty < 4 {
            self.difficulty = Some(difficulty);
        } else {
            self.difficulty = None;
        }
    }
}

impl Default for FilterState {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crossterm::event::{KeyCode, KeyModifiers};

    #[test]
    fn topic_filter_state_loads_topics_from_file() {
        let state = TopicFilterState::new();
        assert!(!state.all_topics.is_empty());
        assert!(state.all_topics.contains(&"Array".to_string()));
        assert!(
            state
                .all_topics
                .contains(&"Dynamic Programming".to_string())
        );
        assert_eq!(state.cursor(), 0);
    }

    #[test]
    fn topic_filter_state_navigation_and_toggle() {
        let mut state = TopicFilterState::new();
        let first_topic = state.all_topics[0].clone();

        state.toggle_current();
        assert!(state.selected_topics.contains(&first_topic));

        state.toggle_current();
        assert!(!state.selected_topics.contains(&first_topic));

        state.next();
        assert_eq!(state.cursor(), 1);

        state.previous();
        assert_eq!(state.cursor(), 0);

        state.toggle_current();
        state.clear();
        assert!(state.selected_topics.is_empty());
    }

    #[test]
    fn topic_filter_state_scroll_down_and_up() {
        let mut state = TopicFilterState::new();
        assert!(!state.all_topics.is_empty());

        let initial = state.cursor();
        state.scroll_down(10);
        assert_eq!(
            state.cursor(),
            (initial + 10).min(state.all_topics.len() - 1)
        );

        state.scroll_up(10);
        assert_eq!(state.cursor(), initial);
    }

    #[test]
    fn filter_state_apply_with_topic_filter() {
        let mut filters = FilterState::default();
        let problems = vec![
            ProblemSummary {
                id: 1,
                acceptance: 50.0,
                accepted: 100,
                difficulty: 1,
                slug: "two-sum".to_string(),
                status: None,
                submitted: 200,
                title: "Two Sum".to_string(),
                is_paid: false,
                topics: vec!["Array".to_string(), "Hash Table".to_string()],
            },
            ProblemSummary {
                id: 2,
                acceptance: 40.0,
                accepted: 80,
                difficulty: 2,
                slug: "add-two-numbers".to_string(),
                status: None,
                submitted: 200,
                title: "Add Two Numbers".to_string(),
                is_paid: false,
                topics: vec!["Linked List".to_string(), "Math".to_string()],
            },
        ];

        // Initially no topic selected, returns all problems
        let matched = filters.apply(&problems, "");
        assert_eq!(matched, vec![0, 1]);

        // Filter by "Array" topic
        filters.topics.selected_topics.insert("Array".to_string());
        let matched = filters.apply(&problems, "");
        assert_eq!(matched, vec![0]);
    }

    #[test]
    fn test_topic_filter_search_filtering() {
        let mut state = TopicFilterState::new();
        let total = state.all_topics.len();
        assert!(total > 0);
        assert_eq!(state.filtered_topics.len(), total);

        // Type "array" into search input
        let keys = ['a', 'r', 'r', 'a', 'y'];
        for c in keys {
            let key = KeyEvent::new(KeyCode::Char(c), KeyModifiers::NONE);
            state.handle_key(&key);
        }

        assert_eq!(state.search_input.value(), "array");
        assert_eq!(state.filtered_topics, vec!["Array", "Suffix Array"]);

        // Clear search
        state.clear_search();
        assert_eq!(state.search_input.value(), "");
        assert_eq!(state.filtered_topics.len(), total);
    }
}
