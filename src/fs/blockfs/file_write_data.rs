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
use super::file_consts::{DATA_BYTES, INDEX_PTR_BASE, MAX_PTRS, PTR_BYTES};
use super::write_u64::write_u64;
use super::{BlockFsError, BlockFsMount};
use crate::fs::cryptoblock::PLAIN_BLOCK_BYTES;

pub(super) fn write_chunks(
    key: &[u8; 32],
    mount: &mut BlockFsMount,
    data: &[u8],
    index: &mut [u8],
) -> Result<u32, BlockFsError> {
    let mut count = 0u32;
    for chunk in data.chunks(DATA_BYTES) {
        // Local floor: the index block holds at most MAX_PTRS pointers. Callers
        // already bound data.len() by MAX_FILE_BYTES, but keep this so a relaxed
        // guard or a second caller can never write past the index buffer.
        if count as usize >= MAX_PTRS {
            return Err(BlockFsError::OutOfSpace);
        }
        let lba = alloc_block(mount)?;
        let mut block = [0u8; PLAIN_BLOCK_BYTES];
        block[..chunk.len()].copy_from_slice(chunk);
        crate::fs::cryptoblock::write(key, lba, &block).map_err(BlockFsError::CryptoBlock)?;
        write_u64(index, INDEX_PTR_BASE + count as usize * PTR_BYTES, lba);
        count += 1;
    }
    Ok(count)
}
