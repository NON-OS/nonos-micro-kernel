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

//! Reading the frame a trampoline points a shim at.

use x86_64::structures::idt::InterruptStackFrame;

/// Read the frame the trampoline built.
///
/// The trampoline passes a pointer into the live interrupt stack it just
/// laid out, valid for the duration of the shim; that is the single safety
/// fact every shim relies on.
#[inline(always)]
pub(super) fn frame(p: *const InterruptStackFrame) -> InterruptStackFrame {
    /*
     * SAFETY: the pointer is into the live interrupt stack the trampoline
     * just laid out, valid for the duration of the shim.
     */
    unsafe { core::ptr::read(p) }
}
