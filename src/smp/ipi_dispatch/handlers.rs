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

//! What each IPI does when it arrives.
//!
//! The work is the same on every architecture; only how the handler gets
//! called and how the controller is told the interrupt is finished differ, and
//! those live in the per-arch registration modules beside this one.
//!
//! Ordering matters for the two that never return. `Panic` and `Stop` signal
//! completion first, because the CPU is about to halt and a controller left
//! holding an unfinished interrupt at that priority will not deliver anything
//! to it again.

use crate::arch::interrupt_controller::{end_of_interrupt, Ipi};

pub(super) fn tlb_shootdown() {
    crate::memory::paging::manager::handle_shootdown_ipi();
    end_of_interrupt(Ipi::TlbShootdown);
}

pub(super) fn reschedule() {
    // Raise this CPU's flag rather than switching here. We are in interrupt
    // context on the interrupted task's stack, and the sender means "you have
    // work", not "switch now". The idle loop and the preemptible return path
    // both act on the flag.
    crate::process::scheduler::preemption::set_reschedule();
    end_of_interrupt(Ipi::Reschedule);
}

pub(super) fn call_function() {
    super::super::ipi::handle_call_function_ipi();
    end_of_interrupt(Ipi::CallFunction);
}

pub(super) fn barrier() {
    super::super::ipi::handle_barrier_ipi();
    end_of_interrupt(Ipi::Barrier);
}

pub(super) fn panic() -> ! {
    end_of_interrupt(Ipi::Panic);
    super::super::ipi_handler::handle_panic_ipi()
}

pub(super) fn stop() -> ! {
    end_of_interrupt(Ipi::Stop);
    super::super::ipi_handler::handle_stop_ipi()
}
