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

use super::context::{log_exception, ExceptionContext};
use crate::interrupts::idt::halt_loop;
use crate::interrupts::stats;

pub fn handle(frame: InterruptStackFrame) {
    let ctx = ExceptionContext::from_frame(&frame);
    log_exception("BOUND RANGE EXCEEDED", &ctx);
    stats::increment_exceptions();

    if ctx.is_user_mode() {
        terminate_user_process(&ctx);
    } else {
        kernel_bounds_error(&ctx);
    }
}

fn terminate_user_process(_ctx: &ExceptionContext) -> ! {
    crate::log::logger::log_error!("User process exceeded array bounds");
    crate::process::exit::exit_and_yield(-11, true)
}

fn kernel_bounds_error(_ctx: &ExceptionContext) -> ! {
    crate::log::logger::log_critical("Kernel bounds check failed");
    halt_loop();
}
