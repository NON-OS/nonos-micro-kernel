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

//! The peripheral-scratch fields the boot ROM reads. All addresses are host-
//! physical (the value a DMA grant reports as `device_addr`).

/// A firmware DRAM image as its chunk physical addresses, one chunk up to 32 KiB.
#[derive(Clone, Copy, Default)]
pub struct DramImage<'a> {
    /// UMAC image chunk addresses, in load order.
    pub umac: &'a [u64],
    /// LMAC image chunk addresses, in load order.
    pub lmac: &'a [u64],
    /// Paged ("virtual") image chunk addresses, in load order.
    pub virt: &'a [u64],
}

/// The addresses and flags the boot ROM reads from peripheral scratch.
pub struct PrphScratch<'a> {
    /// MAC hardware id echoed to firmware (version.mac_id).
    pub mac_id: u16,
    /// Context-info/HW version echoed to firmware (version.version).
    pub version: u16,
    /// FH configuration flags (see [`super::flags`]).
    pub control_flags: u32,
    /// Extended configuration flags (see [`super::flags`]).
    pub control_flags_ext: u32,
    /// Platform-NVM table physical address, or zero until PNVM is loaded.
    pub pnvm_base: u64,
    /// Platform-NVM table size in bytes.
    pub pnvm_size: u32,
    /// Reduce-power table physical address, or zero if none.
    pub reduce_power_base: u64,
    /// Reduce-power table size in bytes.
    pub reduce_power_size: u32,
    /// Free receive-buffer-descriptor ring physical address.
    pub free_rbd_addr: u64,
    /// Firmware image DRAM map.
    pub dram: DramImage<'a>,
}
