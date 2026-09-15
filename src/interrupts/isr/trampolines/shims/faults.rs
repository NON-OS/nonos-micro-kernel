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

//! Vectors that push no error code.

use super::frame;
use crate::interrupts::handlers;
use x86_64::structures::idt::InterruptStackFrame;

#[no_mangle]
extern "C" fn nonos_trap_de(f: *const InterruptStackFrame) {
    handlers::divide_error(frame(f));
}

#[no_mangle]
extern "C" fn nonos_trap_db(f: *const InterruptStackFrame) {
    handlers::debug(frame(f));
}

#[no_mangle]
extern "C" fn nonos_trap_bp(f: *const InterruptStackFrame) {
    handlers::breakpoint(frame(f));
}

#[no_mangle]
extern "C" fn nonos_trap_of(f: *const InterruptStackFrame) {
    handlers::overflow(frame(f));
}

#[no_mangle]
extern "C" fn nonos_trap_br(f: *const InterruptStackFrame) {
    handlers::bound_range_exceeded(frame(f));
}

#[no_mangle]
extern "C" fn nonos_trap_ud(f: *const InterruptStackFrame) {
    handlers::invalid_opcode(frame(f));
}

#[no_mangle]
extern "C" fn nonos_trap_nm(f: *const InterruptStackFrame) {
    handlers::device_not_available(frame(f));
}

#[no_mangle]
extern "C" fn nonos_trap_mf(f: *const InterruptStackFrame) {
    handlers::x87_floating_point(frame(f));
}

#[no_mangle]
extern "C" fn nonos_trap_xf(f: *const InterruptStackFrame) {
    handlers::simd_floating_point(frame(f));
}

#[no_mangle]
extern "C" fn nonos_trap_ve(f: *const InterruptStackFrame) {
    handlers::virtualization(frame(f));
}
