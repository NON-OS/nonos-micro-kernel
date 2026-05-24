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

pub const PTE_VALID: u64 = 1 << 0;
pub const PTE_TABLE: u64 = 1 << 1;
pub const PTE_PAGE: u64 = 1 << 1;
pub const PTE_BLOCK: u64 = 0;
pub const PTE_ATTR_INDX_MASK: u64 = 0x7 << 2;
pub const PTE_NS: u64 = 1 << 5;
pub const PTE_AP_RW_EL1: u64 = 0b00 << 6;
pub const PTE_AP_RW_ALL: u64 = 0b01 << 6;
pub const PTE_AP_RO_EL1: u64 = 0b10 << 6;
pub const PTE_AP_RO_ALL: u64 = 0b11 << 6;
pub const PTE_SH_MASK: u64 = 0x3 << 8;
pub const PTE_SH_NS: u64 = 0b00 << 8;
pub const PTE_SH_OS: u64 = 0b10 << 8;
pub const PTE_SH_IS: u64 = 0b11 << 8;
pub const PTE_AF: u64 = 1 << 10;
pub const PTE_NG: u64 = 1 << 11;
pub const PTE_CONT: u64 = 1 << 52;
pub const PTE_PXN: u64 = 1 << 53;
pub const PTE_UXN: u64 = 1 << 54;
pub const PTE_ADDR_MASK: u64 = 0x0000_FFFF_FFFF_F000;
