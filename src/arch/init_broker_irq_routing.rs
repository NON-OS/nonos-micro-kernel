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

/// Program the routes that carry device interrupts to a CPU, and report how
/// many were installed.
///
/// x86_64 reads the ACPI tables and fills in IO-APIC redirection entries, so the
/// count is the work done. aarch64 has no separate step: the GIC distributor is
/// what routes device interrupts and it was configured during arch bring-up, so
/// the honest answer is that no further routes were needed. Returning an error
/// there would be reporting a failure for a stage that does not exist.
pub(crate) fn init_broker_irq_routing() -> Result<usize, ()> {
    #[cfg(target_arch = "x86_64")]
    return crate::arch::x86_64::interrupt::ioapic::init_from_acpi();

    #[cfg(target_arch = "aarch64")]
    return Ok(0);

    #[cfg(not(any(target_arch = "x86_64", target_arch = "aarch64")))]
    return Err(());
}
