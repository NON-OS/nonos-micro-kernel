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

//! Taking a name out of a directory.

use super::dir_block::find;
use super::dir_block_header::is_record;
use super::dir_chain::records;
use super::dir_consts::ENTRY_BYTES;
use super::dir_edit::remove;
use super::write_node::write_node;
use super::{BlockFsError, BlockFsNode};

/// Remove `name` from whichever record block holds it.
///
/// Emptied blocks stay in the chain. A directory that briefly held a thousand
/// files keeps its blocks, which costs space and saves the next thousand an
/// allocation each; nothing reads an empty record as anything but zero
/// entries.
pub(crate) fn unlink(
    key: &[u8; 32],
    dir_lba: u64,
    dir: &mut BlockFsNode,
    name: &[u8],
) -> Result<(), BlockFsError> {
    for rec in records(key, dir)? {
        let mut block =
            crate::fs::cryptoblock::read(key, rec).map_err(BlockFsError::CryptoBlock)?;
        if !is_record(&block) {
            return Err(BlockFsError::InvalidRecord);
        }
        let Some(i) = find(&block, name) else {
            continue;
        };
        if !remove(&mut block, i) {
            return Err(BlockFsError::InvalidRecord);
        }
        crate::fs::cryptoblock::write(key, rec, &block).map_err(BlockFsError::CryptoBlock)?;
        dir.size = dir.size.saturating_sub(ENTRY_BYTES as u64);
        return write_node(key, dir_lba, dir);
    }
    Err(BlockFsError::NotFound)
}
