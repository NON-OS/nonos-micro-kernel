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

// The "NBLK" wire contract of driver.virtio_blk0, mirrored from that capsule's
// src/protocol/. It is duplicated rather than shared because capsules link no
// common crate; any change on the driver side has to be reflected here.
pub const MAGIC: u32 = 0x4E42_4C4B;
pub const VERSION: u16 = 1;
pub const HDR_LEN: usize = 20;
pub const STATUS_LEN: usize = 4;
pub const RW_REQ_LEN: usize = 12;
pub const CAPACITY_BODY_LEN: usize = 8;
pub const SECTOR_SIZE: usize = 512;
pub const MAX_SECTORS_PER_REQUEST: usize = 64;
pub const MAX_READ_BYTES: usize = SECTOR_SIZE * MAX_SECTORS_PER_REQUEST;
pub const OP_CAPACITY: u16 = 2;
pub const OP_READ_BLOCKS: u16 = 3;
