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
pub const IC_DATA_CMD_READ: u32 = 1 << 8;
pub const IC_DATA_CMD_STOP: u32 = 1 << 9;
pub const IC_DATA_CMD_RESTART: u32 = 1 << 10;
pub const IC_INTR_TX_ABRT: u32 = 1 << 6;
pub const IC_STATUS_TFE: u32 = 1 << 2;
pub const IC_STATUS_RFNE: u32 = 1 << 3;
pub const IC_STATUS_MST_ACTIVITY: u32 = 1 << 5;
pub const IC_ENABLE_ENABLE: u32 = 1;
pub const IC_CLR_TX_ABRT: u64 = 0x54;
pub const TIMEOUT_ITERS: usize = 250_000;
pub const TX_FIFO_DEPTH: u32 = 64;
pub const RX_FIFO_DEPTH: u32 = 64;
pub const IC_CON_MASTER_MODE: u32 = 1 << 0;
pub const IC_CON_SPEED_FAST: u32 = 2 << 1;
pub const IC_CON_RESTART_EN: u32 = 1 << 5;
pub const IC_CON_SLAVE_DISABLE: u32 = 1 << 6;
