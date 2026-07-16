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

use crate::boot::handoff::BootHandoffV1;
use crate::memory::addr::PhysAddr;
use crate::memory::addr::VirtAddr;
use spin::Once;

#[derive(Clone, Copy)]
pub(crate) struct KernelFramebuffer {
    pub width: u32,
    pub height: u32,
    pub stride: u32,
    pub base_va: VirtAddr,
    pub offset: usize,
    pub bgr: bool,
}

impl KernelFramebuffer {
    pub(crate) fn frame_len(self) -> Option<usize> {
        (self.stride as usize).checked_mul(self.height as usize)
    }
}

static FRAMEBUFFER: Once<KernelFramebuffer> = Once::new();

pub(crate) fn framebuffer_state() -> Option<&'static KernelFramebuffer> {
    FRAMEBUFFER.get()
}

pub(super) fn init_framebuffer(handoff: &BootHandoffV1) {
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

// Third breadcrumb segment (green), painted through the kernel's own
// mapping: proves the paging switch survived and the framebuffer mapped.
// The first two segments are painted by boot::entry_marker over the
// bootloader's tables before the switch.
fn paint_mapped_marker(base_va: VirtAddr, offset: usize, stride: u32, width: u32) {
    const SEG_WIDTH: u32 = 180;
    const SEG_GAP: u32 = 20;
    const SEG_HEIGHT: u32 = 10;
    let x0 = 2 * (SEG_WIDTH + SEG_GAP);
    if x0 + SEG_WIDTH > width {
        return;
    }
    let row_px = (stride / 4) as u64;
    let base = (base_va.as_u64() + offset as u64) as *mut u32;
    for y in 0..SEG_HEIGHT as u64 {
        for x in 0..SEG_WIDTH as u64 {
            let off = y * row_px + x0 as u64 + x;
            // SAFETY: bounds checked against the handoff geometry; the
            // mapping was just created above with room for the full frame.
            unsafe { core::ptr::write_volatile(base.add(off as usize), 0xFF00_D060) };
        }
    }
}

fn log_handoff_fb(ptr: u64, stride: u32, format: u32) {
    let mut buf = [0u8; 16];
    crate::sys::serial::print(b"[FB] handoff ptr=0x");
    crate::sys::serial::print(hex_u64(ptr, &mut buf));
    crate::sys::serial::print(b" stride=0x");
    let mut b2 = [0u8; 16];
    crate::sys::serial::print(hex_u64(stride as u64, &mut b2));
    crate::sys::serial::print(b" fmt=0x");
    let mut b3 = [0u8; 16];
    crate::sys::serial::println(hex_u64(format as u64, &mut b3));
}

fn hex_u64(mut v: u64, out: &mut [u8; 16]) -> &[u8] {
    let mut k = out.len();
    loop {
        k -= 1;
        let d = (v & 0xF) as u8;
        out[k] = if d < 10 { b'0' + d } else { b'a' + d - 10 };
        v >>= 4;
        if v == 0 || k == 0 {
            break;
        }
    }
    &out[k..]
}
