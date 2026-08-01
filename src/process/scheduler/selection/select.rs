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

/// Pick a process to run next and claim it, so two CPUs cannot pick the same
/// one. Picking only reads state and `Running` is not set until the arch
/// switch, so without the claim both would switch into the same control block.
/// Callers either switch to what they get back or leave it `Running`, so
/// claiming early does not strand anything.
pub fn select_next_process() -> Option<u32> {
    // Bounded: a lost claim means the state moved, so the next pick sees it.
    // The cap stops a churn spinning here with interrupts off.
    const CLAIM_ATTEMPTS: usize = 8;
    for _ in 0..CLAIM_ATTEMPTS {
        let (pid, band) = pick()?;
        if claim(pid) {
            if let Some(idx) = band {
                LAST_PER_BAND[idx].store(pid, Ordering::Relaxed);
            }
            LAST_SCHEDULED_PID.store(pid, Ordering::Relaxed);
            return Some(pid);
        }
    }
    None
}

/// Take `pid` from `Ready` to `Running` under the state lock, reporting
/// whether this caller made the transition.
fn claim(pid: u32) -> bool {
    use crate::process::nonos_core::{ProcessState, PROCESS_TABLE};
    let Some(pcb) = PROCESS_TABLE.find_by_pid(pid) else {
        return false;
    };
    let mut state = pcb.state.lock();
    if *state == ProcessState::Ready {
        *state = ProcessState::Running;
        true
    } else {
        false
    }
}

/// The candidate and, if it came from a priority band, that band's index. The
/// cursors are not advanced here: a pick that loses its claim never ran.
fn pick() -> Option<(u32, Option<usize>)> {
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
            return Some((pid, Some(idx)));
        }
    }
    select_fallback(&runnable, current).map(|pid| (pid, None))
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
            Some(current)
        } else {
            None
        }
    })
}
