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

use super::state::SCHEDULER_STATS;
use crate::arch::x86_64::idt::without_interrupts;
use crate::process::scheduler::contract::{switch as contract_switch, SwitchIntent};
use core::sync::atomic::{AtomicU32, Ordering};

static YIELD_CALL_TRACE_COUNT: AtomicU32 = AtomicU32::new(0);

fn trace(label: &[u8], pid: u32) {
    if !matches!(pid, 7 | 8 | 0x1c | 0x27)
        || YIELD_CALL_TRACE_COUNT.fetch_add(1, Ordering::Relaxed) >= 32
    {
        return;
    }
    crate::sys::serial::print(b"[YCALL] ");
    crate::sys::serial::println(label);
}

pub fn yield_now() {
    SCHEDULER_STATS.voluntary_yields.fetch_add(1, Ordering::Relaxed);
    let pid = crate::process::current_pid().unwrap_or(0);
    without_interrupts(|| {
        if contract_switch(SwitchIntent::Yield).is_err() {
            return;
        }
    });
    trace(b"returned", pid);
}
