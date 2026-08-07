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

use crate::term::dimensions::COLS;
use crate::term::grid::types::Grid;
use crate::term::vt::csi_cursor::csi_cursor;
use crate::term::vt::csi_edit::csi_edit;
use crate::term::vt::parser::Perform;
use crate::term::vt::utf8::Utf8;

pub struct VtState<'a> {
    pub g: &'a mut Grid,
    /// Rebuilds characters from the bytes they arrive in. It lives across
    /// calls because a character can be split over two reads.
    pub utf8: &'a mut Utf8,
}

impl<'a> Perform for VtState<'a> {
    fn print(&mut self, c: u8) {
        let g = &mut *self.g;
        self.utf8.push(c, |ch| g.put_char(ch));
    }

    fn execute(&mut self, b: u8) {
        match b {
            0x08 => {
                self.g.x = self.g.x.saturating_sub(1);
            }
            0x09 => {
                let next = ((self.g.x / 8) + 1) * 8;
                self.g.x = next.min(COLS - 1);
            }
            0x0A => {
                self.g.carriage_return();
                self.g.line_feed();
            }
            0x0D => {
                self.g.carriage_return();
            }
            _ => {}
        }
    }

    fn csi(&mut self, c: u8, params: &[i64], inter: &[u8]) {
        match c {
            b'A' | b'B' | b'C' | b'D' | b'E' | b'F' | b'G' | b'H' | b'f' | b'd' | b'S' | b'T' => {
                csi_cursor(self.g, c, params);
            }
            b'J' | b'K' | b'P' | b'@' => {
                csi_edit(self.g, c, params);
            }
            b'm' => crate::term::vt::sgr::sgr(self.g, params),
            b'h' => crate::term::vt::decset::decset(self.g, params, inter, true),
            b'l' => crate::term::vt::decset::decset(self.g, params, inter, false),
            b'r' if inter.is_empty() => {
                // DECSTBM: top;bottom are 1-based; an empty request is the full
                // screen. Private `?...r` (restore modes) carries an intermediate
                // and is intentionally left to fall through.
                let top = params.first().copied().filter(|&v| v > 0).unwrap_or(1);
                let bot = params.get(1).copied().filter(|&v| v > 0).unwrap_or(i64::MAX);
                self.g.set_scroll_region((top - 1) as usize, (bot - 1) as usize);
            }
            _ => {}
        }
    }

    fn esc(&mut self, _c: u8, _inter: &[u8]) {}

    fn osc(&mut self, _data: &[u8]) {}
}
