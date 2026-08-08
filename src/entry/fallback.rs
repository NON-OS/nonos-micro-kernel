// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

use core::arch::asm;

pub fn vga_fallback() -> ! {
    // The last resort when there is no framebuffer to draw on: the legacy text
    // buffer at 0xB8000, which only a PC has. Somewhere else there is nothing to
    // write to and the message is skipped, but the halt below still runs, because
    // the point of this path is to stop rather than to print.
    #[cfg(target_arch = "x86_64")]
    {
        const VGA_BUFFER: *mut u8 = 0xB8000 as *mut u8;
        // SAFETY: identity-mapped legacy text memory, present on any PC that
        // reached this point, written within its 80x25 two-bytes-per-cell bound.
        unsafe {
            for i in 0..(80 * 25) {
                *VGA_BUFFER.add(i * 2) = b' ';
                *VGA_BUFFER.add(i * 2 + 1) = 0x1F;
            }
            let msg = b"NONOS v1.0.0 - No framebuffer available";
            for (i, &ch) in msg.iter().enumerate() {
                *VGA_BUFFER.add(i * 2) = ch;
            }
        }
    }
    crate::arch::halt_loop()
}
