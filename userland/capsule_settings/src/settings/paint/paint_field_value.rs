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
    enum_table, kind_of, max_of, Field, KIND_BOOL, KIND_I8, KIND_STR, KIND_U8,
};

use crate::settings::state::{cached_value, State};

use super::field_bool_value::field_bool_value;
use super::field_i8_value::field_i8_value;
use super::field_str_value::field_str_value;
use super::field_u8_value::field_u8_value;
use super::paint_value_bool::paint_value_bool;
use super::paint_value_enum::paint_value_enum;
use super::paint_value_i8::paint_value_i8;
use super::paint_value_str::paint_value_str;
use super::paint_value_u8::paint_value_u8;

pub fn paint_field_value(
    fb: &mut PaintBuffer,
    state: &State,
    field: Field,
    y: u32,
    selected: bool,
) {
    let value = cached_value(state, field);
    match kind_of(field) {
        KIND_BOOL => paint_value_bool(fb, y, field_bool_value(value)),
        KIND_U8 => match field_u8_value(value) {
            v if enum_table(field).is_some() => paint_value_enum(fb, y, field, v),
            v => paint_value_u8(fb, y, v, max_of(field)),
        },
        KIND_I8 => paint_value_i8(fb, y, field_i8_value(value)),
        KIND_STR => paint_value_str(
            fb,
            y,
            field_str_value(state, &value, selected),
            selected && state.editing,
        ),
        _ => {}
    }
}
