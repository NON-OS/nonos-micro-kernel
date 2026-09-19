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

const IA32_MCG_STATUS: u32 = 0x17A;
const IA32_MCG_CAP: u32 = 0x179;

#[derive(Debug, Clone, Copy)]
pub struct MachineCheckStatus {
    pub error_ip_valid: bool,
    pub restart_ip_valid: bool,
    pub in_progress: bool,
    pub bank_count: u8,
}

pub fn handle(frame: InterruptStackFrame) -> ! {
    let ctx = ExceptionContext::from_frame(&frame);
    emit_fatal_notice(b"MC", ctx.instruction_pointer);
    log_exception("MACHINE CHECK", &ctx);
    stats::increment_exceptions();

    let status = read_mcg_status();
    log_machine_check_info(&status);

    dump_mca_banks(status.bank_count);

    crate::log::logger::log_critical("SYSTEM HALTED: Unrecoverable hardware error");
    halt_loop();
}

fn read_mcg_status() -> MachineCheckStatus {
    // SAFETY: Reading MCG_CAP and MCG_STATUS MSRs for machine check information
    let (cap, status) = unsafe {
        let cap: u64;
        let status: u64;
        core::arch::asm!(
            "rdmsr",
            in("ecx") IA32_MCG_CAP,
            out("eax") cap,
            out("edx") _,
            options(nomem, nostack)
        );
        core::arch::asm!(
            "rdmsr",
            in("ecx") IA32_MCG_STATUS,
            out("eax") status,
            out("edx") _,
            options(nomem, nostack)
        );
        (cap, status)
    };

    MachineCheckStatus {
        error_ip_valid: (status & 0x02) != 0,
        restart_ip_valid: (status & 0x01) != 0,
        in_progress: (status & 0x04) != 0,
        bank_count: (cap & 0xFF) as u8,
    }
}

fn log_machine_check_info(status: &MachineCheckStatus) {
    crate::log::logger::log_critical(&alloc::format!(
        "MCE: ip_valid={} restart_valid={} in_progress={} banks={}",
        status.error_ip_valid,
        status.restart_ip_valid,
        status.in_progress,
        status.bank_count
    ));
}

fn dump_mca_banks(count: u8) {
    for bank in 0..count.min(8) {
        let msr_status = 0x401 + (bank as u32 * 4);

        // SAFETY: Reading MCA bank status MSR
        let bank_status: u64 = unsafe {
            let low: u32;
            let high: u32;
            core::arch::asm!(
                "rdmsr",
                in("ecx") msr_status,
                out("eax") low,
                out("edx") high,
                options(nomem, nostack)
            );
            ((high as u64) << 32) | (low as u64)
        };

        if (bank_status & (1 << 63)) != 0 {
            crate::log::logger::log_critical(&alloc::format!(
                "MCA Bank {}: status={:#x}",
                bank,
                bank_status
            ));
        }
    }
}
