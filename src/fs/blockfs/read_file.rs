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

use super::file_consts::{
    DATA_BYTES, INDEX_COUNT_OFFSET, INDEX_MAGIC, INDEX_PTR_BASE, MAX_PTRS, PTR_BYTES,
};
use super::read_u32::read_u32;
use super::read_u64::read_u64;
use super::{BlockFsError, BlockFsNode};

pub fn read_file(
    key: &[u8; 32],
    node: &BlockFsNode,
    out: &mut [u8],
) -> Result<usize, BlockFsError> {
    if node.first_record_lba == 0 || node.size == 0 {
        return Ok(0);
    }
    let index = crate::fs::cryptoblock::read(key, node.first_record_lba)
        .map_err(BlockFsError::CryptoBlock)?;
    if index[0..8] != INDEX_MAGIC[..] {
        return Err(BlockFsError::InvalidRecord);
    }
    let count = (read_u32(&index, INDEX_COUNT_OFFSET) as usize).min(MAX_PTRS);
    let total = (node.size as usize).min(out.len());
    let mut written = 0usize;
    for i in 0..count {
        if written >= total {
            break;
        }
        let lba = read_u64(&index, INDEX_PTR_BASE + i * PTR_BYTES);
        let block = crate::fs::cryptoblock::read(key, lba).map_err(BlockFsError::CryptoBlock)?;
        let n = (total - written).min(DATA_BYTES);
        out[written..written + n].copy_from_slice(&block[..n]);
        written += n;
    }
    Ok(written)
}
