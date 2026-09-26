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

//! The two header fields this reader needs.

use alloc::vec::Vec;

const NAME: usize = 100;

/// The size field is octal text, space or NUL padded.
pub(super) fn octal(field: &[u8]) -> Option<usize> {
    let mut value = 0usize;
    for b in field {
        match b {
            b'0'..=b'7' => value = value.checked_mul(8)?.checked_add((b - b'0') as usize)?,
            b' ' | 0 => break,
            _ => return None,
        }
    }
    Some(value)
}

pub(super) fn name_of(head: &[u8]) -> Vec<u8> {
    let raw = &head[..NAME];
    let end = raw.iter().position(|b| *b == 0).unwrap_or(NAME);
    raw[..end].to_vec()
}
