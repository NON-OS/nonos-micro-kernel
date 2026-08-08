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

// One descriptor per packaged file: a NUL-padded absolute vfs path, then the
// absolute byte extent of its payload on the device. Every extent is bounded
// against the reported capacity and against a total allocation budget before
// any of it is read, so a hostile table cannot exhaust the 48 MB capsule heap.
use alloc::string::String;
use alloc::vec::Vec;

use super::error::BlkError;
use super::store_header::{le_u64, ENTRY_LEN, HEADER_LEN};
use super::wire::SECTOR_SIZE;

pub(super) const NAME_LEN: usize = 96;
pub(super) const MAX_TOTAL_BYTES: u64 = 16 * 1024 * 1024;

pub struct TocEntry {
    pub name: String,
    pub offset: u64,
    pub len: u64,
    pub digest: [u8; 16],
}

pub fn decode(toc: &[u8], count: usize, capacity_bytes: u64) -> Result<Vec<TocEntry>, BlkError> {
    if toc.len() < HEADER_LEN + ENTRY_LEN * count {
        return Err(BlkError::BadContainer);
    }
    let mut entries = Vec::with_capacity(count);
    let mut budget = MAX_TOTAL_BYTES;
    for index in 0..count {
        let base = HEADER_LEN + ENTRY_LEN * index;
        let offset = le_u64(toc, base + NAME_LEN);
        let len = le_u64(toc, base + NAME_LEN + 8);
        let end = offset.checked_add(len).ok_or(BlkError::BadContainer)?;
        if offset % SECTOR_SIZE as u64 != 0 || end > capacity_bytes || len > budget {
            return Err(BlkError::BadContainer);
        }
        budget -= len;
        let name = decode_name(&toc[base..base + NAME_LEN])?;
        let mut digest = [0u8; 16];
        digest.copy_from_slice(&toc[base + NAME_LEN + 16..base + NAME_LEN + 32]);
        entries.push(TocEntry { name, offset, len, digest });
    }
    Ok(entries)
}

fn decode_name(field: &[u8]) -> Result<String, BlkError> {
    let end = field.iter().position(|&b| b == 0).unwrap_or(field.len());
    let name = core::str::from_utf8(&field[..end]).map_err(|_| BlkError::BadContainer)?;
    if !valid_name(name) {
        return Err(BlkError::BadContainer);
    }
    Ok(String::from(name))
}

// The single acceptance predicate for a TOC name, shared with the appender so a
// name that writes cannot be a name that later fails to decode: one such entry
// makes `decode` reject the whole table and takes every installed app with it.
// The NUL and length bounds are implied for a name `decode_name` just carved
// out of a fixed NUL-padded field, and are what the writer actually needs.
pub(super) fn valid_name(name: &str) -> bool {
    !name.is_empty()
        && name.is_ascii()
        && name.len() <= NAME_LEN
        && !name.as_bytes().contains(&0)
}
