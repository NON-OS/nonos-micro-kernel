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

use nonos_libc::{mk_proc_stat, ProcStatEntry, ProcStatHeader};

use super::state::{State, HISTORY};

const MAX_ENTRIES: usize = 64;
const HEADER_LEN: usize = core::mem::size_of::<ProcStatHeader>();
const ENTRY_LEN: usize = core::mem::size_of::<ProcStatEntry>();

pub fn sample(state: &mut State) {
    let mut buf = [0u8; HEADER_LEN + MAX_ENTRIES * ENTRY_LEN];
    let written = mk_proc_stat(buf.as_mut_ptr(), MAX_ENTRIES as u32);
    if written <= 0 {
        return;
    }
    let count = (written as usize).min(MAX_ENTRIES);
    let header: ProcStatHeader =
        unsafe { core::ptr::read_unaligned(buf.as_ptr() as *const ProcStatHeader) };
    let dt = header.total_ticks.saturating_sub(state.last_total_ticks);
    let warmed = state.last_total_ticks != 0 && dt > 0;
    for (i, row) in state.rows.iter_mut().enumerate() {
        if row.pid == 0 {
            state.last_ticks[i] = 0;
            continue;
        }
        let ticks = find_ticks(&buf, count, row.pid);
        if warmed {
            let d = ticks.saturating_sub(state.last_ticks[i]);
            let pct = ((d.saturating_mul(100)) / dt).min(100) as u8;
            let h = row.cpu.head;
            row.cpu.percent[h] = pct;
            row.cpu.head = (h + 1) % HISTORY;
        }
        state.last_ticks[i] = ticks;
    }
    state.last_total_ticks = header.total_ticks;
}

fn find_ticks(buf: &[u8], count: usize, pid: u32) -> u64 {
    for i in 0..count {
        let off = HEADER_LEN + i * ENTRY_LEN;
        if off + ENTRY_LEN > buf.len() {
            return 0;
        }
        let e: ProcStatEntry =
            unsafe { core::ptr::read_unaligned(buf.as_ptr().add(off) as *const ProcStatEntry) };
        if e.pid == pid {
            return e.run_ticks;
        }
    }
    0
}
