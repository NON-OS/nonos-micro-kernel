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
use nonos_policy_proto::Field;

use crate::settings::state::{current_field, FieldValue, State};

use super::bytes::as_str;
use super::control_str::paint_plain;
use super::control_value::{number_text, string_text};
use super::metrics::BODY_PX;
use super::text;
use super::theme::{ACCENT, IDLE, VALUE_FG};

pub fn paint_number(
    fb: &mut PaintBuffer,
    value: FieldValue,
    right: u32,
    screen_y: i32,
    row_h: u32,
) {
    match number_text(value) {
        Some(v) => paint_plain(fb, v.as_str(), right, screen_y, row_h),
        None => paint_plain(fb, "--", right, screen_y, row_h),
    }
}

pub fn paint_string(
    fb: &mut PaintBuffer,
    state: &State,
    field: Field,
    value: &FieldValue,
    right: u32,
    screen_y: i32,
    row_h: u32,
) {
    let editing = state.editing && current_field(state) == Some(field);
    let top = text::centred_top(0, row_h, BODY_PX) + screen_y;
    let fg = if editing { ACCENT } else { VALUE_FG };
    match string_text(state, value, editing) {
        Some(b) => text::right(fb, right, top, as_str(b), fg, BODY_PX),
        None => text::right(fb, right, top, "--", IDLE, BODY_PX),
    };
}
