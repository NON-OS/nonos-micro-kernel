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

//! One refresh: read the whole live table in a single syscall, then rebuild
//! the rows and the machine figures from it. Every field is the kernel's
//! own count; a rate is the difference between two reads over the time
//! between them.

use alloc::vec;

use nonos_libc::{mk_proc_stat, ProcStatHeader};

use super::types::{ENTRY_LEN, HEADER_LEN, MAX_PROCS};
use super::{Sort, State};

/// A percentage is only meaningful over enough ticks to divide by. A read
/// landing four ticks after the last gave every process that ran once a
/// quarter of the window; below this floor the previous figures are held.
const MIN_TICKS: u64 = 32;

impl State {
    pub fn refresh(&mut self) {
        self.refreshes = self.refreshes.wrapping_add(1);
        let mut buf = vec![0u8; HEADER_LEN + MAX_PROCS * ENTRY_LEN];
        let written = mk_proc_stat(buf.as_mut_ptr(), MAX_PROCS as u32);
        if written <= 0 {
            self.status = b"process table unavailable";
            return;
        }
        let count = (written as usize).min(MAX_PROCS);
        // SAFETY: eK@nonos.systems - the kernel wrote HEADER_LEN bytes of the
        // repr(C) header at the start of a buffer this call sized for it.
        let header: ProcStatHeader =
            unsafe { core::ptr::read_unaligned(buf.as_ptr() as *const ProcStatHeader) };
        let dt = header.total_ticks.saturating_sub(self.last_total_ticks);
        let warmed = self.last_total_ticks != 0 && dt >= MIN_TICKS;
        let ms = header.uptime_ms.saturating_sub(self.sys.uptime_ms);
        let (mut rows, prev) = super::refresh_rows::build(self, &buf, count, (dt, ms, warmed));
        if warmed {
            self.sys.update(&header);
            self.prev = prev;
            self.last_total_ticks = header.total_ticks;
        } else if self.last_total_ticks == 0 {
            self.sys.update(&header);
            self.prev = prev;
            self.last_total_ticks = header.total_ticks;
        }
        self.total_mem_kb = rows.iter().map(|r| r.mem_kb).sum();
        self.total_cpu = self.sys.busy_pct as u32;
        match self.sort {
            Sort::Cpu => rows.sort_by(|a, b| b.cpu_pct.cmp(&a.cpu_pct).then(a.pid.cmp(&b.pid))),
            Sort::Mem => rows.sort_by(|a, b| b.mem_kb.cmp(&a.mem_kb).then(a.pid.cmp(&b.pid))),
            Sort::Ipc => rows.sort_by(|a, b| b.ipc_ps.cmp(&a.ipc_ps).then(a.pid.cmp(&b.pid))),
            Sort::Sysc => rows.sort_by(|a, b| b.sysc_ps.cmp(&a.sysc_ps).then(a.pid.cmp(&b.pid))),
            Sort::Name => rows.sort_by(|a, b| a.name().cmp(b.name()).then(a.pid.cmp(&b.pid))),
            Sort::Pid => rows.sort_by(|a, b| a.pid.cmp(&b.pid)),
        }
        self.rows = rows;
        self.status = b"live from the kernel";
        self.finish(warmed);
    }
}
