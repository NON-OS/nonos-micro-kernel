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
//! Writing a version 2 pack index.

extern crate alloc;

use alloc::vec::Vec;

use crate::oid::ObjectId;
use crate::sha1::Sha1;

use super::fanout::fanout;

/// Build the `.idx` for a pack whose trailing checksum is `pack_sha`.
///
/// Version 2 is the format every git since 1.6 writes: a magic and version,
/// the fanout, the sorted ids, their CRCs, then their offsets. Offsets past
/// two gigabytes need a second table; a pack that large is refused instead,
/// because writing a table this cannot read back would be worse than saying
/// no.
pub fn write_index(objects: &[(ObjectId, u64, u32)], pack_sha: &[u8; 20]) -> Option<Vec<u8>> {
    let mut sorted: Vec<(ObjectId, u64)> = objects.iter().map(|(id, at, _)| (*id, *at)).collect();
    sorted.sort_by(|a, b| a.0.as_bytes().cmp(b.0.as_bytes()));

    let mut out = Vec::with_capacity(1024 + objects.len() * 28);
    out.extend_from_slice(&[0xFF, 0x74, 0x4F, 0x63]);
    out.extend_from_slice(&2u32.to_be_bytes());
    for count in fanout(&sorted) {
        out.extend_from_slice(&count.to_be_bytes());
    }
    for (id, _) in &sorted {
        out.extend_from_slice(id.as_bytes());
    }
    for (id, _) in &sorted {
        let crc = objects.iter().find(|(o, _, _)| o == id).map(|(_, _, c)| *c)?;
        out.extend_from_slice(&crc.to_be_bytes());
    }
    for (_, at) in &sorted {
        if *at > u64::from(u32::MAX) & 0x7FFF_FFFF {
            return None;
        }
        out.extend_from_slice(&(*at as u32).to_be_bytes());
    }
    out.extend_from_slice(pack_sha);
    let checksum = Sha1::digest(&out);
    out.extend_from_slice(&checksum);
    Some(out)
}
