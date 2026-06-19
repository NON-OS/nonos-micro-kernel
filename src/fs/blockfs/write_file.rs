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

use super::alloc_block::alloc_block;
use super::commit::commit;
use super::file_consts::{INDEX_COUNT_OFFSET, INDEX_MAGIC, MAX_FILE_BYTES};
use super::file_write_data::write_chunks;
use super::write_node::write_node;
use super::write_u32::write_u32;
use super::{BlockFsError, BlockFsMount, BlockFsNode};
use crate::fs::cryptoblock::PLAIN_BLOCK_BYTES;

pub fn write_file(
    key: &[u8; 32],
    mount: &mut BlockFsMount,
    node_lba: u64,
    node: &mut BlockFsNode,
    data: &[u8],
) -> Result<(), BlockFsError> {
    if data.len() > MAX_FILE_BYTES {
        return Err(BlockFsError::OutOfSpace);
    }
    let index_lba = alloc_block(mount)?;
    let mut index = [0u8; PLAIN_BLOCK_BYTES];
    index[0..8].copy_from_slice(&INDEX_MAGIC);
    let count = write_chunks(key, mount, data, &mut index)?;
    write_u32(&mut index, INDEX_COUNT_OFFSET, count);
    crate::fs::cryptoblock::write(key, index_lba, &index).map_err(BlockFsError::CryptoBlock)?;
    node.first_record_lba = index_lba;
    node.size = data.len() as u64;
    node.blocks = 1 + count as u64;
    commit(key, mount)?;
    write_node(key, node_lba, node)
}
