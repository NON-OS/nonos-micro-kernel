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

use super::theme::{ACCENT, TRACK_BG};

/// Four ascending bars ending at `right_x`. Signal is the driver's 0..=100 RSSI
/// percentage, so an unlit bar means the driver reported that little, not that
/// the panel failed to read it.
pub fn draw_bars(fb: &mut PaintBuffer, right_x: u32, cy: i32, signal: u8) {
    let bar_w = 3u32;
    let gap = 3u32;
    let lit = (signal as u32 * 4 + 99) / 100;
    for i in 0..4u32 {
        let h = 4 + i * 3;
        let x = right_x - (4 - i) * (bar_w + gap);
        let top = cy - (h / 2) as i32;
        if top < 0 {
            continue;
        }
        let argb = if i < lit { ACCENT } else { TRACK_BG };
        fb.fill_round(x, top as u32, bar_w, h, 1, argb);
    }
}
