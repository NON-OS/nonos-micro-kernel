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
//! One object's header inside a pack.

extern crate alloc;

use crate::object::ObjectKind;
use crate::oid::ObjectId;

use super::error::PackError;
use super::varint;

/// What an entry's header said it is.
pub(super) enum EntryKind {
    /// A whole object of this type.
    Whole(ObjectKind),
    /// A delta against an object earlier in this pack, named by how far back.
    OfsDelta(u64),
    /// A delta against an object named by id, which may be outside the pack.
    RefDelta(ObjectId),
}

/// Read the type and size, and the base reference for a delta. Leaves `at` on
/// the first byte of the zlib stream.
pub(super) fn parse(data: &[u8], at: &mut usize) -> Result<(EntryKind, u64), PackError> {
    let first = *data.get(*at).ok_or(PackError::Truncated)?;
    *at += 1;
    let type_bits = (first >> 4) & 0x07;
    let size = varint::size(data, at, u64::from(first & 0x0F), 4)?;

    let kind = match type_bits {
        1 => EntryKind::Whole(ObjectKind::Commit),
        2 => EntryKind::Whole(ObjectKind::Tree),
        3 => EntryKind::Whole(ObjectKind::Blob),
        4 => EntryKind::Whole(ObjectKind::Tag),
        6 => EntryKind::OfsDelta(varint::offset(data, at)?),
        7 => {
            let end = *at + 20;
            if end > data.len() {
                return Err(PackError::Truncated);
            }
            let mut raw = [0u8; 20];
            raw.copy_from_slice(&data[*at..end]);
            *at = end;
            EntryKind::RefDelta(ObjectId::from_bytes(raw))
        }
        other => return Err(PackError::ObjectType(other)),
    };
    Ok((kind, size))
}
