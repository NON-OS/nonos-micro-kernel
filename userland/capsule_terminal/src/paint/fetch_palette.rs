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

pub fn draw_palette(fb: &mut PaintBuffer, x: u32, y: u32) {
    let colors: [u32; 8] = [
        0xFF35D07A, 0xFF5AA9E6, 0xFF7CF0A8, 0xFFE5C07B, 0xFFE5484D, 0xFFB87CF0, 0xFF5AE6D0,
        0xFFB9C2CC,
    ];
    for (i, c) in colors.iter().enumerate() {
        fb.fill_rect(x + i as u32 * 18, y, 14, 10, *c);
    }
}
