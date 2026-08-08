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

//! Header and checksum, then the entries.

extern crate alloc;

use alloc::vec::Vec;

use crate::index::entry::IndexEntry;
use crate::index::error::IndexError;
use crate::sha1::Sha1;

use super::entries::read_entries;

/// Parse a version 2 index. The trailing SHA-1 is checked before any entry is
/// believed, so a truncated or edited index is refused outright.
pub fn parse(data: &[u8]) -> Result<Vec<IndexEntry>, IndexError> {
    if data.len() < 32 {
        return Err(IndexError::Truncated);
    }
    if &data[..4] != b"DIRC" {
        return Err(IndexError::Magic);
    }
    let version = u32::from_be_bytes([data[4], data[5], data[6], data[7]]);
    if version != 2 {
        return Err(IndexError::Version(version));
    }

    let body = &data[..data.len() - 20];
    if Sha1::digest(body) != data[data.len() - 20..] {
        return Err(IndexError::Checksum);
    }

    let count = u32::from_be_bytes([data[8], data[9], data[10], data[11]]) as usize;
    read_entries(body, count)
}
