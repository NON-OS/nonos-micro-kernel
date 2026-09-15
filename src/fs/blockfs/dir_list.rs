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

//! What is in a directory.
//!
//! The filesystem could look a name up and could not say what names exist,
//! which is enough for a kernel opening a path it already knows and not
//! enough for anything a person uses: no file manager, no listing, and no way
//! for a server to learn at boot what it kept last time.
//!
//! Names come back in the order the records hold them, which is creation
//! order with removals compacted over. Sorting is the caller's, since the
//! caller knows whether it wants them by name, by time or not at all.

use alloc::vec::Vec;

use super::dir_block::{lba, name};
use super::dir_block_header::{count, is_record};
use super::dir_chain::records;
use super::{BlockFsError, BlockFsNode};

pub struct DirEntry {
    pub name: Vec<u8>,
    pub node_lba: u64,
}

pub fn list(key: &[u8; 32], dir: &BlockFsNode) -> Result<Vec<DirEntry>, BlockFsError> {
    let mut out = Vec::new();
    for rec in records(key, dir)? {
        let block = crate::fs::cryptoblock::read(key, rec).map_err(BlockFsError::CryptoBlock)?;
        if !is_record(&block) {
            return Err(BlockFsError::InvalidRecord);
        }
        for i in 0..count(&block) {
            let trimmed = trim(name(&block, i));
            /*
             * An entry whose name is entirely padding cannot be looked up by
             * any caller, so it is skipped rather than returned as an empty
             * name that a listing would draw as a blank row.
             */
            if trimmed.is_empty() {
                continue;
            }
            out.push(DirEntry { name: trimmed, node_lba: lba(&block, i) });
        }
    }
    Ok(out)
}

/// Names are stored NUL padded to the full field.
fn trim(field: &[u8]) -> Vec<u8> {
    let end = field.iter().position(|&b| b == 0).unwrap_or(field.len());
    field[..end].to_vec()
}
