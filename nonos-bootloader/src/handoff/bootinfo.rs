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

use bitflags::bitflags;

/// ZeroState capsule boot info for kernel initialization.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct ZeroStateBootInfo {
    pub capsule_base: u64,
    pub capsule_size: u64,
    pub capsule_hash: [u8; 32],
    pub memory_start: u64,
    pub memory_size: u64,
    pub entropy64: [u8; 64],
    pub rtc_utc: u64,
    pub boot_flags: BootModeFlags,
}

impl Default for ZeroStateBootInfo {
    fn default() -> Self {
        Self {
            capsule_base: 0,
            capsule_size: 0,
            capsule_hash: [0u8; 32],
            memory_start: 0,
            memory_size: 0,
            entropy64: [0u8; 64],
            rtc_utc: 0,
            boot_flags: BootModeFlags::empty(),
        }
    }
}

bitflags! {
    /// Boot mode flags indicating security and boot state.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    pub struct BootModeFlags: u32 {
        const SECURE_BOOT = 1 << 0;
        const COLD_START = 1 << 1;
        const WARM_START = 1 << 2;
        const DEBUG_MODE = 1 << 3;
    }
}

/// Parameters for building ZeroStateBootInfo.
pub struct BootInfoParams {
    pub capsule_base: u64,
    pub capsule_size: u64,
    pub capsule_hash: [u8; 32],
    pub memory_start: u64,
    pub memory_size: u64,
    pub entropy64: [u8; 64],
    pub rtc_utc: u64,
    pub boot_flags: BootModeFlags,
}

/// Construct ZeroStateBootInfo from parameters.
pub fn build_bootinfo(p: BootInfoParams) -> ZeroStateBootInfo {
    ZeroStateBootInfo {
        capsule_base: p.capsule_base,
        capsule_size: p.capsule_size,
        capsule_hash: p.capsule_hash,
        memory_start: p.memory_start,
        memory_size: p.memory_size,
        entropy64: p.entropy64,
        rtc_utc: p.rtc_utc,
        boot_flags: p.boot_flags,
    }
}
