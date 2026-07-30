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

//! Binds each IPI vector to its handler in the interrupt dispatch table.
//!
//! The IDT gates for these vectors are installed unconditionally during IDT
//! setup; registering the handlers here is harmless until SMP actually sends
//! an IPI. Without it a delivered IPI would find no handler, never signal
//! end-of-interrupt, and wedge the local APIC.

use super::handlers;
use crate::arch::x86_64::idt::entry::InterruptFrame;
use crate::arch::x86_64::idt::ops::register_handler;
use crate::arch::x86_64::interrupt_controller::{
    IPI_BARRIER, IPI_CALL_FUNCTION, IPI_PANIC, IPI_RESCHEDULE, IPI_STOP, IPI_TLB_SHOOTDOWN,
};

fn tlb_shootdown(_frame: &mut InterruptFrame) {
    handlers::tlb_shootdown();
}

fn reschedule(_frame: &mut InterruptFrame) {
    handlers::reschedule();
}

fn call_function(_frame: &mut InterruptFrame) {
    handlers::call_function();
}

fn barrier(_frame: &mut InterruptFrame) {
    handlers::barrier();
}

fn panic(_frame: &mut InterruptFrame) {
    handlers::panic()
}

fn stop(_frame: &mut InterruptFrame) {
    handlers::stop()
}

pub(crate) fn register_ipi_handlers() {
    let _ = register_handler(IPI_TLB_SHOOTDOWN, tlb_shootdown);
    let _ = register_handler(IPI_RESCHEDULE, reschedule);
    let _ = register_handler(IPI_CALL_FUNCTION, call_function);
    let _ = register_handler(IPI_BARRIER, barrier);
    let _ = register_handler(IPI_PANIC, panic);
    let _ = register_handler(IPI_STOP, stop);
}
