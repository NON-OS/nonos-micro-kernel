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

//! A pill toggle: a rounded track with a circular knob parked at whichever end
//! the state selects. Track size is a parameter so a caller can ask for 46x25.

use nonos_app_skeleton::PaintBuffer;

pub(in crate::editor) fn paint_toggle(
    fb: &mut PaintBuffer,
    rect: (u32, u32, u32, u32),
    on: bool,
    colors: (u32, u32, u32),
) {
    let (x, y, w, h) = rect;
    let (track_on, track_off, knob) = colors;
    if w == 0 || h == 0 {
        return;
    }
    let track = if on { track_on } else { track_off };
    fb.fill_round(x, y, w, h, h / 2, track);
    let inset = (h / 10).max(2);
    let kr = h.saturating_sub(inset * 2) / 2;
    if kr == 0 {
        return;
    }
    let cx = match on {
        true => x + w.saturating_sub(inset + kr),
        false => x + inset + kr,
    };
    fb.circle(cx, y + h / 2, kr, knob);
}
