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

use super::dir_consts::{
    ENTRY_BYTES, MAX_ENTRIES, NAME_BYTES, REC_COUNT_OFFSET, REC_ENTRY_BASE, REC_MAGIC,
};
use super::dir_name_match::name_matches;
use super::read_u32::read_u32;
use super::write_node::write_node;
use super::write_u32::write_u32;
use super::{BlockFsError, BlockFsNode};

pub(crate) fn unlink(
    key: &[u8; 32],
    dir_lba: u64,
    dir: &mut BlockFsNode,
    name: &[u8],
) -> Result<(), BlockFsError> {
    if dir.first_record_lba == 0 {
        return Err(BlockFsError::NotFound);
    }
    let mut block = crate::fs::cryptoblock::read(key, dir.first_record_lba)
        .map_err(BlockFsError::CryptoBlock)?;
    if block[0..8] != REC_MAGIC[..] {
        return Err(BlockFsError::InvalidRecord);
    }
    let count = (read_u32(&block, REC_COUNT_OFFSET) as usize).min(MAX_ENTRIES);
    let mut idx = count;
    for i in 0..count {
        let off = REC_ENTRY_BASE + i * ENTRY_BYTES;
        if name_matches(&block[off..off + NAME_BYTES], name) {
            idx = i;
            break;
        }
    }
    if idx == count {
        return Err(BlockFsError::NotFound);
    }
    for j in idx..count - 1 {
        let dst = REC_ENTRY_BASE + j * ENTRY_BYTES;
        let src = REC_ENTRY_BASE + (j + 1) * ENTRY_BYTES;
        block.copy_within(src..src + ENTRY_BYTES, dst);
    }
    let last = REC_ENTRY_BASE + (count - 1) * ENTRY_BYTES;
    for b in block[last..last + ENTRY_BYTES].iter_mut() {
        *b = 0;
    }
    write_u32(&mut block, REC_COUNT_OFFSET, (count - 1) as u32);
    crate::fs::cryptoblock::write(key, dir.first_record_lba, &block)
        .map_err(BlockFsError::CryptoBlock)?;
    dir.size = dir.size.saturating_sub(ENTRY_BYTES as u64);
    write_node(key, dir_lba, dir)
}
