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

use crate::pm::theme::{DANGER, DANGER_TINT, TITLE};

use super::insp_geom::btn;
use super::metrics::{BODY_PX, INSP_BTN_RADIUS};
use super::text;

// End Process asks the kernel nicely; Force Quit is the loud one, so it takes a
// tinted ground under the same danger outline rather than a second solid fill
// that would read as the safer of the two.
pub fn paint(fb: &mut PaintBuffer) {
    button(fb, 0, b"End Process", DANGER, TITLE);
    button(fb, 1, b"Force Quit", DANGER_TINT, DANGER);
}

// DANGER_TINT carries alpha and fill_round blends, which is what makes the
// tinted ground legal over the pane this capsule has already painted.
fn button(fb: &mut PaintBuffer, index: usize, label: &[u8], ground: u32, tint: u32) {
    let (x, y, w, h) = btn(fb.width, fb.height, index);
    fb.fill_round(x, y, w, h, INSP_BTN_RADIUS, ground);
    fb.stroke_round(x, y, w, h, INSP_BTN_RADIUS, 1, DANGER);
    let top = text::centred_top(y, h, BODY_PX);
    let cx = x + w.saturating_sub(text::width(fb, label, BODY_PX)) / 2;
    text::left(fb, cx, top, label, tint, BODY_PX);
}
