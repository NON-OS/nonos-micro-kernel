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

use super::code::Matrix;
use super::theme;

pub fn draw_matrix(fb: &mut PaintBuffer, matrix: &Matrix, top: u32) -> u32 {
    let span = (matrix.width + 2 * theme::QUIET) as u32;
    let across = fb.width.saturating_sub(2 * theme::MARGIN as u32);
    let down = fb.height.saturating_sub(top + theme::MARGIN as u32);
    let scale = (across.min(down) / span.max(1)).max(1);
    let side = span * scale;
    let left = fb.width.saturating_sub(side) / 2;
    fb.fill_rect(left, top, side, side, theme::MODULE_LIGHT);
    let origin = theme::QUIET as u32 * scale;
    for y in 0..matrix.width {
        for x in 0..matrix.width {
            if !matrix.is_dark(x, y) {
                continue;
            }
            let px = left + origin + x as u32 * scale;
            let py = top + origin + y as u32 * scale;
            fb.fill_rect(px, py, scale, scale, theme::MODULE_DARK);
        }
    }
    top + side
}
