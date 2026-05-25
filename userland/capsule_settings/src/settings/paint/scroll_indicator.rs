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

use crate::settings::manifest::{HEIGHT, WIDTH};
use crate::settings::theme::{ROW_BG_ALT, TAB_ACTIVE_BG};

use super::layout::{BODY_TOP, ROW_H, STATUS_H};

const TRACK_W: u32 = 6;

pub fn paint_scroll_indicator(fb: &mut PaintBuffer, top: usize, rows: usize, total: usize) {
    if total <= rows || rows == 0 {
        return;
    }
    let track_x = WIDTH - TRACK_W;
    let track_y = BODY_TOP;
    let track_h = HEIGHT.saturating_sub(BODY_TOP + STATUS_H);
    fb.fill_rect(track_x, track_y, TRACK_W, track_h, ROW_BG_ALT);
    let total_px = (total as u32) * ROW_H;
    if total_px == 0 {
        return;
    }
    let thumb_h = (track_h * track_h) / total_px;
    let thumb_h = thumb_h.max(8);
    let max_top_px = total_px.saturating_sub(track_h);
    let top_px = (top as u32) * ROW_H;
    let thumb_y = if max_top_px == 0 {
        0
    } else {
        (top_px * (track_h - thumb_h)) / max_top_px
    };
    fb.fill_rect(track_x, track_y + thumb_y, TRACK_W, thumb_h, TAB_ACTIVE_BG);
}
