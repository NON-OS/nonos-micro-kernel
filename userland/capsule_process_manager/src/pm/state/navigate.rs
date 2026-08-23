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

use super::{Screen, Sort, State, FIRST_ROW_Y, ROW_H};

impl State {
    pub(super) fn selected_index(&self) -> Option<usize> {
        self.rows.iter().position(|r| r.pid == self.selected_pid)
    }

    // Keep row `idx` inside the visible window.
    pub(super) fn ensure_visible(&mut self, idx: usize) {
        if idx < self.scroll {
            self.scroll = idx;
        } else if self.visible > 0 && idx >= self.scroll + self.visible {
            self.scroll = idx + 1 - self.visible;
        }
    }

    // Select the process whose row the click landed on, accounting for scroll.
    pub fn select_at(&mut self, y: i32) {
        if y < FIRST_ROW_Y {
            return;
        }
        let idx = self.scroll + ((y - FIRST_ROW_Y) / ROW_H) as usize;
        if let Some(pid) = self.rows.get(idx).map(|r| r.pid) {
            self.disarm();
            self.selected_pid = pid;
        }
    }

    // Move the selection by `delta` rows, scrolling to follow it.
    pub fn move_selection(&mut self, delta: i32) {
        if self.rows.is_empty() {
            return;
        }
        self.disarm();
        let cur = self.selected_index().map(|i| i as i32).unwrap_or(-1);
        let next = (cur + delta).clamp(0, self.rows.len() as i32 - 1) as usize;
        self.selected_pid = self.rows[next].pid;
        self.ensure_visible(next);
    }

    // Scroll the window without moving the selection (wheel / page keys).
    pub fn scroll_by(&mut self, delta: i32) {
        let max = self.rows.len().saturating_sub(self.visible) as i32;
        self.scroll = (self.scroll as i32 + delta).clamp(0, max.max(0)) as usize;
    }

    // Any armed kill is cancelled so a confirmation never survives a screen
    // change.
    pub fn set_screen(&mut self, screen: Screen) {
        self.disarm();
        self.screen = screen;
    }

    // S keeps its old meaning: jump to Security, and back to the table from
    // there, so the shortcut that existed before the sidebar still works.
    pub fn toggle_security(&mut self) {
        let next = match self.screen {
            Screen::Security => Screen::Processes,
            _ => Screen::Security,
        };
        self.set_screen(next);
    }

    pub fn set_sort(&mut self, sort: Sort) {
        self.disarm();
        if self.sort != sort {
            self.sort = sort;
            self.refresh();
        }
    }
}
