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

// VFS wire protocol constants: the frame magic and offsets, the operation
// codes, the open flags, and the seek whence values.

pub(crate) const MAGIC: u32 = 0x4E4F_5646;
pub(crate) const STATUS_OFF: usize = 20;
pub(crate) const BODY_OFF: usize = 24;
pub(crate) const OP_OPEN: u16 = 1;
pub(crate) const OP_CLOSE: u16 = 2;
pub(crate) const OP_READ: u16 = 3;
pub(crate) const OP_WRITE: u16 = 4;
pub(crate) const OP_STAT: u16 = 5;
pub(crate) const OP_LIST: u16 = 6;
pub(crate) const OP_MKDIR: u16 = 8;
pub(crate) const OP_UNLINK: u16 = 9;
pub(crate) const OP_RENAME: u16 = 10;
pub(crate) const OP_RMDIR: u16 = 11;
pub(crate) const OP_TRUNCATE: u16 = 13;
pub(crate) const OP_SEEK: u16 = 16;
pub(crate) const O_CREATE: u32 = 1;
pub(crate) const O_TRUNC: u32 = 1 << 1;
pub(crate) const O_APPEND: u32 = 1 << 2;
pub(crate) const SEEK_SET: u8 = 0;
pub(crate) const SEEK_CUR: u8 = 1;
pub(crate) const SEEK_END: u8 = 2;
