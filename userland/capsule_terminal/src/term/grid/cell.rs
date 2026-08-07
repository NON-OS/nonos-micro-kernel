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

pub const F_BOLD: u8 = 1;
pub const F_UNDERLINE: u8 = 2;
pub const F_REVERSE: u8 = 4;
/// The right half of a character drawn two columns wide. It holds no glyph
/// of its own; the cell before it drew both halves.
pub const F_WIDE_TAIL: u8 = 8;

#[derive(Clone, Copy)]
pub struct Cell {
    pub ch: char,
    // Full ARGB foreground and background, so 24-bit colour is preserved.
    pub fg: u32,
    pub bg: u32,
    pub flags: u8,
}

impl Cell {
    pub const fn blank() -> Cell {
        Cell {
            ch: ' ',
            fg: crate::term::vt::color::DEFAULT_FG,
            bg: crate::term::vt::color::DEFAULT_BG,
            flags: 0,
        }
    }
}
