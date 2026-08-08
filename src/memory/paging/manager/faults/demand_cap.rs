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

// Per-process demand-fault budget. Stacks, heap, and code are mapped eagerly,
// so a healthy capsule demand-faults essentially nothing; sustained demand
// faulting means a stray access or a runaway write loop. Capping the pages a
// single process can demand-back turns a runaway capsule into a kill instead
// of a system-wide OOM (the failure that forced the wallet capsule to be held
// back) and bounds a deliberate frame-exhaustion attempt.

use spin::Mutex;

const MAX_TRACKED: usize = 128;
const MAX_DEMAND_PAGES: u64 = 4096; // 16 MiB of demand-backed pages per process

#[derive(Clone, Copy)]
struct Counter {
    pid: u32,
    pages: u64,
}

struct Budget {
    counters: [Counter; MAX_TRACKED],
    // Pages admitted for processes that found no free slot. Bounded by the
    // same per-process budget, so the saturated case is metered rather than
    // open, and reset when a slot frees because the pressure that caused it
    // is gone.
    overflow: u64,
}

static BUDGET: Mutex<Budget> =
    Mutex::new(Budget { counters: [Counter { pid: 0, pages: 0 }; MAX_TRACKED], overflow: 0 });

// Charge one demand-faulted page to `pid`. Returns false once the process is
// over budget so the fault handler can refuse and let the process be killed.
pub(super) fn charge(pid: u32) -> bool {
    if pid == 0 {
        return true;
    }
    let budget = &mut *BUDGET.lock();
    let table = &mut budget.counters;
    if let Some(c) = table.iter_mut().find(|c| c.pid == pid) {
        if c.pages >= MAX_DEMAND_PAGES {
            // Budget exhausted: the fault is refused (the caller kills the
            // process). Report it once so a runaway capsule is visible on the
            // serial log instead of silently exhausting memory.
            if c.pages == MAX_DEMAND_PAGES {
                c.pages = c.pages.saturating_add(1);
                crate::sys::serial::print(
                    b"[DEMAND-CAP] per-process page budget hit, killing pid=",
                );
                crate::sys::serial::print_hex(pid as u64);
                crate::sys::serial::println(b"");
            }
            return false;
        }
        c.pages = c.pages.saturating_add(1);
        return true;
    }
    if let Some(c) = table.iter_mut().find(|c| c.pid == 0 || !alive(c.pid)) {
        *c = Counter { pid, pages: 1 };
        budget.overflow = 0;
        return true;
    }
    // Table saturated with live faulting processes. Admitting freely here left
    // the cap with an unmetered path around it: 128 processes that fault once
    // each pin every slot, and the next process demand-backs memory with no
    // budget at all. Charge a shared overflow budget instead, so an untracked
    // process is bounded by one process's worth of pages and then refused.
    if budget.overflow > MAX_DEMAND_PAGES {
        return false;
    }
    if budget.overflow == MAX_DEMAND_PAGES {
        budget.overflow = budget.overflow.saturating_add(1);
        crate::sys::serial::print(
            b"[DEMAND-CAP] tracking table saturated, overflow budget spent, killing pid=",
        );
        crate::sys::serial::print_hex(pid as u64);
        crate::sys::serial::println(b"");
        return false;
    }
    budget.overflow = budget.overflow.saturating_add(1);
    true
}

fn alive(pid: u32) -> bool {
    crate::process::get_process_table().find_by_pid(pid).is_some()
}
