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

//! Finding a record block with room, and growing the chain when there is none.

use super::alloc_block::alloc_block;
use super::commit::commit;
use super::dir_block_header::{has_room, set_next};
use super::dir_chain::records;
use super::dir_consts::REC_MAGIC;
use super::dir_record_alloc::ensure_record;
use super::{BlockFsError, BlockFsMount, BlockFsNode};
use crate::fs::cryptoblock::PLAIN_BLOCK_BYTES;

/// The address of a record block this directory can take another entry in.
///
/// The first with room, so removals leave gaps that are filled before the
/// chain grows again. When every block is full a new one is allocated and
/// linked from the last, which is the step that used to be `DirectoryFull`.
pub(super) fn record_with_room(
    key: &[u8; 32],
    mount: &mut BlockFsMount,
    dir: &mut BlockFsNode,
) -> Result<u64, BlockFsError> {
    ensure_record(key, mount, dir)?;
    let chain = records(key, dir)?;
    let mut last = 0;
    for lba in chain {
        let block = crate::fs::cryptoblock::read(key, lba).map_err(BlockFsError::CryptoBlock)?;
        if has_room(&block) {
            return Ok(lba);
        }
        last = lba;
    }
    append(key, mount, last)
}

/// A fresh record block, linked from `after`.
///
/// The new block is written before the link to it, so a power cut between the
/// two leaves an allocated block nothing points at rather than a pointer to
/// a block that was never initialised.
fn append(key: &[u8; 32], mount: &mut BlockFsMount, after: u64) -> Result<u64, BlockFsError> {
    if after == 0 {
        return Err(BlockFsError::InvalidRecord);
    }
    let lba = alloc_block(mount)?;
    commit(key, mount)?;
    let mut empty = [0u8; PLAIN_BLOCK_BYTES];
    empty[0..8].copy_from_slice(&REC_MAGIC);
    crate::fs::cryptoblock::write(key, lba, &empty).map_err(BlockFsError::CryptoBlock)?;

    let mut prev = crate::fs::cryptoblock::read(key, after).map_err(BlockFsError::CryptoBlock)?;
    set_next(&mut prev, lba);
    crate::fs::cryptoblock::write(key, after, &prev).map_err(BlockFsError::CryptoBlock)?;
    Ok(lba)
}
