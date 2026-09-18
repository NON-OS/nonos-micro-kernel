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

//! Taking a new header in: rates need a previous one and hold at zero
//! until it exists.

use nonos_libc::ProcStatHeader;

use super::system::System;

/// `delta` events over `ms` milliseconds as a per-second rate.
pub fn per_second(delta: u64, ms: u64) -> u32 {
    if ms == 0 {
        return 0;
    }
    delta.saturating_mul(1000).checked_div(ms).unwrap_or(0).min(u32::MAX as u64) as u32
}

/// `part` of `whole` as a whole percentage, zero of nothing being zero.
fn pct(part: u64, whole: u64) -> u8 {
    if whole == 0 {
        0
    } else {
        (part.min(whole) * 100 / whole) as u8
    }
}

impl System {
    pub fn update(&mut self, h: &ProcStatHeader) {
        self.cpus = h.cpus_online.max(1);
        self.mem_total_kb = h.mem_total_kb;
        self.mem_free_kb = h.mem_free_kb.min(h.mem_total_kb);
        self.largest_free_kb = h.largest_free_kb;
        self.heap_used_kb = h.heap_used_kb;
        self.heap_total_kb = h.heap_total_kb;
        self.heap_peak_kb = h.heap_peak_kb;
        self.heap_allocs = h.heap_allocs;
        self.uptime_ms = h.uptime_ms;
        self.load = h.load_avg_fixed;
        if let Some(p) = self.prev {
            let ms = h.uptime_ms.saturating_sub(p.uptime_ms);
            let dt = h.total_ticks.saturating_sub(p.total_ticks);
            if dt > 0 {
                self.idle_pct = pct(h.idle_ticks.saturating_sub(p.idle_ticks), dt);
                self.user_pct = pct(h.user_ticks.saturating_sub(p.user_ticks), dt);
                self.kernel_pct = pct(h.kernel_ticks.saturating_sub(p.kernel_ticks), dt);
                self.busy_pct = 100u8.saturating_sub(self.idle_pct);
            }
            self.ctx_ps = per_second(h.context_switches.saturating_sub(p.context_switches), ms);
            self.sysc_ps = per_second(h.syscalls.saturating_sub(p.syscalls), ms);
            self.ipc_ps = per_second(h.ipc_messages.saturating_sub(p.ipc_messages), ms);
            self.irq_ps = per_second(h.interrupts.saturating_sub(p.interrupts), ms);
            self.fault_ps = per_second(h.faults.saturating_sub(p.faults), ms);
        }
        self.prev = Some(*h);
    }
}
