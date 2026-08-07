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

pub const MAGIC: u32 = 0x4E4F_5646; // "NOVF"
pub const VERSION: u16 = 1;

pub const OP_OPEN: u16 = 1;
pub const OP_CLOSE: u16 = 2;
pub const OP_READ: u16 = 3;
pub const OP_WRITE: u16 = 4;
pub const OP_STAT: u16 = 5;
pub const OP_LIST: u16 = 6;
pub const OP_HEALTHCHECK: u16 = 7;
pub const OP_MKDIR: u16 = 8;
pub const OP_UNLINK: u16 = 9;
pub const OP_RENAME: u16 = 10;
pub const OP_RMDIR: u16 = 11;
pub const OP_COPY: u16 = 12;
pub const OP_TRUNCATE: u16 = 13;
pub const OP_USAGE: u16 = 14;
pub const OP_CHMOD: u16 = 15;
pub const OP_SEEK: u16 = 16;
pub const OP_STORE_PERSIST: u16 = 17;
pub const OP_STORE_REMOVE: u16 = 18;
pub const OP_STORE_STATUS: u16 = 19;

// Seek whence values, matching the POSIX ordering std uses.
pub const SEEK_SET: u8 = 0;
pub const SEEK_CUR: u8 = 1;
pub const SEEK_END: u8 = 2;

pub const O_CREATE: u32 = 1 << 0;
pub const O_TRUNC: u32 = 1 << 1;
pub const O_APPEND: u32 = 1 << 2;

pub const MAX_PATH_BYTES: u32 = 256;
pub const MAX_DATA_BYTES: u32 = 65536;
pub const MAX_LIST_BYTES: u32 = 65536;
pub const MAX_PAYLOAD_BYTES: u32 = 65536;

// Distinct from ramfs (4294967297), keyring (4294967298), entropy
// (4294967299), crypto (4294967300).
pub const KERNEL_REPLY_ENDPOINT: u64 = 0x1_0000_0005;

pub const HDR_LEN: usize = 20;

#[derive(Clone, Copy)]
pub struct Request<'a> {
    pub op: u16,
    pub flags: u16,
    pub request_id: u32,
    pub payload: &'a [u8],
}
