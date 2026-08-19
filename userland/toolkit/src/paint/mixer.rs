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


pub fn mix(dst: u32, src: u32, a: u32) -> u32 {
    let ia = 255 - a;
    let dr = (dst >> 16) & 0xFF;
    let dg = (dst >> 8) & 0xFF;
    let db = dst & 0xFF;
    let sr = (src >> 16) & 0xFF;
    let sg = (src >> 8) & 0xFF;
    let sb = src & 0xFF;
    let r = (sr * a + dr * ia + 127) / 255;
    let g = (sg * a + dg * ia + 127) / 255;
    let b = (sb * a + db * ia + 127) / 255;
    (r << 16) | (g << 8) | b
}

pub fn lerp_argb(from: u32, to: u32, t: u32) -> u32 {
    let it = 255 - t;
    let mut out = 0u32;
    let mut shift = 0;
    while shift < 32 {
        let f = (from >> shift) & 0xFF;
        let v = (to >> shift) & 0xFF;
        out |= ((v * t + f * it + 127) / 255) << shift;
        shift += 8;
    }
    out
}
