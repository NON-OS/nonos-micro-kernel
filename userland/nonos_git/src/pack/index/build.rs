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
//! Building index rows straight from a pack.

extern crate alloc;

use alloc::vec::Vec;

use crate::object::frame;
use crate::oid::ObjectId;

use super::super::error::PackError;
use super::super::reader::read_at;
use super::spans::spans;

/// The id, offset and CRC of every object, without holding the pack's
/// contents in memory.
///
/// Two passes. The first finds where entries start, keeping nothing it
/// inflates. The second names each one, which needs its content, so the
/// object is rebuilt at its offset, hashed, and dropped. A reference delta
/// names its base by id, and in a pack the base always comes first, so the
/// ids found so far are enough to place it.
pub fn build(pack: &[u8]) -> Result<Vec<(ObjectId, u64, u32)>, PackError> {
    let spans = spans(pack)?;
    let mut rows: Vec<(ObjectId, u64, u32)> = Vec::with_capacity(spans.len());

    for (offset, crc) in spans {
        let (kind, content) = {
            let seen = &rows;
            let find = |want: &ObjectId| {
                seen.iter().find(|(id, _, _)| id == want).map(|(_, at, _)| *at as usize)
            };
            read_at(pack, offset, &find, 0)?
        };
        let (_framed, id) = frame(kind, &content);
        rows.push((id, offset as u64, crc));
    }
    Ok(rows)
}
