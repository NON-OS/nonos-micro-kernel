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

//! Getting IPI handlers attached to the interrupt controller.
//!
//! Registration is the arch-specific half: x86_64 binds an IDT vector, aarch64
//! registers an SGI with the GIC. Both end up calling the same handlers.

mod handlers;

#[cfg(target_arch = "aarch64")]
mod aarch64;
#[cfg(target_arch = "x86_64")]
mod x86_64;

#[cfg(target_arch = "aarch64")]
pub(crate) use aarch64::register_ipi_handlers;
#[cfg(target_arch = "x86_64")]
pub(crate) use x86_64::register_ipi_handlers;

/// Nothing to register on an arch with no interrupt-controller backend, and
/// nothing that would send an IPI either.
#[cfg(not(any(target_arch = "x86_64", target_arch = "aarch64")))]
pub(crate) fn register_ipi_handlers() {}
