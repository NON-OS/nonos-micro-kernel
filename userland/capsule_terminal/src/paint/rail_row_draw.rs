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

use super::fit_text::width_of;
use super::rail_row::row_h;
use super::rail_text::{clipped, left, lh, RAIL_GAP, RAIL_PX};
use super::shade::elevate;
use crate::layout::Rect;
use crate::term::theme::types::Theme;

const DOT: u32 = 7;
const PILL: &str = "ACTIVE";

/// One list entry: a status dot, the name, the path under it, and the pill that
/// marks the selected row. Every fill is a blend because the rail sits on
/// pixels the frame already painted.
pub fn draw_row(
    fb: &mut PaintBuffer,
    r: Rect,
    dot: u32,
    name: &str,
    sub: &str,
    active: bool,
    t: &Theme,
) {
    if r.h < row_h() || r.w == 0 {
        return;
    }
    if active {
        fb.blend_rect(r.x, r.y, r.w, r.h, elevate(t.bg, 14));
        fb.blend_rect(r.x, r.y, 2, r.h, t.accent);
    }
    let top = r.y + RAIL_GAP / 2;
    fb.blend_rect(r.x + RAIL_GAP, top + (lh().saturating_sub(DOT)) / 2, DOT, DOT, dot);
    let tx = r.x + RAIL_GAP * 2 + DOT;
    let edge = r.x + r.w;
    let mut avail = edge.saturating_sub(tx + RAIL_GAP);
    if active {
        let pw = width_of(fb, PILL, RAIL_PX);
        left(fb, edge.saturating_sub(pw + RAIL_GAP), top, PILL, t.accent);
        avail = avail.saturating_sub(pw + RAIL_GAP);
    }
    clipped(fb, tx, top, avail, name, t.fg);
    clipped(fb, tx, top + lh(), edge.saturating_sub(tx + RAIL_GAP), sub, t.dim);
}
