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

use crate::settings::state::{FieldValue, State};

pub(super) fn field_str_value<'a>(
    state: &'a State,
    value: &'a FieldValue,
    selected: bool,
) -> Option<&'a [u8]> {
    if selected && state.editing {
        return Some(state.edit.as_slice());
    }
    match value {
        FieldValue::Str { bytes, len } => Some(&bytes[..*len]),
        _ => None,
    }
}
