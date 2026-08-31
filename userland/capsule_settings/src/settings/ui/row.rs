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
use nonos_policy_proto::label_of;

use crate::settings::schema::rows::Row;
use crate::settings::state::State;

use super::bytes::as_str;
use super::control;
use super::control_geom::right_edge;
use super::field_note::note_of;
use super::live_value::resolve;
use super::metrics::BODY_PX;
use super::metrics::CARD_RADIUS;
use super::pill::tone_argb;
use super::row_label;
use super::text;
use super::theme::{FOCUS_RING, ROW_HOVER_BG, ROW_LINE};

pub fn paint(
    fb: &mut PaintBuffer,
    state: &State,
    row: &Row,
    card_x: u32,
    card_w: u32,
    screen_y: i32,
    row_h: u32,
    selected: bool,
) {
    if selected && screen_y >= 0 {
        let y = screen_y as u32;
        fb.blend_rect(card_x + 1, y, card_w - 2, row_h, ROW_HOVER_BG);
        fb.stroke_round(card_x + 1, y, card_w - 2, row_h, CARD_RADIUS / 2, 1, FOCUS_RING);
    }
    match row {
        Row::Field(f) => {
            row_label::paint(fb, card_x, screen_y, row_h, as_str(label_of(*f)), note_of(*f));
            control::paint(fb, state, *f, card_x, card_w, screen_y, row_h);
        }
        Row::Live(label, live) => {
            row_label::paint(fb, card_x, screen_y, row_h, label, None);
            let (value, tone) = resolve(state, *live);
            let top = text::centred_top(0, row_h, BODY_PX) + screen_y;
            text::right(
                fb,
                right_edge(card_x, card_w),
                top,
                value.as_str(),
                tone_argb(tone),
                BODY_PX,
            );
        }
        Row::Networks => {}
    }
}

pub fn hairline(fb: &mut PaintBuffer, card_x: u32, card_w: u32, screen_y: i32, row_h: u32) {
    let y = screen_y + row_h as i32 - 1;
    if y >= 0 {
        fb.blend_rect(card_x + 1, y as u32, card_w - 2, 1, ROW_LINE);
    }
}
