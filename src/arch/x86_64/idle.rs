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

/// How many spins to burn when halting would stop the clock. Long enough that
/// the loop is not a busy spin on the scheduler's behalf, short enough that a
/// wake still lands promptly.
const SPIN_ROUNDS: u32 = 4096;

/// `sti; hlt; cli` is the race-free idle: STI's one-instruction shadow means a
/// wake already pending still lands inside the HLT rather than in the gap
/// before it.
///
/// Halting is conditional because it is not always survivable. On a laptop in
/// C1E the LAPIC timer clock gates off during `hlt` and the tick never comes
/// back, so the CPU sleeps forever waiting for the interrupt that would have
/// woken it. Where the timer is not known to count through a halt, the wait
/// becomes a bounded spin with interrupts open, which is wasteful but keeps
/// time moving.
#[inline]
pub(crate) fn wait_for_interrupt() {
    if crate::arch::x86_64::interrupt::apic::idle_timer::halt_safe() {
        // SAFETY: sti/hlt/cli touch no memory and the STI shadow closes the
        // window between unmasking and halting.
        unsafe {
            core::arch::asm!("sti", "hlt", "cli", options(nomem, nostack));
        }
        return;
    }

    // SAFETY: unmasking so the pending wake can land, then remasking below.
    unsafe {
        core::arch::asm!("sti", options(nomem, nostack));
    }
    for _ in 0..SPIN_ROUNDS {
        core::hint::spin_loop();
    }
    // SAFETY: restores the masked state the caller was in.
    unsafe {
        core::arch::asm!("cli", options(nomem, nostack));
    }
}
