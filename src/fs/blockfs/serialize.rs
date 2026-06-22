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
use super::write_u64::write_u64;
use super::BlockFsSuperblock;
use crate::fs::cryptoblock::PLAIN_BLOCK_BYTES;

pub(crate) fn serialize(sb: &BlockFsSuperblock) -> [u8; PLAIN_BLOCK_BYTES] {
    let mut out = [0u8; PLAIN_BLOCK_BYTES];
    out[0..8].copy_from_slice(&MAGIC);
    write_u64(&mut out, 8, VERSION);
    write_u64(&mut out, 16, sb.generation);
    write_u64(&mut out, 24, sb.sectors);
    write_u64(&mut out, 32, sb.root_lba);
    write_u64(&mut out, 40, sb.free_lba);
    write_u64(&mut out, 48, sb.block_bytes);
    out[56..72].copy_from_slice(&sb.uuid);
    let d = digest(&out[0..72]);
    out[72..SUPERBLOCK_BYTES].copy_from_slice(&d);
    out
}
