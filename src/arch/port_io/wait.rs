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

/// Settle for about a bus cycle between register writes.
///
/// The PC trick is a write to the unused POST port 0x80, which the ISA bus
/// takes roughly a microsecond to swallow. No such port exists elsewhere, so
/// this becomes a barrier: the ordering the caller actually wants, without
/// claiming a delay it cannot give.
#[inline]
pub fn io_wait() {
    #[cfg(target_arch = "x86_64")]
    // SAFETY: nothing in this kernel claims port 0x80; the write only burns a
    // bus cycle.
    unsafe {
        super::outb(0x80, 0);
    }
    #[cfg(not(target_arch = "x86_64"))]
    core::sync::atomic::fence(core::sync::atomic::Ordering::SeqCst);
}
