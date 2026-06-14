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











pub const Q_RX: u16 = 0;
pub const Q_TX: u16 = 1;

pub const QUEUE_SIZE: u16 = 8;
pub const VRING_DESC_F_WRITE: u16 = 2;

pub const VQ_DESC_OFFSET: usize = 0;
pub const VQ_AVAIL_OFFSET: usize = 4096;
pub const VQ_USED_OFFSET: usize = 8192;
pub const VQ_REGION_SIZE: usize = 12288;




pub const RX_DESC_COUNT: u16 = QUEUE_SIZE;
pub const TX_DESC_COUNT: u16 = QUEUE_SIZE;




pub const RX_BUFFER_LEN: u32 = 2048;
pub const TX_BUFFER_LEN: u32 = 2048;
