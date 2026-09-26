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

//! A refused syscall, parked until its supervisor answers.

use super::frame::ForeignFrame;
use super::frame_snapshot::{capture, FRAME_WORDS};
use super::registry;
use super::trap_table::park;
use super::trap_wait::wait_for_answer;

/// The kernel's answer to a syscall number it does not know, made by the
/// supervisor rather than by the kernel.
pub fn redirect(nr: u64, args: [u64; 6], frame: &[u64; FRAME_WORDS]) -> Option<u64> {
    let pid = crate::process::current_pid()?;
    let supervisor = registry::supervisor_of(pid)?;
    /*
     * The frame is reachable only while this call is on the stack, and a fork
     * asks for it long afterwards, so it is copied into the control block now.
     */
    let saved = capture(frame, super::frame_cpu::user_rsp());
    crate::process::with_process(pid, |pcb| {
        *pcb.saved_user_context.lock() = Some(saved);
    });
    if !park(ForeignFrame::new(pid, nr, args, saved.rip)) {
        return Some(super::trap_reply::ABANDONED);
    }
    crate::sched::wake_process(supervisor);
    Some(wait_for_answer(pid))
}
