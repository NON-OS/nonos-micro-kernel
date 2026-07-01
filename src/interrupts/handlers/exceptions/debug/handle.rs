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

use x86_64::structures::idt::InterruptStackFrame;

use super::clear::clear_debug_status;
use super::finish::finish_debug_exception;
use super::handle_hardware_breakpoint::handle_hardware_breakpoint;
use super::handle_single_step::handle_single_step;
use super::handle_task_switch::handle_task_switch;
use super::read::read_debug_info;
use crate::interrupts::handlers::exceptions::context::ExceptionContext;
use crate::interrupts::stats;

pub fn handle(frame: InterruptStackFrame) {
    let ctx = ExceptionContext::from_frame(&frame);
    let info = read_debug_info();
    stats::increment_exceptions();

    if info.single_step {
        handle_single_step(&ctx);
    } else if info.breakpoint_0 || info.breakpoint_1 || info.breakpoint_2 || info.breakpoint_3 {
        handle_hardware_breakpoint(&ctx, &info);
    } else if info.task_switch {
        handle_task_switch(&ctx);
    }

    clear_debug_status();
    finish_debug_exception(&ctx);
}
