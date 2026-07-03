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

use super::box_page::TOP;

// Rect fill clipped to the page area below the chrome and to an optional
// screen-space clip rect ([x0, y0, x1, y1]).
pub(super) fn fill_page(
    fb: &mut PaintBuffer,
    x: i32,
    y: i32,
    w: i32,
    h: i32,
    color: u32,
    clip: Option<[i32; 4]>,
) {
    let mut x0 = x.max(0);
    let mut y0 = y.max(TOP);
    let mut x1 = (x + w).min(fb.width as i32);
    let mut y1 = (y + h).min(fb.height as i32);
    if let Some(c) = clip {
        x0 = x0.max(c[0]);
        y0 = y0.max(c[1]);
        x1 = x1.min(c[2]);
        y1 = y1.min(c[3]);
    }
    if x1 <= x0 || y1 <= y0 {
        return;
    }
    fb.fill_rect(x0 as u32, y0 as u32, (x1 - x0) as u32, (y1 - y0) as u32, color);
}
