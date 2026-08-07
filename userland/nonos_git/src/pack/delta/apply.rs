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
//! The instruction loop.

extern crate alloc;

use alloc::vec::Vec;

use super::super::error::PackError;
use super::copy::copy;
use super::size::{header_size, take};

/// Every offset and length is bounds checked against the base and the stated
/// target, so a malformed delta is refused rather than read past either.
pub(in crate::pack) fn apply(base: &[u8], delta: &[u8]) -> Result<Vec<u8>, PackError> {
    let mut at = 0usize;
    let base_len = header_size(delta, &mut at)?;
    let target_len = header_size(delta, &mut at)?;
    if base_len as usize != base.len() {
        return Err(PackError::BadDelta);
    }

    let mut out = Vec::with_capacity(target_len as usize);
    while at < delta.len() {
        let op = take(delta, &mut at)?;
        if op & 0x80 != 0 {
            copy(base, delta, &mut at, op, &mut out)?;
        } else {
            let len = usize::from(op & 0x7F);
            if len == 0 || at + len > delta.len() {
                return Err(PackError::BadDelta);
            }
            out.extend_from_slice(&delta[at..at + len]);
            at += len;
        }
    }

    if out.len() as u64 != target_len {
        return Err(PackError::BadDelta);
    }
    Ok(out)
}
