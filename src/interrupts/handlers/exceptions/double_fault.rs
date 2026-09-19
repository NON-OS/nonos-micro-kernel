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
use crate::arch::x86_64::diag::emit_fatal_notice_nolock;
use crate::interrupts::idt::halt_loop;
use crate::interrupts::stats;
use crate::security::observability::redact::redact_address;

pub fn handle(frame: InterruptStackFrame, error_code: u64) -> ! {
    let ctx = ExceptionContext::from_frame(&frame);
    emit_fatal_notice_nolock(b"DF", ctx.instruction_pointer);
    log_exception("DOUBLE FAULT", &ctx);
    stats::increment_exceptions();

    crate::log::logger::log_critical(&alloc::format!("Double fault error code: {:#x}", error_code));

    dump_stack_info(&ctx);

    crate::log::logger::log_critical("SYSTEM HALTED: Double fault is unrecoverable");
    halt_loop();
}

fn dump_stack_info(ctx: &ExceptionContext) {
    crate::log::logger::log_critical(&alloc::format!(
        "Stack pointer: {}",
        redact_address(ctx.stack_pointer)
    ));
    crate::log::logger::log_critical(&alloc::format!(
        "Instruction pointer: {}",
        redact_address(ctx.instruction_pointer)
    ));
    crate::log::logger::log_critical(&alloc::format!("Code segment: {:#x}", ctx.code_segment));
}
