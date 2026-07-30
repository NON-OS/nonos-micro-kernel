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

extern crate alloc;

pub mod abi;
pub mod acpi;
mod api;
pub mod asm;
pub mod boot;
pub mod console;
pub mod context;
pub mod cpu;
pub mod cpu_random;
pub mod diag;
pub mod gdt;
pub(crate) mod idle;
pub mod idt;
pub mod interrupt;
pub mod interrupt_controller;
#[cfg(feature = "nonos-arch-iommu")]
pub mod iommu;
pub mod multiboot;
pub mod paging;
pub mod pci;
pub mod port;
pub mod serial;
pub mod smm;
pub mod syscall;
pub mod time;
pub mod uefi;
pub mod vga;
pub mod watchdog;

pub use api::{get_stats, init, init_with_acpi, is_initialized, ArchStats};

pub use gdt::{
    IST_DEBUG, IST_DOUBLE_FAULT, IST_GP, IST_MACHINE_CHECK, IST_NMI, IST_PAGE_FAULT,
    SEL_KERNEL_CODE, SEL_KERNEL_DATA, SEL_NULL, SEL_TSS, SEL_USER_CODE, SEL_USER_DATA,
};

pub use idt::{
    are_enabled, disable, enable, without_interrupts, InterruptFrame, IRQ_BASE, VEC_BREAKPOINT,
    VEC_DEBUG, VEC_DIVIDE_ERROR, VEC_DOUBLE_FAULT, VEC_GENERAL_PROTECTION, VEC_NMI, VEC_PAGE_FAULT,
};

pub use x86_64::structures::idt::InterruptStackFrame;

pub use cpu::{cli, hlt, lfence, mfence, pause, rdtsc, sfence, sti};
pub use port::{inb, inl, inw, outb, outl, outw, Port};
pub use serial::{write_str as serial_write_str, SerialWriter};
pub use time::{delay_ms, delay_ns, delay_us, now_ns};
pub use vga::{clear, print_critical, set_color, write_byte, write_str, Color, ColorCode};

#[cfg(feature = "nonos-arch-iommu")]
pub use iommu::types::{
    DomainId as IommuDomainId, IommuPageFlags, SourceId as IommuSourceId, VtdError as IommuVtdError,
};
