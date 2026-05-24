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

use crate::arch::riscv64::cpu::caps;
use crate::arch::riscv64::fpu;
use crate::arch::riscv64::interrupts::cause::ExceptionCode;
use crate::arch::riscv64::interrupts::frame::TrapFrame;
use crate::arch::trap::contract::deliver;

use super::fatal::fatal;

pub fn dispatch(code: ExceptionCode, frame: &mut TrapFrame) {
    match code {
        ExceptionCode::SupervisorEcall => fatal(),
        ExceptionCode::MachineEcall => fatal(),
        ExceptionCode::IllegalInstruction if frame.is_from_user() => illegal_from_user(frame),
        _ => deliver(frame),
    }
}

fn illegal_from_user(frame: &mut TrapFrame) {
    if caps::is_configured() && (caps::has_f() || caps::has_d()) {
        if fpu::try_enable_for_current_task(frame) {
            return;
        }
        fatal()
    } else if caps::has_v() {
        fatal()
    } else {
        fatal()
    }
}
