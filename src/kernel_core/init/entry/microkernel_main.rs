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

use crate::memory::paging::manager::api::create_address_space;
use crate::process::core::{create_process, Priority, ProcessState, CURRENT_PID};
use crate::sys::{boot_log, clock};
use core::sync::atomic::Ordering;

pub fn microkernel_main() -> ! {
    crate::sys::bench::mark(b"microkernel_main_start");
    // bring-up diagnostics silenced:
    // log_acpi_touchpad_onscreen();
    // crate::hardware::broker::device_census();
    boot_log::ok("NONOS", "boot log held; starting userspace");
    let start = clock::uptime_ms();
    let mut guard: u64 = 0;
    while clock::uptime_ms().wrapping_sub(start) < 2500 {
        core::hint::spin_loop();
        guard = guard.wrapping_add(1);
        if guard > 3_000_000_000 {
            break;
        }
    }
    boot_log::ok("UKERNEL", "Creating init");
    let init_pid = match create_process("init", ProcessState::Running, Priority::High) {
        Ok(pid) => pid,
        Err(e) => {
            boot_log::error("Failed to create init process");
            crate::sys::serial::println(b"[FATAL] Init process creation failed");
            crate::sys::serial::println(e.as_bytes());
            crate::arch::halt_loop()
        }
    };
    crate::sys::bench::mark(b"init_process_created");
    if let Err(_) = create_address_space(init_pid) {
        boot_log::error("Failed to create init address space");
        crate::sys::serial::println(b"[FATAL] Init address space creation failed");
        crate::arch::halt_loop()
    }
    if crate::kernel_core::process_spawn::allocate_kernel_stack(init_pid).is_err() {
        boot_log::error("Failed to allocate init kernel stack");
        crate::sys::serial::println(b"[FATAL] Init kernel stack allocation failed");
        crate::arch::halt_loop()
    }
    CURRENT_PID.store(init_pid, Ordering::SeqCst);
    boot_log::ok("UKERNEL", "Entering userspace");
    crate::sys::bench::mark(b"init_enter_userspace");
    crate::userspace::run_init()
}
