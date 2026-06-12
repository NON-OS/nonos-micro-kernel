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

use crate::fwui::font::draw_glyph;
use crate::fwui::metrics::advance;

pub fn text(x: u32, y: u32, s: &[u8], c: u32) {
    let adv = advance();
    let mut cx = x;
    for &b in s {
        if b != b'\n' {
            draw_glyph(cx, y, b as char, c);
            cx += adv;
        }
    }
}
