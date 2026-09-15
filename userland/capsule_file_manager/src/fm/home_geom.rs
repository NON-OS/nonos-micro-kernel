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

use alloc::vec::Vec;

use super::home_rows::HOME_ROW_H;
use super::layout::{FOOTER_H, HEADER_H, SECTION_GAP};
use super::recents_group::rel_time;
use super::screen_list::Line;
use super::screen_row::LABEL_ADV;
use super::sidebar_metrics::line_box;
use super::state::State;

// How many journal entries the Continue Working column offers before the rest
// is left to the storage row, which is pinned to the foot rather than stacked.
const RECENT_MAX: usize = 6;
const SEC_PAD: u32 = 10;

/// The two type sizes the greeting block is laid out from. Its height is derived
/// from the line boxes that actually land, not from a guessed advance, because
/// the font layer clamps a small size up and would overrun a guessed one.
pub const GREET_PX: f32 = 26.0;
pub const DATE_PX: f32 = 15.0;

/// Card heights for the two plate rows.
pub const CAT_H: u32 = 84;
pub const STOR_H: u32 = 72;

/// Baseline of the greeting, and of the date line beneath it.
pub fn greet_y() -> u32 {
    HEADER_H + 16
}

pub fn date_y() -> u32 {
    greet_y() + line_box(GREET_PX)
}

/// Top of the CATEGORIES label, then of the card row it heads.
pub fn cats_label_y() -> u32 {
    date_y() + line_box(DATE_PX) + SEC_PAD
}

pub fn cat_row_y() -> u32 {
    cats_label_y() + LABEL_ADV
}

/// Top of the CONTINUE WORKING label.
pub fn cards_label_y() -> u32 {
    cat_row_y() + CAT_H + SECTION_GAP
}

/// The storage row is pinned above the footer, so it is present at any window
/// height and the journal column takes whatever is left rather than pushing it
/// off the bottom.
pub fn stor_row_y(win_h: u32) -> u32 {
    win_h.saturating_sub(FOOTER_H + STOR_H)
}

pub fn stor_label_y(win_h: u32) -> u32 {
    stor_row_y(win_h).saturating_sub(LABEL_ADV)
}

/// The journal rows Home offers, capped by `RECENT_MAX` and by the height left
/// above the storage row. The painter draws these and `screen_hit` searches
/// them, so a row opens the path it is showing.
pub fn home_lines(state: &State, now: u64) -> Vec<Line> {
    let bottom = stor_label_y(state.win_h).saturating_sub(SEC_PAD);
    let mut y = cards_label_y() + LABEL_ADV;
    let mut out = Vec::new();
    for (ms, path) in state.recents.iter().take(RECENT_MAX) {
        if y + HOME_ROW_H > bottom {
            break;
        }
        out.push(Line::row(y, HOME_ROW_H, path, rel_time(now, *ms), path.ends_with('/')));
        y += HOME_ROW_H;
    }
    out
}
