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

use super::Context;
use crate::state::{CursorState, GrabTable, SubscriptionTable};

impl Context {
    pub const fn new() -> Self {
        Self {
            subscriptions: SubscriptionTable::new(),
            grabs: GrabTable::new(),
            press: None,
            hover: None,
            hover_tick: 0,
            cursor: CursorState::new(),
            compositor_port: 0,
            wm_port: 0,
            shell_pid: 0,
            next_request_id: 1,
            delivered_count: 0,
            dropped_count: 0,
            cursor_x: 0,
            cursor_y: 0,
            cursor_dirty: false,
        }
    }
}
