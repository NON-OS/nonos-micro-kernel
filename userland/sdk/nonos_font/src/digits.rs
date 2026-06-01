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

static TABLE: [GlyphBitmap; 10] = [
    GlyphBitmap::new([0x3C, 0x42, 0x46, 0x4A, 0x52, 0x62, 0x3C, 0x00]),
    GlyphBitmap::new([0x08, 0x18, 0x28, 0x08, 0x08, 0x08, 0x3E, 0x00]),
    GlyphBitmap::new([0x3C, 0x42, 0x02, 0x0C, 0x30, 0x40, 0x7E, 0x00]),
    GlyphBitmap::new([0x3C, 0x42, 0x02, 0x1C, 0x02, 0x42, 0x3C, 0x00]),
    GlyphBitmap::new([0x0C, 0x14, 0x24, 0x44, 0x7E, 0x04, 0x04, 0x00]),
    GlyphBitmap::new([0x7E, 0x40, 0x7C, 0x02, 0x02, 0x42, 0x3C, 0x00]),
    GlyphBitmap::new([0x1C, 0x20, 0x40, 0x7C, 0x42, 0x42, 0x3C, 0x00]),
    GlyphBitmap::new([0x7E, 0x42, 0x04, 0x08, 0x10, 0x10, 0x10, 0x00]),
    GlyphBitmap::new([0x3C, 0x42, 0x42, 0x3C, 0x42, 0x42, 0x3C, 0x00]),
    GlyphBitmap::new([0x3C, 0x42, 0x42, 0x3E, 0x02, 0x04, 0x38, 0x00]),
];

pub(super) fn glyph(ascii: u8) -> Option<&'static GlyphBitmap> {
    if ascii.is_ascii_digit() {
        Some(&TABLE[(ascii - b'0') as usize])
    } else {
        None
    }
}
