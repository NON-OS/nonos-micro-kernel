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

//! Serialize a [`PrphScratch`] into its packed 1724-byte form.

use super::layout::*;
use super::le::{w16, w32, w64, w64_array};
use super::scratch::PrphScratch;

impl PrphScratch<'_> {
    /// Write the structure into the first [`PRPH_SCRATCH_SIZE`] bytes of `buf`,
    /// zeroing that region first. Returns false if `buf` is too small or a DRAM
    /// image carries more than [`MAX_DRAM_ENTRY`] chunks, which would silently
    /// drop firmware rather than fault.
    pub fn write(&self, buf: &mut [u8]) -> bool {
        if buf.len() < PRPH_SCRATCH_SIZE
            || self.dram.umac.len() > MAX_DRAM_ENTRY
            || self.dram.lmac.len() > MAX_DRAM_ENTRY
            || self.dram.virt.len() > MAX_DRAM_ENTRY
        {
            return false;
        }
        buf[..PRPH_SCRATCH_SIZE].fill(0);
        w16(buf, OFF_VERSION_MAC_ID, self.mac_id);
        w16(buf, OFF_VERSION_VERSION, self.version);
        w16(buf, OFF_VERSION_SIZE, CTRL_CFG_DWORDS);
        w32(buf, OFF_CONTROL_FLAGS, self.control_flags);
        w32(buf, OFF_CONTROL_FLAGS_EXT, self.control_flags_ext);
        w64(buf, OFF_PNVM_BASE, self.pnvm_base);
        w32(buf, OFF_PNVM_SIZE, self.pnvm_size);
        w64(buf, OFF_RBD_FREE, self.free_rbd_addr);
        w64(buf, OFF_REDUCE_POWER_BASE, self.reduce_power_base);
        w32(buf, OFF_REDUCE_POWER_SIZE, self.reduce_power_size);
        w64_array(buf, OFF_UMAC_IMG, self.dram.umac);
        w64_array(buf, OFF_LMAC_IMG, self.dram.lmac);
        w64_array(buf, OFF_VIRTUAL_IMG, self.dram.virt);
        true
    }
}
