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
pub unsafe fn cpuid_mem_encrypt() -> (u32, u32) {
    let (eax, ebx): (u32, u32);
    core::arch::asm!("push rbx", "cpuid", "mov {ebx_out:e}, ebx", "pop rbx", in("eax") 0x8000001Fu32, lateout("eax") eax, ebx_out = out(reg) ebx, lateout("ecx") _, lateout("edx") _, options(preserves_flags));
    (eax, ebx)
}

#[cfg(target_arch = "x86_64")]
pub unsafe fn cpuid_addr_sizes() -> u32 {
    let eax: u32;
    core::arch::asm!("push rbx", "cpuid", "pop rbx", in("eax") 0x80000008u32, lateout("eax") eax, lateout("ecx") _, lateout("edx") _, options(preserves_flags));
    eax
}

#[cfg(target_arch = "x86_64")]
pub fn check_tme() -> bool {
    let ecx: u32;
    unsafe {
        core::arch::asm!("push rbx", "cpuid", "pop rbx", in("eax") 7u32, in("ecx") 0u32, lateout("ecx") ecx, lateout("eax") _, lateout("edx") _, options(preserves_flags));
    }
    (ecx & (1 << 13)) != 0
}
