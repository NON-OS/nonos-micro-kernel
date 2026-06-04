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
use crate::constants::{DATA_BUF_LEN, MAX_SECTORS_PER_REQUEST, SECTOR_SIZE};
pub const STATUS_LEN: usize = 4;
pub const RW_HEADER_LEN: usize = 12;
pub const READ_REQ_LEN: usize = RW_HEADER_LEN;
pub const MAX_RW_PAYLOAD_BYTES: u32 = DATA_BUF_LEN as u32;
pub const CAPACITY_PAYLOAD_LEN: usize = 8;
const _: () = assert!(MAX_RW_PAYLOAD_BYTES == MAX_SECTORS_PER_REQUEST * SECTOR_SIZE as u32);
