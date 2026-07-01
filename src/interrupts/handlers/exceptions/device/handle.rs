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

use x86_64::registers::control::{Cr0, Cr0Flags};
use x86_64::structures::idt::InterruptStackFrame;

use super::handle_fpu_emulation::handle_fpu_emulation;
use super::handle_missing_fpu::handle_missing_fpu;
use super::handle_task_switch_fpu::handle_task_switch_fpu;
use crate::interrupts::handlers::exceptions::context::{log_exception, ExceptionContext};
use crate::interrupts::stats;

pub fn handle(frame: InterruptStackFrame) {
    let ctx = ExceptionContext::from_frame(&frame);
    log_exception("DEVICE NOT AVAILABLE", &ctx);
    stats::increment_exceptions();

    let cr0 = Cr0::read();

    if cr0.contains(Cr0Flags::TASK_SWITCHED) {
        handle_task_switch_fpu();
    } else if cr0.contains(Cr0Flags::EMULATE_COPROCESSOR) {
        handle_fpu_emulation(&ctx);
    } else {
        handle_missing_fpu(&ctx);
    }
}
