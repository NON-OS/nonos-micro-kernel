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

use super::super::security::Monitor;
use super::{Filter, History, Screen, Sort, State};

impl State {
    pub fn new() -> Self {
        let mut state = State {
            rows: Vec::new(),
            refreshes: 0,
            status: b"reading process table",
            selected_pid: 0,
            notice: b"up/down select  K end  F force  C/M/N/P sort",
            pending_pid: 0,
            pending_sig: 0,
            sort: Sort::Cpu,
            scroll: 0,
            visible: 1,
            fb_w: 0,
            fb_h: 0,
            total_mem_kb: 0,
            total_cpu: 0,
            last_total_ticks: 0,
            prev: Vec::new(),
            history: History::new(),
            filter: Filter::All,
            screen: Screen::Overview,
            monitor: Monitor::new(),
            alerts: Vec::new(),
            flagged: Vec::new(),
            alert_sel: 0,
            alert_scroll: 0,
            alert_visible: 1,
        };
        state.refresh();
        state
    }
}
