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
//! The forward pass over every entry.

extern crate alloc;

use alloc::vec::Vec;

use crate::object::frame;
use crate::zlib::decompress_prefix;

use super::super::entry::{parse as parse_entry, EntryKind};
use super::super::error::PackError;
use super::super::header;
use super::object::PackObject;
use super::resolve::resolve;

/// Read every object, resolving deltas against the objects already seen. A
/// delta whose base is absent is an error, not a silently dropped object.
pub fn read_pack(data: &[u8]) -> Result<Vec<PackObject>, PackError> {
    let count = header::parse(data)? as usize;
    let mut out: Vec<PackObject> = Vec::with_capacity(count);
    let mut at = header::HEADER_LEN;

    for _ in 0..count {
        let offset = at;
        let (entry, _size) = parse_entry(data, &mut at)?;
        let rest = data.get(at..).ok_or(PackError::Truncated)?;
        let (raw, used) = decompress_prefix(rest).map_err(|_| PackError::Corrupt)?;
        at += used;

        let (kind, content) = match entry {
            EntryKind::Whole(k) => (k, raw),
            EntryKind::OfsDelta(back) => {
                let base = offset.checked_sub(back as usize).ok_or(PackError::MissingBase)?;
                resolve(&out, |o| o.offset == base, &raw)?
            }
            EntryKind::RefDelta(id) => resolve(&out, |o| o.id == id, &raw)?,
        };

        let (_framed, id) = frame(kind, &content);
        out.push(PackObject { id, kind, data: content, offset });
    }
    Ok(out)
}
