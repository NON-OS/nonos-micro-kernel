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
//! Looking an object up in a pack index.

use crate::oid::ObjectId;

const HEADER: usize = 8;
const FANOUT: usize = 256 * 4;
const ID_LEN: usize = 20;

fn be32(data: &[u8], at: usize) -> Option<u32> {
    let bytes = data.get(at..at + 4)?;
    Some(u32::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]))
}

/// The offset `id` sits at in the pack this index describes.
///
/// The first byte of the id picks a range out of the fanout, and the search
/// covers only that range. Ids inside it are sorted, so this is a binary
/// search over at most the objects sharing that first byte.
pub fn lookup(idx: &[u8], id: &ObjectId) -> Option<u64> {
    if idx.len() < HEADER + FANOUT || idx[..4] != [0xFF, 0x74, 0x4F, 0x63] {
        return None;
    }
    if be32(idx, 4)? != 2 {
        return None;
    }
    let first = usize::from(id.as_bytes()[0]);
    let start = if first == 0 { 0 } else { be32(idx, HEADER + (first - 1) * 4)? as usize };
    let end = be32(idx, HEADER + first * 4)? as usize;
    let total = be32(idx, HEADER + 255 * 4)? as usize;

    let ids_at = HEADER + FANOUT;
    let mut lo = start;
    let mut hi = end;
    while lo < hi {
        let mid = lo + (hi - lo) / 2;
        let at = ids_at + mid * ID_LEN;
        let candidate = idx.get(at..at + ID_LEN)?;
        match candidate.cmp(id.as_bytes().as_slice()) {
            core::cmp::Ordering::Less => lo = mid + 1,
            core::cmp::Ordering::Greater => hi = mid,
            core::cmp::Ordering::Equal => {
                let offsets_at = ids_at + total * ID_LEN + total * 4;
                return be32(idx, offsets_at + mid * 4).map(u64::from);
            }
        }
    }
    None
}
