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

//! Reading a byte-held line as characters.

// The line is held as bytes and the scroll window can cut it mid character,
// so only the part that is whole is read. The tail is at most one partial
// character and arrives complete on the next keystroke.
pub fn chars_of(bytes: &[u8]) -> impl Iterator<Item = char> + '_ {
    let whole = match core::str::from_utf8(bytes) {
        Ok(text) => text,
        Err(err) => core::str::from_utf8(&bytes[..err.valid_up_to()]).unwrap_or(""),
    };
    whole.chars()
}

// The nearest character boundary at or before `at`. A byte in the middle of a
// character has its top two bits set to one and zero, which is what marks it
// as a continuation of the byte before.
pub fn char_floor(bytes: &[u8], at: usize) -> usize {
    let mut i = at.min(bytes.len());
    while i > 0 && bytes.get(i).is_some_and(|b| b & 0xC0 == 0x80) {
        i -= 1;
    }
    i
}
