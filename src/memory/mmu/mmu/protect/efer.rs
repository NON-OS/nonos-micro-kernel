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

//! EFER.NXE, which decides whether bit 63 of a page table entry means
//! execute-never or is ignored. Firmware may hand off with it clear, and the
//! directmap the kernel reaches every user page through is built NX, so a
//! clear NXE turns the whole data window back into executable memory.

use super::super::super::constants::{EFER_NXE, MSR_IA32_EFER};

pub(super) fn enable_nx() -> bool {
    let efer = read();
    write(efer | EFER_NXE);
    read() & EFER_NXE != 0
}

fn read() -> u64 {
    let (eax, edx): (u32, u32);
    // SAFETY: ek@nonos.systems - IA32_EFER is architectural on every x86_64
    // part, so the read cannot raise #GP, and reading an MSR has no effect on
    // machine state.
    unsafe {
        core::arch::asm!(
            "rdmsr",
            in("ecx") MSR_IA32_EFER,
            out("eax") eax,
            out("edx") edx,
            options(nomem, nostack, preserves_flags),
        );
    }
    ((edx as u64) << 32) | (eax as u64)
}

fn write(efer: u64) {
    let eax = efer as u32;
    let edx = (efer >> 32) as u32;
    // SAFETY: ek@nonos.systems - the value is the register as just read with
    // one defined bit added, so no reserved field changes; setting NXE only
    // makes an already-set page table bit start being honoured.
    unsafe {
        core::arch::asm!(
            "wrmsr",
            in("ecx") MSR_IA32_EFER,
            in("eax") eax,
            in("edx") edx,
            options(nomem, nostack, preserves_flags),
        );
    }
}
