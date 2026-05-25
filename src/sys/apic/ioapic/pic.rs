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

use crate::sys::io::outb;
use crate::sys::serial;

// 8259A remap-and-mask sequence. Both PICs are programmed to vectors
// 0x20/0x28 so a stray legacy IRQ does not land in a CPU exception
// vector, then every IRQ line is masked. After this the IOAPIC is the
// only delivery path.
pub(super) fn disable_pic() {
    serial::println(b"[APIC] Disabling legacy PIC (8259A)...");

    unsafe {
        // ICW1: start init, ICW4 needed.
        outb(0x20, 0x11);
        outb(0xA0, 0x11);

        // ICW2: vector base (0x20 master, 0x28 slave).
        outb(0x21, 0x20);
        outb(0xA1, 0x28);

        // ICW3: cascade wiring (slave on IR2).
        outb(0x21, 0x04);
        outb(0xA1, 0x02);

        // ICW4: 8086 mode.
        outb(0x21, 0x01);
        outb(0xA1, 0x01);

        // OCW1: mask every line.
        outb(0x21, 0xFF);
        outb(0xA1, 0xFF);
    }

    serial::println(b"[APIC] Legacy PIC disabled");
}
