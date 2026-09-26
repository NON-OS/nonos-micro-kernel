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


//! The Wayland message header and the argument cursor.

pub const HEADER: usize = 8;

pub struct Msg<'a> {
    pub object: u32,
    pub opcode: u16,
    pub args: &'a [u8],
}

/// The next message in `buf`, or nothing if it has not all arrived.
pub fn next(buf: &[u8]) -> Option<(Msg<'_>, usize)> {
    if buf.len() < HEADER {
        return None;
    }
    let object = u32::from_le_bytes([buf[0], buf[1], buf[2], buf[3]]);
    let opcode = u16::from_le_bytes([buf[4], buf[5]]);
    let size = u16::from_le_bytes([buf[6], buf[7]]) as usize;
    if size < HEADER || size > buf.len() {
        return None;
    }
    Some((Msg { object, opcode, args: &buf[HEADER..size] }, size))
}
