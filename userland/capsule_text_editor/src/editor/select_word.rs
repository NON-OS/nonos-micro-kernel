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

//! Select the word under the caret, for double-click. A word is a maximal run
//! of word characters (alphanumeric, `_`, or non-ASCII); on punctuation the
//! single character is selected so a double-click always grabs something.

use super::state::State;

fn is_word(b: u8) -> bool {
    b.is_ascii_alphanumeric() || b == b'_' || b >= 0x80
}

impl State {
    pub fn select_word(&mut self) {
        if self.len == 0 {
            return;
        }
        let at = self.caret.min(self.len.saturating_sub(1));
        if !is_word(self.buf[at]) {
            if self.buf[at] != b'\n' {
                self.sel_anchor = Some(at);
                self.caret = at + 1;
            }
            return;
        }
        let mut s = at;
        while s > 0 && is_word(self.buf[s - 1]) {
            s -= 1;
        }
        let mut e = at;
        while e < self.len && is_word(self.buf[e]) {
            e += 1;
        }
        self.sel_anchor = Some(s);
        self.caret = e;
    }
}
