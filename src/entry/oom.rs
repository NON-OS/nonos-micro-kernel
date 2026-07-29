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

//! What happens when the heap is exhausted.
//!
//! Nothing here may allocate. The console sink and its decimal printer both
//! work without the allocator, which is why the report goes through them
//! rather than the formatter.

use core::alloc::Layout;

use crate::sys::serial;

/// Paint the message into VGA text memory too, so a machine with no serial
/// cable still shows why it stopped. No counterpart elsewhere: text mode is a
/// PC device, and the boards this kernel otherwise targets have no display at
/// all this early.
#[cfg(target_arch = "x86_64")]
fn show_vga_error() {
    const VGA_BASE: *mut u16 = 0xb8000 as *mut u16;
    const WHITE_ON_RED: u16 = 0x4F00;
    let msg = b"OOM: Memory allocation failed - system halted";
    // SAFETY: the VGA text buffer is a fixed 80x25 window and the message is
    // far shorter than one line. Nothing else is writing it by the time the
    // machine reaches here.
    unsafe {
        for (i, &ch) in msg.iter().enumerate() {
            core::ptr::write_volatile(VGA_BASE.add(i), (ch as u16) | WHITE_ON_RED);
        }
    }
}

pub fn handle_oom(layout: Layout) -> ! {
    serial::println(b"");
    serial::println(b"[OOM] ALLOCATION FAILED");
    serial::print(b"[OOM] Requested size: ");
    serial::print_dec(layout.size() as u64);
    serial::print(b" bytes, align: ");
    serial::print_dec(layout.align() as u64);
    serial::println(b"");

    crate::syscall::microkernel::memory::dump_mmap_accounting();
    crate::kernel_core::surface_registry::dump_surface_accounting();
    serial::println(b"[OOM] System halted");

    #[cfg(target_arch = "x86_64")]
    show_vga_error();

    crate::arch::halt_loop()
}
