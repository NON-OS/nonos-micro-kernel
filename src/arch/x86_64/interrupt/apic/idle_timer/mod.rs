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

//! Keep the local-APIC timer running while the CPU is idle.
//!
//! The scheduler tick is the LAPIC timer, and the idle loop halts the CPU
//! with `hlt` waiting for it. On a plain `hlt` (C1) the timer keeps
//! counting, but laptops enable C1E, an enhanced halt that gates the APIC
//! timer clock: the tick then stops the instant the CPU goes idle, which
//! freezes the clock, preemption, and every timer-driven wakeup. Emulators
//! never show this because their virtual APIC timer always runs and they
//! advertise the ARAT feature that promises exactly that.
//!
//! Two defenses, applied at boot:
//!   1. If the CPU advertises ARAT (CPUID.06H:EAX bit 2) the timer is
//!      reliable in every C-state and `hlt` is safe as-is.
//!   2. Otherwise, on Intel, clear the C1E-enable bit in POWER_CTL so `hlt`
//!      stays in plain C1 where the timer runs. When neither guarantee
//!      holds, `halt_safe()` reports false and the idle loop spins instead
//!      of halting, trading power for a tick that cannot stall.

mod arat_supported;
mod consts;
mod halt_safe;
mod init;
mod is_intel;

pub use halt_safe::halt_safe;
pub use init::init;
