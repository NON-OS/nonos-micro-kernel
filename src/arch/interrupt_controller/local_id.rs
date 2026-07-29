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

//! How the interrupt controller names the calling CPU.
//!
//! This is the number a device's interrupt is routed to and the number
//! [`super::send_ipi`] takes as a target. It is the local APIC ID on x86_64
//! and the packed MPIDR affinity on aarch64. It is deliberately not a dense
//! index: neither controller promises one, and code that wants to index a
//! per-CPU array should use the CPU's own id from the SMP layer.

/// The calling CPU, as the interrupt controller names it.
pub(crate) fn local_id() -> u32 {
    #[cfg(target_arch = "x86_64")]
    return crate::arch::x86_64::interrupt_controller::local_id();
    #[cfg(target_arch = "aarch64")]
    return crate::arch::aarch64::interrupt_controller::local_id();
    #[cfg(not(any(target_arch = "x86_64", target_arch = "aarch64")))]
    return 0;
}
