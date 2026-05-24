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
use crate::about::state::{State, VISIBLE_BODY_LINES};
use crate::about::theme::{
    HEADER_HEIGHT, SCROLLBAR_THUMB, SCROLLBAR_TRACK, SCROLLBAR_WIDTH, STATUS_BAR_HEIGHT,
    TAB_BAR_HEIGHT,
};

pub fn paint(state: &State, fb: &mut PaintBuffer) {
    let total = section_line_count(state.section);
    if total <= VISIBLE_BODY_LINES {
        return;
    }
    let track_top = HEADER_HEIGHT + TAB_BAR_HEIGHT;
    let track_height = fb.height - track_top - STATUS_BAR_HEIGHT;
    let track_x = fb.width - SCROLLBAR_WIDTH;
    fb.fill_rect(track_x, track_top, SCROLLBAR_WIDTH, track_height, SCROLLBAR_TRACK);
    let thumb_height = ((VISIBLE_BODY_LINES * track_height) / total).max(8);
    let max_scroll = total - VISIBLE_BODY_LINES;
    let thumb_y = track_top + ((track_height - thumb_height) * state.scroll) / max_scroll.max(1);
    fb.fill_rect(track_x, thumb_y, SCROLLBAR_WIDTH, thumb_height, SCROLLBAR_THUMB);
}
