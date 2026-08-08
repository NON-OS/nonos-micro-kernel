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

//! SMAP: `AC` in RFLAGS decides whether a privileged access to a user page
//! faults. Both instructions are NOPs on a CPU without SMAP, and on one with
//! SMAP left disabled in CR4, so neither needs a feature check.

#[inline(always)]
pub(super) fn allow() {
    // SAFETY: STAC sets one RFLAGS bit and touches nothing else. `deny` is
    // guaranteed to follow by the guard that called this.
    unsafe {
        core::arch::asm!("stac", options(nomem, nostack, preserves_flags));
    }
}

#[inline(always)]
pub(super) fn deny() {
    // SAFETY: CLAC clears the same bit. Safe to run even if it was already
    // clear, which is what makes the guard idempotent.
    unsafe {
        core::arch::asm!("clac", options(nomem, nostack, preserves_flags));
    }
}
