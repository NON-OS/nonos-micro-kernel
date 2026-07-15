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

pub const HBA_CAP: u32 = 0x00;
pub const HBA_GHC: u32 = 0x04;
pub const HBA_IS: u32 = 0x08;
pub const HBA_PI: u32 = 0x0c;
pub const HBA_VS: u32 = 0x10;
pub const HBA_CAP2: u32 = 0x24;

pub const GHC_AE: u32 = 1 << 31;
pub const GHC_HR: u32 = 1 << 0;

pub const PORT_BASE: u32 = 0x100;
pub const PORT_STRIDE: u32 = 0x80;
pub const PORT_CLB: u32 = 0x00;
pub const PORT_CLBU: u32 = 0x04;
pub const PORT_FB: u32 = 0x08;
pub const PORT_FBU: u32 = 0x0c;
pub const PORT_IS: u32 = 0x10;
pub const PORT_IE: u32 = 0x14;
pub const PORT_CMD: u32 = 0x18;
pub const PORT_TFD: u32 = 0x20;
pub const PORT_SIG: u32 = 0x24;
pub const PORT_SSTS: u32 = 0x28;
pub const PORT_SCTL: u32 = 0x2c;
pub const PORT_SERR: u32 = 0x30;
pub const PORT_SACT: u32 = 0x34;
pub const PORT_CI: u32 = 0x38;

pub const CMD_ST: u32 = 1 << 0;
pub const CMD_FRE: u32 = 1 << 4;
pub const CMD_FR: u32 = 1 << 14;
pub const CMD_CR: u32 = 1 << 15;
pub const CMD_POD: u32 = 1 << 2;
pub const CMD_SUD: u32 = 1 << 1;

pub const TFD_ERR: u32 = 1 << 0;
pub const TFD_DRQ: u32 = 1 << 3;
pub const TFD_BSY: u32 = 1 << 7;
pub const IS_ERR_MASK: u32 = 1 << 30 | 1 << 29 | 1 << 28 | 1 << 27;
pub const SCTL_IPM_DISABLE: u32 = 7 << 8;
pub const PORT_IE_DEFAULT: u32 = 0b1_0111;

pub const SIG_SATA: u32 = 0x0000_0101;
pub const SIG_ATAPI: u32 = 0xeb14_0101;
pub const SIG_SEMB: u32 = 0xc33c_0101;
pub const SIG_PM: u32 = 0x9669_0101;

// SATA status/control DET field (PxSSTS / PxSCTL bits [3:0]).
pub const SSTS_DET_MASK: u32 = 0xf;
pub const SSTS_DET_PRESENT: u32 = 0x3; // device present, PHY communication up
pub const SCTL_DET_MASK: u32 = 0xf;
pub const SCTL_DET_COMRESET: u32 = 0x1;
