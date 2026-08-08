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

//! The supervisor write-protect override.
//!
//! x86_64 gates kernel writes to read-only pages on a single bit, `CR0.WP`, so
//! privileged code can lift page protection for the length of one edit.
//! aarch64 has no equivalent: kernel write permission is carried per page in
//! the `AP` bits of the translation tables and cannot be overridden globally.
//! The calls therefore mean different things per architecture and the aarch64
//! side says so rather than pretending the override exists. Code that must
//! write through a read-only kernel mapping on aarch64 installs a writable
//! alias for the frame and drops it again; it does not reach for this.

/// Enforce read-only kernel mappings.
///
/// On aarch64 this is already the standing state, so the call has nothing to
/// turn on and returns having changed no CPU state.
#[inline]
pub fn enable_write_protection() {
    #[cfg(target_arch = "x86_64")]
    {
        // SAFETY: setting CR0.WP only tightens permissions, so it cannot make
        // a previously legal access fault in a way the kernel does not expect.
        unsafe {
            core::arch::asm!(
                "mov {tmp}, cr0",
                "or {tmp:e}, 0x10000",
                "mov cr0, {tmp}",
                tmp = out(reg) _,
                options(nostack, preserves_flags)
            );
        }
    }
}

/// Lift read-only enforcement for kernel writes.
///
/// # Safety
///
/// The caller must restore enforcement before returning to any code that
/// assumes read-only kernel mappings hold.
///
/// On aarch64 there is no such override. The call leaves the tables as they
/// are, so a write to a read-only mapping still faults: a loud fault rather
/// than a silent corruption, and the signal to give the caller a writable
/// alias instead.
#[inline]
pub unsafe fn disable_write_protection() {
    #[cfg(target_arch = "x86_64")]
    {
        // SAFETY: the caller carries the obligation documented above.
        unsafe {
            core::arch::asm!(
                "mov {tmp}, cr0",
                "and {tmp:e}, 0xFFFEFFFF",
                "mov cr0, {tmp}",
                tmp = out(reg) _,
                options(nostack, preserves_flags)
            );
        }
    }
}
