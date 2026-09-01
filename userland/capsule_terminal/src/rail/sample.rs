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

use core::mem::size_of;

use nonos_libc::{mk_proc_stat, mk_uptime_ms, ProcStatEntry, ProcStatHeader};

use super::derive::cpu_pct;
use super::metrics::{Proc, Sample, MAX_PROCS};

const HEADER_LEN: usize = size_of::<ProcStatHeader>();
const ENTRY_LEN: usize = size_of::<ProcStatEntry>();

/// One read of the live process table, with per-process cpu derived against
/// `prev`. The kernel publishes cumulative run ticks only, so a percentage
/// exists solely as a delta between two samples.
pub fn poll(prev: &Sample) -> Sample {
    let mut buf = [0u8; HEADER_LEN + MAX_PROCS * ENTRY_LEN];
    let written = mk_proc_stat(buf.as_mut_ptr(), MAX_PROCS as u32);
    let mut out = Sample::EMPTY;
    out.uptime_ms = mk_uptime_ms().max(0) as u64;
    if written <= 0 {
        return out;
    }
    let header: ProcStatHeader =
        unsafe { core::ptr::read_unaligned(buf.as_ptr() as *const ProcStatHeader) };
    out.total_ticks = header.total_ticks;
    let dt = header.total_ticks.saturating_sub(prev.total_ticks);
    let count = (written as usize).min(MAX_PROCS);
    for i in 0..count {
        let off = HEADER_LEN + i * ENTRY_LEN;
        if off + ENTRY_LEN > buf.len() {
            break;
        }
        let e: ProcStatEntry =
            unsafe { core::ptr::read_unaligned(buf.as_ptr().add(off) as *const ProcStatEntry) };
        let last = prev.live().iter().find(|p| p.pid == e.pid).map(|p| p.run_ticks).unwrap_or(0);
        out.procs[out.n] = Proc {
            pid: e.pid,
            name: e.name,
            name_len: e.name_len,
            cpu_pct: cpu_pct(e.run_ticks.saturating_sub(last), dt),
            mem_kb: e.mem_kb,
            run_ticks: e.run_ticks,
        };
        out.n += 1;
        out.mem_total_kb = out.mem_total_kb.saturating_add(e.mem_kb);
    }
    out.procs[..out.n]
        .sort_unstable_by(|a, b| b.cpu_pct.cmp(&a.cpu_pct).then(b.mem_kb.cmp(&a.mem_kb)));
    out.cpu_pct = out.live().iter().map(|p| p.cpu_pct).sum::<u32>().min(100);
    out
}
