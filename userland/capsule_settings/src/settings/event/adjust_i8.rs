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

use nonos_policy_proto::Field;

use crate::settings::ipc::op_set_i8;
use crate::settings::state::status::StatusKind;
use crate::settings::state::{cached_value, store_value, FieldValue, State};

use super::report::report;

const TZ_MIN: i8 = -12;
const TZ_MAX: i8 = 14;

pub(super) fn adjust_i8(state: &mut State, field: Field, delta: i32) {
    let current = match cached_value(state, field) {
        FieldValue::I8(v) => v as i32,
        _ => 0,
    };
    let next = (current + delta).clamp(TZ_MIN as i32, TZ_MAX as i32) as i8;
    match op_set_i8(state.policy_port, field, next) {
        Ok(()) => {
            store_value(state, field, FieldValue::I8(next));
            state.status.set(StatusKind::Ok, b"updated");
        }
        Err(e) => report(state, e),
    }
}
