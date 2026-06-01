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

use super::types::GlyphBitmap;
use super::{digits, lower, punct, upper};

static GLYPH_UNKNOWN: GlyphBitmap =
    GlyphBitmap::new([0x7E, 0x81, 0xA5, 0x81, 0x99, 0x81, 0x7E, 0x00]);
static GLYPH_SPACE: GlyphBitmap = GlyphBitmap::new([0x00; 8]);

pub fn glyph_for_ascii(ascii: u8) -> &'static GlyphBitmap {
    if ascii == b' ' {
        return &GLYPH_SPACE;
    }
    if let Some(g) = digits::glyph(ascii) {
        return g;
    }
    if ascii.is_ascii_uppercase() {
        return upper::glyph(ascii);
    }
    if ascii.is_ascii_lowercase() {
        return lower::glyph(ascii);
    }
    if let Some(g) = punct::glyph(ascii) {
        return g;
    }
    &GLYPH_UNKNOWN
}
