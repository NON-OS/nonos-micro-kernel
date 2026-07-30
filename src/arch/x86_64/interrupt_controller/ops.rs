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

use super::vector::vector_of;
use crate::arch::interrupt_controller::Ipi;
use crate::arch::x86_64::interrupt::apic;

/// The local APIC ID, as cached at boot from the real BSP.
pub fn local_id() -> u32 {
    apic::id()
}

/// The local APIC works out which interrupt is being finished from its own
/// in-service register, so the kind is not needed here.
pub fn end_of_interrupt(_ipi: Ipi) {
    apic::eoi();
}

pub fn send_ipi(target: u32, ipi: Ipi) -> Result<(), ()> {
    apic::ipi_one(target, vector_of(ipi));
    Ok(())
}

pub fn broadcast_ipi(ipi: Ipi) -> Result<(), ()> {
    apic::ipi_others(vector_of(ipi));
    Ok(())
}
