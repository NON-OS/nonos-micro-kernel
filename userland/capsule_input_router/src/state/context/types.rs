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

use crate::state::{CursorState, GrabTable, SubscriptionTable};

pub struct Context {
    pub subscriptions: SubscriptionTable,
    pub grabs: GrabTable,
    pub cursor: CursorState,
    pub compositor_port: u32,
    pub wm_port: u32,
    pub shell_pid: u32,
    pub next_request_id: u32,
    pub delivered_count: u64,
    pub dropped_count: u64,
}
