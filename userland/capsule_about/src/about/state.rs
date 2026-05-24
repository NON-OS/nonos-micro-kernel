// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

use super::section::{Section, SECTIONS};

pub const VISIBLE_BODY_LINES: u32 = 14;

pub struct State {
    pub section: Section,
    pub scroll: u32,
    pub painted: bool,
}

impl State {
    pub fn new() -> Self {
        State { section: Section::Identity, scroll: 0, painted: false }
    }
    pub fn select_next_section(&mut self) {
        let next = (self.section.index() + 1) % SECTIONS.len();
        self.section = SECTIONS[next];
        self.scroll = 0;
    }
    pub fn select_prev_section(&mut self) {
        let prev = (self.section.index() + SECTIONS.len() - 1) % SECTIONS.len();
        self.section = SECTIONS[prev];
        self.scroll = 0;
    }
    pub fn scroll_line_up(&mut self) {
        self.scroll = self.scroll.saturating_sub(1);
    }
    pub fn scroll_line_down(&mut self, total_lines: u32) {
        let max = total_lines.saturating_sub(VISIBLE_BODY_LINES);
        if self.scroll < max {
            self.scroll += 1;
        }
    }
    pub fn scroll_page_up(&mut self) {
        self.scroll = self.scroll.saturating_sub(VISIBLE_BODY_LINES);
    }
    pub fn scroll_page_down(&mut self, total_lines: u32) {
        let max = total_lines.saturating_sub(VISIBLE_BODY_LINES);
        self.scroll = (self.scroll + VISIBLE_BODY_LINES).min(max);
    }
}
