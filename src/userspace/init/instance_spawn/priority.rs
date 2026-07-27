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

//! Keep init schedulable for exactly as long as a window spawn is queued.
//!
//! `select_next_process` scans the priority bands in order and returns the
//! first band holding a ready process, with no aging. Init drops itself to
//! `Low` once boot is done, so the drain in `init_loop` only runs when no
//! ready `Normal` process exists. One capsule that retries in a bare
//! `mk_yield` loop — a bus driver whose device is absent, and every capsule
//! stacked on it — keeps `Normal` permanently occupied, and then a clicked
//! dock icon queues a spawn that init is never scheduled to perform.
//!
//! Raising init to `Normal` while the queue is non-empty puts it back in the
//! round-robin the busy capsules share, so the drain runs within one round.
//! It returns to `Low` as soon as the queue drains, so steady-state
//! scheduling is exactly what it was before the click.

use core::sync::atomic::{AtomicU32, Ordering};

use crate::interrupts::disable_interrupts_guard;
use crate::process::core::{Priority, PROCESS_TABLE};

static INIT_PID: AtomicU32 = AtomicU32::new(0);

/// Record the pid whose loop drains the queue. Called once, from init.
pub(in crate::userspace::init) fn adopt(pid: u32) {
    INIT_PID.store(pid, Ordering::Relaxed);
}

/// Lift the drain out of the band the scheduler reaches only when nothing
/// else is ready. Called with the queue lock held, right after a push.
pub(super) fn raise() {
    set(Priority::Normal);
}

/// Hand the CPU back to the capsules. Called with the queue lock held,
/// once the queue is observed empty.
pub(super) fn restore() {
    set(Priority::Low);
}

fn set(prio: Priority) {
    let pid = INIT_PID.load(Ordering::Relaxed);
    if pid == 0 {
        return;
    }
    let _irq = disable_interrupts_guard();
    if let Some(pcb) = PROCESS_TABLE.find_by_pid(pid) {
        *pcb.priority.lock() = prio;
    }
}
