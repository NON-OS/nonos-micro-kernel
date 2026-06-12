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

pub fn logo(fb: &mut PaintBuffer, x: u32, y: u32, s: u32) {
    let c = 0xFF66_FFFF;
    let bg = 0xFF0D_1218;
    fb.fill_rect(x + s / 5, y, s * 3 / 5, s / 8, c);
    fb.fill_rect(x + s / 5, y + s * 7 / 8, s * 3 / 5, s / 8, c);
    fb.fill_rect(x, y + s / 5, s / 8, s * 3 / 5, c);
    fb.fill_rect(x + s * 7 / 8, y + s / 5, s / 8, s * 3 / 5, c);
    fb.fill_rect(x + s / 4, y + s / 4, s / 2, s / 2, bg);
    for i in 0..7 {
        fb.fill_rect(x + s / 5 + i * s / 12, y + s * 3 / 4 - i * s / 12, s / 8, s / 8, c);
    }
}
