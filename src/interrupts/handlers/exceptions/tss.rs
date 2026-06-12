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
pub struct TssErrorCode {
    bits: u64,
}

impl TssErrorCode {
    pub const fn from_bits(bits: u64) -> Self {
        Self { bits }
    }

    pub const fn is_external(&self) -> bool {
        (self.bits & 0x01) != 0
    }

    pub const fn descriptor_table(&self) -> DescriptorTable {
        // The mask yields exactly 0..=3, all matched below. The final arm
        // is unreachable in practice but returns a benign value rather
        // than panicking: this runs inside the Invalid-TSS exception
        // handler, where a panic could escalate to a double fault.
        match (self.bits >> 1) & 0x03 {
            0 => DescriptorTable::Gdt,
            1 => DescriptorTable::Idt,
            2 => DescriptorTable::Ldt,
            _ => DescriptorTable::Idt,
        }
    }

    pub const fn selector_index(&self) -> u16 {
        ((self.bits >> 3) & 0x1FFF) as u16
    }
}

#[derive(Debug, Clone, Copy)]
pub enum DescriptorTable {
    Gdt,
    Idt,
    Ldt,
}

pub fn handle(frame: InterruptStackFrame, error_code: u64) {
    let ctx = ExceptionContext::from_frame(&frame);
    log_exception_with_code("INVALID TSS", &ctx, error_code);
    stats::increment_exceptions();

    let tss_error = TssErrorCode::from_bits(error_code);
    log_tss_error(&tss_error);

    crate::log::logger::log_critical("KERNEL PANIC: Invalid TSS");
    halt_loop();
}

fn log_tss_error(error: &TssErrorCode) {
    crate::log::logger::log_error!(
        "TSS Error: external={} table={:?} selector_index={}",
        error.is_external(),
        error.descriptor_table(),
        error.selector_index()
    );
}
