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

use nonos_libc::mk_yield;

use crate::settings::schema::ALL_FIELDS;
use crate::settings::state::{store_value, State};

use super::op_get::op_get;

pub fn hydrate(state: &mut State) {
    if !state.policy_ready {
        return;
    }
    let port = state.policy_port;
    for field in ALL_FIELDS.iter().copied() {
        if let Ok(v) = op_get(port, field) {
            store_value(state, field, v);
        }
        mk_yield();
    }
}
