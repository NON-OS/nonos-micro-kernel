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

use super::{NotifyLevel, SpotlightState, TrayTable};

pub struct Context {
    pub compositor_port: u32,
    pub wm_port: u32,
    pub input_router_port: u32,
    pub input_kind_mask: u32,
    pub input_ready: bool,
    pub wm_notify_ready: bool,
    pub width: u32,
    pub height: u32,
    pub stride: u32,
    pub backing_va: u64,
    pub tray: TrayTable,
    pub spotlight: SpotlightState,
    pub last_notify_level: Option<NotifyLevel>,
    pub next_request_id: u32,
}

impl Context {
    pub fn issue_request_id(&mut self) -> u32 {
        let id = self.next_request_id;
        self.next_request_id = id.wrapping_add(1).max(1);
        id
    }
}
