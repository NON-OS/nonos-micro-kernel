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

use super::context::{log_exception_with_code, ExceptionContext};
use crate::interrupts::idt::halt_loop;
use crate::interrupts::stats;

pub fn handle(frame: InterruptStackFrame, error_code: u64) {
    let ctx = ExceptionContext::from_frame(&frame);
    log_exception_with_code("STACK SEGMENT FAULT", &ctx, error_code);
    stats::increment_exceptions();

    analyze_stack_fault(&ctx, error_code);

    if ctx.is_user_mode() {
        terminate_user_process(&ctx);
    } else {
        kernel_panic(&ctx);
    }
}

fn analyze_stack_fault(ctx: &ExceptionContext, error_code: u64) {
    if error_code == 0 {
        crate::log::logger::log_error!("Stack limit exceeded or not present");
    } else {
        crate::log::logger::log_error!(
            "Stack segment fault: selector_index={}",
            (error_code >> 3) & 0x1FFF
        );
    }

    crate::log::logger::log_error!("Stack pointer at fault: {:#x}", ctx.stack_pointer);
}

fn terminate_user_process(_ctx: &ExceptionContext) -> ! {
    crate::log::logger::log_error!("Terminating user process: stack overflow");
    crate::process::exit::exit_and_yield(-11, true)
}

fn kernel_panic(_ctx: &ExceptionContext) {
    crate::log::logger::log_critical("KERNEL PANIC: Stack segment fault in kernel mode");
    halt_loop();
}
