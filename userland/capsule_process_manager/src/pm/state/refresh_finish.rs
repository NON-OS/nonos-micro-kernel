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

//! After the rows are in: keep the selection and scroll valid, re-run the
//! security view, and record the sample into the histories.

use alloc::vec::Vec;

use super::State;

impl State {
    pub(super) fn finish(&mut self, warmed: bool) {
        // Keep the selection on screen after the list changes. With nothing
        // selected yet, land on the first row so the inspector and the kill
        // action are usable at once.
        if let Some(idx) = self.selected_index() {
            self.ensure_visible(idx);
        } else if let Some(first) = self.rows.first() {
            self.selected_pid = first.pid;
        } else {
            self.selected_pid = 0;
        }
        let max = self.filtered().len().saturating_sub(self.visible);
        if self.scroll > max {
            self.scroll = max;
        }
        self.alerts = self.monitor.evaluate(&self.rows);
        self.clamp_alert_scroll();
        self.flagged = self.alerts.iter().filter(|a| a.pid != 0).map(|a| a.pid).collect();
        // Only a warmed sample goes into the history: an unwarmed pass holds
        // the previous figures, and held figures summed past a byte once
        // showed a peak the machine cannot produce.
        if warmed {
            let h = &mut self.history;
            h.total.push(self.sys.busy_pct, self.sys.mem_used_kb());
            h.user.push(self.sys.user_pct as u32);
            h.syscalls.push(self.sys.sysc_ps);
            h.messages.push(self.sys.ipc_ps);
            h.switches.push(self.sys.ctx_ps);
            h.interrupts.push(self.sys.irq_ps);
        }
        for row in &self.rows {
            self.history.record(row.pid, row.cpu_pct, row.mem_kb);
        }
        let live: Vec<u32> = self.rows.iter().map(|r| r.pid).collect();
        self.history.retain_live(&live);
    }
}
