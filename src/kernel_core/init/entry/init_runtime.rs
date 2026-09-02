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

//! Device interrupt routing and the process runtime, in that order: a capsule
//! that starts before its device's interrupts are routed waits on a line
//! nobody is listening to.

use crate::sys::boot_log;

pub(super) fn init_device_routing() {
    // Latch the boot CPU's controller id before any redirection entry is
    // programmed, so device interrupts route to the CPU that actually exists
    // rather than to id 0, which is only correct under QEMU. This writes the
    // id cache and nothing else: controller mode, base and init state are
    // untouched, so the timer and IPI paths are unaffected.
    crate::arch::interrupt_controller::cache_boot_cpu_id();
    // Named for the job, not the part: x86_64 does this with an IO-APIC and
    // aarch64 with the GIC distributor, which is already up by the time this
    // runs.
    match crate::arch::init_broker_irq_routing() {
        Ok(_) => boot_log::ok("NONOS", "device interrupt routing ready"),
        Err(_) => crate::sys::serial::println(b"[NONOS] device interrupt routing failed"),
    }
}

pub(super) fn init_process_runtime() {
    crate::process::init_process_management();
    crate::elf::loader::init_elf_loader();
    crate::crypto::kernel_keys::init();
    crate::sys::bench::mark(b"process_runtime_ready");
}
