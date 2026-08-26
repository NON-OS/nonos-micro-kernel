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

//! The pane a non-empty query puts on screen: one card listing every matching
//! setting with the section that holds it, so choosing a result navigates there.

use nonos_app_skeleton::PaintBuffer;

use crate::settings::state::State;

use super::bytes::as_str;
use super::card;
use super::metrics::{
    CARD_HEAD_H, CARD_PAD_X, CARD_TITLE_PX, NOTE_PX, PANE_PAD_TOP, PANE_PAD_X, ROW_H,
};
use super::results;
use super::results_geom::{card_h, card_y, row_y};
use super::results_head;
use super::results_row;
use super::row;
use super::text;
use super::theme::{CARD_NOTE_FG, CARD_TITLE_FG};

pub fn paint(fb: &mut PaintBuffer, state: &State, scroll: u32, view_w: u32, view_h: u32) {
    let card_x = PANE_PAD_X;
    let card_w = view_w.saturating_sub(PANE_PAD_X * 2);
    let query = as_str(state.search.as_slice());
    let n = results::count(query);
    results_head::paint(fb, query, n, PANE_PAD_TOP as i32 - scroll as i32);
    let body_y = card_y() as i32 - scroll as i32;
    card::paint_body(fb, card_x, body_y, card_w, card_h(n), view_h);
    let head = if n == 0 { "No matches" } else { "Matching settings" };
    text::left(fb, card_x + CARD_PAD_X, body_y + 14, head, CARD_TITLE_FG, CARD_TITLE_PX);
    if n == 0 {
        let top = text::centred_top(0, ROW_H, NOTE_PX) + body_y + CARD_HEAD_H as i32;
        text::left(fb, card_x + CARD_PAD_X, top, EMPTY, CARD_NOTE_FG, NOTE_PX);
        return;
    }
    for i in 0..n {
        let sy = row_y(i) as i32 - scroll as i32;
        if sy >= view_h as i32 || sy + ROW_H as i32 <= 0 {
            continue;
        }
        let Some((section, _, field)) = results::at(query, i) else { continue };
        results_row::paint(fb, field, section, card_x, card_w, sy, i == state.search_cursor);
        if i + 1 < n {
            row::hairline(fb, card_x, card_w, sy, ROW_H);
        }
    }
}

const EMPTY: &str = "Try a shorter word, or the name the setting uses.";
