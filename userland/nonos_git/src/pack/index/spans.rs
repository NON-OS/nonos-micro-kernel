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
//! Where every entry in a pack starts.

extern crate alloc;

use alloc::vec::Vec;

use crate::zlib::decompress_prefix;

use super::super::entry::parse as parse_entry;
use super::super::error::PackError;
use super::super::header;
use super::crc::crc32;

/// Walk the pack recording each entry's offset and the CRC of its bytes.
///
/// Finding where an entry ends means inflating it, but nothing here keeps
/// what comes out. That is the whole point: a thirty megabyte pack is walked
/// with one object in memory at a time rather than all of them.
pub(super) fn spans(pack: &[u8]) -> Result<Vec<(usize, u32)>, PackError> {
    let count = header::parse(pack)? as usize;
    let mut out = Vec::with_capacity(count);
    let mut at = header::HEADER_LEN;

    for _ in 0..count {
        let start = at;
        let (_entry, _size) = parse_entry(pack, &mut at)?;
        let rest = pack.get(at..).ok_or(PackError::Truncated)?;
        let (_discarded, used) = decompress_prefix(rest).map_err(|_| PackError::Corrupt)?;
        at += used;
        let span = pack.get(start..at).ok_or(PackError::Truncated)?;
        out.push((start, crc32(span)));
    }
    Ok(out)
}
