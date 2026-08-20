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
use nonos_toolkit::font::ttf::line_height;

use crate::settings::schema::rows::Block;
use crate::settings::state::State;

use super::card_pill::block_pill;
use super::clip::visible;
use super::metrics::{CARD_HEAD_H, CARD_PAD_X, CARD_RADIUS, CARD_TITLE_PX, NOTE_PX, PILL_H};
use super::pill;
use super::text;
use super::theme::{CARD_BG, CARD_BORDER, CARD_NOTE_FG, CARD_TITLE_FG};

pub fn paint_body(fb: &mut PaintBuffer, x: u32, screen_y: i32, w: u32, h: u32, view_h: u32) {
    let Some((top, height)) = visible(screen_y, h, view_h) else { return };
    fb.fill_round(x, top, w, height, CARD_RADIUS, CARD_BG);
    fb.stroke_round(x, top, w, height, CARD_RADIUS, 1, CARD_BORDER);
}

pub fn paint_head(fb: &mut PaintBuffer, state: &State, b: &Block, x: u32, screen_y: i32, w: u32) {
    let title_top = screen_y + 14;
    text::left(fb, x + CARD_PAD_X, title_top, b.title, CARD_TITLE_FG, CARD_TITLE_PX);
    if let Some(note) = b.note {
        let note_top = title_top + line_height(CARD_TITLE_PX) + 2;
        text::left(fb, x + CARD_PAD_X, note_top, note, CARD_NOTE_FG, NOTE_PX);
    }
    let Some((label, tone)) = block_pill(state, b) else { return };
    let pill_y = screen_y + ((CARD_HEAD_H - PILL_H) / 2) as i32;
    if pill_y >= 0 {
        pill::draw(fb, x + w - CARD_PAD_X, pill_y as u32, label.as_str(), tone);
    }
}
