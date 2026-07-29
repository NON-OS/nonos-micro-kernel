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

use super::intid::intid_of;
use crate::arch::aarch64::cpu::cpu_affinity;
use crate::arch::aarch64::gic;
use crate::arch::interrupt_controller::Ipi;

/// The GIC addresses a CPU by affinity, so that is what this returns. It is
/// the same value a redistributor reports as its owner.
pub fn local_id() -> u32 {
    cpu_affinity()
}

/// The GIC has no in-service register to consult: the handler has to name the
/// INTID it is finishing, or the running priority never drops.
pub fn end_of_interrupt(ipi: Ipi) {
    gic::end_interrupt(intid_of(ipi));
}

pub fn send_ipi(target: u32, ipi: Ipi) -> Result<(), ()> {
    gic::send_sgi(target, intid_of(ipi))
}

pub fn broadcast_ipi(ipi: Ipi) -> Result<(), ()> {
    gic::send_sgi_all_others(intid_of(ipi))
}
