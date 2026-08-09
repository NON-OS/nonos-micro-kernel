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

//! Wire constants for the vfs_pool protocol.

pub(super) const VFS_PORT: u32 = 4104;
pub(super) const MAGIC: u32 = 0x4E4F_5646;
pub(super) const VERSION: u16 = 1;
pub(super) const HDR_LEN: usize = 20;

pub(super) const OP_OPEN: u16 = 1;
pub(super) const OP_CLOSE: u16 = 2;
pub(super) const OP_LIST: u16 = 6;
pub(super) const OP_MKDIR: u16 = 8;
pub(super) const OP_UNLINK: u16 = 9;
pub(super) const OP_RENAME: u16 = 10;
pub(super) const OP_RMDIR: u16 = 11;
pub(super) const OP_STORE_STATUS: u16 = 19;

pub(super) const O_CREATE: u32 = 1;

pub(super) const TIMEOUT_MS: u64 = 300;

/// Longest path or name the wire header can carry (one length byte).
pub(super) const MAX_NAME: usize = 255;
