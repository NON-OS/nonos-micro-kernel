// NØNOS Operating System
// Copyright (C) 2026 NØNOS Contributors
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

use super::section::Section;

pub struct UiState {
    section: usize,
    rows: [usize; 7],
    pub elapsed_ms: u64,
    pub timeout_ms: u64,
    pub interacted: bool,
}

impl UiState {
    pub fn new(timeout_ms: u64) -> Self {
        Self { section: 0, rows: [0; 7], elapsed_ms: 0, timeout_ms, interacted: false }
    }
    pub fn section(&self) -> Section {
        Section::ALL[self.section]
    }
    pub fn row(&self) -> usize {
        self.rows[self.section]
    }
    fn set_row(&mut self, n: usize) {
        self.rows[self.section] = n;
    }
    pub fn next_section(&mut self) {
        self.section = (self.section + 1) % 7;
        self.touch();
    }
    pub fn prev_section(&mut self) {
        self.section = self.section.checked_sub(1).unwrap_or(6);
        self.touch();
    }
    pub fn move_up(&mut self, n: usize) {
        if n != 0 {
            let r = self.row();
            self.set_row(r.checked_sub(1).unwrap_or(n - 1));
            self.touch();
        }
    }
    pub fn move_down(&mut self, n: usize) {
        if n != 0 {
            let r = self.row();
            self.set_row((r + 1) % n);
            self.touch();
        }
    }
    pub fn clamp(&mut self, n: usize) {
        if n == 0 {
            self.set_row(0);
        } else if self.row() >= n {
            self.set_row(n - 1);
        }
    }
    pub fn touch(&mut self) {
        self.interacted = true;
        self.elapsed_ms = 0;
    }
    pub fn timed_out(&self) -> bool {
        self.timeout_ms != 0 && !self.interacted && self.elapsed_ms >= self.timeout_ms
    }
}
