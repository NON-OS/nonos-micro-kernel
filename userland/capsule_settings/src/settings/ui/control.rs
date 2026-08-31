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
use nonos_policy_proto::{enum_table, kind_of, Field, KIND_BOOL, KIND_I8, KIND_STR, KIND_U8};

use crate::settings::schema::read_only;
use crate::settings::state::{cached_value, FieldValue, State};

use super::control_geom::{right_edge, slider_x, switch_rect};
use super::control_str::{paint_bool_text, paint_choice, paint_number, paint_string};
use super::control_value::percent;
use super::metrics::SLIDER_KNOB_R;
use super::slider;
use super::switch;

pub fn paint(
    fb: &mut PaintBuffer,
    state: &State,
    field: Field,
    card_x: u32,
    card_w: u32,
    screen_y: i32,
    row_h: u32,
) {
    let value = cached_value(state, field);
    let cy = screen_y + (row_h / 2) as i32;
    let right = right_edge(card_x, card_w);
    match kind_of(field) {
        KIND_BOOL if read_only(field) => paint_bool_text(fb, value, right, screen_y, row_h),
        KIND_BOOL => {
            let (x, y) = switch_rect(card_x, card_w, screen_y, row_h);
            if y >= 0 {
                switch::draw(fb, x, y as u32, matches!(value, FieldValue::Bool(true)));
            }
        }
        KIND_U8 if enum_table(field).is_some() => {
            paint_choice(fb, field, value, right, screen_y, row_h)
        }
        KIND_U8 => {
            let sx = slider_x(card_x, card_w);
            if cy >= SLIDER_KNOB_R as i32 {
                slider::draw(fb, sx, cy as u32, percent(field, value));
            }
            paint_number(fb, value, sx.saturating_sub(12), screen_y, row_h);
        }
        KIND_I8 => paint_number(fb, value, right, screen_y, row_h),
        KIND_STR => paint_string(fb, state, field, &value, right, screen_y, row_h),
        _ => {}
    }
}
