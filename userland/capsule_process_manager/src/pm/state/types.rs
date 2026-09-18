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

//! The monitor's state: the rows the kernel reported, the machine figures
//! beside them, and everything the screens need to hold still between
//! refreshes (selection, scroll, sort, filter, findings).

use alloc::vec::Vec;

use nonos_libc::{ProcStatEntry, ProcStatHeader};

use super::super::security::{Alert, Monitor};
use super::system::{Prev, System};
use super::{Filter, History, Query, Row, Screen, Sort};

/// Ceiling on processes read in one pass; the whole live set fits in one call.
pub(super) const MAX_PROCS: usize = 256;
/// Signals the monitor can send. SIGTERM asks, SIGKILL forces.
pub const SIGTERM: u64 = 15;
pub const SIGKILL: u64 = 9;
pub(super) const HEADER_LEN: usize = core::mem::size_of::<ProcStatHeader>();
pub(super) const ENTRY_LEN: usize = core::mem::size_of::<ProcStatEntry>();

pub struct State {
    pub rows: Vec<Row>,
    pub sys: System,
    pub refreshes: u32,
    pub status: &'static [u8],
    pub selected_pid: u32,
    pub notice: &'static [u8],
    pub pending_pid: u32,
    pub(super) pending_sig: u64,
    pub sort: Sort,
    pub scroll: usize,
    pub visible: usize,
    pub fb_w: u32,
    pub fb_h: u32,
    /// Resident sum over the table, and the processor's busy share.
    pub total_mem_kb: u64,
    pub total_cpu: u32,
    pub(super) last_total_ticks: u64,
    pub(super) prev: Vec<Prev>,
    pub history: History,
    pub filter: Filter,
    pub(super) query: Query,
    pub screen: Screen,
    pub monitor: Monitor,
    pub alerts: Vec<Alert>,
    pub flagged: Vec<u32>,
    pub alert_sel: usize,
    pub alert_scroll: usize,
    pub alert_visible: usize,
    pub help_open: bool,
}
