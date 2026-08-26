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

use crate::settings::state::FieldValue;

use super::chevron;
use super::control_value::choice_text;
use super::metrics::BODY_PX;
use super::text;
use super::theme::{IDLE, VALUE_FG};

pub fn paint_plain(fb: &mut PaintBuffer, s: &str, right: u32, screen_y: i32, row_h: u32) {
    let top = text::centred_top(0, row_h, BODY_PX) + screen_y;
    text::right(fb, right, top, s, VALUE_FG, BODY_PX);
}

pub fn paint_bool_text(
    fb: &mut PaintBuffer,
    value: FieldValue,
    right: u32,
    screen_y: i32,
    row_h: u32,
) {
    let label = match value {
        FieldValue::Bool(true) => "Yes",
        FieldValue::Bool(false) => "No",
        _ => "Unknown",
    };
    paint_plain(fb, label, right, screen_y, row_h);
}

pub fn paint_choice(
    fb: &mut PaintBuffer,
    field: Field,
    value: FieldValue,
    right: u32,
    screen_y: i32,
    row_h: u32,
) {
    let cy = screen_y + (row_h / 2) as i32;
    chevron::draw(fb, right, cy, IDLE);
    match choice_text(field, value) {
        Some(v) => paint_plain(fb, v.as_str(), right - 17, screen_y, row_h),
        None => paint_plain(fb, "--", right - 17, screen_y, row_h),
    }
}

pub use super::control_field::{paint_number, paint_string};
