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

use super::constants::{NODE_BYTES, NODE_DIGEST_OFFSET, NODE_MAGIC, VERSION};
use super::digest::digest;
use super::read_u16::read_u16;
use super::read_u32::read_u32;
use super::read_u64::read_u64;
use super::{BlockFsError, BlockFsNode};

pub(crate) fn deserialize_node(buf: &[u8]) -> Result<BlockFsNode, BlockFsError> {
    if buf.len() < NODE_BYTES || buf[0..8] != NODE_MAGIC[..] || read_u64(buf, 8) != VERSION {
        return Err(BlockFsError::InvalidSuperblock);
    }
    let expect = digest(&buf[0..NODE_DIGEST_OFFSET]);
    if buf[NODE_DIGEST_OFFSET..NODE_DIGEST_OFFSET + 32] != expect[..] {
        return Err(BlockFsError::InvalidSuperblock);
    }
    let mut stored = [0u8; 32];
    stored.copy_from_slice(&buf[NODE_DIGEST_OFFSET..NODE_DIGEST_OFFSET + 32]);
    Ok(BlockFsNode {
        generation: read_u64(buf, 16),
        mode: read_u16(buf, 24),
        uid: read_u32(buf, 28),
        gid: read_u32(buf, 32),
        links: read_u32(buf, 36),
        size: read_u64(buf, 40),
        blocks: read_u64(buf, 48),
        ctime: read_u64(buf, 56),
        mtime: read_u64(buf, 64),
        atime: read_u64(buf, 72),
        first_record_lba: read_u64(buf, 80),
        digest: stored,
    })
}
