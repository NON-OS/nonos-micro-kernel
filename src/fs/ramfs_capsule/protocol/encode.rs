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

use alloc::vec::Vec;

use super::types::{HDR_LEN, OP_CLOSE, OP_OPEN, OP_READ, OP_TRUNCATE, OP_WRITE};

fn header(seq: u32, op: u16) -> Vec<u8> {
    let mut out = Vec::with_capacity(HDR_LEN);
    out.extend_from_slice(&seq.to_le_bytes());
    out.extend_from_slice(&op.to_le_bytes());
    out.extend_from_slice(&0u16.to_le_bytes());
    out
}

pub(in crate::fs::ramfs_capsule) fn encode_open(seq: u32, flags: u32, path: &str) -> Vec<u8> {
    let mut out = header(seq, OP_OPEN);
    out.extend_from_slice(&flags.to_le_bytes());
    out.extend_from_slice(&(path.len() as u16).to_le_bytes());
    out.extend_from_slice(path.as_bytes());
    out
}

pub(in crate::fs::ramfs_capsule) fn encode_read(
    seq: u32,
    handle: u64,
    offset: u64,
    count: u32,
) -> Vec<u8> {
    let mut out = header(seq, OP_READ);
    out.extend_from_slice(&handle.to_le_bytes());
    out.extend_from_slice(&offset.to_le_bytes());
    out.extend_from_slice(&count.to_le_bytes());
    out
}

pub(in crate::fs::ramfs_capsule) fn encode_write(
    seq: u32,
    handle: u64,
    offset: u64,
    data: &[u8],
) -> Vec<u8> {
    let mut out = header(seq, OP_WRITE);
    out.extend_from_slice(&handle.to_le_bytes());
    out.extend_from_slice(&offset.to_le_bytes());
    out.extend_from_slice(data);
    out
}

pub(in crate::fs::ramfs_capsule) fn encode_truncate(seq: u32, handle: u64, length: u64) -> Vec<u8> {
    let mut out = header(seq, OP_TRUNCATE);
    out.extend_from_slice(&handle.to_le_bytes());
    out.extend_from_slice(&length.to_le_bytes());
    out
}

pub(in crate::fs::ramfs_capsule) fn encode_close(seq: u32, handle: u64) -> Vec<u8> {
    let mut out = header(seq, OP_CLOSE);
    out.extend_from_slice(&handle.to_le_bytes());
    out
}
