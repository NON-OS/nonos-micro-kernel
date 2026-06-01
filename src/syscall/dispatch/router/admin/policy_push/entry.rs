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

use crate::sys::policy::PolicyField;

use super::super::errno::E_INVAL;
use super::bool_arg::push_bool_arg;
use super::i8_arg::push_i8_arg;
use super::kinds::{KIND_BOOL, KIND_I8, KIND_STR};
use super::string_arg::push_string_arg;

pub(in crate::syscall::dispatch::router::admin) fn policy_push(
    field_id: u64,
    kind: u64,
    value_ptr: u64,
    value_len: u64,
) -> i64 {
    let field = match PolicyField::from_u32(field_id as u32) {
        Some(f) => f,
        None => return E_INVAL,
    };
    match kind as u32 {
        KIND_BOOL => push_bool_arg(field, value_ptr),
        KIND_I8 => push_i8_arg(field, value_ptr),
        KIND_STR => push_string_arg(field, value_ptr, value_len as usize),
        _ => E_INVAL,
    }
}
