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

//! PAN: `PSTATE.PAN` set makes an EL1 access to an EL0-accessible page fault.
//! Clearing it opens the window, setting it closes it again.
//!
//! The bit is written through the `PAN` pseudo-register, which only exists
//! when the core implements FEAT_PAN. On a core without it there is nothing to
//! clear and nothing to restore: EL1 could already reach user pages, and the
//! window is permanently open. That is weaker than x86 with SMAP and is why
//! the feature is checked rather than assumed.

use crate::arch::aarch64::cpu::{has_feature, CpuFeature};

/// `MSR PAN, #0`, encoded by hand so the assembler takes it without the build
/// enabling `+pan` everywhere. `PAN` is `MSR (immediate)` with op1 = 0, op2 = 4.
#[inline(always)]
pub(super) fn allow() {
    if !supported() {
        return;
    }
    // SAFETY: FEAT_PAN is implemented, so the pseudo-register exists and is
    // writable at EL1. Writing it changes only PSTATE.PAN, and the guard that
    // called this restores it.
    unsafe {
        core::arch::asm!("msr pan, #0", options(nomem, nostack, preserves_flags));
    }
}

#[inline(always)]
pub(super) fn deny() {
    if !supported() {
        return;
    }
    // SAFETY: as for `allow`. Setting the bit is the safe direction: it can
    // only turn an access that would have succeeded into a fault.
    unsafe {
        core::arch::asm!("msr pan, #1", options(nomem, nostack, preserves_flags));
    }
}

fn supported() -> bool {
    has_feature(CpuFeature::Pan)
}
