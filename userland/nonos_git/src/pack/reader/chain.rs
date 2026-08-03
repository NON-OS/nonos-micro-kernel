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
//! Following a delta chain to its base.

extern crate alloc;

use alloc::vec::Vec;

use crate::object::ObjectKind;
use crate::oid::ObjectId;

use super::super::entry::{parse as parse_entry, EntryKind};
use super::super::error::PackError;

/// How long a delta chain may be before this gives up.
///
/// Git packs keep chains short, and a pack claiming a longer one is either
/// hostile or damaged. Following it without a bound would run out of stack
/// or memory before it ran out of pack.
const MAX_CHAIN: usize = 64;

/// Every offset from `offset` back to its base, and the base's kind.
///
/// Only entry headers are read here, never the compressed bodies, so finding
/// the chain costs nothing beyond parsing a few bytes per link.
pub(super) fn chain<F>(
    pack: &[u8],
    offset: usize,
    find: &F,
) -> Result<(Vec<usize>, ObjectKind), PackError>
where
    F: Fn(&ObjectId) -> Option<usize>,
{
    let mut links = Vec::new();
    let mut at = offset;
    loop {
        if links.len() > MAX_CHAIN {
            return Err(PackError::MissingBase);
        }
        let mut cursor = at;
        let (entry, _size) = parse_entry(pack, &mut cursor)?;
        links.push(at);
        match entry {
            EntryKind::Whole(kind) => return Ok((links, kind)),
            EntryKind::OfsDelta(back) => {
                at = at.checked_sub(back as usize).ok_or(PackError::MissingBase)?;
            }
            EntryKind::RefDelta(id) => {
                at = find(&id).ok_or(PackError::MissingBase)?;
            }
        }
    }
}
