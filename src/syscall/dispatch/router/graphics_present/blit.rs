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

use super::consts::{EFAULT, EINVAL, ENOTSUP};
use crate::syscall::dispatch::util;
use crate::syscall::SyscallResult;
use crate::usercopy::copy_from_user;

pub(super) fn blit(
    display: u64,
    surface: u64,
    span: usize,
    x: u64,
    y: u64,
    w: u64,
    h: u64,
    full: bool,
) -> SyscallResult {
    if display != 0 {
        return util::errno(EINVAL);
    }
    // `span` is the surface byte length from the attach record. Userland
    // mappings are not tracked in proc VMAs, so the size comes from the
    // surface registry; copy_from_user validates each source page below.
    let Some(fb) = crate::kernel_core::init::framebuffer::framebuffer_state() else {
        return util::errno(ENOTSUP);
    };
    if fb.frame_len().is_none() {
        return util::errno(EINVAL);
    };
    let fb_w = fb.width as usize;
    let fb_h = fb.height as usize;
    let fb_stride_bytes = fb.stride as usize;
    let bytes_per_pixel = core::mem::size_of::<u32>();
    let Some(src_len) = fb_w.checked_mul(fb_h).and_then(|px| px.checked_mul(bytes_per_pixel))
    else {
        return util::errno(EINVAL);
    };
    if span < src_len {
        return util::errno(EINVAL);
    }
    let rect_x = x as usize;
    let rect_y = y as usize;
    let rect_w = if full { fb_w } else { w as usize };
    let rect_h = if full { fb_h } else { h as usize };
    if rect_w == 0 || rect_h == 0 {
        return util::errno(EINVAL);
    }
    if rect_x >= fb_w || rect_y >= fb_h {
        return util::errno(EINVAL);
    }
    if rect_x.saturating_add(rect_w) > fb_w || rect_y.saturating_add(rect_h) > fb_h {
        return util::errno(EINVAL);
    }
    // Four-byte aligned so the destination stores below can move whole pixels.
    // A bare `[u8; N]` is only byte aligned, and reading `u32`s out of it would
    // depend on where the stack happened to put it.
    #[repr(align(4))]
    struct Bounce([u8; 4096]);
    let mut bounce = Bounce([0u8; 4096]);
    let dst = (fb.base_va.as_u64() + fb.offset as u64) as *mut u8;
    let row_bytes = rect_w * bytes_per_pixel;
    let src_stride = fb_w * bytes_per_pixel;
    let dst_stride = fb_stride_bytes;
    for row in 0..rect_h {
        let src_row_off = ((rect_y + row) * src_stride) + (rect_x * bytes_per_pixel);
        let dst_row_off = ((rect_y + row) * dst_stride) + (rect_x * bytes_per_pixel);
        let mut copied = 0usize;
        while copied < row_bytes {
            let chunk = core::cmp::min(bounce.0.len(), row_bytes - copied);
            let src = surface + (src_row_off + copied) as u64;
            if copy_from_user(src, &mut bounce.0[..chunk]).is_err() {
                return util::errno(EFAULT);
            }
            // Surface pixels are ARGB8888, stored B,G,R,A in memory, which is
            // exactly what a BGR framebuffer scans out. RGB firmware needs the
            // red and blue channels exchanged, exactly once: this swap was
            // duplicated, so it cancelled and RGB displays scanned out with red
            // and blue reversed. Chunks stay pixel-aligned (row_bytes and the
            // bounce length are both multiples of four).
            if !fb.bgr {
                for px in bounce.0[..chunk].chunks_exact_mut(4) {
                    px.swap(0, 2);
                }
            }
            let dst_off = dst_row_off + copied;
            // SAFETY: the rectangle was clamped to the framebuffer above, and
            // `dst_off + chunk` stays inside the row, so every store lands in
            // the mapped framebuffer.
            let dst_ptr = unsafe { dst.add(dst_off) };
            // Whole pixels: the framebuffer is write-combining, so the store
            // count is the cost and byte-wise was four times as many. Falls
            // back if a firmware stride leaves the destination unaligned.
            if (dst_ptr as usize) % 4 == 0 && chunk % 4 == 0 {
                for (i, px) in bounce.0[..chunk].chunks_exact(4).enumerate() {
                    let word = u32::from_ne_bytes([px[0], px[1], px[2], px[3]]);
                    // SAFETY: `dst_ptr` is 4-byte aligned on this branch and
                    // `i < chunk / 4`, so the write is in bounds and aligned.
                    unsafe { core::ptr::write_volatile((dst_ptr as *mut u32).add(i), word) };
                }
            } else {
                for (i, &b) in bounce.0[..chunk].iter().enumerate() {
                    // SAFETY: `i < chunk` and the row was bounds checked.
                    unsafe { core::ptr::write_volatile(dst_ptr.add(i), b) };
                }
            }
            copied += chunk;
        }
    }
    SyscallResult::success_audited(0)
}
