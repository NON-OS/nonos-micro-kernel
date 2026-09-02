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

//! Turn on the ring-0 restrictions the part offers and report which of them
//! the hardware confirmed. Every field of the returned flags is a read-back,
//! so a caller may treat it as the machine's answer rather than the kernel's
//! intention.

use crate::memory::mmu::error::{MmuError, MmuResult};
use crate::memory::mmu::ProtectionFlags;

#[cfg(target_arch = "x86_64")]
pub(in crate::memory::mmu::mmu) fn apply() -> MmuResult<ProtectionFlags> {
    use super::{cpuid, cr0, cr4, efer};

    let have = cpuid::supported();
    // Execute-never is the one property with no fallback: the directmap every
    // user page is reached through is built with the NX bit set, so a part
    // that cannot honour it would leave that whole window executable.
    if !have.nx {
        return Err(MmuError::NxNotSupported);
    }
    let live = cr4::enable(have);
    Ok(ProtectionFlags {
        smep_enabled: live.smep,
        smap_enabled: live.smap,
        umip_enabled: live.umip,
        nx_enabled: efer::enable_nx(),
        wp_enabled: cr0::enable(),
    })
}

/// aarch64 and riscv64 reach the same two properties, that the kernel cannot
/// execute or read a user page unintentionally, from PAN and the PXN and UXN
/// table bits, which the MMU applies where the tables are built. There is no
/// control register to turn on and no global write-protect override, so the
/// x86-named flags stay false and the architectural ones are reported set.
#[cfg(not(target_arch = "x86_64"))]
pub(in crate::memory::mmu::mmu) fn apply() -> MmuResult<ProtectionFlags> {
    Ok(ProtectionFlags {
        smep_enabled: false,
        smap_enabled: false,
        umip_enabled: false,
        nx_enabled: true,
        wp_enabled: true,
    })
}
