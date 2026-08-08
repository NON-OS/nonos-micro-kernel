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

use crate::arch::riscv64::context::save_user_frame;
use crate::arch::riscv64::cpu::csr::{clear_csr, SIP, SIP_SSIP};
use crate::arch::riscv64::interrupts::cause::InterruptCode;
use crate::arch::riscv64::interrupts::frame::TrapFrame;
use crate::arch::riscv64::plic::{claim_interrupt, complete_interrupt, dispatch_irq};
use crate::arch::riscv64::timer;
use crate::process::scheduler::preemption::set_reschedule;
use core::sync::atomic::Ordering;

use super::fatal::fatal;

pub fn dispatch(code: InterruptCode, frame: &mut TrapFrame) {
    if frame.is_from_user() {
        save_user_frame(frame);
    }

    match code {
        InterruptCode::SupervisorTimer => timer::handle_timer_interrupt(),
        InterruptCode::SupervisorExternal => handle_external(),
        InterruptCode::SupervisorSoftware => handle_software(),
        InterruptCode::MachineSoftware
        | InterruptCode::MachineTimer
        | InterruptCode::MachineExternal => fatal(),
        InterruptCode::UserSoftware | InterruptCode::UserTimer | InterruptCode::UserExternal => {
            fatal()
        }
        InterruptCode::Unknown(_) => fatal(),
    }
}

fn handle_external() {
    let irq = match claim_interrupt() {
        Ok(Some(i)) => i,
        Ok(None) => return,
        _ => return,
    };
    if dispatch_irq(irq) {
        if complete_interrupt(irq).is_err() {
            fatal();
        }
        return;
    }
    if complete_interrupt(irq).is_err() {
        fatal();
    }
    fatal()
}

fn handle_software() {
    if clear_csr(SIP, SIP_SSIP).is_err() {
        fatal();
    }
    set_reschedule();
}
