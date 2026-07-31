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

//! Where the firmware lands.
//!
//! Boot loaders on this arch pass the device tree in x0, so that is the only
//! argument. Everything the kernel needs to know about the board comes out of
//! it: RAM, console, interrupt controller, timer and CPU count.
//!
//! From `microkernel_init` onward the path is the same code x86_64 runs. The
//! two arches differ in how they answer the questions in `KernelHandoff`, not
//! in what happens afterwards.

use super::info::BootInfo;
use crate::boot::handoff::KernelHandoff;
use crate::sys::serial;

#[no_mangle]
pub extern "C" fn kernel_entry(dtb_ptr: u64) -> ! {
    let mut info = BootInfo::default();

    // FP and SIMD before any compiled Rust runs. The compiler uses the vector
    // registers for ordinary things like a struct copy, and reaching one with
    // CPACR_EL1.FPEN clear traps, which at this point means a fault with no
    // vectors installed to report it.
    crate::arch::aarch64::cpu::init_cpu();

    // Console next, on the default base, so everything after can report its own
    // failure. The device tree may name a different UART, in which case `init`
    // reopens the console on that one; until then the default is all there is,
    // and a wrong guess costs less than a silent boot.
    crate::arch::aarch64::uart::init_uart(info.uart_base);
    serial::print(b"[NONOS] dtb at ");
    serial::print_hex(dtb_ptr);
    serial::println(b"");

let parsed = super::dtb_adapter::populate(dtb_ptr, &mut info);

    // Brings up the console, puts the MMU and caches into a known state, then
    // installs the vector table, the GIC and the timer.
    super::init(&info);

    if parsed {
        serial::println(b"[NONOS] aarch64 boot init done");
    } else {
        // The defaults left in place describe the QEMU virt board. Say so:
        // on any other board they are wrong, and a silent boot that wanders
        // into the wrong MMIO is far worse than a loud one.
        serial::println(b"[NONOS] no usable device tree, assuming QEMU virt");
    }

    // Arm the bootstrap heap before any shared code runs. It hands out of a
    // static region, so it needs nothing but the MMU that init brought up.
    // x86_64 does this inside its own early setup, which this path does not
    // share; without it the first allocation is the frame allocator's bitmap
    // and it fails with the heap reporting no memory at all.
    crate::memory::heap::manager::init_bootstrap();

    let handoff = KernelHandoff::from_aarch64(&info);
    crate::kernel_core::microkernel_init(&handoff);
    crate::kernel_core::microkernel_main()
}
