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

use super::constants::{MAGIC, SUPERBLOCK_BYTES, VERSION};
use super::digest::digest;
use super::read_u64::read_u64;
use super::{BlockFsError, BlockFsSuperblock};

pub(crate) fn deserialize(buf: &[u8]) -> Result<BlockFsSuperblock, BlockFsError> {
    if buf.len() < SUPERBLOCK_BYTES || buf[0..8] != MAGIC[..] || read_u64(buf, 8) != VERSION {
        return Err(BlockFsError::InvalidSuperblock);
    }
    let expect = digest(&buf[0..72]);
    if buf[72..SUPERBLOCK_BYTES] != expect[..] {
        return Err(BlockFsError::InvalidSuperblock);
    }
    let mut uuid = [0u8; 16];
    let mut stored = [0u8; 32];
    uuid.copy_from_slice(&buf[56..72]);
    stored.copy_from_slice(&buf[72..SUPERBLOCK_BYTES]);
    Ok(BlockFsSuperblock {
        generation: read_u64(buf, 16),
        sectors: read_u64(buf, 24),
        root_lba: read_u64(buf, 32),
        free_lba: read_u64(buf, 40),
        block_bytes: read_u64(buf, 48),
        uuid,
        digest: stored,
    })
}
