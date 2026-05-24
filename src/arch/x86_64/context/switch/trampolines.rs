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

//! `cpu_switch` resume targets for the context-switch rewrite. A parked
//! `kernel_rsp` carries one of these as its `ret` address: when `cpu_switch`
//! lands here the running task is identified by `CURRENT_PID` (set by the
//! dispatcher before the switch), and the trampoline drives the existing
//! first-entry / preempt-resume tail (TSS/CR3/state/FPU + iretq). Not yet
//! routed (phase 3): baked into `kernel_rsp` but reached only after the
//! phase-4 cutover.

use core::sync::atomic::Ordering;

use crate::process::core::{CURRENT_PID, PROCESS_TABLE};

/// `ret` target for a never-run task: consumes `pending_user_entry` and iretqs
/// to CPL=3. Only returns here if the task had no pending entry (then parks).
#[allow(dead_code)]
pub(crate) extern "C" fn first_entry_trampoline() -> ! {
    let pid = CURRENT_PID.load(Ordering::SeqCst);
    if let Some(pcb) = PROCESS_TABLE.find_by_pid(pid) {
        super::first_entry::try_first_entry(&pcb, pid);
    }
    crate::arch::halt_loop()
}

/// `ret` target for a preempted CPL=3 task: restores the captured
/// `saved_user_context` and iretqs. Parks only if the snapshot was missing.
#[allow(dead_code)]
pub(crate) extern "C" fn resume_user_trampoline() -> ! {
    let pid = CURRENT_PID.load(Ordering::SeqCst);
    if let Some(pcb) = PROCESS_TABLE.find_by_pid(pid) {
        super::resume::try_resume(&pcb, pid);
    }
    crate::arch::halt_loop()
}
