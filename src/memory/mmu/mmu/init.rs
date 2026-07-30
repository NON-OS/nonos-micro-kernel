// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

use super::super::constants::*;
use super::super::error::{MmuError, MmuResult};
use super::core::MMU;
use core::arch::asm;

impl MMU {
    pub fn initialize(&self) -> MmuResult<()> {
        let mut init_guard = self.initialized.lock();
        if *init_guard {
            return Ok(());
        }
        self.enable_smep_smap()?;
        self.enable_nx_bit()?;
        let cr3_guard = self.current_cr3.lock();
        if *cr3_guard == 0 {
            drop(cr3_guard);
            self.setup_initial_page_tables()?;
        }
        *init_guard = true;
        Ok(())
    }

/// Turn on the ring-0 access restrictions this part offers.
///
/// SMEP, SMAP and UMIP are CR4 bits found by CPUID, so this is an x86_64
/// mechanism end to end. aarch64 gets the same two properties, that the
/// kernel cannot execute or read a user page unintentionally, from PAN and
/// the PXN and UXN table bits, which the MMU sets when it builds the
/// tables. The flags below keep their x86 names and stay false there,
/// because it is those specific mechanisms they describe.
pub(super) fn enable_smep_smap(&self) -> MmuResult<()> {
    #[cfg(target_arch = "x86_64")]
    {
        let (_, ebx, ecx, _) = Self::cpuid(CPUID_FEATURES_LEAF, 0);
        let has_smep = (ebx & CPUID_EBX_SMEP) != 0;
        let has_smap = (ebx & CPUID_EBX_SMAP) != 0;
        let has_umip = (ecx & CPUID_ECX_UMIP) != 0;
        // SAFETY: reading CR4 has no side effect, and each bit set only
        // narrows what ring 0 is permitted to do.
        unsafe {
            let mut cr4: u64;
            asm!("mov {}, cr4", out(reg) cr4, options(nostack, preserves_flags));
            if has_smep {
                cr4 |= CR4_SMEP;
            }
            if has_smap {
                cr4 |= CR4_SMAP;
            }
            if has_umip {
                cr4 |= CR4_UMIP;
            }
            asm!("mov cr4, {}", in(reg) cr4, options(nostack, preserves_flags));
        }
        let mut flags = self.protection_flags.lock();
        flags.smep_enabled = has_smep;
        flags.smap_enabled = has_smap;
        flags.umip_enabled = has_umip;
    }
    Ok(())
}

/// Make sure execute-never is honoured on user pages.
///
/// On x86_64 that is EFER.NXE, an MSR the firmware may leave clear, so it
/// is read back and set. On aarch64 execute-never is not optional: PXN and
/// UXN are architectural page-table bits that the MMU always applies, so
/// the property holds with nothing to switch on.
pub(super) fn enable_nx_bit(&self) -> MmuResult<()> {
    #[cfg(target_arch = "x86_64")]
    {
        let (_, _, _, edx) = Self::cpuid(CPUID_EXTENDED_LEAF, 0);
        if (edx & CPUID_EDX_NX) == 0 {
            return Err(MmuError::NxNotSupported);
        }
        // SAFETY: EFER is a defined architectural MSR; only the NXE bit is
        // changed and the rest is written back as read.
        unsafe {
            let mut eax: u32;
            let mut edx: u32;
            asm!("rdmsr", in("ecx") MSR_IA32_EFER, out("eax") eax, out("edx") edx, options(nostack, preserves_flags));
            let mut efer = ((edx as u64) << 32) | (eax as u64);
            efer |= EFER_NXE;
            let eax2 = (efer & 0xFFFF_FFFF) as u32;
            let edx2 = (efer >> 32) as u32;
            asm!("wrmsr", in("ecx") MSR_IA32_EFER, in("eax") eax2, in("edx") edx2, options(nostack, preserves_flags));
        }
    }
    self.protection_flags.lock().nx_enabled = true;
    Ok(())
}

    pub(super) fn cpuid(leaf: u32, subleaf: u32) -> (u32, u32, u32, u32) {
        let result = core::arch::x86_64::__cpuid_count(leaf, subleaf);
        (result.eax, result.ebx, result.ecx, result.edx)
    }
}
