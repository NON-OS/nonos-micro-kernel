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

use super::metrics::{SLIDER_KNOB_R, SLIDER_TRACK_H, SLIDER_W};
use super::theme::{ACCENT, SWITCH_KNOB_ON, TRACK_BG};

/// `value` is already normalised to 0..=100 by the caller, so the control does
/// not need to know a field's maximum.
pub fn draw(fb: &mut PaintBuffer, x: u32, cy: u32, value: u32) {
    let y = cy.saturating_sub(SLIDER_TRACK_H / 2);
    fb.fill_round(x, y, SLIDER_W, SLIDER_TRACK_H, SLIDER_TRACK_H / 2, TRACK_BG);
    let filled = (SLIDER_W * value.min(100)) / 100;
    if filled > 0 {
        fb.fill_round(x, y, filled, SLIDER_TRACK_H, SLIDER_TRACK_H / 2, ACCENT);
    }
    fb.circle(x + filled, cy, SLIDER_KNOB_R, SWITCH_KNOB_ON);
}
