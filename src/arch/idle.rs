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

//! Waiting when there is nothing to run.
//!
//! Two things have to hold however the CPU idles. Interrupts must be open
//! across the wait, or the wake that ends it can never arrive; and the window
//! between opening them and entering the wait must not be able to swallow a
//! wake that was already pending. Both architectures give exactly one
//! instruction pairing that closes that race, and this is where the kernel
//! asks for it rather than writing either one out.
//!
//! Returns with interrupts closed again, the state the caller was in.

/// Open interrupts, wait for one, close them again.
#[inline]
pub(crate) fn wait_for_interrupt() {
    #[cfg(target_arch = "x86_64")]
    {
        crate::arch::x86_64::idle::wait_for_interrupt();
    }
    #[cfg(target_arch = "aarch64")]
    {
        // SAFETY: `wfi` is a hint and architecturally a no-op if a wake is
        // already pending, so the wake cannot be lost between unmasking and
        // waiting the way it can on a CPU that needs a shadow. The generic
        // timer counts through `wfi` by definition, so unlike x86_64 there is
        // no part on which this stops the tick.
        unsafe {
            core::arch::asm!("msr daifclr, #2", "wfi", "msr daifset, #2", options(nomem, nostack));
        }
    }
    #[cfg(not(any(target_arch = "x86_64", target_arch = "aarch64")))]
    {
        core::hint::spin_loop();
    }
}
