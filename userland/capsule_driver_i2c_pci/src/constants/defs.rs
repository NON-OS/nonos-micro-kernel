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
pub const INTEL_VENDOR_ID: u16 = 0x8086;
pub const BAR_INDEX: u32 = 0;
pub const BAR_OFFSET: u64 = 0;
pub const PAGE_MASK: u64 = 0xFFF;

pub const IC_CON: u64 = 0x00;
pub const IC_TAR: u64 = 0x04;
pub const IC_DATA_CMD: u64 = 0x10;
pub const IC_SS_SCL_HCNT: u64 = 0x14;
pub const IC_SS_SCL_LCNT: u64 = 0x18;
pub const IC_FS_SCL_HCNT: u64 = 0x1C;
pub const IC_FS_SCL_LCNT: u64 = 0x20;
pub const IC_FS_SPKLEN: u64 = 0xA0;
pub const IC_INTR_MASK: u64 = 0x30;
pub const IC_RAW_INTR_STAT: u64 = 0x34;
pub const IC_RX_TL: u64 = 0x38;
pub const IC_TX_TL: u64 = 0x3C;
pub const IC_CLR_INTR: u64 = 0x40;
pub const IC_ENABLE: u64 = 0x6C;
pub const IC_STATUS: u64 = 0x70;
pub const IC_TXFLR: u64 = 0x74;
pub const IC_RXFLR: u64 = 0x78;
pub const IC_SDA_HOLD: u64 = 0x7C;
pub const IC_TX_ABRT_SOURCE: u64 = 0x80;
pub const IC_ENABLE_STATUS: u64 = 0x9C;
pub const IC_COMP_PARAM_1: u64 = 0xF4;
pub const IC_COMP_TYPE: u64 = 0xFC;
