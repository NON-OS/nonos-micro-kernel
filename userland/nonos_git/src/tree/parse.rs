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

use alloc::vec::Vec;

use crate::oid::ObjectId;

use super::entry::TreeEntry;
use super::is_sorted::is_sorted_and_unique;
use super::mode::Mode;
use super::name::check_name;

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

/// Rejects anything git would not have written: an unknown mode, a name that
/// could escape its directory, or a wrong order.
pub fn parse(content: &[u8]) -> Result<Vec<TreeEntry>, TreeError> {
    let mut entries = Vec::new();
    let mut i = 0usize;

    while i < content.len() {
        let space = find(content, i, b' ').ok_or(TreeError::Truncated)?;
        let mode = Mode::from_bytes(&content[i..space]).ok_or(TreeError::Mode)?;

        let nul = find(content, space + 1, 0).ok_or(TreeError::Truncated)?;
        let name = check_name(&content[space + 1..nul])?;

        let end = nul + 21;
        if end > content.len() {
            return Err(TreeError::Truncated);
        }
        let mut raw = [0u8; 20];
        raw.copy_from_slice(&content[nul + 1..end]);

        entries.push(TreeEntry { mode, name, id: ObjectId::from_bytes(raw) });
        i = end;
    }

    if !is_sorted_and_unique(&entries) {
        return Err(TreeError::Order);
    }
    Ok(entries)
}

fn find(data: &[u8], from: usize, byte: u8) -> Option<usize> {
    data.iter().skip(from).position(|b| *b == byte).map(|p| p + from)
}
