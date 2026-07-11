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

//! Caret movement: one char left/right (UTF-8 aware), whole visual lines up and
//! down keeping the column, and to the start or end of the current visual line.

use super::byte_at::byte_at;
use super::position_at::position_at;
use super::state::State;

fn is_continuation(b: u8) -> bool {
    b & 0b1100_0000 == 0b1000_0000
}

impl State {
    pub fn caret_left(&mut self) {
        if self.caret == 0 {
            return;
        }
        let mut c = self.caret - 1;
        while c > 0 && is_continuation(self.buf[c]) {
            c -= 1;
        }
        self.caret = c;
    }

    pub fn caret_right(&mut self) {
        if self.caret >= self.len {
            return;
        }
        let mut c = self.caret + 1;
        while c < self.len && is_continuation(self.buf[c]) {
            c += 1;
        }
        self.caret = c;
    }

    pub fn caret_up_by(&mut self, n: u32) {
        let (line, col) = position_at(&self.buf[..self.len], self.wrap_cols, self.caret);
        let target = line.saturating_sub(n);
        self.caret = byte_at(&self.buf[..self.len], self.wrap_cols, target, col);
    }

    pub fn caret_down_by(&mut self, n: u32) {
        let (line, col) = position_at(&self.buf[..self.len], self.wrap_cols, self.caret);
        self.caret = byte_at(&self.buf[..self.len], self.wrap_cols, line + n, col);
    }

    // Smart Home: go to the first non-whitespace character of the line, or to
    // the true start if already there, the way code editors behave.
    pub fn caret_home(&mut self) {
        let (line, _) = position_at(&self.buf[..self.len], self.wrap_cols, self.caret);
        let bol = byte_at(&self.buf[..self.len], self.wrap_cols, line, 0);
        let mut first = bol;
        while first < self.len && matches!(self.buf[first], b' ' | b'\t') {
            first += 1;
        }
        self.caret = if self.caret == first { bol } else { first };
    }

    pub fn caret_end(&mut self) {
        let (line, _) = position_at(&self.buf[..self.len], self.wrap_cols, self.caret);
        self.caret = byte_at(&self.buf[..self.len], self.wrap_cols, line, u32::MAX);
    }
}
