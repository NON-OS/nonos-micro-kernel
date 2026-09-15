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

//! Packed field offsets and sizes for `iwl_prph_scratch`. ctrl_cfg begins at
//! zero, so these are also the offsets inside `iwl_prph_scratch_ctrl_cfg`. The
//! DRAM map holds three image arrays then an fseq array, each `__le64`.

/// Entries per firmware image DRAM array (`IWL_MAX_DRAM_ENTRY`).
pub const MAX_DRAM_ENTRY: usize = 64;
/// Entries in the FSEQ image DRAM array (`IWL_NUM_DRAM_FSEQ_ENTRIES`).
pub const FSEQ_ENTRIES: usize = 8;
/// Total size of the packed `iwl_prph_scratch` structure, in bytes.
pub const PRPH_SCRATCH_SIZE: usize = 1724;

pub(super) const OFF_VERSION_MAC_ID: usize = 0;
pub(super) const OFF_VERSION_VERSION: usize = 2;
pub(super) const OFF_VERSION_SIZE: usize = 4;
pub(super) const OFF_CONTROL_FLAGS: usize = 8;
pub(super) const OFF_CONTROL_FLAGS_EXT: usize = 12;
pub(super) const OFF_PNVM_BASE: usize = 16;
pub(super) const OFF_PNVM_SIZE: usize = 24;
pub(super) const OFF_RBD_FREE: usize = 48;
pub(super) const OFF_REDUCE_POWER_BASE: usize = 60;
pub(super) const OFF_REDUCE_POWER_SIZE: usize = 68;
pub(super) const OFF_UMAC_IMG: usize = 124;
pub(super) const OFF_LMAC_IMG: usize = OFF_UMAC_IMG + MAX_DRAM_ENTRY * 8;
pub(super) const OFF_VIRTUAL_IMG: usize = OFF_LMAC_IMG + MAX_DRAM_ENTRY * 8;
/// ctrl_cfg size in dwords, written into the version.size field.
pub(super) const CTRL_CFG_DWORDS: u16 = 84 / 4;
