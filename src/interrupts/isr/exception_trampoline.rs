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

//! Swapgs trampolines for the CPL=3-reachable exceptions a capsule can
//! trigger. Like #PF (see `page_fault_trampoline`), their plain
//! `extern "x86-interrupt"` wrappers ran the handler on the user GS base (0);
//! the terminate path then writes `gs:0x28` in the scheduler resume and
//! faults. Each stub `swapgs`-es when the fault came from CPL=3 so the
//! handler runs on the kernel per-CPU base.

use x86_64::structures::idt::InterruptStackFrame;

use super::tramp_err::exc_tramp_err;
use super::tramp_noerr::exc_tramp_noerr;

exc_tramp_err!(gpf_trampoline, gpf_trap, crate::interrupts::handlers::general_protection_fault);
exc_tramp_noerr!(ud_trampoline, ud_trap, crate::interrupts::handlers::invalid_opcode);
exc_tramp_noerr!(de_trampoline, de_trap, crate::interrupts::handlers::divide_error);
exc_tramp_noerr!(br_trampoline, br_trap, crate::interrupts::handlers::bound_range_exceeded);

// #AC pushes an error code (always 0) but its handler ignores it; drop it.
fn ac_handler(frame: InterruptStackFrame, _error_code: u64) {
    crate::interrupts::handlers::alignment_check(frame);
}
exc_tramp_err!(ac_trampoline, ac_trap, ac_handler);
exc_tramp_noerr!(of_trampoline, of_trap, crate::interrupts::handlers::overflow);
exc_tramp_noerr!(bp_trampoline, bp_trap, crate::interrupts::handlers::breakpoint);
exc_tramp_noerr!(db_trampoline, db_trap, crate::interrupts::handlers::debug);
exc_tramp_err!(ss_trampoline, ss_trap, crate::interrupts::handlers::stack_segment_fault);
exc_tramp_noerr!(xf_trampoline, xf_trap, crate::interrupts::handlers::simd_floating_point);
