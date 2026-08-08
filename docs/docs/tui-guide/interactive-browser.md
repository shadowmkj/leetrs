---
id: interactive-browser
title: Interactive TUI Browser
sidebar_position: 1
---

# 🖥️ Interactive TUI Browser

`leetrs` provides a feature-rich, interactive Terminal User Interface (TUI) powered by **Ratatui** and **crossterm**.

---

## 🎹 Keyboard Controls & Shortcuts

### Navigation & Problem Selection

| Key | Action |
|---|---|
| `j` / `Down` | Move table selection down one row |
| `k` / `Up` | Move table selection up one row |
| `g g` | Jump directly to top of problem list |
| `G` | Jump directly to bottom of problem list |
| `Ctrl + d` | Scroll down half-page |
| `Ctrl + u` | Scroll up half-page |
| `Enter` | Select highlighted problem and launch configured editor |
| `o` | Open selected problem directly in default system web browser |

---

### Filtering & Visual Overlays

| Key | Action |
|---|---|
| `/` | Activate fuzzy search input bar (search by title or numeric ID) |
| `1` | Filter by **Easy** difficulty |
| `2` | Filter by **Medium** difficulty |
| `3` | Filter by **Hard** difficulty |
| `4` | Clear difficulty filter (show all difficulties) |
| `t` | Open **Topic Overlay Filter Modal** |
| `Tab` | Cycle focus between UI widgets |
| `?` | Toggle interactive **Help Screen Overlay** |
| `Esc` | Clear search query / close active popup or modal |
| `q` | Quit TUI |

---

## 🎨 Visual Indicators

The table displays rich visual feedback for every problem:

- **Status Column**:
  - `✔` (Green) — Solved / Accepted (`ac`)
  - `🔒` (Yellow) — Paid / Premium problem gate
- **Difficulty Column**:
  - `Easy` — Styled in Vibrant Green
  - `Medium` — Styled in Yellow
  - `Hard` — Styled in Red
- **Acceptance Column**: Formatted percentage (e.g. `52.4%`).
