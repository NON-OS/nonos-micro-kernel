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

//! The IDT gates for the IPI vectors.
//!
//! Installed into the table the kernel actually loads, while that table is
//! being built, so the gates exist before any CPU loads it.
//!
//! They used to be registrations into the handler array under
//! `arch::x86_64::idt` instead. That array belongs to a second interrupt
//! descriptor table whose `init` has no caller, so the kernel has never run on
//! it and nothing in that array is ever consulted. The note here used to say
//! the gates were "installed unconditionally during IDT setup", and went on to
//! describe what happens when they are not: a delivered IPI finds no handler,
//! never signals end-of-interrupt, and wedges the local APIC. That was a
//! description of the live behaviour, written as a reassurance.
//!
//! What it cost: the live table has no gate at these vectors, so every TLB
//! shootdown IPI sent to an application processor arrived on a not-present
//! gate. The AP took a segment-not-present fault instead of flushing and never
//! acknowledged, and the boot CPU halted the machine on the shootdown timeout.
//! No kernel with more than one CPU online could complete a page-table
//! mutation.

use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};

use super::handlers;
use crate::arch::x86_64::interrupt_controller::{
    IPI_BARRIER, IPI_CALL_FUNCTION, IPI_PANIC, IPI_RESCHEDULE, IPI_STOP, IPI_TLB_SHOOTDOWN,
};

extern "x86-interrupt" fn tlb_shootdown(_frame: InterruptStackFrame) {
    handlers::tlb_shootdown();
}

extern "x86-interrupt" fn reschedule(_frame: InterruptStackFrame) {
    handlers::reschedule();
}

extern "x86-interrupt" fn call_function(_frame: InterruptStackFrame) {
    handlers::call_function();
}

extern "x86-interrupt" fn barrier(_frame: InterruptStackFrame) {
    handlers::barrier();
}

// These two never return, but an interrupt gate takes a handler typed as
// returning unit, so the divergence stays inside the call rather than in the
// signature.
extern "x86-interrupt" fn panic(_frame: InterruptStackFrame) {
    handlers::panic()
}

extern "x86-interrupt" fn stop(_frame: InterruptStackFrame) {
    handlers::stop()
}

/// Install a gate for every IPI vector.
pub(crate) fn install_gates(idt: &mut InterruptDescriptorTable) {
    idt[IPI_TLB_SHOOTDOWN as usize].set_handler_fn(tlb_shootdown);
    idt[IPI_RESCHEDULE as usize].set_handler_fn(reschedule);
    idt[IPI_CALL_FUNCTION as usize].set_handler_fn(call_function);
    idt[IPI_BARRIER as usize].set_handler_fn(barrier);
    // These two halt the CPU and never return, which is why each signals
    // end-of-interrupt before it does.
    idt[IPI_PANIC as usize].set_handler_fn(panic);
    idt[IPI_STOP as usize].set_handler_fn(stop);
}
