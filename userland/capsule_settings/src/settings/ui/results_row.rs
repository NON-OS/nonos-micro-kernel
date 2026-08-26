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
use nonos_policy_proto::{label_of, Field};

use crate::settings::section::Section;
use crate::settings::section_text::nav;

use super::bytes::as_str;
use super::chevron;
use super::control_geom::right_edge;
use super::metrics::{BODY_PX, CARD_PAD_X, CARD_RADIUS, ROW_H};
use super::text;
use super::theme::{FOCUS_RING, LABEL_FG, ROW_HOVER_BG, SUBLABEL_FG};

pub fn paint(
    fb: &mut PaintBuffer,
    field: Field,
    section: Section,
    card_x: u32,
    card_w: u32,
    screen_y: i32,
    selected: bool,
) {
    if selected && screen_y >= 0 {
        let y = screen_y as u32;
        fb.blend_rect(card_x + 1, y, card_w - 2, ROW_H, ROW_HOVER_BG);
        fb.stroke_round(card_x + 1, y, card_w - 2, ROW_H, CARD_RADIUS / 2, 1, FOCUS_RING);
    }
    let top = text::centred_top(0, ROW_H, BODY_PX) + screen_y;
    text::left(fb, card_x + CARD_PAD_X, top, as_str(label_of(field)), LABEL_FG, BODY_PX);
    let right = right_edge(card_x, card_w);
    chevron::draw(fb, right, screen_y + (ROW_H / 2) as i32, SUBLABEL_FG);
    text::right(fb, right.saturating_sub(16), top, nav(section), SUBLABEL_FG, BODY_PX);
}
