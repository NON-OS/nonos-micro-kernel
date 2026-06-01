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
    crate::arch::x86_64::diag::dump_trap(b"UD", &frame, None, None);
    let ctx = ExceptionContext::from_frame(&frame);
    log_exception("INVALID OPCODE", &ctx);
    stats::increment_exceptions();

    log_instruction_bytes(&ctx);

    if ctx.is_user_mode() {
        terminate_user_process(&ctx);
    } else {
        kernel_panic(&ctx);
    }
}

fn log_instruction_bytes(ctx: &ExceptionContext) {
    // SAFETY: Reading instruction bytes from code segment for diagnostic purposes
    let ptr = ctx.instruction_pointer as *const u8;
    let bytes: [u8; 16] = unsafe {
        let mut buf = [0u8; 16];
        for i in 0..16 {
            buf[i] = *ptr.add(i);
        }
        buf
    };

    crate::log::logger::log_error!(
        "Instruction bytes: {:02x} {:02x} {:02x} {:02x} {:02x} {:02x} {:02x} {:02x}",
        bytes[0],
        bytes[1],
        bytes[2],
        bytes[3],
        bytes[4],
        bytes[5],
        bytes[6],
        bytes[7]
    );
}

fn terminate_user_process(_ctx: &ExceptionContext) -> ! {
    crate::log::logger::log_error!("Terminating user process: invalid instruction");
    crate::process::exit::exit_and_yield(-4, true)
}

fn kernel_panic(_ctx: &ExceptionContext) {
    crate::log::logger::log_critical("KERNEL PANIC: Invalid opcode in kernel mode");
    halt_loop();
}
