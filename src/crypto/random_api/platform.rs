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

//! Machine state sampled for entropy.
//!
//! None of these is a random number. They are quantities that differ between
//! two runs of the same code on the same machine, which is what the pool wants
//! to stir in alongside a real generator.

/// The CPU's free-running cycle counter.
#[inline]
pub(super) fn read_cycle_counter() -> u64 {
    crate::arch::read_time_counter()
}

/// The calling frame's stack pointer.
#[inline]
pub(super) fn read_stack_pointer() -> u64 {
    crate::arch::stack_pointer()
}

/// A second clock, read independently of the cycle counter.
///
/// The point is to sample something whose phase relative to the cycle counter
/// is not fixed, so the difference between two readings carries jitter the
/// counter alone would not. On a PC that is the 8254's channel-0 counter,
/// running at its own 1.19 MHz off a separate crystal. Elsewhere there is no
/// second free-running clock the kernel can latch this cheaply, so this
/// returns the low half of the same counter: still varying, but correlated,
/// and callers must not count it as independent entropy.
#[inline]
pub(super) fn read_second_clock() -> u16 {
    #[cfg(target_arch = "x86_64")]
    {
        const PIT_CHANNEL0: u16 = 0x40;
        const PIT_COMMAND: u16 = 0x43;
        const LATCH_CHANNEL0: u8 = 0x00;

        // SAFETY: the 8254 is not claimed by any driver in this kernel. The
        // latch command freezes channel 0's count for the two reads that
        // follow and changes no timer configuration.
        unsafe {
            crate::sys::io::outb(PIT_COMMAND, LATCH_CHANNEL0);
            let low = crate::sys::io::inb(PIT_CHANNEL0);
            let high = crate::sys::io::inb(PIT_CHANNEL0);
            return ((high as u16) << 8) | (low as u16);
        }
    }
    #[cfg(not(target_arch = "x86_64"))]
    {
        return (crate::arch::read_time_counter() & 0xFFFF) as u16;
    }
}
