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

use crate::process::core::Priority;

const TICK_INTERVAL_MS: u64 = 1000;

pub(crate) fn init_loop() -> ! {
    let mut last_tick = 0u64;
    #[cfg(feature = "microkernel-setup-wizard")]
    let mut desktop_started = false;
    let mut boosted = false;
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
        // Init runs at Priority::Low so an idle system spends its cycles on the
        // apps, but the window-instance drain below (and the focus-frame
        // delivery inside it) must not be starved: a busy-yielding app with a
        // network fetch in flight would otherwise keep a low-priority init off
        // the single CPU, so a dock click never opened its second window. Raise
        // to Normal while there is queued window work and drop back to Low when
        // idle, so the drain runs promptly without making an idle init costly.
        let want = crate::userspace::init::instance_spawns_pending();
        if want != boosted {
            set_init_priority(if want { Priority::Normal } else { Priority::Low });
            boosted = want;
        }
        // Perform any window-instance spawns the shell requested. Running
        // them here, in init's context, keeps the heavy spawn out of the
        // calling capsule's syscall, which is what stopped the caller from
        // resuming (it faulted on its own code under the wrong page tables).
        crate::userspace::init::service_instance_spawns();
        crate::sched::yield_now();
    }
}

// Set init's own scheduling priority. Mirrors `lower_init_priority` in entry.rs;
// used to lift the drain out of starvation while there is a window to open, then
// return to Low when the queue is empty.
fn set_init_priority(p: Priority) {
    use crate::process::core::{CURRENT_PID, PROCESS_TABLE};
    use core::sync::atomic::Ordering;
    let pid = CURRENT_PID.load(Ordering::Relaxed);
    if let Some(pcb) = PROCESS_TABLE.find_by_pid(pid) {
        *pcb.priority.lock() = p;
    }
}
