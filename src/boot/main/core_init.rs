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

pub fn init_core_systems() {
    serial::init();
    serial::println(b"[NONOS] Kernel entry - SSE enabled");
    crate::arch::x86_64::time::timer::init_boot_time();
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
    super::init_memory_encryption();
    bus::pci::init();
    serial::println(b"[NONOS] PCI enumerated");
    crate::sys::bench::mark(b"kernel_pci_ready");
    seed_hardware_broker();
    init_entropy();
    init_boot_session_nonce();
    super::init_token_signing_key();
    crate::sys::bench::mark(b"kernel_core_ready");
}

fn init_acpi_tables() {
    if let Some(handoff) = crate::boot::handoff::get_handoff() {
        if let Some(rsdp) = handoff.acpi_rsdp() {
            crate::arch::x86_64::acpi::set_rsdp_address(rsdp);
        }
    }
    match crate::arch::x86_64::acpi::init() {
        Ok(()) => serial::println(b"[NONOS] ACPI tables parsed"),
        Err(_) => serial::println(b"[NONOS] ACPI init failed; legacy fallbacks engaged"),
    }
}

fn seed_hardware_broker() {
    let devices = match crate::drivers::pci::manager::scan_and_collect_safe() {
        Ok(v) => v,
        Err(_) => {
            serial::println(b"[NONOS] PCI scan failed; broker table empty");
            return;
        }
    };
    crate::hardware::broker::init_from_pci(&devices);
    let _ = crate::hardware::broker::register_legacy_platform_devices();
    crate::hardware::broker::register_acpi_i2c();
    crate::hardware::broker::register_acpi_gpio();
    serial::println(b"[NONOS] hardware broker seeded");
}

fn init_entropy() {
    if crate::drivers::init_virtio_rng().is_ok() {
        serial::println(b"[NONOS] VirtIO-RNG ready");
    } else {
        serial::println(b"[NONOS] Software RNG");
    }
}

fn init_boot_session_nonce() {
    match crate::security::boot_session::init_once_from_rng() {
        Ok(()) => serial::println(b"[NONOS] boot session nonce latched"),
        Err(_) => {
            serial::println(b"[FATAL] boot session nonce init failed");
            crate::arch::halt_loop();
        }
    }
}

#[cfg(feature = "nonos-user-entry-proof")]
fn print_syscall_msrs() {
    use crate::arch::x86_64::diag::print_hex_u64;
    use crate::arch::x86_64::syscall::msr::{
        read_msr, EFER_SCE, IA32_EFER, IA32_FMASK, IA32_LSTAR, IA32_STAR,
    };
    let efer = read_msr(IA32_EFER);
    serial::print(b"[SYSCALL-MSR] EFER=");
    print_hex_u64(efer);
    serial::print(b" SCE=");
    print_hex_u64(efer & EFER_SCE);
    serial::println(b"");
    serial::print(b"[SYSCALL-MSR] STAR=");
    print_hex_u64(read_msr(IA32_STAR));
    serial::println(b"");
    serial::print(b"[SYSCALL-MSR] LSTAR=");
    print_hex_u64(read_msr(IA32_LSTAR));
    serial::println(b"");
    serial::print(b"[SYSCALL-MSR] SFMASK=");
    print_hex_u64(read_msr(IA32_FMASK));
    serial::println(b"");
}
