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
use super::ui::metrics::SCROLL_STEP;

// Scroll is in pixels, not rows: the screens are cards and gauges of differing
// heights, so a row index no longer names a position on any of them. The frame
// funnel records the pane height and the active screen's content height on the
// way through, which is the only place both are known at once.
pub struct State {
    pub section: Section,
    pub scroll: u32,
    pub fb_w: u32,
    pub fb_h: u32,
    pub view_h: u32,
    pub content_h: u32,
}

impl State {
    pub fn new() -> Self {
        State { section: Section::Overview, scroll: 0, fb_w: 0, fb_h: 0, view_h: 0, content_h: 0 }
    }
    pub fn record_extent(&mut self, view_h: u32, content_h: u32) {
        self.view_h = view_h;
        self.content_h = content_h;
        self.scroll = self.scroll.min(self.max_scroll());
    }
    pub fn max_scroll(&self) -> u32 {
        self.content_h.saturating_sub(self.view_h)
    }
    pub fn select(&mut self, section: Section) -> bool {
        if self.section == section {
            return false;
        }
        self.section = section;
        self.scroll = 0;
        true
    }
    pub fn select_next_section(&mut self) {
        let next = (self.section.index() + 1) % SECTIONS.len();
        self.select(SECTIONS[next]);
    }
    pub fn select_prev_section(&mut self) {
        let prev = (self.section.index() + SECTIONS.len() - 1) % SECTIONS.len();
        self.select(SECTIONS[prev]);
    }
    pub fn scroll_line_up(&mut self) {
        self.scroll = self.scroll.saturating_sub(SCROLL_STEP);
    }
    pub fn scroll_line_down(&mut self) {
        self.scroll = (self.scroll + SCROLL_STEP).min(self.max_scroll());
    }
    pub fn scroll_page_up(&mut self) {
        self.scroll = self.scroll.saturating_sub(self.view_h.max(SCROLL_STEP));
    }
    pub fn scroll_page_down(&mut self) {
        self.scroll = (self.scroll + self.view_h.max(SCROLL_STEP)).min(self.max_scroll());
    }
}
