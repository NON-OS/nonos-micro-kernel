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

// Binds the IPI vectors (0x40-0x44) to their handlers in the interrupt
// dispatch table and signals LAPIC end-of-interrupt. The IDT gates for these
// vectors are installed unconditionally in idt setup; registering the handlers
// here is harmless until SMP actually sends IPIs. Without this, a delivered IPI
// would find no handler, never EOI, and wedge the local APIC.

use crate::arch::x86_64::idt::entry::InterruptFrame;
use crate::arch::x86_64::idt::ops::register_handler;
use crate::arch::x86_64::interrupt::apic::eoi;

use super::constants::{IPI_PANIC, IPI_RESCHEDULE, IPI_STOP, IPI_TLB_SHOOTDOWN};
use super::ipi::IPI_CALL_FUNCTION;

fn ipi_tlb_shootdown(_frame: &mut InterruptFrame) {
    super::tlb::handle_tlb_shootdown_ipi();
    eoi();
}

fn ipi_reschedule(_frame: &mut InterruptFrame) {
    // Acknowledge only. A migrated task is picked up by the target CPU's next
    // timer tick; triggering an immediate reschedule from interrupt context is
    // wired separately once the per-CPU scheduler path is live.
    eoi();
}

fn ipi_panic(_frame: &mut InterruptFrame) {
    eoi();
    super::ipi_handler::handle_panic_ipi();
}

fn ipi_stop(_frame: &mut InterruptFrame) {
    eoi();
    super::ipi_handler::handle_stop_ipi();
}

fn ipi_call_function(_frame: &mut InterruptFrame) {
    super::ipi::handle_call_function_ipi();
    eoi();
}

pub(crate) fn register_ipi_handlers() {
    let _ = register_handler(IPI_TLB_SHOOTDOWN, ipi_tlb_shootdown);
    let _ = register_handler(IPI_RESCHEDULE, ipi_reschedule);
    let _ = register_handler(IPI_PANIC, ipi_panic);
    let _ = register_handler(IPI_STOP, ipi_stop);
    let _ = register_handler(IPI_CALL_FUNCTION, ipi_call_function);
}
