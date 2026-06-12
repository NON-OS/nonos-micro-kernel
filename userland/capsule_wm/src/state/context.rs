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

use super::SubscriptionList;
use crate::focus::FocusModel;
use crate::window::WindowTable;
use crate::z_order::ZStack;

pub struct Context {
    pub compositor_port: u32,
    pub display_width: u32,
    pub display_height: u32,
    pub windows: WindowTable,
    pub focus: FocusModel,
    pub z: ZStack,
    pub subscriptions: SubscriptionList,
    pub next_request_id: u32,
    pub input_router_pid: u32,
}

impl Context {
    pub fn issue_request_id(&mut self) -> u32 {
        let id = self.next_request_id;
        self.next_request_id = id.wrapping_add(1).max(1);
        id
    }
}
