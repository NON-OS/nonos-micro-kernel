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
//! Turning a read pack into the rows an index is built from.

extern crate alloc;

use alloc::vec::Vec;

use crate::oid::ObjectId;

use super::super::reader::PackObject;
use super::crc::crc32;

/// Each object's id, its offset, and the CRC of its bytes in the pack.
///
/// An entry runs from its own offset to the next one, and the last runs to the
/// start of the pack's twenty byte trailer. Objects come back from the reader
/// in pack order, so the spans follow from their offsets without a second
/// walk of the file.
pub fn entries(pack: &[u8], objects: &[PackObject]) -> Option<Vec<(ObjectId, u64, u32)>> {
    let end = pack.len().checked_sub(20)?;
    let mut out = Vec::with_capacity(objects.len());
    for (i, object) in objects.iter().enumerate() {
        let next = match objects.get(i + 1) {
            Some(o) => o.offset,
            None => end,
        };
        if object.offset > next || next > end {
            return None;
        }
        let span = pack.get(object.offset..next)?;
        out.push((object.id, object.offset as u64, crc32(span)));
    }
    Some(out)
}
