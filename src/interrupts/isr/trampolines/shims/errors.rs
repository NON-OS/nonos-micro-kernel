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

//! Vectors that push an error code, handed through in rsi.

use super::frame;
use crate::interrupts::handlers;
use x86_64::structures::idt::InterruptStackFrame;

#[no_mangle]
extern "C" fn nonos_trap_ts(f: *const InterruptStackFrame, error_code: u64) {
    handlers::invalid_tss(frame(f), error_code);
}

#[no_mangle]
extern "C" fn nonos_trap_np(f: *const InterruptStackFrame, error_code: u64) {
    handlers::segment_not_present(frame(f), error_code);
}

#[no_mangle]
extern "C" fn nonos_trap_ss(f: *const InterruptStackFrame, error_code: u64) {
    handlers::stack_segment_fault(frame(f), error_code);
}

#[no_mangle]
extern "C" fn nonos_trap_gpf(f: *const InterruptStackFrame, error_code: u64) {
    handlers::general_protection_fault(frame(f), error_code);
}

// Alignment check pushes an error code the handler has no use for; it is
// received to keep the vector ABI honest and dropped here.
#[no_mangle]
extern "C" fn nonos_trap_ac(f: *const InterruptStackFrame, _error_code: u64) {
    handlers::alignment_check(frame(f));
}

#[no_mangle]
extern "C" fn nonos_trap_pf(f: *const InterruptStackFrame, error_code: u64) {
    handlers::page_fault(frame(f), error_code);
}
