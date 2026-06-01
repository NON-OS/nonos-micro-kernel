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

pub fn blit_argb(
    backing_va: u64,
    stride_bytes: u32,
    backing_w: u32,
    backing_h: u32,
    src: &[u32],
    src_w: u32,
    src_h: u32,
) {
    if src_w == 0 || src_h == 0 || backing_w == 0 || backing_h == 0 {
        return;
    }
    let stride_px = (stride_bytes / 4) as usize;
    let dst_ptr = backing_va as *mut u32;
    for dy in 0..backing_h {
        let sy = (dy as u64 * src_h as u64 / backing_h as u64) as u32;
        let sy = if sy >= src_h { src_h - 1 } else { sy };
        let row_off_dst = (dy as usize) * stride_px;
        let row_off_src = (sy as usize) * (src_w as usize);
        for dx in 0..backing_w {
            let sx = (dx as u64 * src_w as u64 / backing_w as u64) as u32;
            let sx = if sx >= src_w { src_w - 1 } else { sx };
            let pixel = src[row_off_src + sx as usize];
            unsafe {
                core::ptr::write_volatile(dst_ptr.add(row_off_dst + dx as usize), pixel);
            }
        }
    }
}
