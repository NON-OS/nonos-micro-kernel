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

#[cfg(target_arch = "x86_64")]
use super::cpuid::{check_tme, cpuid_addr_sizes, cpuid_mem_encrypt};
use super::types::MemoryProtection;

pub fn detect_memory_protection() -> MemoryProtection {
    let mut prot = MemoryProtection::default();
    #[cfg(target_arch = "x86_64")]
    unsafe {
        detect_x86(&mut prot);
    }
    prot
}

#[cfg(target_arch = "x86_64")]
unsafe fn detect_x86(p: &mut MemoryProtection) {
    let (eax, _) = cpuid_mem_encrypt();
    let addr = cpuid_addr_sizes();
    p.sme_available = (eax & 1) != 0;
    p.sev_available = (eax & 2) != 0;
    p.tme_available = check_tme();
    p.dep_enabled = true;
    p.aslr_supported = true;
    p.physical_bits = (addr & 0xFF) as u8;
    p.linear_bits = ((addr >> 8) & 0xFF) as u8;
}
