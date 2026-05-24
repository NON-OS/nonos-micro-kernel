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
use super::theme::{HEADER_HEIGHT, LINE_HEIGHT, SECTION_TOP_PADDING, STATUS_BAR_HEIGHT, TAB_BAR_HEIGHT};

pub const VISIBLE_BODY_LINES: u32 = 14;

pub struct State {
    pub section: Section,
    pub scroll: u32,
    pub painted: bool,
    pub last_visible_lines: u32,
}

impl State {
    pub fn new() -> Self {
        State { section: Section::Identity, scroll: 0, painted: false, last_visible_lines: VISIBLE_BODY_LINES }
    }
    pub fn record_visible_lines(&mut self, lines: u32) {
        self.last_visible_lines = lines.max(1);
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
        let max = total_lines.saturating_sub(self.last_visible_lines);
        if self.scroll < max {
            self.scroll += 1;
        }
    }
    pub fn scroll_page_up(&mut self) {
        self.scroll = self.scroll.saturating_sub(self.last_visible_lines);
    }
    pub fn scroll_page_down(&mut self, total_lines: u32) {
        let max = total_lines.saturating_sub(self.last_visible_lines);
        self.scroll = (self.scroll + self.last_visible_lines).min(max);
    }
}

pub fn visible_lines_for(height: u32) -> u32 {
    let body_top = HEADER_HEIGHT.saturating_add(TAB_BAR_HEIGHT).saturating_add(SECTION_TOP_PADDING);
    let body_height = height.saturating_sub(body_top).saturating_sub(STATUS_BAR_HEIGHT);
    (body_height / LINE_HEIGHT).max(1)
}
