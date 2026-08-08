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

use nonos_policy_proto::{kind_of, KIND_I8, KIND_U8};

use crate::settings::schema::read_only;
use crate::settings::state::{current_field, State};

use super::adjust_i8::adjust_i8;
use super::adjust_u8::adjust_u8;

pub fn adjust(state: &mut State, delta: i32) {
    let field = match current_field(state) {
        Some(f) => f,
        None => return,
    };
    // Left and right reach here directly, so the read-only gate lives on both
    // routes rather than only on the toggle.
    if read_only(field) {
        return;
    }
    match kind_of(field) {
        KIND_U8 => adjust_u8(state, field, delta),
        KIND_I8 => adjust_i8(state, field, delta),
        _ => {}
    }
}
