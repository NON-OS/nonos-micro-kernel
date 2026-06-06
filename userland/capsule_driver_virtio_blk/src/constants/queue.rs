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
pub const MAX_QUEUE_SIZE: u16 = 256;
pub const VRING_DESC_F_NEXT: u16 = 1;
pub const VRING_DESC_F_WRITE: u16 = 2;
pub const VQ_DESC_OFFSET: usize = 0;
pub const VQ_REGION_SIZE: usize = 16384;
pub const SECTOR_SIZE: usize = 512;
pub const MAX_SECTORS_PER_REQUEST: u32 = 64;
pub const HEADER_OFFSET: usize = 0;
pub const HEADER_BUF_LEN: u64 = 4096;
pub const STATUS_OFFSET: usize = 64;
pub const DATA_BUF_LEN: u64 = (MAX_SECTORS_PER_REQUEST as u64) * (SECTOR_SIZE as u64);
