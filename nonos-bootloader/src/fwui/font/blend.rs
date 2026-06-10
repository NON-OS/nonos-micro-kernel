// NØNOS Operating System
// Copyright (C) 2026 NØNOS Contributors
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

pub fn blend(dst: u32, fg: u32, cov: u8) -> u32 {
    if cov == 0 {
        return dst;
    }
    if cov == 255 {
        return fg;
    }
    let a = cov as u32;
    let ia = 255 - a;
    let r = (((fg >> 16) & 0xFF) * a + ((dst >> 16) & 0xFF) * ia) / 255;
    let g = (((fg >> 8) & 0xFF) * a + ((dst >> 8) & 0xFF) * ia) / 255;
    let b = ((fg & 0xFF) * a + (dst & 0xFF) * ia) / 255;
    0xFF00_0000 | (r << 16) | (g << 8) | b
}
