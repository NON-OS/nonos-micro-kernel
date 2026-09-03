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

use crate::about::state::State;

use super::super::chrome::Rect;
use super::super::metrics::CARD_GAP;
use super::{system_build, system_runtime, system_space, system_uptime};

const LEFT_H: u32 = system_build::BUILD_H + CARD_GAP + system_uptime::HEIGHT;
const RIGHT_H: u32 = system_space::HEIGHT + CARD_GAP + system_runtime::HEIGHT;

// Two columns of fixed-height cards, so the extent is the taller of the two rather
// than a measurement. Build and Uptime are what the image is and how long it has
// been itself; Address space and Runtime are where it lives and what it is doing.
pub fn content_h(_rect: &Rect) -> u32 {
    if LEFT_H > RIGHT_H {
        LEFT_H
    } else {
        RIGHT_H
    }
}

pub fn paint(state: &State, fb: &mut PaintBuffer, rect: &Rect) {
    let mut pane = fb.sub(rect.x, rect.y, rect.w, rect.h);
    let y = -(state.scroll as i32);
    let col = (rect.w.saturating_sub(CARD_GAP)) / 2;
    let right_x = col + CARD_GAP;
    let right_w = rect.w.saturating_sub(right_x);
    system_build::paint(&mut pane, 0, y, col);
    system_uptime::paint(&mut pane, 0, y + (system_build::BUILD_H + CARD_GAP) as i32, col);
    system_space::paint(&mut pane, right_x, y, right_w);
    let runtime_y = y + (system_space::HEIGHT + CARD_GAP) as i32;
    system_runtime::paint(&mut pane, right_x, runtime_y, right_w);
}
