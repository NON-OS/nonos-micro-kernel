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

use super::chrome_glow::glow_out;
use super::theme::{
    HAIR, PILL_HOVER, PILL_IDLE, PILL_INK, PILL_INK_ON, PILL_ON, PILL_ON_LINE,
};

/// The three states a pill control can be in. `Active` is the only one that
/// spends the accent hue, so selection stays the one saturated thing on screen.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum PillState {
    Idle,
    Hover,
    Active,
}

/// A true pill: the corner radius is half the height, so `h` alone sets the
/// shape. Used by toolbar buttons, filter chips and the `+ New` button.
pub fn pill(fb: &mut PaintBuffer, x: u32, y: u32, w: u32, h: u32, state: PillState) {
    pill_r(fb, x, y, w, h, h / 2, state);
}

/// The same plate with an explicit radius, for controls that want a squarer
/// shoulder than a true pill. Blends throughout, so it is safe over live paint.
pub fn pill_r(fb: &mut PaintBuffer, x: u32, y: u32, w: u32, h: u32, r: u32, state: PillState) {
    if w == 0 || h == 0 {
        return;
    }
    let (fill, line) = match state {
        PillState::Idle => (PILL_IDLE, HAIR),
        PillState::Hover => (PILL_HOVER, HAIR),
        PillState::Active => (PILL_ON, PILL_ON_LINE),
    };
    if state == PillState::Active {
        glow_out(fb, x, y, w, h, r, 3);
    }
    fb.fill_round(x, y, w, h, r, fill);
    fb.stroke_round(x, y, w, h, r, 1, line);
}

/// The label colour that pairs with a pill drawn in `state`.
pub fn pill_ink(state: PillState) -> u32 {
    match state {
        PillState::Active => PILL_INK_ON,
        _ => PILL_INK,
    }
}
