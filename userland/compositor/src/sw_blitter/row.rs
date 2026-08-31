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

//! Compositing one row of a layer onto the frame. Holds no surface, so
//! `mechanism_proofs` includes it and checks it pixel for pixel.

/// Source-over one row of ARGB pixels onto `dst`.
///
/// Opaque pixels are taken in runs so the common case is a memcpy rather than
/// an access per pixel: a layer is opaque across almost its whole width, and
/// this is every window of the desktop on the software path.
pub fn composite_row(src: &[u32], dst: &mut [u32]) {
    let w = core::cmp::min(src.len(), dst.len());
    let mut col = 0usize;
    while col < w {
        let a = src[col] >> 24;
        if a == 0xFF {
            let start = col;
            while col < w && src[col] >> 24 == 0xFF {
                col += 1;
            }
            dst[start..col].copy_from_slice(&src[start..col]);
        } else if a == 0 {
            // Fully transparent: leave whatever is underneath.
            col += 1;
        } else {
            dst[col] = blend(src[col], dst[col], a);
            col += 1;
        }
    }
}

/// Source-over blend of two ARGB pixels, weighted by the source alpha. The
/// result is written back fully opaque, since the frame has no alpha.
pub fn blend(src: u32, dst: u32, a: u32) -> u32 {
    let ia = 255 - a;
    let rb = (src & 0x00FF_00FF) * a + (dst & 0x00FF_00FF) * ia;
    let g = ((src >> 8) & 0xFF) * a + ((dst >> 8) & 0xFF) * ia;
    let rb = ((rb + 0x0001_0001 + ((rb >> 8) & 0x00FF_00FF)) >> 8) & 0x00FF_00FF;
    let g = (g + 1 + (g >> 8)) >> 8;
    0xFF00_0000 | rb | (g << 8)
}
