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
use nonos_libc::mk_time_millis;

use super::home_cat_paint::paint_cats;
use super::home_date::date_line;
use super::home_geom::{
    cards_label_y, cats_label_y, date_y, greet_y, home_lines, stor_label_y, DATE_PX, GREET_PX,
};
use super::home_rows::recent_row;
use super::home_storage::paint_storage_row;
use super::layout::{CONTENT_X, PAD_X};
use super::screen_row::{section_label, LABEL_ADV};
use super::state::State;
use super::theme::{INK, INK2, INK3};

pub fn paint_home(state: &State, fb: &mut PaintBuffer) {
    let x = CONTENT_X + PAD_X;
    let w = fb.width.saturating_sub(CONTENT_X + PAD_X * 2);
    let now = mk_time_millis().max(0) as u64;
    let _ = fb.text_ttf(x as i32, greet_y() as i32, greeting(now), INK, GREET_PX);
    let date = date_line(now);
    let _ = fb.text_ttf(x as i32, date_y() as i32, date.as_str(), INK2, DATE_PX);
    section_label(fb, x, cats_label_y(), "CATEGORIES");
    paint_cats(state, fb);
    section_label(fb, x, cards_label_y(), "CONTINUE WORKING");
    recent_rows(state, fb, x, w, now);
    section_label(fb, x, stor_label_y(state.win_h), "STORAGE & DEVICES");
    paint_storage_row(state, fb);
}

// The wall clock is the only time source, and `fmt_time` already reads a civil
// time-of-day out of it the same way, so a UTC hour is a sound derivation here.
fn greeting(now_ms: u64) -> &'static str {
    match now_ms / 1000 % 86_400 / 3600 {
        0..=11 => "Good morning.",
        12..=17 => "Good afternoon.",
        _ => "Good evening.",
    }
}

// Draws the rows `home_geom` laid out; an empty journal says so in their place
// rather than leaving the band blank.
fn recent_rows(state: &State, fb: &mut PaintBuffer, x: u32, w: u32, now: u64) {
    let lines = home_lines(state, now);
    if lines.is_empty() {
        let y = cards_label_y() + LABEL_ADV;
        let note = "Nothing opened yet. Files you open land here.";
        let _ = fb.text_ttf(x as i32, y as i32, note, INK3, 15.0);
        return;
    }
    for line in &lines {
        recent_row(fb, x, line.y, w, line.path.as_str(), line.meta.as_str());
    }
}
