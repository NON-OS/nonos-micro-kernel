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

use super::header_slots::TOOL_H;
use super::layout::{content_w, content_x, FOOTER_H};
use super::sel_model::ACTIONS;
use super::sel_slots::labelled_w;

// Band metrics: its own height, the inner padding, the gap between actions, and
// the clearance it keeps above the footer.
pub const BAND_H: u32 = 74;
pub const PAD: u32 = 12;
pub const GAP: u32 = 6;
pub const SUM_PX: f32 = 14.0;
const FOOT_GAP: u32 = 10;

/// The floating band above the footer. `labelled` records whether the seven
/// actions had room for their measured labels; both the painter and the
/// hit-test read it from here, so a narrow window collapses both at once.
pub struct SelBand {
    pub x: u32,
    pub y: u32,
    pub w: u32,
    pub h: u32,
    pub labelled: bool,
}

/// The band over the listing column, sized from the same two functions the list
/// and the grid subtract, so it can never stray under the info panel.
pub fn band(win_w: u32, win_h: u32) -> SelBand {
    let w = content_w(win_w);
    let need: u32 =
        ACTIONS.iter().map(|a| labelled_w(a.2)).sum::<u32>() + GAP * 6 + PAD * 2;
    SelBand {
        x: content_x(),
        y: win_h.saturating_sub(FOOTER_H + FOOT_GAP + BAND_H),
        w,
        h: BAND_H,
        labelled: need <= w,
    }
}

/// Top of the action row.
pub fn action_y(b: &SelBand) -> u32 {
    b.y + b.h.saturating_sub(PAD + TOOL_H)
}

/// Top of the summary line above it.
pub fn summary_y(b: &SelBand) -> u32 {
    b.y + PAD
}
