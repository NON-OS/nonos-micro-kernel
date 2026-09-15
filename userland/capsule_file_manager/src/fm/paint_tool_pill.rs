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

use super::chrome_pill::{pill_r, PillState};
use super::header_slots::{Slot, TOOL_H, TOOL_PAD, TOOL_PX};
use super::theme::R_CARD;

// Every slot width came from `pill_w`, which is the measured label plus TOOL_PAD
// on each side, so padding the pen by TOOL_PAD centres the label exactly.
pub fn text_y(y: u32) -> i32 {
    y as i32 + (TOOL_H as i32 - TOOL_PX as i32) / 2 - 2
}

/// A standalone labelled control. The caller passes the ink so a control with
/// nothing to act on can dim its own label.
pub fn pill(fb: &mut PaintBuffer, slot: &Slot, y: u32, label: &str, ink: u32) {
    pill_r(fb, slot.x, y, slot.w, TOOL_H, R_CARD, PillState::Idle);
    let _ = fb.text_ttf((slot.x + TOOL_PAD) as i32, text_y(y), label, ink, TOOL_PX);
}
