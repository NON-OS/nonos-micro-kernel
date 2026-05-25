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

use nonos_policy_proto::{Field, E_NOT_FOUND, KIND_STR, OP_GET};
use crate::store::{get_str, STRING_CAP};

use super::super::respond;

pub fn handle(pid: u32, field: Field) {
    let mut buf = [0u8; STRING_CAP];
    match get_str::get(field, &mut buf) {
        Some(n) => respond::ok(pid, OP_GET, field as u32, KIND_STR, &buf[..n]),
        None => respond::err(pid, OP_GET, field as u32, KIND_STR, E_NOT_FOUND),
    }
}
