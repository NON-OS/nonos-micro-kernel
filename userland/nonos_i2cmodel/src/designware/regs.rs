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

//! Register offsets and bits, as the DesignWare databook and Intel's LPSS
//! wrapper lay them out. Kept apart from the driver's own constants on
//! purpose: a model that borrowed the driver's table could not catch the
//! driver's table being wrong.

pub const IC_CON: u64 = 0x00;
pub const IC_TAR: u64 = 0x04;
pub const IC_DATA_CMD: u64 = 0x10;
pub const IC_SS_SCL_HCNT: u64 = 0x14;
pub const IC_SS_SCL_LCNT: u64 = 0x18;
pub const IC_FS_SCL_HCNT: u64 = 0x1C;
pub const IC_FS_SCL_LCNT: u64 = 0x20;
pub const IC_INTR_MASK: u64 = 0x30;
pub const IC_RAW_INTR_STAT: u64 = 0x34;
pub const IC_RX_TL: u64 = 0x38;
pub const IC_TX_TL: u64 = 0x3C;
pub const IC_CLR_INTR: u64 = 0x40;
pub const IC_CLR_TX_ABRT: u64 = 0x54;
pub const IC_ENABLE: u64 = 0x6C;
pub const IC_STATUS: u64 = 0x70;
pub const IC_TXFLR: u64 = 0x74;
pub const IC_RXFLR: u64 = 0x78;
pub const IC_SDA_HOLD: u64 = 0x7C;
pub const IC_TX_ABRT_SOURCE: u64 = 0x80;
pub const IC_ENABLE_STATUS: u64 = 0x9C;
pub const IC_FS_SPKLEN: u64 = 0xA0;
pub const IC_COMP_PARAM_1: u64 = 0xF4;
pub const IC_COMP_VERSION: u64 = 0xF8;
pub const IC_COMP_TYPE: u64 = 0xFC;
/// The core occupies the first page of the BAR; the LPSS private block
/// follows at 0x200, and its reset register gates the core.
pub const CORE_END: u64 = 0x100;
pub const LPSS_PRIV_RESETS: u64 = 0x204;
pub const LPSS_RESETS_RELEASED: u32 = 0x7;

pub const COMP_TYPE_VALUE: u32 = 0x4457_0140;
pub const COMP_VERSION_VALUE: u32 = 0x3230_312A;

pub const CON_RESTART_EN: u32 = 1 << 5;
pub const CMD_READ: u32 = 1 << 8;
pub const CMD_STOP: u32 = 1 << 9;
pub const CMD_RESTART: u32 = 1 << 10;

pub const STATUS_ACTIVITY: u32 = 1 << 0;
pub const STATUS_TFNF: u32 = 1 << 1;
pub const STATUS_TFE: u32 = 1 << 2;
pub const STATUS_RFNE: u32 = 1 << 3;
pub const STATUS_RFF: u32 = 1 << 4;
pub const STATUS_MST_ACTIVITY: u32 = 1 << 5;

pub const INTR_RX_OVER: u32 = 1 << 1;
pub const INTR_TX_OVER: u32 = 1 << 3;
pub const INTR_TX_ABRT: u32 = 1 << 6;

pub const ABRT_7B_ADDR_NOACK: u32 = 1 << 0;
pub const ABRT_TXDATA_NOACK: u32 = 1 << 3;
