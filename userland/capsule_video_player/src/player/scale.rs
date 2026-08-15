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

use alloc::vec::Vec;

pub fn letterbox(sw: u32, sh: u32, dw: u32, dh: u32) -> (u32, u32, u32, u32) {
    if sw == 0 || sh == 0 {
        return (0, 0, 0, 0);
    }
    let wide = (sw as u64) * (dh as u64) >= (dw as u64) * (sh as u64);
    let (w, h) = if wide {
        (dw, (((sh as u64) * (dw as u64)) / sw as u64) as u32)
    } else {
        ((((sw as u64) * (dh as u64)) / sh as u64) as u32, dh)
    };
    ((dw.saturating_sub(w)) / 2, (dh.saturating_sub(h)) / 2, w, h)
}

pub fn column_map(sw: u32, dw: u32) -> Vec<u32> {
    let mut m = Vec::with_capacity(dw as usize);
    for i in 0..dw {
        m.push((((i as u64) * (sw as u64)) / dw.max(1) as u64) as u32);
    }
    m
}

pub fn scale_into(
    src: &[u32],
    sw: u32,
    sh: u32,
    dst: &mut [u32],
    dst_stride: u32,
    rect: (u32, u32, u32, u32),
    cols: &[u32],
) {
    let (rx, ry, rw, rh) = rect;
    if rw == 0 || rh == 0 || sw == 0 || sh == 0 || cols.len() < rw as usize {
        return;
    }
    let last = sw as usize - 1;
    for row in 0..rh {
        let sy = (((row as u64) * (sh as u64)) / rh as u64) as u32;
        let s0 = (sy as usize) * (sw as usize);
        let d0 = ((ry + row) as usize) * (dst_stride as usize) + rx as usize;
        if s0 + sw as usize > src.len() || d0 + rw as usize > dst.len() {
            return;
        }
        for col in 0..rw as usize {
            let sx = (cols[col] as usize).min(last);
            dst[d0 + col] = src[s0 + sx];
        }
    }
}
