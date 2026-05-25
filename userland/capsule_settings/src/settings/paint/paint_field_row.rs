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

use nonos_app_skeleton::PaintBuffer;
use nonos_policy_proto::{
    enum_table, kind_of, label_of, max_of, Field, KIND_BOOL, KIND_I8, KIND_STR, KIND_U8,
};

use crate::settings::manifest::WIDTH;
use crate::settings::state::{cached_value, FieldValue, State};
use crate::settings::theme::{LABEL_FG, ROW_BG, ROW_BG_ALT, ROW_SELECTED_BG};

use super::layout::{LABEL_LEFT, ROW_H};
use super::paint_value_bool::paint_value_bool;
use super::paint_value_enum::paint_value_enum;
use super::paint_value_i8::paint_value_i8;
use super::paint_value_str::paint_value_str;
use super::paint_value_u8::paint_value_u8;

pub fn paint_field_row(
    fb: &mut PaintBuffer,
    state: &State,
    field: Field,
    row_index: usize,
    y: u32,
    selected: bool,
) {
    let bg = if selected {
        ROW_SELECTED_BG
    } else if row_index % 2 == 0 {
        ROW_BG
    } else {
        ROW_BG_ALT
    };
    fb.fill_rect(0, y, WIDTH, ROW_H, bg);
    fb.text(LABEL_LEFT, y + 5, label_of(field), LABEL_FG);
    let value = cached_value(state, field);
    match kind_of(field) {
        KIND_BOOL => {
            let v = match value {
                FieldValue::Bool(b) => Some(b),
                _ => None,
            };
            paint_value_bool(fb, y + 5, v);
        }
        KIND_U8 => {
            let v = match value {
                FieldValue::U8(n) => Some(n),
                _ => None,
            };
            if enum_table(field).is_some() {
                paint_value_enum(fb, y + 5, field, v);
            } else {
                paint_value_u8(fb, y + 5, v, max_of(field));
            }
        }
        KIND_I8 => {
            let v = match value {
                FieldValue::I8(n) => Some(n),
                _ => None,
            };
            paint_value_i8(fb, y + 5, v);
        }
        KIND_STR => {
            let editing = selected && state.editing;
            let bytes: Option<&[u8]> = if editing {
                Some(state.edit.as_slice())
            } else {
                match &value {
                    FieldValue::Str { bytes, len } => Some(&bytes[..*len]),
                    _ => None,
                }
            };
            paint_value_str(fb, y + 5, bytes, editing);
        }
        _ => {}
    }
}
