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

use super::icon_draw::draw;
use super::icon_path::Icon;
use super::list_cols::{Cols, CHECK_S};
use super::theme::{CY, HAIR, PILL_ON, PILL_ON_LINE};

const BOX_R: u32 = 5;
const TICK_INSET: u32 = 3;

/// The row's selection box, reflecting the shared selection model. Checked
/// spends the accent the way every other active control in the app does;
/// unchecked is a bare hairline square. Both fills blend, so the box is safe
/// over a row tile that is already down.
pub fn check_box(fb: &mut PaintBuffer, x: u32, y: u32, on: bool) {
    if !on {
        fb.stroke_round(x, y, CHECK_S, CHECK_S, BOX_R, 1, HAIR);
        return;
    }
    fb.fill_round(x, y, CHECK_S, CHECK_S, BOX_R, PILL_ON);
    fb.stroke_round(x, y, CHECK_S, CHECK_S, BOX_R, 1, PILL_ON_LINE);
    draw(fb, Icon::Check, x + TICK_INSET, y + TICK_INSET, CHECK_S - TICK_INSET * 2, CY);
}

/// Whether `x` falls in the checkbox, tested against the same gutter column
/// `check_box` was drawn into.
pub fn check_hit(c: &Cols, x: u32) -> bool {
    x >= c.check_x && x < c.check_x + CHECK_S
}
