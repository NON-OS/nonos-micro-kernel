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
    base_va: u64,
    stride_bytes: u32,
    surface_width: u32,
    surface_height: u32,
    rect_x: u32,
    rect_y: u32,
    rect_w: u32,
    rect_h: u32,
    argb: u32,
) {
    if rect_w == 0 || rect_h == 0 || rect_x >= surface_width || rect_y >= surface_height {
        return;
    }
    let w = core::cmp::min(rect_w, surface_width - rect_x);
    let h = core::cmp::min(rect_h, surface_height - rect_y);
    let stride = stride_bytes as usize;
    for row in 0..h as usize {
        let row_base = base_va as usize + (rect_y as usize + row) * stride + rect_x as usize * 4;
        for col in 0..w as usize {
            let cell = (row_base + col * 4) as *mut u32;
            // SAFETY: rect is clipped to the surface above.
            unsafe { core::ptr::write_volatile(cell, argb) };
        }
    }
}

/// Alpha-blit an RGBA8 source (bytes R,G,B,A, row-major), area-averaged into the
/// destination rect and composited over the surface. Lets the dock render a real
/// anti-aliased icon asset instead of a 1-bit bitmap. Mirrors the app-skeleton
/// blit but writes the raw surface directly.
#[allow(clippy::too_many_arguments)]
pub fn blit_rgba8_scaled(
    base_va: u64,
    stride_bytes: u32,
    surface_width: u32,
    surface_height: u32,
    dx: u32,
    dy: u32,
    dw: u32,
    dh: u32,
    rgba: &[u8],
    sw: u32,
    sh: u32,
) {
    if dw == 0 || dh == 0 || sw == 0 || sh == 0 {
        return;
    }
    if rgba.len() < (sw as usize) * (sh as usize) * 4 {
        return;
    }
    let stride = stride_bytes as usize;
    for row in 0..dh {
        let py = dy + row;
        if py >= surface_height {
            break;
        }
        let sy0 = row * sh / dh;
        let sy1 = core::cmp::max(sy0 + 1, (row + 1) * sh / dh).min(sh);
        for col in 0..dw {
            let px = dx + col;
            if px >= surface_width {
                break;
            }
            let sx0 = col * sw / dw;
            let sx1 = core::cmp::max(sx0 + 1, (col + 1) * sw / dw).min(sw);
            let (mut sr, mut sg, mut sb, mut sa, mut cnt) = (0u32, 0u32, 0u32, 0u32, 0u32);
            for syy in sy0..sy1 {
                let bse = (syy as usize * sw as usize) * 4;
                for sxx in sx0..sx1 {
                    let i = bse + sxx as usize * 4;
                    let a = rgba[i + 3] as u32;
                    sr += rgba[i] as u32 * a;
                    sg += rgba[i + 1] as u32 * a;
                    sb += rgba[i + 2] as u32 * a;
                    sa += a;
                    cnt += 1;
                }
            }
            if cnt == 0 || sa == 0 {
                continue;
            }
            let a = sa / cnt;
            let r = (sr / sa).min(255);
            let g = (sg / sa).min(255);
            let b = (sb / sa).min(255);
            let cell = (base_va as usize + py as usize * stride + px as usize * 4) as *mut u32;
            let out = if a >= 255 {
                (r << 16) | (g << 8) | b
            } else {
                // SAFETY: cell is inside the surface (clipped above).
                let d = unsafe { core::ptr::read_volatile(cell) };
                let dr = (d >> 16) & 0xFF;
                let dg = (d >> 8) & 0xFF;
                let db = d & 0xFF;
                let ia = 255 - a;
                (((r * a + dr * ia) / 255) << 16)
                    | (((g * a + dg * ia) / 255) << 8)
                    | ((b * a + db * ia) / 255)
            };
            // SAFETY: cell is inside the surface (clipped above).
            unsafe { core::ptr::write_volatile(cell, 0xFF00_0000 | out) };
        }
    }
}

/// Like `blit_rgba8_scaled` but recolors the source: it uses only the source
/// alpha (shape coverage) and paints it in `tint`, so a single teal logo asset
/// renders crisply in any per-app accent colour.
#[allow(clippy::too_many_arguments)]
pub fn blit_rgba8_tinted(
    base_va: u64,
    stride_bytes: u32,
    surface_width: u32,
    surface_height: u32,
    dx: u32,
    dy: u32,
    dw: u32,
    dh: u32,
    rgba: &[u8],
    sw: u32,
    sh: u32,
    tint: u32,
) {
    if dw == 0 || dh == 0 || sw == 0 || sh == 0 {
        return;
    }
    if rgba.len() < (sw as usize) * (sh as usize) * 4 {
        return;
    }
    let tr = (tint >> 16) & 0xFF;
    let tg = (tint >> 8) & 0xFF;
    let tb = tint & 0xFF;
    let stride = stride_bytes as usize;
    for row in 0..dh {
        let py = dy + row;
        if py >= surface_height {
            break;
        }
        let sy0 = row * sh / dh;
        let sy1 = core::cmp::max(sy0 + 1, (row + 1) * sh / dh).min(sh);
        for col in 0..dw {
            let px = dx + col;
            if px >= surface_width {
                break;
            }
            let sx0 = col * sw / dw;
            let sx1 = core::cmp::max(sx0 + 1, (col + 1) * sw / dw).min(sw);
            let (mut sa, mut cnt) = (0u32, 0u32);
            for syy in sy0..sy1 {
                let bse = (syy as usize * sw as usize) * 4;
                for sxx in sx0..sx1 {
                    sa += rgba[bse + sxx as usize * 4 + 3] as u32;
                    cnt += 1;
                }
            }
            if cnt == 0 || sa == 0 {
                continue;
            }
            let a = sa / cnt;
            let cell = (base_va as usize + py as usize * stride + px as usize * 4) as *mut u32;
            let out = if a >= 255 {
                (tr << 16) | (tg << 8) | tb
            } else {
                // SAFETY: cell is inside the surface (clipped above).
                let d = unsafe { core::ptr::read_volatile(cell) };
                let dr = (d >> 16) & 0xFF;
                let dg = (d >> 8) & 0xFF;
                let db = d & 0xFF;
                let ia = 255 - a;
                (((tr * a + dr * ia) / 255) << 16)
                    | (((tg * a + dg * ia) / 255) << 8)
                    | ((tb * a + db * ia) / 255)
            };
            // SAFETY: cell is inside the surface (clipped above).
            unsafe { core::ptr::write_volatile(cell, 0xFF00_0000 | out) };
        }
    }
}
