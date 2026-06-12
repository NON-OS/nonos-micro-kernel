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

use super::types::TaskbarState;

pub fn expire_taskbar_visibility(state: &mut TaskbarState, now_ms: i64) -> bool {
    if !state.visible || state.reveal_until_ms == 0 || state.reveal_until_ms > now_ms {
        return false;
    }
    state.reveal_until_ms = 0;
    if state.open.iter().any(|open| *open) {
        state.visible = false;
        return true;
    }
    false
}
