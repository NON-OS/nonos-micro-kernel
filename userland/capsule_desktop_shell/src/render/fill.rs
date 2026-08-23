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
        let (y0f, y1f) = source_span(row, sh, dh);
        let sy_last = core::cmp::min((y1f - 1) >> 16, sh as u64 - 1);
        for col in 0..dw {
            let px = dx + col;
            if px >= surface_width {
                break;
            }
            let (x0f, x1f) = source_span(col, sw, dw);
            let sx_last = core::cmp::min((x1f - 1) >> 16, sw as u64 - 1);
            let (mut sr, mut sg, mut sb, mut sa, mut wsum) = (0u64, 0u64, 0u64, 0u64, 0u64);
            for syy in (y0f >> 16)..=sy_last {
                let wy = core::cmp::min(y1f, (syy + 1) << 16) - core::cmp::max(y0f, syy << 16);
                let bse = (syy as usize * sw as usize) * 4;
                for sxx in (x0f >> 16)..=sx_last {
                    let wx = core::cmp::min(x1f, (sxx + 1) << 16) - core::cmp::max(x0f, sxx << 16);
                    let w = wx * wy;
                    let i = bse + sxx as usize * 4;
                    let a = rgba[i + 3] as u64;
                    sr += rgba[i] as u64 * a * w;
                    sg += rgba[i + 1] as u64 * a * w;
                    sb += rgba[i + 2] as u64 * a * w;
                    sa += a * w;
                    wsum += w;
                }
            }
            if wsum == 0 || sa == 0 {
                continue;
            }
            let a = (sa / wsum) as u32;
            let r = ((sr / sa) as u32).min(255);
            let g = ((sg / sa) as u32).min(255);
            let b = ((sb / sa) as u32).min(255);
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

fn source_span(index: u32, src: u32, dst: u32) -> (u64, u64) {
    let lo = ((index as u64 * src as u64) << 16) / dst as u64;
    let hi = (((index as u64 + 1) * src as u64) << 16) / dst as u64;
    (lo, core::cmp::max(hi, lo + 1))
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
        let (y0f, y1f) = source_span(row, sh, dh);
        let sy_last = core::cmp::min((y1f - 1) >> 16, sh as u64 - 1);
        for col in 0..dw {
            let px = dx + col;
            if px >= surface_width {
                break;
            }
            let (x0f, x1f) = source_span(col, sw, dw);
            let sx_last = core::cmp::min((x1f - 1) >> 16, sw as u64 - 1);
            let (mut sa, mut wsum) = (0u64, 0u64);
            for syy in (y0f >> 16)..=sy_last {
                let wy = core::cmp::min(y1f, (syy + 1) << 16) - core::cmp::max(y0f, syy << 16);
                let bse = (syy as usize * sw as usize) * 4;
                for sxx in (x0f >> 16)..=sx_last {
                    let wx = core::cmp::min(x1f, (sxx + 1) << 16) - core::cmp::max(x0f, sxx << 16);
                    let w = wx * wy;
                    sa += rgba[bse + sxx as usize * 4 + 3] as u64 * w;
                    wsum += w;
                }
            }
            if wsum == 0 || sa == 0 {
                continue;
            }
            let a = (sa / wsum) as u32;
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
