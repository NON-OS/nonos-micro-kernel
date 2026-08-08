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

use core::arch::asm;

use crate::arch::aarch64::cpu::features::{has_feature, CpuFeature};

/// Stop speculation running past this point.
///
/// `sb` says exactly that, but it only exists from ARMv8.5, so it goes behind a
/// feature check and is never executed blind. A part without it gets `dsb sy`
/// then `isb`, the sequence the architecture defines for the same purpose: drain
/// what is outstanding, then flush the pipeline so nothing fetched under an
/// earlier prediction survives.
///
/// The check matters more than it looks. `sb` on a part that lacks it is an
/// undefined instruction, and this runs during early bring-up, so the fault
/// arrives as a synchronous exception and the machine stops before the console
/// can explain itself.
pub fn speculative_barrier() {
    if has_feature(CpuFeature::Sb) {
        // SAFETY: the probe read ID_AA64ISAR1_EL1 and found SB implemented here.
        // The instruction touches no memory.
        unsafe {
            asm!("sb", options(nomem, nostack));
        }
    } else {
        // SAFETY: both are ARMv8.0 baseline, so always implemented.
        unsafe {
            asm!("dsb sy", "isb", options(nomem, nostack));
        }
    }
}

/// Discard branch prediction state carried across a privilege change.
pub fn clear_prediction_state() {
    speculative_barrier();
}
