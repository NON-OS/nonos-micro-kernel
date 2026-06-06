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

use core::sync::atomic::{AtomicBool, Ordering};

use crate::arch::x86_64::asm::syscall_entry_asm;
use crate::arch::x86_64::gdt::constants::{
    SEL_KERNEL_CODE_RAW, SEL_KERNEL_DATA_RAW, SEL_USER_CODE, SEL_USER_DATA, SEL_USER_DATA_RAW,
};
use crate::arch::x86_64::syscall::msr;

static INITIALIZED: AtomicBool = AtomicBool::new(false);

// Programs LSTAR/STAR/SFMASK and enables EFER.SCE so the `syscall`
// instruction at CPL=3 enters `syscall_entry_asm` at CPL=0.
//
// STAR is encoded so SYSRET delivers CS = SEL_USER_CODE (0x23) and
// SS = SEL_USER_DATA (0x1B) — see `msr::setup_star` for the SDM
// derivation. SYSCALL delivers CS = SEL_KERNEL_CODE (0x08) and
// SS = SEL_KERNEL_DATA (0x10).
pub fn init() -> Result<(), &'static str> {
    if INITIALIZED.swap(true, Ordering::SeqCst) {
        return Err("syscall already initialized");
    }
    msr::setup_star(SEL_KERNEL_CODE_RAW, SEL_USER_DATA_RAW)?;
    msr::setup_lstar(syscall_entry_asm as *const () as u64);
    msr::setup_fmask();
    msr::enable_sce();

    let star = msr::read_msr(msr::IA32_STAR);
    let syscall_cs = ((star >> 32) & 0xFFFF) as u16;
    let syscall_ss = syscall_cs.wrapping_add(8);
    let sysret_base = ((star >> 48) & 0xFFFF) as u16;
    let sysret_ss = sysret_base.wrapping_add(8) | 3;
    let sysret_cs = sysret_base.wrapping_add(16) | 3;
    if syscall_cs != SEL_KERNEL_CODE_RAW
        || syscall_ss != SEL_KERNEL_DATA_RAW
        || sysret_cs != SEL_USER_CODE
        || sysret_ss != SEL_USER_DATA
    {
        return Err("STAR encoding produces wrong SYSCALL/SYSRET selectors");
    }
    Ok(())
}
