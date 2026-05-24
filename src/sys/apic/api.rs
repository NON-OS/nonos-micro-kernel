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

use super::ioapic::{enable_irq, init_ioapic, IOAPIC_INIT};
use super::local::{init_local_apic, LAPIC_INIT};
use super::vectors::{IRQ_KEYBOARD, IRQ_MOUSE, VECTOR_KEYBOARD, VECTOR_MOUSE};
use crate::arch::x86_64::idt::register_irq_handler;
use crate::sys::serial;
use core::sync::atomic::Ordering;

pub fn init() {
    init_local_apic();
    init_ioapic();
    init_broker_ioapic();
}

// Populate the capability-broker's IO-APIC registry from the same MADT
// descriptors the legacy programmer adopts. Without this the broker's
// `locate()`/`program_route_external` find no chip and every capsule
// `MkIrqBind` (INTx) fails with GsiNotFound. The legacy path keeps
// driving kernel IRQs; the broker drives capsule device GSIs.
fn init_broker_ioapic() {
    use crate::arch::x86_64::acpi;
    use crate::arch::x86_64::interrupt::nonos_ioapic::{
        init as broker_init, IsoFlags, MadtIoApic, MadtIso, MadtNmi,
    };
    use alloc::vec::Vec;

    let mut ioapics: Vec<MadtIoApic> = acpi::ioapics()
        .iter()
        .map(|i| MadtIoApic { phys_base: i.address, gsi_base: i.gsi_base })
        .collect();
    if ioapics.is_empty() {
        ioapics.push(MadtIoApic { phys_base: 0xFEC0_0000, gsi_base: 0 });
    }
    let isos: Vec<MadtIso> = acpi::interrupt_overrides()
        .iter()
        .map(|o| MadtIso {
            bus_irq: o.source_irq,
            gsi: o.gsi,
            flags: IsoFlags::from_polarity_trigger(o.polarity, o.trigger_mode),
        })
        .collect();
    let nmis: [MadtNmi; 0] = [];
    if unsafe { broker_init(&ioapics, &isos, &nmis) }.is_err() {
        serial::println(b"[APIC] broker IOAPIC init skipped");
    }
}

pub fn is_init() -> bool {
    LAPIC_INIT.load(Ordering::Relaxed) && IOAPIC_INIT.load(Ordering::Relaxed)
}

pub fn setup_keyboard_irq() {
    if !is_init() {
        init();
    }
    enable_irq(IRQ_KEYBOARD, VECTOR_KEYBOARD);
    serial::println(b"[APIC] Keyboard IRQ enabled");
}

fn mouse_irq_handler(_irq: u8) {
    crate::interrupts::handlers::irq::mouse();
}

pub fn setup_mouse_irq() {
    if !is_init() {
        init();
    }
    let _ = register_irq_handler(IRQ_MOUSE, mouse_irq_handler);
    enable_irq(IRQ_MOUSE, VECTOR_MOUSE);
    serial::println(b"[APIC] Mouse IRQ enabled");
}
