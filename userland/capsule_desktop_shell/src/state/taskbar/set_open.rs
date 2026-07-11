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

use super::types::{TaskbarState, TASKBAR_NO_ACTIVE};

pub fn set_taskbar_open(state: &mut TaskbarState, index: usize, open: bool) {
    if index >= state.open.len() {
        return;
    }
    state.open[index] = open;
    if open {
        state.active = index as u8;
        // The dock is a persistent taskbar: opening a window must not hide the
        // one place a new window is launched from.
        return;
    }
    if state.active != index as u8 {
        return;
    }
    state.active = TASKBAR_NO_ACTIVE;
    for next in (0..state.open.len()).rev() {
        if state.open[next] {
            state.active = next as u8;
            return;
        }
    }
    state.visible = true;
}
