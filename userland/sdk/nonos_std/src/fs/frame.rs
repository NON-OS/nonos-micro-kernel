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

pub(super) const MAGIC: u32 = 0x4E4F_5646;
pub(super) const VERSION: u16 = 1;
pub(super) const STATUS_OFF: usize = 20;
pub(super) const BODY_OFF: usize = 24;
pub(super) const OP_OPEN: u16 = 1;
pub(super) const OP_CLOSE: u16 = 2;
pub(super) const OP_READ: u16 = 3;
pub(super) const OP_WRITE: u16 = 4;
pub(super) const O_CREATE: u32 = 1;
pub(super) const O_TRUNC: u32 = 1 << 1;

pub(super) fn frame(op: u16, body: &[u8]) -> Vec<u8> {
    let mut f = Vec::with_capacity(STATUS_OFF + body.len());
    f.extend_from_slice(&MAGIC.to_le_bytes());
    f.extend_from_slice(&VERSION.to_le_bytes());
    f.extend_from_slice(&op.to_le_bytes());
    f.extend_from_slice(&0u16.to_le_bytes());
    f.extend_from_slice(&0u16.to_le_bytes());
    f.extend_from_slice(&0u32.to_le_bytes());
    f.extend_from_slice(&(body.len() as u32).to_le_bytes());
    f.extend_from_slice(body);
    f
}
