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

//! Enable IA32_EFER.NXE before installing a PML4 that carries NX bits.
//!
//! With NXE clear, page-table bit 63 is reserved and the first access
//! through any NX-marked mapping (the kernel directmap, every data
//! segment) raises a reserved-bit page fault with no IDT installed,
//! which triple-faults the machine. QEMU's OVMF leaves NXE set, so the
//! gap only shows on real firmware that boots with it clear, as HP
//! consumer laptops do. The bootloader owns its CPU state: set the bit
//! explicitly instead of inheriting whatever the firmware chose.

const IA32_EFER: u32 = 0xC000_0080;
const EFER_NXE: u64 = 1 << 11;
const CPUID_EXT_FEATURES: u32 = 0x8000_0001;
const CPUID_NX_BIT: u32 = 1 << 20;

/// Set EFER.NXE when the CPU supports the NX feature. Every x86_64
/// processor does; the CPUID gate is defense against exotic
/// virtualization profiles that mask it, where the NX-carrying
/// mappings would be unusable and the caller must not install them.
/// Returns whether NXE is on when this returns.
pub fn enable_nxe() -> bool {
    let (mut eax, mut ebx, mut ecx, mut edx): (u32, u32, u32, u32);
    unsafe {
        core::arch::asm!(
            "push rbx",
            "cpuid",
            "mov {ebx_out:e}, ebx",
            "pop rbx",
            inout("eax") CPUID_EXT_FEATURES => eax,
            ebx_out = out(reg) ebx,
            out("ecx") ecx,
            out("edx") edx,
            options(nostack, preserves_flags),
        );
    }
    let _ = (eax, ebx, ecx);
    if edx & CPUID_NX_BIT == 0 {
        return false;
    }
    unsafe {
        let (mut lo, mut hi): (u32, u32);
        core::arch::asm!(
            "rdmsr",
            in("ecx") IA32_EFER,
            out("eax") lo,
            out("edx") hi,
            options(nomem, nostack, preserves_flags),
        );
        let efer = ((hi as u64) << 32) | lo as u64;
        if efer & EFER_NXE == 0 {
            let new = efer | EFER_NXE;
            lo = new as u32;
            hi = (new >> 32) as u32;
            core::arch::asm!(
                "wrmsr",
                in("ecx") IA32_EFER,
                in("eax") lo,
                in("edx") hi,
                options(nomem, nostack, preserves_flags),
            );
        }
    }
    true
}
