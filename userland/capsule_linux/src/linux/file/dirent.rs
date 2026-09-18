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


//! `struct linux_dirent64`. A libc walks by adding d_reclen, so a wrong
//! reclen lands the reader mid-record.

use alloc::vec::Vec;

/// d_ino, d_off, d_reclen, d_type: 8 + 8 + 2 + 1.
pub const HEADER: usize = 19;

// OP_LIST does not say file or directory. DT_UNKNOWN makes the caller
// stat; guessing DT_REG makes it skip the stat and be wrong.
pub const DT_UNKNOWN: u8 = 0;

/// The bytes one entry occupies, name and terminator included.
pub fn record_len(name: &[u8]) -> usize {
    (HEADER + name.len() + 1).div_ceil(8) * 8
}

pub fn encode(out: &mut Vec<u8>, name: &[u8], index: u64, kind: u8) {
    let len = record_len(name);
    let start = out.len();
    out.extend_from_slice(&(index + 1).to_le_bytes());
    out.extend_from_slice(&(index as i64 + 1).to_le_bytes());
    out.extend_from_slice(&(len as u16).to_le_bytes());
    out.push(kind);
    out.extend_from_slice(name);
    out.resize(start + len, 0);
}
