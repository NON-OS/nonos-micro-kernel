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

//! Walking the entry records.

extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

use crate::index::entry::IndexEntry;
use crate::index::error::IndexError;
use crate::oid::ObjectId;

use super::mode::mode_from_word;
use super::path::check_path;

/// Fixed part of an entry, before its variable-length path.
const FIXED: usize = 62;

pub(super) fn read_entries(body: &[u8], count: usize) -> Result<Vec<IndexEntry>, IndexError> {
    let mut entries = Vec::with_capacity(count);
    let mut pos = 12usize;

    for _ in 0..count {
        let start = pos;
        if pos + FIXED > body.len() {
            return Err(IndexError::Truncated);
        }
        let at =
            |o: usize| u32::from_be_bytes(body[pos + o..pos + o + 4].try_into().unwrap_or([0; 4]));
        let mode_word = at(24);
        let size = at(36);
        let mut raw = [0u8; 20];
        raw.copy_from_slice(&body[pos + 40..pos + 60]);
        pos += FIXED;

        // The path runs to the first NUL: the flags' length field is only
        // twelve bits, so a long path cannot be trusted from the header.
        let end = body[pos..].iter().position(|b| *b == 0).ok_or(IndexError::Truncated)? + pos;
        let path = core::str::from_utf8(&body[pos..end]).map_err(|_| IndexError::Entry)?;
        check_path(path)?;

        entries.push(IndexEntry {
            path: String::from(path),
            mode: mode_from_word(mode_word).ok_or(IndexError::Entry)?,
            id: ObjectId::from_bytes(raw),
            size,
        });

        pos = end + 1 + (8 - ((end + 1 - start) % 8)) % 8;
    }
    Ok(entries)
}
