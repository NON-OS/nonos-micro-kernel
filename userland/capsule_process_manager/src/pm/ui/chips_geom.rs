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

use crate::pm::state::{Filter, Screen, FILTERS};

use super::super::metrics::{
    BODY_PX, CHIP_GAP, CHIP_H, CHIP_PAD_X, HEAD_H, PANE_PAD_TOP, PANE_PAD_X, SEARCH_META_GAP,
    SIDEBAR_W, TITLE_PX,
};
use super::super::text;

// The chips narrow the process list, so the row only exists on the screens that
// draw one. Origin, hit test and painter all gate on this one predicate.
fn listed(screen: Screen) -> bool {
    matches!(screen, Screen::Overview | Screen::Processes | Screen::Authority)
}

// The single chip formula. Measured, never a glyph count: the body face is
// proportional, so a label's width is only ever what the rasterizer says.
pub fn chip_w(filter: Filter) -> u32 {
    text::width_of(filter.label(), BODY_PX) + CHIP_PAD_X * 2
}

pub fn width() -> u32 {
    let bare: u32 = FILTERS.iter().map(|f| chip_w(*f)).sum();
    bare + CHIP_GAP * (FILTERS.len() as u32 - 1)
}

// The row hangs off the measured screen name rather than a fixed column, so a
// longer title pushes the chips right instead of drawing underneath them.
pub fn origin(screen: Screen) -> Option<u32> {
    if !listed(screen) {
        return None;
    }
    let title = text::width_of(screen.nav_label(), TITLE_PX);
    Some(SIDEBAR_W + PANE_PAD_X + title + SEARCH_META_GAP)
}

pub fn top() -> u32 {
    PANE_PAD_TOP + HEAD_H.saturating_sub(CHIP_H) / 2
}

pub fn at(screen: Screen, x: i32, y: i32) -> Option<Filter> {
    let mut cx = origin(screen)?;
    let y0 = top();
    if y < y0 as i32 || y >= (y0 + CHIP_H) as i32 {
        return None;
    }
    for &filter in FILTERS.iter() {
        let w = chip_w(filter);
        if x >= cx as i32 && x < (cx + w) as i32 {
            return Some(filter);
        }
        cx += w + CHIP_GAP;
    }
    None
}
