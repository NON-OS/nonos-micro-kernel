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

use super::super::preemption::SCHEDULER_STATS;
use super::run_queue::{add_to_run_queue, remove_from_run_queue};
use crate::interrupts::disable_interrupts_guard;
use alloc::collections::BTreeMap;
use core::sync::atomic::Ordering;

// The timer tick sweeps this table (check_sleeping_processes) to wake expired
// sleepers, so it is reached from interrupt context. Every normal-path access
// holds interrupts off while it has the lock, or a tick landing mid-update would
// spin on a lock the interrupted code cannot release. The sweep itself already
// runs with interrupts off, so its guards are simply no-ops.
static SLEEPING_PROCESSES: spin::RwLock<BTreeMap<u32, u64>> = spin::RwLock::new(BTreeMap::new());

pub fn sleep_until(pid: u32, wake_time_ms: u64) {
    use crate::process::nonos_core::{ProcessState, PROCESS_TABLE};
    let _irq = disable_interrupts_guard();
    SLEEPING_PROCESSES.write().insert(pid, wake_time_ms);
    if let Some(pcb) = PROCESS_TABLE.find_by_pid(pid) {
        *pcb.state.lock() = ProcessState::Sleeping;
    }
    remove_from_run_queue(pid);
}

pub fn wake_process(pid: u32) {
    use crate::process::nonos_core::{ProcessState, PROCESS_TABLE};
    let _irq = disable_interrupts_guard();
    let mut woke = false;
    if let Some(pcb) = PROCESS_TABLE.find_by_pid(pid) {
        let mut state = pcb.state.lock();
        if *state == ProcessState::Sleeping {
            *state = ProcessState::Ready;
            woke = true;
        }
    }
    // Only a wake that actually transitioned the process may strip its sleep
    // deadline: a wake landing on a Running/Ready target must not destroy the
    // timeout of a sleep the target is about to enter (or re-enter), or that
    // sleep becomes unwakeable by the tick sweep.
    if woke {
        SLEEPING_PROCESSES.write().remove(&pid);
        add_to_run_queue(pid);
        SCHEDULER_STATS.wakeups.fetch_add(1, Ordering::Relaxed);
    }
}

pub fn is_sleeping(pid: u32) -> bool {
    let _irq = disable_interrupts_guard();
    SLEEPING_PROCESSES.read().contains_key(&pid)
}

pub fn get_remaining_sleep(pid: u32) -> Option<u64> {
    let _irq = disable_interrupts_guard();
    let sleeping = SLEEPING_PROCESSES.read();
    if let Some(&wake_time) = sleeping.get(&pid) {
        let now = crate::time::timestamp_millis();
        if wake_time > now {
            Some(wake_time - now)
        } else {
            Some(0)
        }
    } else {
        None
    }
}

pub fn check_sleeping_processes() {
    let _irq = disable_interrupts_guard();
    let current_time_ms = crate::time::timestamp_millis();
    let mut pids_to_wake = [0u32; 64];
    let mut count = 0usize;
    {
        let sleeping = SLEEPING_PROCESSES.read();
        for (&pid, &wt) in sleeping.iter() {
            if current_time_ms >= wt {
                if count < pids_to_wake.len() {
                    pids_to_wake[count] = pid;
                    count += 1;
                }
            }
        }
    }
    for &pid in &pids_to_wake[..count] {
        // The deadline has passed, so the entry is spent regardless of
        // whether the wake transitions the process (it may already be
        // Running via an early-return path); leaving a stale entry behind
        // would make this sweep re-chew it every tick until the 64-slot
        // budget is exhausted.
        SLEEPING_PROCESSES.write().remove(&pid);
        wake_process(pid);
    }
}
