// NØNOS Operating System
// Copyright (C) 2026 NØNOS Contributors
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

//! Adding a name to a directory.

use super::dir_block_header::is_record;
use super::dir_consts::{ENTRY_BYTES, NAME_BYTES};
use super::dir_edit::append;
use super::dir_lookup::lookup;
use super::dir_room::record_with_room;
use super::write_node::write_node;
use super::{BlockFsError, BlockFsMount, BlockFsNode};

/// Link `name` to `child_lba` in `dir`.
///
/// A name that already exists anywhere in the chain is refused: two entries
/// with one name would resolve to whichever record came first and leak the
/// other file, which nothing could then reach or remove.
pub fn link(
    key: &[u8; 32],
    mount: &mut BlockFsMount,
    dir_lba: u64,
    dir: &mut BlockFsNode,
    name: &[u8],
    child_lba: u64,
) -> Result<(), BlockFsError> {
    if name.is_empty() || name.len() > NAME_BYTES {
        return Err(BlockFsError::InvalidName);
    }
    if lookup(key, dir, name)?.is_some() {
        return Err(BlockFsError::InvalidName);
    }
    let rec_lba = record_with_room(key, mount, dir)?;
    let mut block = crate::fs::cryptoblock::read(key, rec_lba).map_err(BlockFsError::CryptoBlock)?;
    if !is_record(&block) {
        return Err(BlockFsError::InvalidRecord);
    }
    /*
     * record_with_room only answers with a block that had room, so a refusal
     * here means the block changed underneath us or its count is corrupt.
     * Neither is a full directory, and saying so would send a caller looking
     * for space that is already there.
     */
    if append(&mut block, name, child_lba).is_none() {
        return Err(BlockFsError::InvalidRecord);
    }
    crate::fs::cryptoblock::write(key, rec_lba, &block).map_err(BlockFsError::CryptoBlock)?;
    dir.size += ENTRY_BYTES as u64;
    write_node(key, dir_lba, dir)
}
