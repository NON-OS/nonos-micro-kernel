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

use super::types::{TaskbarState, TASKBAR_APP_MAX, TASKBAR_NO_ACTIVE};

pub fn new_taskbar_state() -> TaskbarState {
    TaskbarState {
        open: [false; TASKBAR_APP_MAX],
        pulse_until_ms: [0; TASKBAR_APP_MAX],
        reveal_until_ms: 0,
        active: TASKBAR_NO_ACTIVE,
        visible: true,
    }
}
