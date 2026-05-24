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

use crate::about::section_render::section_line_count;
use crate::about::state::State;
use crate::about::theme::{
    HEADER_HEIGHT, SCROLLBAR_THUMB, SCROLLBAR_TRACK, SCROLLBAR_WIDTH, STATUS_BAR_HEIGHT,
    TAB_BAR_HEIGHT,
};

pub fn paint(state: &State, fb: &mut PaintBuffer) {
    let total = section_line_count(state.section);
    let visible = state.last_visible_lines;
    if total <= visible {
        return;
    }
    let track_top = HEADER_HEIGHT.saturating_add(TAB_BAR_HEIGHT);
    let track_height = fb
        .height
        .saturating_sub(track_top)
        .saturating_sub(STATUS_BAR_HEIGHT);
    if track_height == 0 || fb.width < SCROLLBAR_WIDTH {
        return;
    }
    let track_x = fb.width - SCROLLBAR_WIDTH;
    fb.fill_rect(track_x, track_top, SCROLLBAR_WIDTH, track_height, SCROLLBAR_TRACK);
    let thumb_height =
        ((visible as u64 * track_height as u64) / total as u64).max(8) as u32;
    let thumb_height = thumb_height.min(track_height);
    let max_scroll = total.saturating_sub(visible).max(1);
    let usable = track_height.saturating_sub(thumb_height);
    let thumb_offset = ((usable as u64 * state.scroll as u64) / max_scroll as u64) as u32;
    let thumb_y = track_top.saturating_add(thumb_offset);
    fb.fill_rect(track_x, thumb_y, SCROLLBAR_WIDTH, thumb_height, SCROLLBAR_THUMB);
}
