// NØNOS Operating System
// Copyright (C) 2026 NØNOS Contributors
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

use super::constants::{HANDOFF_MAGIC, HANDOFF_VERSION};
use super::framebuffer::FramebufferInfo;
use super::memory::MemoryMap;
use super::security::{Measurements, RngSeed, ZkAttestation};
use super::system::{AcpiInfo, Modules, SmbiosInfo, Timing};
use crate::firmware::FirmwareHandoff;
use core::mem::size_of;

/// Boot handoff structure passed to kernel. Layout is ABI-stable for kernel compatibility.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct BootHandoffV1 {
    pub magic: u32,
    pub version: u16,
    pub size: u16,
    pub flags: u64,
    pub entry_point: u64,
    pub fb: FramebufferInfo,
    pub mmap: MemoryMap,
    pub acpi: AcpiInfo,
    pub smbios: SmbiosInfo,
    pub modules: Modules,
    pub timing: Timing,
    pub meas: Measurements,
    pub rng: RngSeed,
    pub zk: ZkAttestation,
    pub firmware: FirmwareHandoff,
    pub cmdline_ptr: u64,
}

impl BootHandoffV1 {
    /// Validate handoff struct integrity: magic, version, size, and memory map consistency.
    pub fn is_valid(&self) -> bool {
        if self.magic != HANDOFF_MAGIC {
            return false;
        }
        if self.version != HANDOFF_VERSION {
            return false;
        }
        if self.size as usize != size_of::<Self>() {
            return false;
        }
        if self.entry_point == 0 {
            return false;
        }
        if self.mmap.entry_count > 0 && self.mmap.ptr == 0 {
            return false;
        }
        if self.mmap.entry_count > 0 && self.mmap.entry_size == 0 {
            return false;
        }
        true
    }
}
