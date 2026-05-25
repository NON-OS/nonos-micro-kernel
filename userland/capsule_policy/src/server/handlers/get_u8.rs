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

use nonos_policy_proto::{Field, E_NOT_FOUND, KIND_U8, OP_GET};
use crate::store::get_u8;

use super::super::respond;

pub fn handle(pid: u32, field: Field) {
    match get_u8::get(field) {
        Some(v) => respond::ok(pid, OP_GET, field as u32, KIND_U8, &[v]),
        None => respond::err(pid, OP_GET, field as u32, KIND_U8, E_NOT_FOUND),
    }
}
