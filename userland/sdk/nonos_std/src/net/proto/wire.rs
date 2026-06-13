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

pub(crate) const MAGIC: u32 = 0x4E53_4B54;
pub(crate) const VERSION: u16 = 1;
pub(crate) const HDR_LEN: usize = 20;
pub(crate) const ERRNO_OFF: usize = 8;
pub(crate) const BODY_OFF: usize = 20;

pub(crate) const OP_SOCKET: u16 = 2;
pub(crate) const OP_BIND: u16 = 3;
pub(crate) const OP_LISTEN: u16 = 4;
pub(crate) const OP_ACCEPT: u16 = 5;
pub(crate) const OP_CONNECT: u16 = 6;
pub(crate) const OP_SEND: u16 = 7;
pub(crate) const OP_RECV: u16 = 8;
pub(crate) const OP_CLOSE: u16 = 9;

pub(crate) fn frame(op: u16, body: &[u8]) -> Vec<u8> {
    let mut f = Vec::with_capacity(HDR_LEN + body.len());
    f.extend_from_slice(&MAGIC.to_le_bytes());
    f.extend_from_slice(&VERSION.to_le_bytes());
    f.extend_from_slice(&op.to_le_bytes());
    f.extend_from_slice(&0u32.to_le_bytes());
    f.extend_from_slice(&0u32.to_le_bytes());
    f.extend_from_slice(&(body.len() as u32).to_le_bytes());
    f.extend_from_slice(body);
    f
}

pub(crate) fn errno(buf: &[u8]) -> u16 {
    u16::from_le_bytes([buf[ERRNO_OFF], buf[ERRNO_OFF + 1]])
}
