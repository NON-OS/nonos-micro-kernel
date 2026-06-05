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

pub const DESC_BYTES: usize = 16;

pub const RX_DESC_COUNT: usize = 32;
pub const TX_DESC_COUNT: usize = 32;

pub const RX_RING_BYTES: usize = RX_DESC_COUNT * DESC_BYTES;
pub const TX_RING_BYTES: usize = TX_DESC_COUNT * DESC_BYTES;

pub const RX_BUFFER_LEN: usize = 2048;
pub const TX_BUFFER_LEN: usize = 2048;

pub const RX_BUFFER_POOL_BYTES: usize = RX_DESC_COUNT * RX_BUFFER_LEN;
pub const TX_BUFFER_POOL_BYTES: usize = TX_DESC_COUNT * TX_BUFFER_LEN;

pub const RX_STATUS_DD: u8 = 1 << 0;
pub const RX_STATUS_EOP: u8 = 1 << 1;

pub const TX_CMD_EOP: u8 = 1 << 0;
pub const TX_CMD_IFCS: u8 = 1 << 1;
pub const TX_CMD_RS: u8 = 1 << 3;
pub const TX_STATUS_DD: u8 = 1 << 0;
