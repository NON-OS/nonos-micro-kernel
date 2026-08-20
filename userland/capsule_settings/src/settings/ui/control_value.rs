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

use nonos_policy_proto::{enum_label, max_of, Field};

use crate::settings::state::{FieldValue, State};

use super::valbuf::ValBuf;

/// The text an inline choice shows, or the number a numeric row shows. Returns
/// `None` when the panel has not read the field back from policy yet, so the row
/// can say so instead of printing a default that was never stored.
pub fn choice_text(field: Field, value: FieldValue) -> Option<ValBuf> {
    let FieldValue::U8(v) = value else { return None };
    let mut b = ValBuf::new();
    match enum_label(field, v) {
        Some(label) => b.push_bytes(label),
        None => b.push_u32(v as u32),
    }
    Some(b)
}

pub fn number_text(value: FieldValue) -> Option<ValBuf> {
    let mut b = ValBuf::new();
    match value {
        FieldValue::U8(v) => b.push_u32(v as u32),
        FieldValue::I8(v) => b.push_i8(v),
        _ => return None,
    }
    Some(b)
}

pub fn percent(field: Field, value: FieldValue) -> u32 {
    let FieldValue::U8(v) = value else { return 0 };
    let max = max_of(field).max(1) as u32;
    (v as u32 * 100) / max
}

pub fn string_text<'a>(state: &'a State, value: &'a FieldValue, editing: bool) -> Option<&'a [u8]> {
    if editing {
        return Some(state.edit.as_slice());
    }
    match value {
        FieldValue::Str { bytes, len } => Some(&bytes[..*len]),
        _ => None,
    }
}
