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

//! Signalling that an interrupt has been handled.
//!
//! Every handler owes the controller exactly one of these, and skipping it
//! wedges the CPU: the controller keeps the priority raised and delivers
//! nothing further. The local APIC infers which interrupt is being finished
//! from its own in-service state, while the GIC needs to be told, so the
//! interrupt is named here even though one backend ignores it.

use super::kind::Ipi;

/// Finish the interrupt the calling handler was entered for.
pub(crate) fn end_of_interrupt(ipi: Ipi) {
    #[cfg(target_arch = "x86_64")]
    crate::arch::x86_64::interrupt_controller::end_of_interrupt(ipi);
    #[cfg(target_arch = "aarch64")]
    crate::arch::aarch64::interrupt_controller::end_of_interrupt(ipi);
    #[cfg(not(any(target_arch = "x86_64", target_arch = "aarch64")))]
    let _ = ipi;
}
