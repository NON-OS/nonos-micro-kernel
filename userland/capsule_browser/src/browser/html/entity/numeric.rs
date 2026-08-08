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

/// A numeric reference, decimal or hexadecimal.
///
/// The replacement character stands in for anything outside Unicode or in a
/// surrogate range, which is what the specification asks for and what keeps
/// a malformed page from losing the text around the reference.
pub fn numeric(name: &str) -> Option<char> {
    let digits = name.strip_prefix('#')?;
    let code = match digits.strip_prefix(['x', 'X']) {
        Some(hex) if !hex.is_empty() => u32::from_str_radix(hex, 16).ok()?,
        Some(_) => return None,
        None if !digits.is_empty() => digits.parse::<u32>().ok()?,
        None => return None,
    };
    // Numbers written for the old Windows code page, which pages still
    // carry, name control positions that were never those characters.
    let code = match code {
        0x80 => 0x20AC,
        0x82 => 0x201A,
        0x83 => 0x0192,
        0x84 => 0x201E,
        0x85 => 0x2026,
        0x86 => 0x2020,
        0x87 => 0x2021,
        0x88 => 0x02C6,
        0x89 => 0x2030,
        0x8A => 0x0160,
        0x8B => 0x2039,
        0x8C => 0x0152,
        0x91 => 0x2018,
        0x92 => 0x2019,
        0x93 => 0x201C,
        0x94 => 0x201D,
        0x95 => 0x2022,
        0x96 => 0x2013,
        0x97 => 0x2014,
        0x98 => 0x02DC,
        0x99 => 0x2122,
        0x9A => 0x0161,
        0x9B => 0x203A,
        0x9C => 0x0153,
        0x9F => 0x0178,
        other => other,
    };
    match char::from_u32(code) {
        Some(c) if !c.is_control() || c == '\n' || c == '\t' => Some(c),
        _ => None,
    }
}
