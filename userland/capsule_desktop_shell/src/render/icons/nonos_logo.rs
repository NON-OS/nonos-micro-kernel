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

use crate::state::Context;

use super::super::fill::fill_rect;

pub fn paint(ctx: &Context, x: u32, y: u32, size: u32, color: u32) {
    for row in 0..super::nonos_logo_bits::H {
        let (left, right) = super::nonos_logo_bits::ROWS[row as usize];
        for col in 0..super::nonos_logo_bits::W {
            let bits = if col < 32 { left } else { right };
            let bit = if col < 32 { 31 - col } else { 63 - col };
            if bits & (1 << bit) == 0 {
                continue;
            }
            let px = x + col * size / super::nonos_logo_bits::W;
            let py = y + row * size / super::nonos_logo_bits::H;
            let nx = x + (col + 1) * size / super::nonos_logo_bits::W;
            let ny = y + (row + 1) * size / super::nonos_logo_bits::H;
            fill_rect(
                ctx.backing_va,
                ctx.stride,
                ctx.width,
                ctx.height,
                px,
                py,
                nx.saturating_sub(px).max(1),
                ny.saturating_sub(py).max(1),
                color,
            );
        }
    }
}
