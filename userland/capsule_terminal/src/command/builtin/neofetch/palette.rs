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

//! The eight-block ANSI colour strip printed under the info column.

use alloc::vec::Vec;

const CODES: [&str; 8] = [
    "\x1b[31m", "\x1b[32m", "\x1b[33m", "\x1b[34m", "\x1b[35m", "\x1b[36m", "\x1b[37m",
    "\x1b[90m",
];

const BLOCK: &str = "██";

/// The strip as a `(plain, styled)` pair.
///
/// `styled` carries one colour escape per block and is what the grid renders;
/// `plain` is the same blocks with no escapes at all, so a redirected
/// `neofetch` writes glyphs to the file rather than terminal control codes.
pub fn palette() -> (Vec<u8>, Vec<u8>) {
    let mut plain = Vec::with_capacity(CODES.len() * BLOCK.len());
    let mut styled = Vec::with_capacity(CODES.len() * (BLOCK.len() + 5));
    for code in CODES {
        styled.extend_from_slice(code.as_bytes());
        styled.extend_from_slice(BLOCK.as_bytes());
        plain.extend_from_slice(BLOCK.as_bytes());
    }
    (plain, styled)
}
