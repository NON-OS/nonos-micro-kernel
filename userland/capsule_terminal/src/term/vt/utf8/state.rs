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

//! What the decoder carries between bytes.

/// Accumulates bytes until they form a character.
#[derive(Default)]
pub struct Utf8 {
    /// Bits gathered so far from the bytes of the current sequence.
    pub(super) acc: u32,
    /// Continuation bytes still expected.
    pub(super) left: u8,
    /// How many the sequence asked for, kept so an overlong encoding can be
    /// told from a legitimate one of the same value.
    pub(super) width: u8,
}

impl Utf8 {
    pub(super) fn begin(&mut self, bits: u32, follow: u8) {
        self.acc = bits;
        self.left = follow;
        self.width = follow;
    }

    pub(super) fn reset(&mut self) {
        self.acc = 0;
        self.left = 0;
        self.width = 0;
    }
}
