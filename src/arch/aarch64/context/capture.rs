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

use core::sync::atomic::Ordering;

use crate::arch::aarch64::exceptions::frame::ExceptionFrame;
use crate::process::core::{CURRENT_PID, PROCESS_TABLE};

use super::types::SavedUser;

mod gprs;

pub fn save_user_frame(frame: &ExceptionFrame) {
    let pid = CURRENT_PID.load(Ordering::Acquire);
    if pid == 0 {
        return;
    }
    let pcb = match PROCESS_TABLE.find_by_pid(pid) {
        Some(p) => p,
        None => return,
    };
    let kstack = pcb.kernel_stack_top.load(Ordering::Acquire);
    let mut saved = SavedUser::zeroed();
    gprs::copy(&mut saved, frame);
    saved.sp_el0 = frame.sp;
    saved.elr_el1 = frame.elr;
    saved.spsr_el1 = frame.spsr;
    saved.kernel_sp = kstack;
    *pcb.saved_user_context.lock() = Some(saved);
}
