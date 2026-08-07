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

pub const NAME: &[u8] = b"vfs_pool";
pub const MAGIC: u32 = 0x4E4F_5646;
pub const OP_OPEN: u16 = 1;
pub const OP_CLOSE: u16 = 2;
pub const OP_READ: u16 = 3;
pub const OP_WRITE: u16 = 4;
pub const OP_STAT: u16 = 5;
pub const OP_LIST: u16 = 6;
pub const OP_MKDIR: u16 = 8;
pub const OP_UNLINK: u16 = 9;
pub const OP_RENAME: u16 = 10;
pub const OP_RMDIR: u16 = 11;
pub const OP_COPY: u16 = 12;
pub const OP_TRUNCATE: u16 = 13;
pub const OP_USAGE: u16 = 14;
pub const OP_CHMOD: u16 = 15;
pub const OP_STORE_PERSIST: u16 = 17;
pub const OP_STORE_REMOVE: u16 = 18;
pub const OP_STORE_STATUS: u16 = 19;
pub const O_CREATE: u32 = 1 << 0;
pub const O_TRUNC: u32 = 1 << 1;
