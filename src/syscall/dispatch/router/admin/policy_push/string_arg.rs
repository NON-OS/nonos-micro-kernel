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

extern crate alloc;

use alloc::vec;

use crate::sys::policy::{push_string, PolicyField};
use crate::usercopy::{copy_from_user, validate_user_read};

use super::super::errno::{E_FAULT, E_INVAL};
use super::kinds::STR_MAX;
use super::map_err::map_err;

pub(super) fn push_string_arg(field: PolicyField, value_ptr: u64, value_len: usize) -> i64 {
    if value_len > STR_MAX {
        return E_INVAL;
    }
    if value_len == 0 {
        return map_err(push_string(field, ""));
    }
    if value_ptr == 0 || validate_user_read(value_ptr, value_len).is_err() {
        return E_FAULT;
    }
    let mut buf = vec![0u8; value_len];
    if copy_from_user(value_ptr, &mut buf).is_err() {
        return E_FAULT;
    }
    let text = match core::str::from_utf8(&buf) {
        Ok(s) => s,
        Err(_) => return E_INVAL,
    };
    map_err(push_string(field, text))
}
