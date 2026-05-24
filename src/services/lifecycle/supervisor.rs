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

use super::state::CapsuleState;

pub struct Supervised {
    pub name: &'static str,
    pub state_fn: fn() -> &'static CapsuleState,
    pub spawn_fn: fn() -> Result<(), &'static str>,
}

pub fn tick(now_ms: u64, supervised: &[Supervised]) -> u32 {
    let mut respawned = 0u32;
    for entry in supervised {
        let state = (entry.state_fn)();
        if state.is_alive() {
            continue;
        }
        if state.pid() == 0 && state.generation() == 0 {
            continue;
        }
        if !state.should_respawn(now_ms) {
            continue;
        }
        state.record_exit(now_ms);
        if (entry.spawn_fn)().is_ok() {
            respawned += 1;
        }
    }
    respawned
}
