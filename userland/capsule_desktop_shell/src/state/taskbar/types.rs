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

pub const TASKBAR_APP_MAX: usize = crate::state::apps::LAUNCHER_APPS.len();
pub const TASKBAR_NO_ACTIVE: u8 = 0xFF;

pub struct TaskbarState {
    pub open: [bool; TASKBAR_APP_MAX],
    pub pulse_until_ms: [i64; TASKBAR_APP_MAX],
    pub reveal_until_ms: i64,
    pub active: u8,
    pub visible: bool,
}
