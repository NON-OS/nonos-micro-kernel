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

use crate::arch::riscv64::interrupts::cause::{ExceptionCode, TrapCause};
use crate::arch::riscv64::interrupts::frame::TrapFrame;

use super::{exception, interrupt, syscall};

#[no_mangle]
pub extern "C" fn riscv64_trap_dispatch(frame: *mut TrapFrame) {
    let frame = unsafe { &mut *frame };
    match TrapCause::from_scause(frame.scause) {
        TrapCause::Interrupt(code) => interrupt::dispatch(code, frame),
        TrapCause::Exception(ExceptionCode::UserEcall) => syscall::dispatch_ecall(frame),
        TrapCause::Exception(code) => exception::dispatch(code, frame),
    }
}
