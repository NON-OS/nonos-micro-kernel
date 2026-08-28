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

use crate::snake::theme::{ACCENT, MUTED, TITLE, TRACK_BG};

use super::metrics::TOGGLE_KNOB;
use super::rect::Rect;

// A switch, not a checkbox: the track carries the state colour and the knob
// travels the width the rect actually has rather than a nominal one.
pub fn paint(fb: &mut PaintBuffer, r: Rect, on: bool, locked: bool) {
    let radius = r.3 / 2;
    let track = if locked {
        TRACK_BG
    } else if on {
        ACCENT
    } else {
        TRACK_BG
    };
    fb.fill_round(r.0, r.1, r.2, r.3, radius, track);
    let knob = TOGGLE_KNOB.min(r.3);
    let pad = r.3.saturating_sub(knob) / 2;
    let travel = r.2.saturating_sub(knob + pad * 2);
    let x = r.0 + pad + if on { travel } else { 0 };
    let ink = if locked { MUTED } else { TITLE };
    fb.circle(x + knob / 2, r.1 + r.3 / 2, knob / 2, ink);
}
