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

//! How many columns a character takes.
//!
//! A grid assumes every character is one cell wide. That holds for Latin and
//! stops holding the moment anything else arrives: a CJK ideograph or an
//! emoji is drawn two columns wide, and treating it as one puts the next
//! character on top of its right half. Every column after it on that line is
//! then wrong, and so is anything that counts columns to place a cursor.

/// Ranges drawn two columns wide.
///
/// Taken from the East Asian Wide and Fullwidth classes, which is where the
/// double width ones live. Ranges rather than a table per codepoint: the
/// blocks are contiguous and a table would be tens of kilobytes to say the
/// same thing.
const WIDE: &[(u32, u32)] = &[
    (0x1100, 0x115F),   // Hangul Jamo initial consonants
    (0x2E80, 0x303E),   // CJK radicals, Kangxi, CJK symbols
    (0x3041, 0x33FF),   // Kana, Bopomofo, Hangul compatibility, enclosed
    (0x3400, 0x4DBF),   // CJK unified ideographs extension A
    (0x4E00, 0x9FFF),   // CJK unified ideographs
    (0xA000, 0xA4CF),   // Yi syllables and radicals
    (0xA960, 0xA97F),   // Hangul Jamo extended A
    (0xAC00, 0xD7A3),   // Hangul syllables
    (0xF900, 0xFAFF),   // CJK compatibility ideographs
    (0xFE10, 0xFE19),   // Vertical forms
    (0xFE30, 0xFE6F),   // CJK compatibility forms, small form variants
    (0xFF00, 0xFF60),   // Fullwidth forms
    (0xFFE0, 0xFFE6),   // Fullwidth signs
    (0x1F300, 0x1F64F), // Symbols, pictographs, emoticons
    (0x1F900, 0x1F9FF), // Supplemental symbols and pictographs
    (0x20000, 0x3FFFD), // CJK unified ideographs, later extensions
];

/// Columns `ch` occupies.
///
/// Never zero. Combining marks are the case that would justify zero, and
/// composing them onto the character before is work this grid does not do,
/// so they are given their own cell instead: a decomposed accent shows as a
/// separate mark rather than being silently dropped.
pub fn char_width(ch: char) -> usize {
    let cp = ch as u32;
    for &(lo, hi) in WIDE {
        if cp < lo {
            break;
        }
        if cp <= hi {
            return 2;
        }
    }
    1
}
