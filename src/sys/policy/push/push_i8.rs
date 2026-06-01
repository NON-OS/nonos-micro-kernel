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

use crate::sys::policy::field_id::PolicyField;
use crate::sys::policy::timezone::set_timezone_offset;

use super::error::PolicyPushError;

pub fn push_i8(field: PolicyField, value: i8) -> Result<(), PolicyPushError> {
    match field {
        PolicyField::TimezoneOffset => {
            set_timezone_offset(value);
            Ok(())
        }
        _ => Err(PolicyPushError::WrongType),
    }
}
