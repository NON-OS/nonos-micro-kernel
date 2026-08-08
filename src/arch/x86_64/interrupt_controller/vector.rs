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

//! Which interrupt vector carries which IPI.
//!
//! These sit above the 0x20..0x30 range the legacy PICs and the IO-APIC's
//! first redirection entries use, and below the spurious vector. The IDT gates
//! for all of them are installed at boot, and `smp::ipi_dispatch` binds a handler to
//! each; this table is the one place that decides which number means what.

use crate::arch::interrupt_controller::Ipi;

pub const IPI_TLB_SHOOTDOWN: u8 = 0x40;
pub const IPI_RESCHEDULE: u8 = 0x41;
pub const IPI_PANIC: u8 = 0x42;
pub const IPI_STOP: u8 = 0x43;
pub const IPI_CALL_FUNCTION: u8 = 0x44;
pub const IPI_BARRIER: u8 = 0x45;

pub const fn vector_of(ipi: Ipi) -> u8 {
    match ipi {
        Ipi::TlbShootdown => IPI_TLB_SHOOTDOWN,
        Ipi::Reschedule => IPI_RESCHEDULE,
        Ipi::CallFunction => IPI_CALL_FUNCTION,
        Ipi::Barrier => IPI_BARRIER,
        Ipi::Panic => IPI_PANIC,
        Ipi::Stop => IPI_STOP,
    }
}
