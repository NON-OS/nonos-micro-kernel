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
//! One copy instruction.

extern crate alloc;

use alloc::vec::Vec;

use super::super::error::PackError;
use super::size::take;

/// The flag bits say which offset and length bytes are present; an absent byte
/// means that part is zero.
pub(super) fn copy(
    base: &[u8],
    delta: &[u8],
    at: &mut usize,
    op: u8,
    out: &mut Vec<u8>,
) -> Result<(), PackError> {
    let mut offset = 0usize;
    let mut len = 0usize;
    for i in 0..4 {
        if op & (1 << i) != 0 {
            offset |= usize::from(take(delta, at)?) << (i * 8);
        }
    }
    for i in 0..3 {
        if op & (0x10 << i) != 0 {
            len |= usize::from(take(delta, at)?) << (i * 8);
        }
    }
    // Zero length means 0x10000: how the format encodes the largest copy.
    if len == 0 {
        len = 0x1_0000;
    }
    let end = offset.checked_add(len).ok_or(PackError::BadDelta)?;
    if end > base.len() {
        return Err(PackError::BadDelta);
    }
    out.extend_from_slice(&base[offset..end]);
    Ok(())
}
