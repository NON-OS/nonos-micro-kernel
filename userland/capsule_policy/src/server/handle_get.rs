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

use nonos_policy_proto::{kind_of, Field, KIND_BOOL, KIND_I8, KIND_STR, KIND_U8};

use super::handlers::{get_bool, get_i8, get_str, get_u8};

pub fn dispatch(pid: u32, field: Field) {
    match kind_of(field) {
        KIND_BOOL => get_bool::handle(pid, field),
        KIND_U8 => get_u8::handle(pid, field),
        KIND_I8 => get_i8::handle(pid, field),
        KIND_STR => get_str::handle(pid, field),
        _ => {}
    }
}
