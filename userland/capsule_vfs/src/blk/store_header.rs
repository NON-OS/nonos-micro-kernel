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

// Framing of the package container tools/nonos-store-pack writes at LBA 256: a
// 32-byte header followed by `count` fixed-size descriptors. The device is
// outside this capsule's trust boundary, so a foreign or corrupt sector 0 has
// to fail here rather than size an allocation later.
use super::error::BlkError;

const MAGIC: &[u8; 8] = b"NONOSTR1";
const VERSION: u32 = 1;

pub const HEADER_LEN: usize = 32;
pub const ENTRY_LEN: usize = 128;
pub const MAX_ENTRIES: usize = 64;

pub fn entry_count(head: &[u8]) -> Result<usize, BlkError> {
    if head.len() < HEADER_LEN || &head[0..8] != MAGIC || le_u32(head, 8) != VERSION {
        return Err(BlkError::BadContainer);
    }
    let count = le_u32(head, 12) as usize;
    if count > MAX_ENTRIES {
        return Err(BlkError::BadContainer);
    }
    Ok(count)
}

pub fn le_u32(buf: &[u8], off: usize) -> u32 {
    u32::from_le_bytes([buf[off], buf[off + 1], buf[off + 2], buf[off + 3]])
}

pub fn le_u64(buf: &[u8], off: usize) -> u64 {
    let mut bytes = [0u8; 8];
    bytes.copy_from_slice(&buf[off..off + 8]);
    u64::from_le_bytes(bytes)
}
