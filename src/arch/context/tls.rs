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

// Arch-neutral write of the calling task's user TLS register. On x86_64
// that is MSR_FS_BASE, which the syscall return path leaves untouched, so
// a direct write here survives into user mode; the scheduler re-installs
// the PCB's value on every later switch. aarch64 and riscv64 return their
// trap frames wholesale (tpidr_el0 / tp live in the saved frame), so a
// bare register write here would be clobbered on return; those ports
// report failure until their frames carry the TLS slot.

pub fn set_user_tls(base: u64) -> bool {
    #[cfg(target_arch = "x86_64")]
    {
        // SAFETY: MSR_FS_BASE only redirects user-mode fs-segment
        // addressing; the kernel itself never addresses through fs. The
        // base was validated as a readable user VA at the syscall boundary.
        unsafe { crate::arch::x86_64::gdt::fs_gs::set_fs_base(base) };
        true
    }
    #[cfg(not(target_arch = "x86_64"))]
    {
        let _ = base;
        false
    }
}
