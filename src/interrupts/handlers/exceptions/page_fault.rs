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

use crate::memory::addr::VirtAddr;
use x86_64::registers::control::Cr2;
use x86_64::structures::idt::InterruptStackFrame;

use super::context::{log_page_fault, ExceptionContext, PageFaultContext, PageFaultErrorCode};
use crate::interrupts::idt::halt_loop;
use crate::interrupts::safety::set_interrupt_context;
use crate::interrupts::stats;
use crate::memory::hardening;
use crate::memory::paging::manager::api as paging;
use crate::security::observability::redact::redact_address;

/// # Safety
/// Page fault handler. Sets interrupt context for safe nesting detection.
pub fn handle(frame: InterruptStackFrame, error_code: u64) {
    let _ctx = set_interrupt_context();

    let accessed_address = Cr2::read().as_u64();
    crate::arch::x86_64::diag::dump_trap(
        b"PF",
        &frame,
        Some(error_code),
        Some(accessed_address),
    );
    #[cfg(feature = "nonos-trap-kstack-writer")]
    {
        let cpl = (frame.code_segment as u64 & 0x3) as u8;
        let pid_now = crate::process::current_pid().unwrap_or(0);
        if cpl == 3 && pid_now == 2 {
            crate::arch::x86_64::diag::dump_user_around_rsp(
                frame.stack_pointer.as_u64(),
            );
        }
    }
    let exception = ExceptionContext::from_frame(&frame);
    let error = PageFaultErrorCode::from_bits(error_code);

    let ctx = PageFaultContext { exception, accessed_address, error_code: error };

    log_page_fault(&ctx);
    stats::increment_page_faults();

    if try_handle_fault(&ctx, error_code) {
        return;
    }

    if exception.is_user_mode() {
        terminate_user_process(&ctx);
    } else {
        kernel_panic(&ctx);
    }
}

fn try_handle_fault(ctx: &PageFaultContext, error_code: u64) -> bool {
    let virt_addr = VirtAddr::new(ctx.accessed_address);

    if hardening::check_guard_page_access(virt_addr) {
        crate::log::logger::log_critical("Guard page violation detected");
        return false;
    }

    if let Ok(()) = paging::handle_page_fault(virt_addr, error_code) {
        crate::log::logger::log_debug!("Page fault handled successfully");
        return true;
    }

    false
}

fn terminate_user_process(ctx: &PageFaultContext) -> ! {
    crate::log::logger::log_error!(
        "Segmentation fault: user process accessed invalid address {}",
        redact_address(ctx.accessed_address)
    );
    crate::process::exit::exit_and_yield(-11, true)
}

fn kernel_panic(ctx: &PageFaultContext) {
    crate::log::logger::log_critical(&alloc::format!(
        "KERNEL PANIC: Page fault at address {}",
        redact_address(ctx.accessed_address)
    ));

    if ctx.error_code.is_instruction_fetch() {
        crate::log::logger::log_critical("Attempted to execute from non-executable page");
    } else if ctx.error_code.is_write() {
        crate::log::logger::log_critical("Attempted to write to read-only page");
    } else {
        crate::log::logger::log_critical("Attempted to read from non-present page");
    }

    halt_loop();
}
