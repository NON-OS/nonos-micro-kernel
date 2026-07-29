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

//! Is the table that receives traps still the one the kernel installed?
//!
//! A rootkit that owns one entry owns every trap that lands on it, so the
//! monitor asks this on a schedule. x86_64 keeps the entries in an IDT and
//! aarch64 in a vector page that `VBAR_EL1` points at; both amount to the
//! same question and neither is answerable from shared code.

/// Whether the trap vectors are installed and unmodified.
///
/// `false` is a finding, not an error: the caller reports it rather than
/// acting on it.
pub(crate) fn is_intact() -> bool {
    #[cfg(target_arch = "x86_64")]
    {
        crate::arch::x86_64::idt::verify_idt_integrity()
    }
    #[cfg(target_arch = "aarch64")]
    {
        crate::arch::aarch64::exceptions::vectors_installed()
    }
    #[cfg(not(any(target_arch = "x86_64", target_arch = "aarch64")))]
    {
        false
    }
}
