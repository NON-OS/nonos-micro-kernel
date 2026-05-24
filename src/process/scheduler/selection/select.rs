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

use super::super::dispatch::get_runnable_pids;
use crate::process::nonos_core::Priority;
use core::sync::atomic::{AtomicU32, Ordering};

pub static LAST_SCHEDULED_PID: AtomicU32 = AtomicU32::new(0);

pub fn select_next_process() -> Option<u32> {
    use crate::process::nonos_core::CURRENT_PID;
    let current = CURRENT_PID.load(Ordering::Relaxed);
    let runnable = get_runnable_pids();
    if runnable.is_empty() {
        return None;
    }
    let last = LAST_SCHEDULED_PID.load(Ordering::Relaxed);
    for prio in
        [Priority::RealTime, Priority::High, Priority::Normal, Priority::Low, Priority::Idle]
    {
        if let Some(pid) = select_by_priority(&runnable, last, current, prio) {
            LAST_SCHEDULED_PID.store(pid, Ordering::Relaxed);
            return Some(pid);
        }
    }
    select_fallback(&runnable, current)
}

fn select_by_priority(pids: &[u32], last: u32, current: u32, prio: Priority) -> Option<u32> {
    use crate::process::nonos_core::{ProcessState, PROCESS_TABLE};
    let start = pids.iter().position(|&p| p > last).unwrap_or(0);
    for &pid in pids[start..].iter().chain(pids[..start].iter()) {
        if pid == current {
            continue;
        }
        if let Some(pcb) = PROCESS_TABLE.find_by_pid(pid) {
            let state = *pcb.state.lock();
            let proc_prio = *pcb.priority.lock();
            if state == ProcessState::Ready && proc_prio == prio {
                return Some(pid);
            }
        }
    }
    None
}

fn select_fallback(pids: &[u32], current: u32) -> Option<u32> {
    use crate::process::nonos_core::{ProcessState, PROCESS_TABLE};
    if !pids.contains(&current) {
        return None;
    }
    PROCESS_TABLE.find_by_pid(current).and_then(|pcb| {
        if *pcb.state.lock() == ProcessState::Ready {
            LAST_SCHEDULED_PID.store(current, Ordering::Relaxed);
            Some(current)
        } else {
            None
        }
    })
}
