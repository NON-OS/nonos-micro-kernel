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

use crate::settings::ipc::op_set_str;
use crate::settings::state::cache::STRING_CAP;
use crate::settings::state::status::StatusKind;
use crate::settings::state::{
    current_field, edit_commit, store_value, FieldValue, State,
};

use super::report::report;

pub fn commit_string(state: &mut State) {
    let field = match current_field(state) {
        Some(f) => f,
        None => return,
    };
    let edit = edit_commit(state);
    let bytes = edit.as_slice();
    match op_set_str(state.policy_port, field, bytes) {
        Ok(()) => {
            let mut snapshot = [0u8; STRING_CAP];
            snapshot[..bytes.len()].copy_from_slice(bytes);
            store_value(state, field, FieldValue::Str { bytes: snapshot, len: bytes.len() });
            state.status.set(StatusKind::Ok, b"updated");
        }
        Err(e) => report(state, e),
    }
}
