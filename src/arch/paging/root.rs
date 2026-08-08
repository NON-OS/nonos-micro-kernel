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

//! The active page-table root register: `CR3` on x86_64, `TTBR0_EL1` on
//! aarch64. Both registers pack more than an address, so reads mask the
//! table's physical base out of the surrounding fields (PCID on x86_64, ASID
//! and CnP on aarch64) and writes take that base plus the address-space id the
//! architecture wants alongside it.

/// Physical base of the page table the CPU is translating through.
#[inline]
pub fn read_root() -> u64 {
    #[cfg(target_arch = "x86_64")]
    {
        let cr3: u64;
        // SAFETY: reading CR3 has no side effects and no memory operand.
        unsafe {
            core::arch::asm!("mov {}, cr3", out(reg) cr3, options(nostack, preserves_flags));
        }
        // CR3[11:0] holds PCID or the cache-control bits, never address.
        cr3 & !0xFFF
    }
    #[cfg(target_arch = "aarch64")]
    {
        // TTBR0_EL1 is ASID[63:48] : BADDR[47:1] : CnP[0].
        crate::arch::aarch64::mmu::read_ttbr0() & 0x0000_FFFF_FFFF_F000
    }
    #[cfg(not(any(target_arch = "x86_64", target_arch = "aarch64")))]
    {
        0
    }
}

/// Point the CPU at `root_pa`, tagging the translation with `asid` where the
/// architecture carries the address-space id in the same register. On x86_64
/// the id rides in CR3[11:0] as a PCID; passing 0 keeps the classic behaviour
/// of flushing the non-global TLB entries on every write.
#[inline]
pub fn write_root(root_pa: u64, asid: u16) {
    #[cfg(target_arch = "x86_64")]
    {
        let value = (root_pa & !0xFFF) | (asid as u64 & 0xFFF);
        // SAFETY: the caller owns `root_pa` as a live, correctly built table;
        // installing it is the point of the call.
        unsafe {
            core::arch::asm!("mov cr3, {}", in(reg) value, options(nostack, preserves_flags));
        }
    }
    #[cfg(target_arch = "aarch64")]
    {
        crate::arch::aarch64::mmu::set_ttbr0(root_pa & 0x0000_FFFF_FFFF_F000, asid);
    }
    #[cfg(not(any(target_arch = "x86_64", target_arch = "aarch64")))]
    {
        let _ = (root_pa, asid);
    }
}
