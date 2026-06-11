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

// CPUID.80000001H:EDX[26] (PDPE1GB) reports 1 GiB page support. It is
// an optional feature: present on QEMU's default CPU and essentially
// all bare metal from the last decade, but absent on some hypervisor
// CPU profiles, notably VirtualBox's default. Where it is absent,
// setting the PS bit in a PDPTE is a reserved-bit page fault, so the
// identity and directmap builders consult this to choose 1 GiB leaves
// where allowed and 2 MiB leaves where not.
pub fn supports_1gib_pages() -> bool {
    #[cfg(target_arch = "x86_64")]
    unsafe {
        let edx: u32;
        core::arch::asm!(
            "push rbx", "cpuid", "pop rbx",
            inout("eax") 0x8000_0001u32 => _,
            out("ecx") _, out("edx") edx,
            options(nostack, preserves_flags)
        );
        (edx & (1 << 26)) != 0
    }
    #[cfg(not(target_arch = "x86_64"))]
    {
        false
    }
}
