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

//! The rows of one refresh, each entry paired with what the same pid
//! reported last time so every rate is a real difference.

use alloc::vec::Vec;

use nonos_libc::ProcStatEntry;

use super::system::Prev;
use super::system_update::per_second;
use super::types::{ENTRY_LEN, HEADER_LEN};
use super::{Rates, Row, State};

/// `(dt, ms, warmed)`: ticks and milliseconds since the last accepted read.
pub(super) fn build(
    state: &State,
    buf: &[u8],
    count: usize,
    window: (u64, u64, bool),
) -> (Vec<Row>, Vec<Prev>) {
    let (dt, ms, warmed) = window;
    let mut rows = Vec::with_capacity(count);
    let mut prev = Vec::with_capacity(count);
    for i in 0..count {
        let off = HEADER_LEN + i * ENTRY_LEN;
        if off + ENTRY_LEN > buf.len() {
            break;
        }
        // SAFETY: eK@nonos.systems - `off + ENTRY_LEN` is inside the buffer the
        // kernel filled with `count` repr(C) entries.
        let e: ProcStatEntry =
            unsafe { core::ptr::read_unaligned(buf.as_ptr().add(off) as *const ProcStatEntry) };
        let last = state.prev.iter().find(|p| p.pid == e.pid).copied();
        let held = state.rows.iter().find(|r| r.pid == e.pid);
        let ipc = e.ipc_tx.saturating_add(e.ipc_rx);
        let rates = match (warmed, last) {
            (true, Some(p)) => Rates {
                cpu_pct: (e.run_ticks.saturating_sub(p.ticks).saturating_mul(100) / dt.max(1))
                    .min(100) as u8,
                user_pct: (e.user_ticks.saturating_sub(p.user_ticks).saturating_mul(100)
                    / dt.max(1))
                .min(100) as u8,
                sysc_ps: per_second(e.syscalls.saturating_sub(p.syscalls), ms),
                ipc_ps: per_second(ipc.saturating_sub(p.ipc), ms),
                fault_ps: per_second(e.faults.saturating_sub(p.faults), ms),
            },
            _ => held.map(|r| r.rates()).unwrap_or_default(),
        };
        rows.push(Row::from_entry(&e, rates));
        prev.push(Prev {
            pid: e.pid,
            ticks: e.run_ticks,
            user_ticks: e.user_ticks,
            syscalls: e.syscalls,
            ipc,
            faults: e.faults,
        });
    }
    (rows, prev)
}
