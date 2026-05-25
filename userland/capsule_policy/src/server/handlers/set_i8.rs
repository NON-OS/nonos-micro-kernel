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

use nonos_policy_proto::{Field, E_BAD_LEN, E_INVAL, KIND_I8, OP_SET};
use crate::push;
use crate::store::set_i8;

use super::super::respond;

pub fn handle(pid: u32, field: Field, payload: &[u8]) {
    if payload.len() != 1 {
        respond::err(pid, OP_SET, field as u32, KIND_I8, E_BAD_LEN);
        return;
    }
    let value = payload[0] as i8;
    if !set_i8::set(field, value) {
        respond::err(pid, OP_SET, field as u32, KIND_I8, E_INVAL);
        return;
    }
    push::on_i8_set(field, value);
    respond::ok(pid, OP_SET, field as u32, KIND_I8, &[]);
}
