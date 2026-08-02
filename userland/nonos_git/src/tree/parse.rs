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

//! Reading a tree's content bytes back into entries.

extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

use crate::oid::ObjectId;

use super::entry::{Mode, TreeEntry};
use super::is_sorted::is_sorted_and_unique;

/// Why a byte slice is not a well-formed tree.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum TreeError {
    /// An entry was cut short: no space, no NUL, or fewer than 20 id bytes.
    Truncated,
    /// The mode field was not one git writes.
    Mode,
    /// A name was empty, held a NUL or a `/`, or was `.` or `..`.
    Name,
    /// Entries were out of order or a name appeared twice.
    Order,
}

/// Parse tree content into entries, rejecting anything git would not have
/// written: an unknown mode, a name that could escape its directory, or an
/// order that would mean a different tree hash than the bytes claim.
pub fn parse(content: &[u8]) -> Result<Vec<TreeEntry>, TreeError> {
    let mut entries = Vec::new();
    let mut i = 0usize;

    while i < content.len() {
        let space = find(content, i, b' ').ok_or(TreeError::Truncated)?;
        let mode = Mode::from_bytes(&content[i..space]).ok_or(TreeError::Mode)?;

        let nul = find(content, space + 1, 0).ok_or(TreeError::Truncated)?;
        let name_bytes = &content[space + 1..nul];
        let name = check_name(name_bytes)?;

        let id_start = nul + 1;
        let id_end = id_start + 20;
        if id_end > content.len() {
            return Err(TreeError::Truncated);
        }
        let mut raw = [0u8; 20];
        raw.copy_from_slice(&content[id_start..id_end]);

        entries.push(TreeEntry { mode, name, id: ObjectId::from_bytes(raw) });
        i = id_end;
    }

    if !is_sorted_and_unique(&entries) {
        return Err(TreeError::Order);
    }
    Ok(entries)
}

/// A tree entry name is a single path component. Rejecting `/`, `.` and `..`
/// here is what stops a hostile tree from writing outside the work tree when it
/// is later checked out.
fn check_name(bytes: &[u8]) -> Result<String, TreeError> {
    if bytes.is_empty() || bytes == b"." || bytes == b".." {
        return Err(TreeError::Name);
    }
    if bytes.contains(&b'/') || bytes.contains(&0) {
        return Err(TreeError::Name);
    }
    core::str::from_utf8(bytes).map(String::from).map_err(|_| TreeError::Name)
}

fn find(data: &[u8], from: usize, byte: u8) -> Option<usize> {
    data.iter().skip(from).position(|b| *b == byte).map(|p| p + from)
}
