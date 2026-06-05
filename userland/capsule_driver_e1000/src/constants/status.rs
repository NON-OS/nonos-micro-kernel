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

pub const CTRL_LRST: u32 = 1 << 3;
pub const CTRL_ASDE: u32 = 1 << 5;
pub const CTRL_SLU: u32 = 1 << 6;
pub const CTRL_RST: u32 = 1 << 26;

pub const STATUS_LU: u32 = 1 << 1;

pub const RCTL_EN: u32 = 1 << 1;
pub const RCTL_BAM: u32 = 1 << 15;
pub const RCTL_BSIZE_2048: u32 = 0;
pub const RCTL_SECRC: u32 = 1 << 26;

pub const TCTL_EN: u32 = 1 << 1;
pub const TCTL_PSP: u32 = 1 << 3;
pub const TCTL_CT_SHIFT: u32 = 4;
pub const TCTL_COLD_SHIFT: u32 = 12;
pub const TCTL_CT_DEFAULT: u32 = 0x10 << TCTL_CT_SHIFT;
pub const TCTL_COLD_FULL_DUPLEX: u32 = 0x40 << TCTL_COLD_SHIFT;

pub const EERD_START: u32 = 1 << 0;
pub const EERD_DONE: u32 = 1 << 4;
pub const EERD_ADDR_SHIFT: u32 = 8;
pub const EERD_DATA_SHIFT: u32 = 16;

pub const RAH_AV: u32 = 1 << 31;
