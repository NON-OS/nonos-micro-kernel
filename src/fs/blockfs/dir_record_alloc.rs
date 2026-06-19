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
use super::dir_consts::REC_MAGIC;
use super::{BlockFsError, BlockFsMount, BlockFsNode};
use crate::fs::cryptoblock::PLAIN_BLOCK_BYTES;

pub(super) fn ensure_record(
    key: &[u8; 32],
    mount: &mut BlockFsMount,
    dir: &mut BlockFsNode,
) -> Result<u64, BlockFsError> {
    if dir.first_record_lba != 0 {
        return Ok(dir.first_record_lba);
    }
    let lba = alloc_block(mount)?;
    commit(key, mount)?;
    let mut empty = [0u8; PLAIN_BLOCK_BYTES];
    empty[0..8].copy_from_slice(&REC_MAGIC);
    crate::fs::cryptoblock::write(key, lba, &empty).map_err(BlockFsError::CryptoBlock)?;
    dir.first_record_lba = lba;
    Ok(lba)
}
