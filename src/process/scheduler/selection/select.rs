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
        static EMPTY_SHOWN: AtomicU32 = AtomicU32::new(0);
        if EMPTY_SHOWN.fetch_add(1, Ordering::Relaxed) < 2 {
            crate::sys::serial::traceln(b"[SCHED] runnable empty");
        }
        return None;
    }
    {
        static RUNNABLE_SHOWN: AtomicU32 = AtomicU32::new(0);
        if RUNNABLE_SHOWN.fetch_add(1, Ordering::Relaxed) < 2 {
            crate::sys::serial::trace(b"[SCHED] runnable count=");
            crate::sys::serial::trace_hex(runnable.len() as u64);
            crate::sys::serial::traceln(b"");
        }
    }
    let last = LAST_SCHEDULED_PID.load(Ordering::Relaxed);
    {
        use crate::process::nonos_core::PROCESS_TABLE;
        static QDUMP: AtomicU32 = AtomicU32::new(0);
        if current == 0x1F && QDUMP.fetch_add(1, Ordering::Relaxed) < 40 {
            crate::sys::serial::trace(b"[QDUMP] last=");
            crate::sys::serial::trace_hex(last as u64);
            crate::sys::serial::trace(b" rq=[");
            for &p in runnable.iter() {
                crate::sys::serial::trace_hex(p as u64);
                if let Some(pcb) = PROCESS_TABLE.find_by_pid(p) {
                    let s = *pcb.state.lock();
                    let tag: &[u8] = match s {
                        crate::process::nonos_core::ProcessState::Ready => b":R,",
                        crate::process::nonos_core::ProcessState::Running => b":U,",
                        crate::process::nonos_core::ProcessState::Sleeping => b":S,",
                        _ => b":?,",
                    };
                    crate::sys::serial::trace(tag);
                } else {
                    crate::sys::serial::trace(b":?,");
                }
            }
            crate::sys::serial::traceln(b"]");
        }
    }
    for prio in
        [Priority::RealTime, Priority::High, Priority::Normal, Priority::Low, Priority::Idle]
    {
        if let Some(pid) = select_by_priority(&runnable, last, current, prio) {
            LAST_SCHEDULED_PID.store(pid, Ordering::Relaxed);
            {
                static PICKED_SHOWN: AtomicU32 = AtomicU32::new(0);
                if PICKED_SHOWN.fetch_add(1, Ordering::Relaxed) < 4000 {
                    crate::sys::serial::trace(b"[SCHED] picked pid=");
                    crate::sys::serial::trace_hex(pid as u64);
                    crate::sys::serial::traceln(b"");
                }
            }
            return Some(pid);
        }
    }
    select_fallback(&runnable, current)
}

fn select_by_priority(pids: &[u32], last: u32, current: u32, prio: Priority) -> Option<u32> {
    use crate::process::nonos_core::{ProcessState, PROCESS_TABLE};
    let start = pids.iter().position(|&p| p == last).map(|i| i + 1).unwrap_or(0);
    for &pid in pids[start..].iter().chain(pids[..start].iter()) {
        if pid == current {
            continue;
        }
        if let Some(pcb) = PROCESS_TABLE.find_by_pid(pid) {
            let state = *pcb.state.lock();
            let proc_prio = *pcb.priority.lock();
            if (pid == 0x17 || pid == 9) && prio == Priority::Normal {
                static SHOWN_TGT: AtomicU32 = AtomicU32::new(0);
                if SHOWN_TGT.fetch_add(1, Ordering::Relaxed) < 60 {
                    let state_tag: &[u8] = match state {
                        ProcessState::New => b"new",
                        ProcessState::Ready => b"ready",
                        ProcessState::Running => b"running",
                        ProcessState::Sleeping => b"sleeping",
                        ProcessState::Stopped => b"stopped",
                        ProcessState::Zombie(_) => b"zombie",
                        ProcessState::Terminated(_) => b"terminated",
                    };
                    let prio_match: &[u8] = if proc_prio == prio { b"prio=ok" } else { b"prio=mismatch" };
                    crate::sys::serial::trace(b"[SEL] pid=");
                    crate::sys::serial::trace_hex(pid as u64);
                    crate::sys::serial::trace(b" state=");
                    crate::sys::serial::trace(state_tag);
                    crate::sys::serial::trace(b" ");
                    crate::sys::serial::trace(prio_match);
                    crate::sys::serial::trace(b" current=");
                    crate::sys::serial::trace_hex(current as u64);
                    crate::sys::serial::traceln(b"");
                }
            }
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
