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

//! `cpu_switch` resume target for a never-run task. Its `kernel_rsp` carries
//! this as the `ret` address of a fake switch frame (seeded in setup). The
//! dispatcher (`prepare_resume`) has already installed TSS/CR3/state/FPU for
//! the task, so the trampoline only consumes `pending_user_entry` and iretqs
//! to CPL=3. An already-run task needs no trampoline: it resumes inside its
//! own parked preempt/yield call.

use core::sync::atomic::Ordering;

use crate::process::core::{CURRENT_PID, PROCESS_TABLE};
use crate::process::userspace::transitions::return_to_usermode;

pub(crate) extern "C" fn first_entry_trampoline() -> ! {
    let pid = CURRENT_PID.load(Ordering::SeqCst);
    let frame = PROCESS_TABLE
        .find_by_pid(pid)
        .and_then(|pcb| pcb.pending_user_entry.lock().take());
    match frame {
        Some(f) => unsafe { return_to_usermode(&f as *const _) },
        None => crate::arch::halt_loop(),
    }
}
