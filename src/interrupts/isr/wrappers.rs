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

//! Plain `extern "x86-interrupt"` entries. Only NMI, #DF and #MC are installed
//! from here: none is capsule-reachable, and the latter two end the machine
//! rather than return to it. The rest duplicate trampolined vectors.

use x86_64::structures::idt::{InterruptStackFrame, PageFaultErrorCode};

use crate::interrupts::handlers;

pub extern "x86-interrupt" fn isr_divide_error(frame: InterruptStackFrame) {
    handlers::divide_error(frame);
}

pub extern "x86-interrupt" fn isr_debug(frame: InterruptStackFrame) {
    handlers::debug(frame);
}

pub extern "x86-interrupt" fn isr_nmi(frame: InterruptStackFrame) {
    handlers::nmi(frame);
}

pub extern "x86-interrupt" fn isr_breakpoint(frame: InterruptStackFrame) {
    handlers::breakpoint(frame);
}

pub extern "x86-interrupt" fn isr_overflow(frame: InterruptStackFrame) {
    handlers::overflow(frame);
}

pub extern "x86-interrupt" fn isr_bound_range(frame: InterruptStackFrame) {
    handlers::bound_range_exceeded(frame);
}

pub extern "x86-interrupt" fn isr_invalid_opcode(frame: InterruptStackFrame) {
    handlers::invalid_opcode(frame);
}

pub extern "x86-interrupt" fn isr_double_fault(frame: InterruptStackFrame, code: u64) -> ! {
    handlers::double_fault(frame, code)
}

pub extern "x86-interrupt" fn isr_page_fault(frame: InterruptStackFrame, code: PageFaultErrorCode) {
    handlers::page_fault(frame, code.bits());
}

pub extern "x86-interrupt" fn isr_alignment_check(frame: InterruptStackFrame, _error_code: u64) {
    handlers::alignment_check(frame);
}

pub extern "x86-interrupt" fn isr_machine_check(frame: InterruptStackFrame) -> ! {
    handlers::machine_check(frame)
}

pub extern "x86-interrupt" fn isr_simd_fp(frame: InterruptStackFrame) {
    handlers::simd_floating_point(frame);
}

pub extern "x86-interrupt" fn irq_timer(_frame: InterruptStackFrame) {
    handlers::timer();
}
