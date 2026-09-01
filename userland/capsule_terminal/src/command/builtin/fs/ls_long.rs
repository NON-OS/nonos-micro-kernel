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

//! The `-l` row. There are no uid, gid or permission bits in this system, so
//! the mode column carries only the two facts the vfs actually reports: the
//! directory flag and the writable bit.

use alloc::vec::Vec;

use super::ls_num::{decimal, human_size, pad_left};

pub struct Row {
    pub name: Vec<u8>,
    pub size: u64,
    pub is_dir: bool,
    pub mtime: u64,
    pub writable: bool,
}

pub fn long_row(row: &Row, human: bool) -> Vec<u8> {
    let mut line = Vec::new();
    line.push(if row.is_dir { b'd' } else { b'-' });
    line.push(b'r');
    line.push(if row.writable { b'w' } else { b'-' });
    line.push(b'-');
    line.push(b' ');
    let size = if human { human_size(row.size) } else { decimal(row.size) };
    pad_left(&mut line, &size, 9);
    line.push(b' ');
    let stamp = if row.mtime == 0 { Vec::from(&b"-"[..]) } else { decimal(row.mtime) };
    pad_left(&mut line, &stamp, 14);
    line.push(b' ');
    line.extend_from_slice(&row.name);
    line
}
