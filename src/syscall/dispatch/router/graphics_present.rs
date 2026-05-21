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

use crate::syscall::SyscallResult;
use crate::usercopy::copy_from_user;

const EFAULT: i32 = 14;
const EINVAL: i32 = 22;
const ENOTSUP: i32 = 95;

pub(super) fn handle(display: u64, surface: u64) -> SyscallResult {
    blit(display, surface, 0, 0, 0, 0, true)
}

pub(super) fn handle_rect(
    display: u64,
    surface: u64,
    x: u64,
    y: u64,
    w: u64,
    h: u64,
) -> SyscallResult {
    blit(display, surface, x, y, w, h, false)
}

fn blit(display: u64, surface: u64, x: u64, y: u64, w: u64, h: u64, full: bool) -> SyscallResult {
    if display != 0 {
        return super::super::util::errno(EINVAL);
    }
    let span = match super::graphics_backend::surface_span_for_id(surface) {
        Ok(v) => v,
        Err(e) => return super::super::util::errno(e),
    };
    let Some(fb) = crate::kernel_core::init::framebuffer::framebuffer_state() else {
        return super::super::util::errno(ENOTSUP);
    };
    if fb.frame_len().is_none() {
        return super::super::util::errno(EINVAL);
    };
    let fb_w = fb.width as usize;
    let fb_h = fb.height as usize;
    let fb_stride_bytes = fb.stride as usize;
    let bytes_per_pixel = core::mem::size_of::<u32>();
    let Some(src_len) = fb_w.checked_mul(fb_h).and_then(|px| px.checked_mul(bytes_per_pixel))
    else {
        return super::super::util::errno(EINVAL);
    };
    if span < src_len {
        return super::super::util::errno(EINVAL);
    }
    let rect_x = x as usize;
    let rect_y = y as usize;
    let rect_w = if full { fb_w } else { w as usize };
    let rect_h = if full { fb_h } else { h as usize };
    if rect_w == 0 || rect_h == 0 {
        return super::super::util::errno(EINVAL);
    }
    if rect_x >= fb_w || rect_y >= fb_h {
        return super::super::util::errno(EINVAL);
    }
    if rect_x.saturating_add(rect_w) > fb_w || rect_y.saturating_add(rect_h) > fb_h {
        return super::super::util::errno(EINVAL);
    }
    let mut bounce = [0u8; 4096];
    let dst = (fb.base_va.as_u64() + fb.offset as u64) as *mut u8;
    let row_bytes = rect_w * bytes_per_pixel;
    let src_stride = fb_w * bytes_per_pixel;
    let dst_stride = fb_stride_bytes;
    for row in 0..rect_h {
        let src_row_off = ((rect_y + row) * src_stride) + (rect_x * bytes_per_pixel);
        let dst_row_off = ((rect_y + row) * dst_stride) + (rect_x * bytes_per_pixel);
        let mut copied = 0usize;
        while copied < row_bytes {
            let chunk = core::cmp::min(bounce.len(), row_bytes - copied);
            let src = surface + (src_row_off + copied) as u64;
            if copy_from_user(src, &mut bounce[..chunk]).is_err() {
                return super::super::util::errno(EFAULT);
            }
            let dst_off = dst_row_off + copied;
            for i in 0..chunk {
                unsafe { core::ptr::write_volatile(dst.add(dst_off + i), bounce[i]) };
            }
            copied += chunk;
        }
    }
    // The GOP framebuffer is reached through the write-back directmap when
    // map_framebuffer is unavailable; flush so the stores hit the scanout.
    unsafe { core::arch::asm!("wbinvd", options(nostack, preserves_flags)) };
    SyscallResult::success_audited(0)
}
