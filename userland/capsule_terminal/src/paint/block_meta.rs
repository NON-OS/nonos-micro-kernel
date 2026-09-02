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

use crate::term::block::{Block, Status};
use crate::term::theme::types::Theme;

/// Size the meta is drawn at. Below the body, because it annotates a command
/// rather than being part of what the command said.
const META_PX: f32 = 11.0;

/// Space between the run marks and the fields they sit beside.
const GAP: i32 = 8;

/// Side of the square the outcome mark is drawn inside, near the cap height of
/// the text it sits beside.
const MARK: i32 = 9;

/// What a command did, at the right hand end of the line that started it.
///
/// Drawn right to left from the edge, so the columns line up down the
/// scrollback whatever each command was called. A reader scanning for the one
/// that failed is looking down a column, not reading each line.
pub(super) fn draw_meta(
    fb: &mut PaintBuffer,
    b: &Block,
    stripe: u32,
    y: u32,
    max_x: u32,
    t: &Theme,
) {
    let baseline = (y + 1) as i32;
    let mut right = status_mark(fb, b.status, max_x as i32, baseline, t);

    if let Ok(ts) = core::str::from_utf8(&b.ts) {
        right -= fb.measure_ttf(ts, META_PX);
        let _ = fb.text_ttf(right, baseline, ts, t.dim, META_PX);
        right -= GAP;
    }

    let (dbuf, dlen) = crate::term::dur::fmt_dur(b.dur_ms);
    if let Ok(dur) = core::str::from_utf8(&dbuf[..dlen]) {
        let x = right - fb.measure_ttf(dur, META_PX);
        let _ = fb.text_ttf(x, baseline, dur, stripe, META_PX);
    }
}

/// The outcome, as strokes rather than a character, and the left edge it
/// leaves behind.
///
/// The UI face carries no tick and no cross, so asking it for one drew a
/// missing-glyph box. Two lines say the same thing and are legible at a size
/// no font in the tree renders well at. A running command has no outcome yet
/// and gets nothing, which is why the column is empty while it works.
fn status_mark(fb: &mut PaintBuffer, s: Status, right: i32, top: i32, t: &Theme) -> i32 {
    let x = right - MARK;
    let mid = top + MARK / 2;
    match s {
        Status::Running => return right,
        Status::Ok => {
            for d in 0..2 {
                let pts = [
                    (x, mid + d),
                    (x + MARK / 3, mid + MARK / 3 + d),
                    (x + MARK, mid - MARK / 2 + d),
                ];
                fb.polyline_aa(&pts, t.ok);
            }
        }
        Status::Err => {
            fb.line_aa(x, mid - MARK / 2, x + MARK, mid + MARK / 2, t.err);
            fb.line_aa(x, mid + MARK / 2, x + MARK, mid - MARK / 2, t.err);
        }
    }
    x - GAP
}
