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
    log_exception("ALIGNMENT CHECK", &ctx);
    stats::increment_exceptions();

    if ctx.is_user_mode() {
        handle_user_alignment(&ctx);
    } else {
        handle_kernel_alignment(&ctx);
    }
}

fn handle_user_alignment(_ctx: &ExceptionContext) -> ! {
    crate::log::logger::log_warning!(
        "User process alignment check at {:#x}",
        _ctx.instruction_pointer
    );
    crate::process::exit::exit_and_yield(-7, true)
}

fn handle_kernel_alignment(_ctx: &ExceptionContext) -> ! {
    crate::log::logger::log_error!("Kernel alignment check at {:#x}", _ctx.instruction_pointer);
    halt_loop();
}
