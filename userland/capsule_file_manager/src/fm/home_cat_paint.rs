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

extern crate alloc;

use alloc::string::String;

use nonos_app_skeleton::PaintBuffer;

use super::chrome_card::Plate;
use super::home_cat_geom::{cat_cells, CatCell};
use super::home_cats::{Cat, CATS};
use super::home_count::count_line;
use super::icon_draw::draw;
use super::icon_path::Icon;
use super::measure_text::truncate_to_width;
use super::state::State;
use super::theme::{INK, INK3, PANEL, R_CARD, TINT_BOT, TINT_TOP};

const PAD: u32 = 11;
const ICON: u32 = 22;
const CHEV: u32 = 12;
const TITLE_PX: f32 = 15.0;
const SUB_PX: f32 = 13.0;

/// Draws the category row from the same cells `home_hit` tests against.
pub fn paint_cats(state: &State, fb: &mut PaintBuffer) {
    for cell in cat_cells(state.win_w) {
        card(state, fb, &cell, &CATS[cell.idx]);
    }
}

// A card with nothing behind it keeps its plate but spends no accent on it: the
// glyph and the title drop to the label ink, which is how this crate marks a
// control that is drawn but not wired to a backend.
fn card(state: &State, fb: &mut PaintBuffer, c: &CatCell, cat: &Cat) {
    Plate::new(PANEL).radius(R_CARD).tint(TINT_TOP, TINT_BOT).draw(fb, c.x, c.y, c.w, c.h);
    let ink = if cat.counted { INK } else { INK3 };
    let tint = if cat.counted { cat.tint } else { INK3 };
    draw(fb, cat.icon, c.x + PAD, c.y + PAD, ICON, tint);
    let cx = c.x + c.w.saturating_sub(PAD + CHEV);
    draw(fb, Icon::Chevron, cx, c.y + PAD + (ICON - CHEV) / 2, CHEV, INK3);
    let inner = c.w.saturating_sub(PAD * 2);
    let title = truncate_to_width(fb, cat.title, TITLE_PX, inner);
    let _ = fb.text_ttf((c.x + PAD) as i32, (c.y + PAD + ICON + 6) as i32, title, ink, TITLE_PX);
    let line = figure(state, cat);
    let cut = truncate_to_width(fb, line.as_str(), SUB_PX, inner);
    let _ = fb.text_ttf((c.x + PAD) as i32, (c.y + PAD + ICON + 28) as i32, cut, INK3, SUB_PX);
}

// Recents counts the journal already held in state; a prefix card counts the
// cached walk; anything else has no backend and says so.
fn figure(state: &State, cat: &Cat) -> String {
    match cat.path {
        Some(path) => count_line(state, path),
        None if cat.counted => alloc::format!("{} items", state.recents.len()),
        None => String::from("No backend"),
    }
}
