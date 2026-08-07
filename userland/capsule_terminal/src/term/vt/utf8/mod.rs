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

//! Rebuilding characters from the bytes they arrive in.
//!
//! Output reaches the terminal a byte at a time, and a character above ASCII
//! is spread across two to four of them. Printing each byte as it lands turns
//! one character into that many pieces of mojibake, which is what a grid of
//! bytes can do and no more.

use finish::{finish, REPLACEMENT};

mod finish;
mod state;

pub use state::Utf8;

impl Utf8 {
    /// Offer one byte and emit the characters it completes.
    ///
    /// Zero, one or two come out. Two happens where a sequence is abandoned
    /// part way: the bytes already taken are reported as damaged, and then
    /// the byte that ended them is read on its own, because it is far more
    /// likely to start a valid character than to belong to the broken one.
    pub fn push<F: FnMut(char)>(&mut self, b: u8, mut emit: F) {
        if self.left > 0 {
            if b & 0xC0 != 0x80 {
                self.reset();
                emit(REPLACEMENT);
                self.push(b, emit);
                return;
            }
            self.acc = (self.acc << 6) | (b as u32 & 0x3F);
            self.left -= 1;
            if self.left > 0 {
                return;
            }
            let width = self.width;
            let acc = self.acc;
            self.reset();
            emit(finish(acc, width));
            return;
        }

        match b {
            0x00..=0x7F => emit(b as char),
            // A continuation byte with nothing to continue.
            0x80..=0xBF => emit(REPLACEMENT),
            0xC0..=0xDF => self.begin(b as u32 & 0x1F, 1),
            0xE0..=0xEF => self.begin(b as u32 & 0x0F, 2),
            0xF0..=0xF4 => self.begin(b as u32 & 0x07, 3),
            // 0xF5 and above encode nothing inside the Unicode range.
            _ => emit(REPLACEMENT),
        }
    }
}
