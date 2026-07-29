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

//! Claims the five software-generated interrupts the kernel sends IPIs on.
//!
//! SGIs are banked per CPU, so one registration on the boot CPU covers the
//! whole system: the table the GIC dispatcher consults is keyed by INTID, and
//! every core raises the same INTID for the same meaning.

use super::handlers;
use crate::arch::aarch64::gic::register_irq_handler;
use crate::arch::aarch64::interrupt_controller::intid_of;
use crate::arch::interrupt_controller::Ipi;
use crate::sys::serial;

fn tlb_shootdown(_intid: u32) {
    handlers::tlb_shootdown();
}

fn reschedule(_intid: u32) {
    handlers::reschedule();
}

fn call_function(_intid: u32) {
    handlers::call_function();
}

fn barrier(_intid: u32) {
    handlers::barrier();
}

fn panic(_intid: u32) {
    handlers::panic()
}

fn stop(_intid: u32) {
    handlers::stop()
}

pub(crate) fn register_ipi_handlers() {
    let table: [(Ipi, fn(u32)); 6] = [
        (Ipi::TlbShootdown, tlb_shootdown),
        (Ipi::Reschedule, reschedule),
        (Ipi::CallFunction, call_function),
        (Ipi::Barrier, barrier),
        (Ipi::Panic, panic),
        (Ipi::Stop, stop),
    ];
    for (ipi, handler) in table {
        if register_irq_handler(intid_of(ipi), handler).is_err() {
            // An unclaimed SGI is delivered and then dropped, so the sender
            // waits out its timeout instead of being answered. Worth saying.
            serial::println(b"[SMP] could not claim an IPI SGI");
        }
    }
}
