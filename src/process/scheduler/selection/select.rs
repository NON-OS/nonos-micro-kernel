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

static LAST_PER_BAND: [AtomicU32; 5] =
    [AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0)];

pub fn select_next_process() -> Option<u32> {
    use crate::process::nonos_core::CURRENT_PID;
    let current = CURRENT_PID.load(Ordering::Relaxed);
    let runnable = get_runnable_pids();
    if runnable.is_empty() {
        return None;
    }
    for (idx, prio) in
        [Priority::RealTime, Priority::High, Priority::Normal, Priority::Low, Priority::Idle]
            .into_iter()
            .enumerate()
    {
        let band_last = LAST_PER_BAND[idx].load(Ordering::Relaxed);
        if let Some(pid) = select_by_priority(&runnable, band_last, current, prio) {
            LAST_PER_BAND[idx].store(pid, Ordering::Relaxed);
            LAST_SCHEDULED_PID.store(pid, Ordering::Relaxed);
            return Some(pid);
        }
    }
    select_fallback(&runnable, current)
}

fn select_by_priority(pids: &[u32], last: u32, current: u32, prio: Priority) -> Option<u32> {
    use crate::process::nonos_core::{ProcessState, PROCESS_TABLE};
    let mut after: Option<u32> = None;
    let mut lowest: Option<u32> = None;
    for &pid in pids.iter() {
        if pid == current {
            continue;
        }
        let Some(pcb) = PROCESS_TABLE.find_by_pid(pid) else {
            continue;
        };
        if *pcb.state.lock() != ProcessState::Ready || *pcb.priority.lock() != prio {
            continue;
        }
        lowest = Some(lowest.map_or(pid, |m| core::cmp::min(m, pid)));
        if pid > last {
            after = Some(after.map_or(pid, |m| core::cmp::min(m, pid)));
        }
    }
    after.or(lowest)
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
