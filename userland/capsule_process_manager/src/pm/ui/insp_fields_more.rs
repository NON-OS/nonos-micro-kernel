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

//! The kernel's counters for one process: the rate now and the total since
//! it started, on one line each, so a reader sees both what it is doing and
//! what it has done.

use nonos_app_skeleton::PaintBuffer;

use crate::pm::format::mem_human;
use crate::pm::format_labels::{regions, split};
use crate::pm::format_sys::{count_human, rate_human};
use crate::pm::state::Row;
use crate::pm::theme::FOREGROUND;

use super::insp_fields::field;

/// "842/s  1.2M": the rate, two spaces, the cumulative count.
fn rate_and_total(per_second: u32, total: u64, out: &mut [u8]) -> usize {
    let mut n = rate_human(per_second, out);
    for &c in b"/s  " {
        if n < out.len() {
            out[n] = c;
            n += 1;
        }
    }
    n + count_human(total, &mut out[n..])
}

pub fn block(fb: &mut PaintBuffer, x: u32, y: u32, row: &Row) -> u32 {
    let mut buf = [0u8; 32];
    let n = rate_and_total(row.sysc_ps, row.syscalls, &mut buf);
    let mut y = field(fb, x, y, b"Syscalls", &buf[..n], FOREGROUND);
    let n = rate_and_total(row.ipc_ps, row.ipc_tx.saturating_add(row.ipc_rx), &mut buf);
    y = field(fb, x, y, b"Messages", &buf[..n], FOREGROUND);
    let n = rate_and_total(row.fault_ps, row.faults, &mut buf);
    y = field(fb, x, y, b"Page faults", &buf[..n], FOREGROUND);
    let n = count_human(row.switches, &mut buf);
    y = field(fb, x, y, b"Switched in", &buf[..n], FOREGROUND);
    let n = split(row.user_pct, row.cpu_pct.saturating_sub(row.user_pct), &mut buf);
    y = field(fb, x, y, b"User / kernel", &buf[..n], FOREGROUND);
    let n = mem_human(row.mapped_kb, &mut buf);
    let n = regions(row.vma_count, &mut buf, n);
    field(fb, x, y, b"Mmapped", &buf[..n], FOREGROUND)
}
