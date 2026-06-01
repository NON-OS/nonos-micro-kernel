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

use super::{CursorState, GrabTable, SubscriptionTable};

pub struct Context {
    pub subscriptions: SubscriptionTable,
    pub grabs: GrabTable,
    pub cursor: CursorState,
    pub compositor_port: u32,
    pub wm_port: u32,
    pub shell_pid: u32,
    pub focus_pid: u32,
    pub next_request_id: u32,
    pub delivered_count: u64,
    pub dropped_count: u64,
}

impl Context {
    pub const fn new() -> Self {
        Self {
            subscriptions: SubscriptionTable::new(),
            grabs: GrabTable::new(),
            cursor: CursorState::new(),
            compositor_port: 0,
            wm_port: 0,
            shell_pid: 0,
            focus_pid: 0,
            next_request_id: 1,
            delivered_count: 0,
            dropped_count: 0,
        }
    }

    pub fn issue_request_id(&mut self) -> u32 {
        let id = self.next_request_id;
        self.next_request_id = id.wrapping_add(1).max(1);
        id
    }

    pub fn record(&mut self, delivered: u32) {
        if delivered == 0 {
            self.dropped_count = self.dropped_count.saturating_add(1);
        } else {
            self.delivered_count = self.delivered_count.saturating_add(delivered as u64);
        }
    }
}
