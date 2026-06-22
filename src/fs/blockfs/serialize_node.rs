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

use super::constants::{NODE_DIGEST_OFFSET, NODE_MAGIC, VERSION};
use super::digest::digest;
use super::write_u16::write_u16;
use super::write_u32::write_u32;
use super::write_u64::write_u64;
use super::BlockFsNode;
use crate::fs::cryptoblock::PLAIN_BLOCK_BYTES;

pub(crate) fn serialize_node(node: &BlockFsNode) -> [u8; PLAIN_BLOCK_BYTES] {
    let mut out = [0u8; PLAIN_BLOCK_BYTES];
    out[0..8].copy_from_slice(&NODE_MAGIC);
    write_u64(&mut out, 8, VERSION);
    write_u64(&mut out, 16, node.generation);
    write_u16(&mut out, 24, node.mode);
    write_u32(&mut out, 28, node.uid);
    write_u32(&mut out, 32, node.gid);
    write_u32(&mut out, 36, node.links);
    write_u64(&mut out, 40, node.size);
    write_u64(&mut out, 48, node.blocks);
    write_u64(&mut out, 56, node.ctime);
    write_u64(&mut out, 64, node.mtime);
    write_u64(&mut out, 72, node.atime);
    write_u64(&mut out, 80, node.first_record_lba);
    let d = digest(&out[0..NODE_DIGEST_OFFSET]);
    out[NODE_DIGEST_OFFSET..NODE_DIGEST_OFFSET + 32].copy_from_slice(&d);
    out
}
