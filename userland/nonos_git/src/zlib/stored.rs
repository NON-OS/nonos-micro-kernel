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

//! A stored block: byte-align, read LEN and its complement, copy LEN bytes.

extern crate alloc;

use alloc::vec::Vec;

use super::bit_reader::BitReader;
use super::error::InflateError;

pub(super) fn inflate_stored(r: &mut BitReader<'_>, out: &mut Vec<u8>) -> Result<(), InflateError> {
    r.align();
    if r.byte + 4 > r.data.len() {
        return Err(InflateError::Truncated);
    }
    let len = u16::from_le_bytes([r.data[r.byte], r.data[r.byte + 1]]);
    let nlen = u16::from_le_bytes([r.data[r.byte + 2], r.data[r.byte + 3]]);
    if len != !nlen {
        return Err(InflateError::Invalid);
    }
    r.byte += 4;
    let end = r.byte + len as usize;
    if end > r.data.len() {
        return Err(InflateError::Truncated);
    }
    out.extend_from_slice(&r.data[r.byte..end]);
    r.byte = end;
    Ok(())
}
