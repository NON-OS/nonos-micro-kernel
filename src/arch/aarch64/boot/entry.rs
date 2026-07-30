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

    let handoff = KernelHandoff::from_aarch64(&info);
    crate::kernel_core::microkernel_init(&handoff);
    crate::kernel_core::microkernel_main()
}
