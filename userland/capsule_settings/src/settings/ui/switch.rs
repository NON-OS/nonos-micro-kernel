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

use super::metrics::{KNOB_INSET, SWITCH_H, SWITCH_W};
use super::theme::{ACCENT, SWITCH_KNOB_OFF, SWITCH_KNOB_ON, SWITCH_OFF_BG, SWITCH_OFF_BORDER};

/// The track is drawn over chrome the row already painted, so it fills with an
/// opaque colour rather than an alpha one: `fill_round` writes, it never blends.
pub fn draw(fb: &mut PaintBuffer, x: u32, y: u32, on: bool) {
    let r = SWITCH_H / 2;
    let (bg, border) = if on { (ACCENT, ACCENT) } else { (SWITCH_OFF_BG, SWITCH_OFF_BORDER) };
    fb.fill_round(x, y, SWITCH_W, SWITCH_H, r, bg);
    fb.stroke_round(x, y, SWITCH_W, SWITCH_H, r, 1, border);
    let knob_r = (SWITCH_H - KNOB_INSET * 2) / 2;
    let cy = y + SWITCH_H / 2;
    let cx = if on { x + SWITCH_W - KNOB_INSET - knob_r } else { x + KNOB_INSET + knob_r };
    let knob = if on { SWITCH_KNOB_ON } else { SWITCH_KNOB_OFF };
    fb.circle(cx, cy, knob_r, knob);
}
