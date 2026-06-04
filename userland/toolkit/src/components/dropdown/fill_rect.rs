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
pub fn fill_rect(
    buf: &mut [u32],
    stride: usize,
    w: u32,
    h: u32,
    rect: (u32, u32, u32, u32),
    color: u32,
) {
    let (x, y, rw, rh) = rect;
    let mut py = y.min(h) as usize;
    while py < (y + rh).min(h) as usize {
        let mut px = x.min(w) as usize;
        while px < (x + rw).min(w) as usize {
            let i = py.saturating_mul(stride).saturating_add(px);
            if i < buf.len() {
                buf[i] = color;
            }
            px += 1;
        }
        py += 1;
    }
}
