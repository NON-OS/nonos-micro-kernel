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

use super::log::log_handoff_fb;
use super::marker::paint_mapped_marker;
use super::state::{KernelFramebuffer, FRAMEBUFFER};
use crate::boot::handoff::BootHandoffV1;
use crate::memory::addr::PhysAddr;

pub(crate) fn init_framebuffer(handoff: &BootHandoffV1) {
    let Some(fb) = handoff.framebuffer() else {
        return;
    };
    if fb.width == 0 || fb.height == 0 || fb.stride == 0 || fb.ptr == 0 {
        return;
    }
    log_handoff_fb(fb.ptr, fb.stride, fb.pixel_format);
    let row_bytes = (fb.width as u64).saturating_mul(core::mem::size_of::<u32>() as u64);
    if (fb.stride as u64) < row_bytes {
        return;
    }
    let Some(frame_len) = (fb.stride as usize).checked_mul(fb.height as usize) else {
        return;
    };
    let base = fb.ptr & !0xFFF;
    let offset = (fb.ptr - base) as usize;
    let fb_size = core::cmp::max(fb.size as usize, frame_len);
    let Some(map_len) = offset.checked_add(fb_size) else {
        return;
    };
    let Ok(base_va) = crate::memory::mmio::map_framebuffer(PhysAddr::new(base), map_len) else {
        return;
    };
    let bgr = matches!(fb.pixel_format, 1 | 3);
    FRAMEBUFFER.call_once(|| KernelFramebuffer {
        width: fb.width,
        height: fb.height,
        stride: fb.stride,
        base_va,
        offset,
        bgr,
    });
    paint_mapped_marker(base_va, offset, fb.stride, fb.width);
}
