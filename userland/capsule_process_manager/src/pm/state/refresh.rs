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

use alloc::vec;
use alloc::vec::Vec;

use nonos_libc::{mk_proc_stat, ProcStatEntry, ProcStatHeader};

use super::types::{ENTRY_LEN, HEADER_LEN, MAX_PROCS};
use super::{Row, Sort, State};

impl State {
    // Read the whole live process table in one syscall and rebuild the rows.
    // Every field is the kernel's own truth; cpu percent is this process's tick
    // delta over the total tick delta since the last read.
    pub fn refresh(&mut self) {
        self.refreshes = self.refreshes.wrapping_add(1);
        let mut buf = vec![0u8; HEADER_LEN + MAX_PROCS * ENTRY_LEN];
        let written = mk_proc_stat(buf.as_mut_ptr(), MAX_PROCS as u32);
        if written <= 0 {
            self.status = b"process table unavailable";
            return;
        }
        let count = (written as usize).min(MAX_PROCS);
        let header: ProcStatHeader =
            unsafe { core::ptr::read_unaligned(buf.as_ptr() as *const ProcStatHeader) };
        let dt = header.total_ticks.saturating_sub(self.last_total_ticks);
        let warmed = self.last_total_ticks != 0 && dt > 0;

        let mut rows = Vec::with_capacity(count);
        let mut prev = Vec::with_capacity(count);
        for i in 0..count {
            let off = HEADER_LEN + i * ENTRY_LEN;
            if off + ENTRY_LEN > buf.len() {
                break;
            }
            let e: ProcStatEntry =
                unsafe { core::ptr::read_unaligned(buf.as_ptr().add(off) as *const ProcStatEntry) };
            let cpu_pct = if warmed {
                let last =
                    self.prev.iter().find(|(p, _)| *p == e.pid).map(|(_, t)| *t).unwrap_or(0);
                let d = e.run_ticks.saturating_sub(last);
                (d.saturating_mul(100) / dt).min(100) as u8
            } else {
                0
            };
            rows.push(Row {
                pid: e.pid,
                name: e.name,
                name_len: e.name_len,
                state: e.state,
                caps: e.caps,
                mem_kb: e.mem_kb,
                cpu_pct,
                uptime_ms: e.uptime_ms,
            });
            prev.push((e.pid, e.run_ticks));
        }
        // Totals across the whole live set.
        self.total_mem_kb = rows.iter().map(|r| r.mem_kb).sum();
        self.total_cpu = rows.iter().map(|r| r.cpu_pct as u32).sum();

        match self.sort {
            Sort::Cpu => rows.sort_by(|a, b| b.cpu_pct.cmp(&a.cpu_pct).then(a.pid.cmp(&b.pid))),
            Sort::Mem => rows.sort_by(|a, b| b.mem_kb.cmp(&a.mem_kb).then(a.pid.cmp(&b.pid))),
            Sort::Name => rows.sort_by(|a, b| a.name().cmp(b.name()).then(a.pid.cmp(&b.pid))),
            Sort::Pid => rows.sort_by(|a, b| a.pid.cmp(&b.pid)),
        }

        self.rows = rows;
        self.prev = prev;
        self.last_total_ticks = header.total_ticks;
        self.status = b"live: name, pid, state, cpu, memory, caps";

        // Keep the selection on screen after the list changes. With nothing
        // selected yet, land on the first row so the detail line and the kill
        // action are usable immediately.
        if let Some(idx) = self.selected_index() {
            self.ensure_visible(idx);
        } else if let Some(first) = self.rows.first() {
            self.selected_pid = first.pid;
        } else {
            self.selected_pid = 0;
        }
        let max = self.rows.len().saturating_sub(self.visible);
        if self.scroll > max {
            self.scroll = max;
        }

        // Re-run the security view over the same rows the kernel just gave us,
        // then keep the findings selection valid against the new list.
        self.alerts = self.monitor.evaluate(&self.rows);
        self.clamp_alert_scroll();
        self.flagged = self.alerts.iter().filter(|a| a.pid != 0).map(|a| a.pid).collect();
        self.history.total.push(self.total_cpu.min(255) as u8, self.total_mem_kb);
        for row in &self.rows {
            self.history.record(row.pid, row.cpu_pct, row.mem_kb);
        }
        let live: Vec<u32> = self.rows.iter().map(|r| r.pid).collect();
        self.history.retain_live(&live);
    }
}
