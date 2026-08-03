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
use crate::oid::ObjectId;
use crate::zlib::decompress_prefix;

use super::super::delta::apply;
use super::super::entry::parse as parse_entry;
use super::super::error::PackError;
use super::chain::chain;

/// The object at `offset`, with its delta chain resolved.
///
/// The chain is walked to its base first, reading only entry headers, and the
/// deltas are then applied from the base forward. Recursing instead would
/// hold every intermediate object alive at once, which on a large pack costs
/// several times the pack itself.
///
/// A reference delta names its base by id rather than position, which cannot
/// be followed without an index, so `find` is asked where that id lives.
pub fn read_at<F>(
    pack: &[u8],
    offset: usize,
    find: &F,
    _depth: u32,
) -> Result<(ObjectKind, Vec<u8>), PackError>
where
    F: Fn(&ObjectId) -> Option<usize>,
{
    let (mut links, kind) = chain(pack, offset, find)?;

    // The base is last, so walk back applying each delta onto the running
    // content. Only that content and one delta are alive at any moment.
    let mut content = inflate_at(pack, links.pop().ok_or(PackError::MissingBase)?)?;
    while let Some(delta_at) = links.pop() {
        let raw = inflate_at(pack, delta_at)?;
        content = apply(&content, &raw)?;
    }
    Ok((kind, content))
}

fn inflate_at(pack: &[u8], offset: usize) -> Result<Vec<u8>, PackError> {
    let mut at = offset;
    let (_entry, _size) = parse_entry(pack, &mut at)?;
    let rest = pack.get(at..).ok_or(PackError::Truncated)?;
    let (raw, _used) = decompress_prefix(rest).map_err(|_| PackError::Corrupt)?;
    Ok(raw)
}
