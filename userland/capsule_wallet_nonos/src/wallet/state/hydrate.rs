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

use crate::wallet::ipc::{decode_rails, lookup_keyring, lookup_self_pid, read_rails};
use crate::wallet::state::State;

use super::filter_rails::filter_rails;

pub fn hydrate(state: &mut State) {
    if state.keyring_port == 0 {
        state.keyring_port = lookup_keyring().unwrap_or(0);
    }
    if state.owner_pid == 0 {
        state.owner_pid = lookup_self_pid().unwrap_or(0);
    }
    if state.keyring_port == 0 || state.owner_pid == 0 {
        state.status = b"keyring unavailable";
        return;
    }
    match read_rails(state.keyring_port).and_then(|rx| decode_rails(&rx, &mut state.rails)) {
        Ok(n) => {
            state.rail_count = filter_rails(&mut state.rails, n);
            state.status = b"wallet ready";
        }
        Err(_) => state.status = b"rail refresh failed",
    }
}
