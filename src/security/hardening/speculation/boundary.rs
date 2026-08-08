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

//! Crossing the privilege boundary.

/// Entering the kernel from user code.
#[inline]
pub fn kernel_entry() {
    #[cfg(target_arch = "x86_64")]
    super::super::spectre_mitigations::kernel_entry_mitigations();

    #[cfg(target_arch = "aarch64")]
    crate::arch::aarch64::security::spectre::enter_kernel();
}

/// Returning to user code.
#[inline]
pub fn kernel_exit() {
    #[cfg(target_arch = "x86_64")]
    super::super::spectre_mitigations::kernel_exit_mitigations();

    #[cfg(target_arch = "aarch64")]
    crate::arch::aarch64::security::spectre::exit_kernel();
}
