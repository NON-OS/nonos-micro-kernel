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

use super::constants::TEXT_LEFT;
use crate::term::block::Block;
use crate::term::theme::types::Theme;

/// Size the meta is drawn at. Below the body, because it annotates a command
/// rather than being part of what the command said.
const META_PX: f32 = 11.0;

/// Space between the run marks and the fields they sit beside.
const GAP: i32 = 8;

/// What a command did, at the right hand end of the line that started it.
///
/// Drawn right to left from the edge, so the columns line up down the
/// scrollback whatever each command was called. A reader scanning for the one
/// that failed is looking down a column, not reading each line.
pub(super) fn draw_meta(fb: &mut PaintBuffer, b: &Block, stripe: u32, y: u32, t: &Theme) {
    let baseline = (y + 1) as i32;
    let mut right = fb.width.saturating_sub(TEXT_LEFT) as i32;

    if let Ok(ts) = core::str::from_utf8(&b.ts) {
        right -= fb.measure_ttf(ts, META_PX);
        let _ = fb.text_ttf(right, baseline, ts, t.dim, META_PX);
        right -= GAP;
    }

    let (dbuf, dlen) = crate::term::dur::fmt_dur(b.dur_ms);
    if let Ok(dur) = core::str::from_utf8(&dbuf[..dlen]) {
        right -= fb.measure_ttf(dur, META_PX);
        let _ = fb.text_ttf(right, baseline, dur, stripe, META_PX);
        right -= GAP;
    }

    // No separate status glyph. The faces here do not carry a tick or a
    // cross, so one drew as a missing-glyph box, which says less than
    // nothing. The duration takes the stripe colour instead: the outcome is
    // already stated by the stripe down the left of the block, and this
    // repeats it where the eye is reading.
    let _ = stripe;
}
