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

//! The `struct stat` an x86_64 Linux program expects, filled by hand.

/// Bytes of a `struct stat` on this architecture.
pub const STAT_LEN: usize = 144;

pub const S_IFREG: u32 = 0o100000;
pub const S_IFDIR: u32 = 0o040000;

const OFF_NLINK: usize = 16;
const OFF_MODE: usize = 24;
const OFF_SIZE: usize = 48;
const OFF_BLKSIZE: usize = 56;
const OFF_BLOCKS: usize = 64;

pub fn build(size: u64, is_dir: bool) -> [u8; STAT_LEN] {
    let mut out = [0u8; STAT_LEN];
    let mode = if is_dir { S_IFDIR | 0o755 } else { S_IFREG | 0o644 };
    put64(&mut out, OFF_NLINK, 1);
    put32(&mut out, OFF_MODE, mode);
    put64(&mut out, OFF_SIZE, size);
    put64(&mut out, OFF_BLKSIZE, 4096);
    put64(&mut out, OFF_BLOCKS, size.div_ceil(512));
    out
}

fn put32(out: &mut [u8; STAT_LEN], at: usize, value: u32) {
    out[at..at + 4].copy_from_slice(&value.to_le_bytes());
}

fn put64(out: &mut [u8; STAT_LEN], at: usize, value: u64) {
    out[at..at + 8].copy_from_slice(&value.to_le_bytes());
}
