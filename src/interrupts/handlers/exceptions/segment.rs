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
pub struct SegmentErrorCode {
    bits: u64,
}

impl SegmentErrorCode {
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
}

pub fn handle_not_present(frame: InterruptStackFrame, error_code: u64) {
    let ctx = ExceptionContext::from_frame(&frame);
    log_exception_with_code("SEGMENT NOT PRESENT", &ctx, error_code);
    stats::increment_exceptions();

    let seg_error = SegmentErrorCode::from_bits(error_code);
    log_segment_error(&seg_error);

    if ctx.is_user_mode() {
        terminate_user_process(&ctx);
    } else {
        kernel_panic(&ctx);
    }
}

fn log_segment_error(error: &SegmentErrorCode) {
    crate::log::logger::log_error!(
        "Segment Error: external={} idt={} ldt={} index={}",
        error.is_external(),
        error.is_idt(),
        error.is_ldt(),
        error.selector_index()
    );
}

fn terminate_user_process(_ctx: &ExceptionContext) -> ! {
    crate::log::logger::log_error!("Terminating user process: segment not present");
    crate::process::exit::exit_and_yield(-12, true)
}

fn kernel_panic(_ctx: &ExceptionContext) {
    crate::log::logger::log_critical("KERNEL PANIC: Segment not present in kernel mode");
    halt_loop();
}
