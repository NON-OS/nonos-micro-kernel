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

//! Supervisor write-protect control for the paging manager. The override
//! itself is architectural (see `arch::paging::write_protect`, which documents
//! why aarch64 has none); the scoped helper is shared so callers restore
//! enforcement on every path out.

/// Enforce read-only kernel mappings.
#[inline]
pub fn enable_write_protection() {
    crate::arch::paging::enable_write_protection();
}

/// Lift read-only enforcement for kernel writes.
///
/// # Safety
///
/// Enforcement must be restored before returning to code that assumes
/// read-only kernel mappings hold. Prefer `with_write_protection_disabled`,
/// which restores it for you.
#[inline]
pub unsafe fn disable_write_protection() {
    // SAFETY: the caller carries the obligation documented above.
    unsafe { crate::arch::paging::disable_write_protection() };
}

/// Run `f` with kernel write protection lifted, restoring it afterwards.
///
/// # Safety
///
/// `f` must confine its writes to the read-only kernel mapping the caller
/// meant to edit; while it runs, no page-level protection stands in the way.
#[inline]
pub unsafe fn with_write_protection_disabled<F, R>(f: F) -> R
where
    F: FnOnce() -> R,
{
    // SAFETY: enforcement is restored on the way out, so the window is exactly
    // the body of `f`.
    unsafe {
        disable_write_protection();
        let result = f();
        enable_write_protection();
        result
    }
}
