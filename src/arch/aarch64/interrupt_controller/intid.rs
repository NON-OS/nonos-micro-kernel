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

//! Which SGI carries which IPI.
//!
//! INTIDs 0 to 15 are software-generated interrupts and are the only ones a
//! CPU may raise on another. They are private per CPU, so every core uses the
//! same five numbers for the same five meanings.

use crate::arch::interrupt_controller::Ipi;

pub const SGI_TLB_SHOOTDOWN: u32 = 0;
pub const SGI_RESCHEDULE: u32 = 1;
pub const SGI_CALL_FUNCTION: u32 = 2;
pub const SGI_BARRIER: u32 = 3;
pub const SGI_PANIC: u32 = 4;
pub const SGI_STOP: u32 = 5;

pub const fn intid_of(ipi: Ipi) -> u32 {
    match ipi {
        Ipi::TlbShootdown => SGI_TLB_SHOOTDOWN,
        Ipi::Reschedule => SGI_RESCHEDULE,
        Ipi::CallFunction => SGI_CALL_FUNCTION,
        Ipi::Barrier => SGI_BARRIER,
        Ipi::Panic => SGI_PANIC,
        Ipi::Stop => SGI_STOP,
    }
}
