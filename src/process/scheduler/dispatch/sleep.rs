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
use alloc::collections::BTreeMap;
use core::sync::atomic::Ordering;

static SLEEPING_PROCESSES: spin::RwLock<BTreeMap<u32, u64>> = spin::RwLock::new(BTreeMap::new());

pub fn sleep_until(pid: u32, wake_time_ms: u64) {
    use crate::process::nonos_core::{ProcessState, PROCESS_TABLE};
    SLEEPING_PROCESSES.write().insert(pid, wake_time_ms);
    if let Some(pcb) = PROCESS_TABLE.find_by_pid(pid) {
        *pcb.state.lock() = ProcessState::Sleeping;
    }
    remove_from_run_queue(pid);
    crate::log_debug!("Process {} sleeping until {} ms", pid, wake_time_ms);
}

pub fn wake_process(pid: u32) {
    use crate::process::nonos_core::{ProcessState, PROCESS_TABLE};
    SLEEPING_PROCESSES.write().remove(&pid);
    let mut woke = false;
    if let Some(pcb) = PROCESS_TABLE.find_by_pid(pid) {
        let mut state = pcb.state.lock();
        if *state == ProcessState::Sleeping {
            *state = ProcessState::Ready;
            woke = true;
        }
    }
    if woke {
        add_to_run_queue(pid);
        SCHEDULER_STATS.wakeups.fetch_add(1, Ordering::Relaxed);
        crate::log_debug!("Process {} woken up", pid);
    }
}

pub fn is_sleeping(pid: u32) -> bool {
    SLEEPING_PROCESSES.read().contains_key(&pid)
}

pub fn get_remaining_sleep(pid: u32) -> Option<u64> {
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
        wake_process(pid);
    }
}
