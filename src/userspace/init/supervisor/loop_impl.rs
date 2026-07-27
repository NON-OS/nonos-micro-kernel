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

//! Init's residual loop after every capsule has been spawned. Walks
//! the lifecycle registry once per second; any capsule that exited is
//! observed `Dead` on its next IPC. The kernel does not actively
//! probe capsules — liveness arrives through the existing process
//! state machine.

const TICK_INTERVAL_MS: u64 = 1000;
const PARK_SLICE_MS: u64 = 20;

pub(crate) fn init_loop() -> ! {
    let mut last_tick = 0u64;
    #[cfg(feature = "microkernel-setup-wizard")]
    let mut desktop_started = false;
    loop {
        let now = crate::time::timestamp_millis();
        if now >= last_tick + TICK_INTERVAL_MS {
            crate::services::lifecycle::tick();
            last_tick = now;
        }
        #[cfg(feature = "microkernel-setup-wizard")]
        if !desktop_started && !crate::userspace::capsule_setup_wizard::shared_state().is_alive() {
            super::super::spawn_plan::spawn_post_wizard();
            desktop_started = true;
        }
        // Perform any window-instance spawns the shell requested. Running
        // them here, in init's context, keeps the heavy spawn out of the
        // calling capsule's syscall, which is what stopped the caller from
        // resuming (it faulted on its own code under the wrong page tables).
        crate::userspace::init::service_instance_spawns();
        park();
    }
}

// A bare yield left init permanently runnable, so `select_next_process`
// never came up empty and the scheduler's `sti; hlt` idle path was
// unreachable: the vCPU spun at full load with an idle desktop. Sleeping
// on a short deadline takes init off the run queue between passes, which
// lets the CPU actually halt, while still draining the shell's window
// spawn requests inside one compositor frame. Falling back to the yield
// keeps the loop live if init runs before its pid is current.
fn park() {
    let Some(pid) = crate::process::current_pid() else {
        crate::sched::yield_now();
        return;
    };
    let wake = crate::time::timestamp_millis().saturating_add(PARK_SLICE_MS);
    crate::sched::sleep_until(pid, wake);
    crate::sched::yield_now();
}
