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

use crate::wallet::theme::{BG, ELEV_HI, ELEV_LO, LINE, PANEL};

// A raised surface: panel fill, a top highlight and bottom shadow for depth,
// hairline sides, and clipped corners so it reads as a soft card, not a box.
pub fn card(fb: &mut PaintBuffer, x: u32, y: u32, w: u32, h: u32) {
    if w == 0 || h == 0 {
        return;
    }
    fb.fill_rect(x, y, w, h, PANEL);
    fb.fill_rect(x, y, w, 1, ELEV_HI);
    fb.fill_rect(x, y + h - 1, w, 1, ELEV_LO);
    fb.fill_rect(x, y, 1, h, LINE);
    fb.fill_rect(x + w - 1, y, 1, h, LINE);
    let rx = x + w - 1;
    let ry = y + h - 1;
    fb.fill_rect(x, y, 1, 1, BG);
    fb.fill_rect(rx, y, 1, 1, BG);
    fb.fill_rect(x, ry, 1, 1, BG);
    fb.fill_rect(rx, ry, 1, 1, BG);
}
