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
use super::dir_lookup::lookup;
use super::dir_record_alloc::ensure_record;
use super::read_u32::read_u32;
use super::write_node::write_node;
use super::write_u32::write_u32;
use super::write_u64::write_u64;
use super::{BlockFsError, BlockFsMount, BlockFsNode};

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
    let rec_lba = ensure_record(key, mount, dir)?;
    let mut block =
        crate::fs::cryptoblock::read(key, rec_lba).map_err(BlockFsError::CryptoBlock)?;
    if block[0..8] != REC_MAGIC[..] {
        return Err(BlockFsError::InvalidRecord);
    }
    let count = read_u32(&block, REC_COUNT_OFFSET) as usize;
    if count >= MAX_ENTRIES {
        return Err(BlockFsError::DirectoryFull);
    }
    let off = REC_ENTRY_BASE + count * ENTRY_BYTES;
    block[off..off + name.len()].copy_from_slice(name);
    for b in block[off + name.len()..off + NAME_BYTES].iter_mut() {
        *b = 0;
    }
    write_u64(&mut block, off + NAME_BYTES, child_lba);
    write_u32(&mut block, REC_COUNT_OFFSET, (count + 1) as u32);
    crate::fs::cryptoblock::write(key, rec_lba, &block).map_err(BlockFsError::CryptoBlock)?;
    dir.size += ENTRY_BYTES as u64;
    write_node(key, dir_lba, dir)
}
