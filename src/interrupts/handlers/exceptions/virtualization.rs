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
use crate::arch::x86_64::diag::emit_fatal_notice;
use crate::interrupts::idt::halt_loop;
use crate::interrupts::stats;

const VE_INFO_ADDRESS: u64 = 0;

pub fn handle(frame: InterruptStackFrame) {
    let ctx = ExceptionContext::from_frame(&frame);
    if !ctx.is_user_mode() {
        emit_fatal_notice(b"VE", ctx.instruction_pointer);
    }
    log_exception("VIRTUALIZATION EXCEPTION", &ctx);
    stats::increment_exceptions();

    let ve_info = read_ve_info();
    log_ve_details(&ve_info);

    if ctx.is_user_mode() {
        handle_user_mode_ve(&ctx, &ve_info);
    } else {
        handle_kernel_mode_ve(&ctx, &ve_info);
    }
}

#[derive(Debug, Clone, Copy)]
struct VeInfo {
    exit_reason: u32,
    exit_qualification: u64,
    guest_linear_address: u64,
    guest_physical_address: u64,
}

fn read_ve_info() -> VeInfo {
    if VE_INFO_ADDRESS == 0 {
        return VeInfo {
            exit_reason: 0,
            exit_qualification: 0,
            guest_linear_address: 0,
            guest_physical_address: 0,
        };
    }

    // SAFETY: Reading from VE info page if configured
    unsafe {
        let info_ptr = VE_INFO_ADDRESS as *const u64;
        VeInfo {
            exit_reason: (info_ptr.read_volatile() & 0xFFFF_FFFF) as u32,
            exit_qualification: info_ptr.add(1).read_volatile(),
            guest_linear_address: info_ptr.add(2).read_volatile(),
            guest_physical_address: info_ptr.add(3).read_volatile(),
        }
    }
}

fn log_ve_details(info: &VeInfo) {
    let reason_name = match info.exit_reason {
        0 => "Exception/NMI",
        1 => "External Interrupt",
        48 => "EPT Violation",
        49 => "EPT Misconfiguration",
        56 => "APIC Access",
        _ => "Unknown",
    };

    crate::log::logger::log_error!(
        "VE: reason={} ({}) qual={:#x} gla={:#x} gpa={:#x}",
        info.exit_reason,
        reason_name,
        info.exit_qualification,
        info.guest_linear_address,
        info.guest_physical_address
    );
}

fn handle_user_mode_ve(ctx: &ExceptionContext, info: &VeInfo) -> ! {
    crate::log::logger::log_error!(
        "User process triggered virtualization exception at {:#x}",
        ctx.instruction_pointer
    );

    if info.exit_reason == 48 {
        crate::log::logger::log_error!("EPT violation: address={:#x}", info.guest_physical_address);
    }

    crate::process::exit::exit_and_yield(-7, true)
}

fn handle_kernel_mode_ve(ctx: &ExceptionContext, info: &VeInfo) {
    crate::log::logger::log_critical(&alloc::format!(
        "KERNEL PANIC: Virtualization exception at {:#x}",
        ctx.instruction_pointer
    ));

    crate::log::logger::log_critical(&alloc::format!(
        "VE reason={} qual={:#x} gpa={:#x}",
        info.exit_reason,
        info.exit_qualification,
        info.guest_physical_address
    ));

    halt_loop();
}
