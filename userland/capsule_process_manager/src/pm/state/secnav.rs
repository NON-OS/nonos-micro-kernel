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

use super::{Screen, State};

impl State {
    // Move the highlighted finding, keeping it inside the visible window.
    pub fn alert_move(&mut self, delta: i32) {
        if self.alerts.is_empty() {
            self.alert_sel = 0;
            self.alert_scroll = 0;
            return;
        }
        let last = self.alerts.len() - 1;
        let next = (self.alert_sel as i32 + delta).clamp(0, last as i32) as usize;
        self.alert_sel = next;
        let height = self.alert_visible.max(1);
        if next < self.alert_scroll {
            self.alert_scroll = next;
        } else if next >= self.alert_scroll + height {
            self.alert_scroll = next + 1 - height;
        }
    }

    pub fn alert_scroll_by(&mut self, delta: i32) {
        let max = self.alerts.len().saturating_sub(self.alert_visible) as i32;
        self.alert_scroll = (self.alert_scroll as i32 + delta).clamp(0, max.max(0)) as usize;
    }

    // Keep the finding selection and scroll valid after the list changes under
    // them on a refresh.
    pub fn clamp_alert_scroll(&mut self) {
        if self.alerts.is_empty() {
            self.alert_sel = 0;
            self.alert_scroll = 0;
            return;
        }
        let last = self.alerts.len() - 1;
        if self.alert_sel > last {
            self.alert_sel = last;
        }
        let max = self.alerts.len().saturating_sub(self.alert_visible);
        if self.alert_scroll > max {
            self.alert_scroll = max;
        }
    }

    // Jump from the highlighted finding to the process it names: switch to the
    // process table and select that pid. Findings with no pid (system-wide, such
    // as a stopped service) do nothing.
    pub fn jump_to_alert(&mut self) {
        let Some(alert) = self.alerts.get(self.alert_sel) else {
            return;
        };
        let pid = alert.pid;
        if pid == 0 {
            return;
        }
        if self.rows.iter().any(|r| r.pid == pid) {
            self.selected_pid = pid;
            self.screen = Screen::Processes;
            if let Some(idx) = self.selected_index() {
                self.ensure_visible(idx);
            }
        }
    }
}
