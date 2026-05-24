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

use alloc::sync::Arc;
use core::sync::atomic::Ordering;

use crate::process::core::{Pid, ProcessControlBlock, PROCESS_TABLE};
use crate::process::userspace::types::InterruptFrame;

use super::{build_initial_switch_frame, first_entry_trampoline, UserEntry};

// Top of canonical low half. iretq into CPL=3 only with entry/rsp here.
const USER_VA_MAX: u64 = 0x0000_7FFF_FFFF_FFFF;

#[derive(Debug, Clone, Copy)]
pub enum SetupError {
    NoSuchProcess,
    NonUserEntry,
    NonUserStack,
}

// x86 builder: assembles the iretq 5-tuple via InterruptFrame and
// stages it on the PCB. `UserEntry == InterruptFrame` by the alias in
// `super::aliases`, so the field write is the same shape callers used
// before the arch-neutral facade.
pub fn setup_initial_user_pcb_x86_64(
    pid: Pid,
    entry: u64,
    user_rsp: u64,
) -> Result<(), SetupError> {
    if entry == 0 || entry > USER_VA_MAX {
        return Err(SetupError::NonUserEntry);
    }
    if user_rsp == 0 || user_rsp > USER_VA_MAX {
        return Err(SetupError::NonUserStack);
    }
    let pcb = PROCESS_TABLE.find_by_pid(pid).ok_or(SetupError::NoSuchProcess)?;
    let frame: UserEntry = InterruptFrame::for_user_entry(entry, user_rsp);
    *pcb.pending_user_entry.lock() = Some(frame);
    seed_first_entry_kernel_rsp(&pcb);
    Ok(())
}

fn seed_first_entry_kernel_rsp(pcb: &Arc<ProcessControlBlock>) {
    let kstack = pcb.kernel_stack_top.load(Ordering::Acquire);
    if kstack == 0 {
        return;
    }
    let rsp = build_initial_switch_frame(kstack, first_entry_trampoline as u64);
    pcb.kernel_rsp.store(rsp, Ordering::Release);
    #[cfg(feature = "nonos-cpuswitch-selftest")]
    {
        crate::sys::serial::print(b"[CPUSWITCH] first-entry kernel_rsp set pid=");
        crate::arch::x86_64::diag::print_hex_u64(pcb.pid as u64);
        crate::sys::serial::println(b"");
    }
}
