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

//! Turning gathered bits into a character, and what to do when they are not.

/// What a malformed sequence is replaced with. Substituting rather than
/// dropping keeps a column count that matches what was sent.
pub const REPLACEMENT: char = '\u{FFFD}';

/// Turn gathered bits into a character, refusing the encodings that are
/// shorter forms written long. Those decode to the same value as a shorter
/// sequence and exist only to slip past a check that reads the short one.
pub fn finish(acc: u32, width: u8) -> char {
    let overlong = match width {
        1 => acc < 0x80,
        2 => acc < 0x800,
        _ => acc < 0x10000,
    };
    if overlong {
        return REPLACEMENT;
    }
    // from_u32 also refuses surrogate halves, which encode cleanly in UTF-8
    // and name nothing.
    char::from_u32(acc).unwrap_or(REPLACEMENT)
}
