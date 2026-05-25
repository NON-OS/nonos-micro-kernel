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

use crate::sys::policy::{push_bool, PolicyField};
use crate::usercopy::{copy_from_user, validate_user_read};

use super::super::errno::E_FAULT;
use super::map_err::map_err;

pub(super) fn push_bool_arg(field: PolicyField, value_ptr: u64) -> i64 {
    let mut buf = [0u8; 1];
    if value_ptr == 0 || validate_user_read(value_ptr, 1).is_err() {
        return E_FAULT;
    }
    if copy_from_user(value_ptr, &mut buf).is_err() {
        return E_FAULT;
    }
    map_err(push_bool(field, buf[0] != 0))
}
