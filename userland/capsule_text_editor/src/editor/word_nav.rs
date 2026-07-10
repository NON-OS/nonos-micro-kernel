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

//! Whole-word caret motion (Ctrl+Left/Right) and word deletion (Ctrl+Backspace).
//! A word run is a maximal span of one character class: whitespace, word
//! characters (alphanumeric or `_`), or punctuation. Moving skips any leading
//! whitespace and then one run, matching what editors do.

use super::state::State;

fn class(b: u8) -> u8 {
    if b == b' ' || b == b'\t' {
        0
    } else if b == b'\n' {
        1
    } else if b.is_ascii_alphanumeric() || b == b'_' || b >= 0x80 {
        2
    } else {
        3
    }
}

impl State {
    // Offset one word to the left of `from`, skipping trailing whitespace then
    // one run. Newlines are their own class so motion stops at line ends.
    fn word_boundary_left(&self, from: usize) -> usize {
        let b = &self.buf[..self.len];
        let mut c = from.min(b.len());
        while c > 0 && class(b[c - 1]) == 0 {
            c -= 1;
        }
        if c > 0 {
            let cl = class(b[c - 1]);
            while c > 0 && class(b[c - 1]) == cl {
                c -= 1;
            }
        }
        c
    }

    fn word_boundary_right(&self, from: usize) -> usize {
        let b = &self.buf[..self.len];
        let n = b.len();
        let mut c = from.min(n);
        while c < n && class(b[c]) == 0 {
            c += 1;
        }
        if c < n {
            let cl = class(b[c]);
            while c < n && class(b[c]) == cl {
                c += 1;
            }
        }
        c
    }

    pub fn word_left(&mut self) {
        self.caret = self.word_boundary_left(self.caret);
    }

    pub fn word_right(&mut self) {
        self.caret = self.word_boundary_right(self.caret);
    }

    // Delete from the caret back to the previous word boundary, as one edit.
    pub fn delete_word_left(&mut self) -> bool {
        let start = self.word_boundary_left(self.caret);
        if start < self.caret {
            self.apply_edit(start, self.caret - start, b"")
        } else {
            false
        }
    }
}
