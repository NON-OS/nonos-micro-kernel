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
    for row in 0..super::logo_bits::LOGO_H {
        let (left, right) = super::logo_bits::LOGO_ROWS[row as usize];
        for col in 0..super::logo_bits::LOGO_W {
            let bits = if col < 32 { left } else { right };
            let bit = if col < 32 { 31 - col } else { 63 - col };
            if bits & (1 << bit) == 0 {
                continue;
            }
            let px = x + col * s / super::logo_bits::LOGO_W;
            let py = y + row * s / super::logo_bits::LOGO_H;
            let nx = x + (col + 1) * s / super::logo_bits::LOGO_W;
            let ny = y + (row + 1) * s / super::logo_bits::LOGO_H;
            fb.fill_rect(px, py, nx.saturating_sub(px).max(1), ny.saturating_sub(py).max(1), c);
        }
    }
}
