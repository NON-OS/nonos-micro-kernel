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
//! Reading one object out of a pack, without reading the rest.

extern crate alloc;

use alloc::vec::Vec;

use crate::object::ObjectKind;
use crate::zlib::decompress_prefix;

use super::super::delta::apply;
use super::super::entry::{parse as parse_entry, EntryKind};
use super::super::error::PackError;

/// How long a delta chain may be before this gives up.
///
/// Git packs keep chains short, and a pack claiming a longer one is either
/// hostile or damaged. Following it without a bound would recurse until the
/// stack ran out.
const MAX_CHAIN: u32 = 64;

/// The object at `offset`, with its delta chain resolved.
///
/// A reference delta names its base by id rather than position, which cannot
/// be followed without an index, so `find` is asked where that id lives.
pub fn read_at<F>(
    pack: &[u8],
    offset: usize,
    find: &F,
    depth: u32,
) -> Result<(ObjectKind, Vec<u8>), PackError>
where
    F: Fn(&crate::oid::ObjectId) -> Option<usize>,
{
    if depth > MAX_CHAIN {
        return Err(PackError::MissingBase);
    }
    let mut at = offset;
    let (entry, _size) = parse_entry(pack, &mut at)?;
    let rest = pack.get(at..).ok_or(PackError::Truncated)?;
    let (raw, _used) = decompress_prefix(rest).map_err(|_| PackError::Corrupt)?;

    match entry {
        EntryKind::Whole(kind) => Ok((kind, raw)),
        EntryKind::OfsDelta(back) => {
            let base_at = offset.checked_sub(back as usize).ok_or(PackError::MissingBase)?;
            let (kind, base) = read_at(pack, base_at, find, depth + 1)?;
            Ok((kind, apply(&base, &raw)?))
        }
        EntryKind::RefDelta(id) => {
            let base_at = find(&id).ok_or(PackError::MissingBase)?;
            let (kind, base) = read_at(pack, base_at, find, depth + 1)?;
            Ok((kind, apply(&base, &raw)?))
        }
    }
}
