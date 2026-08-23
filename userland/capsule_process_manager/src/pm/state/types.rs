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

use alloc::vec::Vec;

use nonos_libc::{ProcStatEntry, ProcStatHeader, PROC_NAME_LEN};

use super::super::security::{Alert, Monitor};
use super::{Filter, History, Query, Screen, Sort};

// Ceiling on processes read in one pass. Well above a full desktop; the whole
// live set fits in a single syscall so the view is never truncated in practice.
pub(super) const MAX_PROCS: usize = 256;

// Signals the monitor can send. SIGTERM asks nicely, SIGKILL forces it; both
// go through the kernel teardown that zeroizes the process.
pub const SIGTERM: u64 = 15;
pub const SIGKILL: u64 = 9;

pub(super) const HEADER_LEN: usize = core::mem::size_of::<ProcStatHeader>();
pub(super) const ENTRY_LEN: usize = core::mem::size_of::<ProcStatEntry>();

// One process, as the kernel reports it: identity, scheduler state, granted
// authority, resident memory, and the cpu share computed from tick deltas.
#[derive(Clone)]
pub struct Row {
    pub pid: u32,
    pub name: [u8; PROC_NAME_LEN],
    pub name_len: u8,
    pub state: u8,
    pub caps: u64,
    pub mem_kb: u64,
    pub cpu_pct: u8,
    // Milliseconds this process has been alive, computed by the kernel; it grows
    // every refresh, so the table reads as live even for an idle process.
    pub uptime_ms: u64,
}

impl Row {
    pub fn name(&self) -> &[u8] {
        &self.name[..(self.name_len as usize).min(PROC_NAME_LEN)]
    }
}

pub struct State {
    pub rows: Vec<Row>,
    pub refreshes: u32,
    pub status: &'static [u8],
    // The pid the user selected (0 = none), tracked by pid so it follows the
    // process across refreshes even as the sort order changes.
    pub selected_pid: u32,
    // Feedback from the last kill action, shown in the footer.
    pub notice: &'static [u8],
    // A kill that has been armed and is waiting for a confirming second press;
    // (pid, signal). 0 means nothing is armed.
    pub pending_pid: u32,
    pub(super) pending_sig: u64,
    pub sort: Sort,
    // First visible row and how many rows fit; paint sets `visible`, navigation
    // reads it to page and to keep the selection on screen.
    pub scroll: usize,
    pub visible: usize,
    // Frame size the last paint measured against. An input event carries no
    // dimensions, and the hit test must ask the geometry the same ones.
    pub fb_w: u32,
    pub fb_h: u32,
    // Live totals across the whole table, recomputed each refresh.
    pub total_mem_kb: u64,
    pub total_cpu: u32,
    pub(super) last_total_ticks: u64,
    // (pid, run_ticks) from the previous sample, so cpu percent is a real delta
    // per process rather than a cumulative total.
    pub(super) prev: Vec<(u32, u64)>,
    // Sampled cpu/memory history behind every sparkline, and the narrowing the
    // filter chips apply on top of the sort order.
    pub history: History,
    pub filter: Filter,
    // The head-band search field. It narrows `filtered()` alongside the chips
    // and, while focused, takes the keyboard off the letter shortcuts.
    pub(super) query: Query,
    // Which screen is shown, and the live security view over the same rows.
    pub screen: Screen,
    pub monitor: Monitor,
    pub alerts: Vec<Alert>,
    // Pids the monitor named this refresh, so the Flagged filter and anything
    // else that marks a row read from one list rather than re-deriving it.
    pub flagged: Vec<u32>,
    // Selection and scroll within the security panel's findings list;
    // `alert_visible` is how many findings fit, set by the panel painter.
    pub alert_sel: usize,
    pub alert_scroll: usize,
    pub alert_visible: usize,
}
