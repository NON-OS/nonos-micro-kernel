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

use core::alloc::Layout;

fn serial_byte(b: u8) {
    unsafe {
        core::arch::asm!("out dx, al", in("dx") 0x3F8u16, in("al") b, options(nomem, nostack));
    }
}

fn serial_str(s: &[u8]) {
    for &b in s {
        serial_byte(b);
    }
}

fn serial_num(mut n: usize) {
    if n == 0 {
        serial_byte(b'0');
        return;
    }
    let mut buf = [0u8; 20];
    let mut i = 0;
    while n > 0 {
        buf[i] = b'0' + (n % 10) as u8;
        n /= 10;
        i += 1;
    }
    while i > 0 {
        i -= 1;
        serial_byte(buf[i]);
    }
}

fn show_vga_error() {
    unsafe {
        let vga_base = 0xb8000 as *mut u16;
        let msg = b"OOM: Memory allocation failed - system halted";
        let attr: u16 = 0x4F00;
        for (i, &ch) in msg.iter().enumerate() {
            core::ptr::write_volatile(vga_base.add(i), (ch as u16) | attr);
        }
    }
}

pub fn handle_oom(layout: Layout) -> ! {
    serial_str(b"\r\n[OOM] ALLOCATION FAILED\r\n[OOM] Requested size: ");
    serial_num(layout.size());
    serial_str(b" bytes, align: ");
    serial_num(layout.align());
    serial_str(b"\r\n");
    crate::syscall::microkernel::memory::dump_mmap_accounting();
    crate::kernel_core::surface_registry::dump_surface_accounting();
    serial_str(b"\r\n[OOM] System halted\r\n");
    show_vga_error();
    loop {
        unsafe {
            core::arch::asm!("hlt", options(nomem, nostack, preserves_flags));
        }
    }
}
