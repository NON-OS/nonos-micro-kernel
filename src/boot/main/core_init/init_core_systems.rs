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

use crate::sys::{apic, idt, serial};
use crate::{bus, interrupts};
use core::arch::asm;

use super::acpi_tables::init_acpi_tables;
#[cfg(feature = "nonos-user-entry-proof")]
use super::syscall_msrs::print_syscall_msrs;

pub fn init_core_systems() {
    serial::init();
    serial::println(b"[NONOS] Kernel entry - SSE enabled");
    crate::arch::x86_64::time::timer::init_boot_time();
    // Anchor elapsed time here, before anything can ask for it. The call above
    // still sets up the PC's own clocks; this latches the counter reading and
    // rate that crate::time measures every duration from.
    crate::time::anchor();
    crate::sys::timer::tsc::init_default();
    crate::sys::bench::mark(b"kernel_entry");
    if crate::arch::x86_64::gdt::init().is_err() {
        serial::println(b"[FATAL] arch GDT init failed");
        crate::arch::halt_loop();
    }
    serial::println(b"[NONOS] GDT configured");
    if crate::arch::x86_64::syscall::init().is_err() {
        serial::println(b"[FATAL] arch syscall init failed");
        crate::arch::halt_loop();
    }
    serial::println(b"[NONOS] SYSCALL configured");
    #[cfg(feature = "nonos-user-entry-proof")]
    print_syscall_msrs();
    unsafe {
        idt::setup();
    }
    serial::println(b"[NONOS] Early IDT configured");
    crate::memory::heap::manager::init_bootstrap();
    serial::println(b"[NONOS] Global allocator initialized");
    interrupts::init_idt();
    serial::println(b"[NONOS] Full IDT loaded");
    crate::sys::bench::mark(b"kernel_idt_ready");
    init_acpi_tables();
    apic::init();
    serial::println(b"[NONOS] APIC initialized");
    // Ensure a halted core keeps receiving LAPIC timer ticks (C1E on
    // laptops otherwise gates the timer clock and freezes the scheduler
    // tick the moment the CPU idles) before the timer is armed.
    crate::arch::x86_64::interrupt::apic::idle_timer::init();
    if crate::arch::x86_64::interrupt::apic::preemption::install_on_bsp().is_err() {
        serial::println(b"[FATAL] preemption timer install failed");
        crate::arch::halt_loop();
    }
    serial::println(b"[NONOS] Preemption timer armed");
    crate::sys::bench::mark(b"kernel_timer_ready");
    // PS/2 input + IRQ wiring belongs to the legacy tree. The
    // microkernel boot path does not bring up keyboard/mouse rings;
    // input is owned by future capsule migration (input capsule).
    unsafe {
        asm!("sti", options(nomem, nostack));
    }
    serial::println(b"[NONOS] Interrupts enabled");
    super::super::init_memory_encryption();
    bus::pci::init();
    serial::println(b"[NONOS] PCI enumerated");
    crate::sys::bench::mark(b"kernel_pci_ready");
    // Wants the DMAR bases from the ACPI parse and an MMIO mapper that can
    // hand out a register window, so it runs here, not beside the parse.
    #[cfg(feature = "nonos-arch-iommu")]
    crate::arch::x86_64::iommu::unit::report::init();
    #[cfg(feature = "nonos-arch-iommu")]
    crate::arch::x86_64::iommu::unit::bringup::init();
    crate::kernel_core::init::init_platform_baseline();
    crate::sys::bench::mark(b"kernel_core_ready");
}
