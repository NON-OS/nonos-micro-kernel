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

#[derive(Debug, Clone, Copy)]
pub struct GpfErrorCode {
    bits: u64,
}

impl GpfErrorCode {
    pub const fn from_bits(bits: u64) -> Self {
        Self { bits }
    }

    pub const fn is_external(&self) -> bool {
        (self.bits & 0x01) != 0
    }

    pub const fn is_idt(&self) -> bool {
        ((self.bits >> 1) & 0x01) != 0
    }

    pub const fn is_ldt(&self) -> bool {
        ((self.bits >> 2) & 0x01) != 0
    }

    pub const fn selector_index(&self) -> u16 {
        ((self.bits >> 3) & 0x1FFF) as u16
    }

    pub const fn is_null_selector(&self) -> bool {
        self.bits == 0
    }
}

pub fn handle(frame: InterruptStackFrame, error_code: u64) {
    crate::arch::x86_64::diag::dump_trap(b"GP", &frame, Some(error_code), None);
    let ctx = ExceptionContext::from_frame(&frame);
    log_exception_with_code("GENERAL PROTECTION FAULT", &ctx, error_code);
    stats::increment_exceptions();

    let gpf_error = GpfErrorCode::from_bits(error_code);
    analyze_gpf(&ctx, &gpf_error);

    if ctx.is_user_mode() {
        terminate_user_process();
    } else {
        kernel_panic(&ctx);
    }
}

fn analyze_gpf(ctx: &ExceptionContext, error: &GpfErrorCode) {
    if error.is_null_selector() {
        crate::log::logger::log_error!("GPF: Null selector or general violation");
        log_instruction_context(ctx);
    } else {
        crate::log::logger::log_error!(
            "GPF: external={} idt={} ldt={} index={}",
            error.is_external(),
            error.is_idt(),
            error.is_ldt(),
            error.selector_index()
        );
    }
}

fn log_instruction_context(ctx: &ExceptionContext) {
    // SAFETY: Reading instruction bytes for diagnostic purposes
    let ptr = ctx.instruction_pointer as *const u8;
    let bytes: [u8; 8] = unsafe {
        let mut buf = [0u8; 8];
        for i in 0..8 {
            buf[i] = *ptr.add(i);
        }
        buf
    };

    crate::log::logger::log_error!(
        "Instruction at fault: {:02x} {:02x} {:02x} {:02x} {:02x} {:02x} {:02x} {:02x}",
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

// Tear the faulting capsule down through the canonical exit path and
// transfer control to the next runnable process via the scheduler's
// first-entry / resume helpers. Never returns; the dying capsule's
// kernel stack is queued for deferred release on the next live
// capsule's timer tick.
fn terminate_user_process() -> ! {
    crate::log::logger::log_error!("Terminating user process: general protection fault");
    crate::process::exit::exit_and_yield(0x8000_0000u32 as i32, true)
}

fn kernel_panic(_ctx: &ExceptionContext) {
    crate::log::logger::log_critical("KERNEL PANIC: General protection fault in kernel mode");
    halt_loop();
}
