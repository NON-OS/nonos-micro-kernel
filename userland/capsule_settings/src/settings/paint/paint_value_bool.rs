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

use crate::settings::theme::{TAB_ACTIVE_BG, TAB_BG, VALUE_FG, VALUE_FG_FALSE};

use super::layout::VALUE_LEFT;

// A drawn switch reads at a glance down a column of eighteen rows in a way
// that "[x] enabled" does not. Track and knob are plain rects: the buffer has
// no rounded primitive, and a square switch beats a faked one.
const TRACK_W: u32 = 34;
const TRACK_H: u32 = 14;
const KNOB: u32 = 10;
const LABEL_GAP: u32 = 12;

pub fn paint_value_bool(fb: &mut PaintBuffer, y: u32, value: Option<bool>) {
    let Some(on) = value else {
        // Not fetched yet. No switch is drawn, so an unknown never reads as
        // an "off" the user might try to turn on.
        fb.text(VALUE_LEFT, y, b"...", VALUE_FG_FALSE);
        return;
    };
    let track_y = y + 1;
    fb.fill_rect(VALUE_LEFT, track_y, TRACK_W, TRACK_H, if on { TAB_ACTIVE_BG } else { TAB_BG });
    let knob_x = if on { VALUE_LEFT + TRACK_W - KNOB - 2 } else { VALUE_LEFT + 2 };
    fb.fill_rect(knob_x, track_y + 2, KNOB, KNOB, if on { 0xFFFFFFFF } else { VALUE_FG_FALSE });
    let (text, color): (&[u8], u32) =
        if on { (b"enabled", VALUE_FG) } else { (b"disabled", VALUE_FG_FALSE) };
    fb.text(VALUE_LEFT + TRACK_W + LABEL_GAP, y, text, color);
}
