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

//! Control-flag bits for `iwl_prph_scratch.control`. Values from the Linux
//! `iwl_prph_scratch_flags` and `iwl_prph_scratch_ext_flags` enums.

pub const CTRL_IMR_DEBUG_EN: u32 = 1 << 1;
pub const CTRL_EARLY_DEBUG_EN: u32 = 1 << 4;
pub const CTRL_EDBG_DEST_DRAM: u32 = 1 << 8;
pub const CTRL_RB_SIZE_4K: u32 = 1 << 16;
pub const CTRL_MTR_MODE: u32 = 1 << 17;
pub const CTRL_MTR_FORMAT: u32 = (1 << 18) | (1 << 19);
pub const CTRL_RB_SIZE_EXT_MASK: u32 = 0xf << 20;
pub const CTRL_RB_SIZE_EXT_8K: u32 = 8 << 20;
pub const CTRL_RB_SIZE_EXT_12K: u32 = 9 << 20;
pub const CTRL_RB_SIZE_EXT_16K: u32 = 10 << 20;
pub const CTRL_SCU_FORCE_ACTIVE: u32 = 1 << 29;

pub const CTRL_EXT_URM_FW: u32 = 1 << 4;
pub const CTRL_EXT_URM_PERM: u32 = 1 << 5;
pub const CTRL_EXT_32KHZ_CLK_VALID: u32 = 1 << 8;
