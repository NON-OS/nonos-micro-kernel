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
}

impl KernelFramebuffer {
	pub(crate) fn frame_len(self) -> Option<usize> {
		(self.stride as usize)
			.checked_mul(self.height as usize)
	}
}

static FRAMEBUFFER: Once<KernelFramebuffer> = Once::new();

pub(crate) fn framebuffer_state() -> Option<&'static KernelFramebuffer> {
	FRAMEBUFFER.get()
}

fn fill_rect(fb: &KernelFramebuffer, x0: u32, y0: u32, w: u32, h: u32, color: u32) {
	let start = fb.base_va.as_u64() as usize + fb.offset;
	let x_end = (x0 + w).min(fb.width);
	let y_end = (y0 + h).min(fb.height);
	for y in y0..y_end {
		let row = start + (y as usize) * fb.stride as usize;
		for x in x0..x_end {
			// SAFETY: x<width, y<height, stride>=width*4: within frame.
			unsafe { core::ptr::write_volatile((row + (x as usize) * 4) as *mut u32, color) };
		}
	}
}

pub(crate) fn draw_desktop() {
	let Some(fb) = framebuffer_state() else {
		crate::sys::serial::println(b"[DESKTOP] no framebuffer state");
		return;
	};
	fill_rect(fb, 0, 0, fb.width, fb.height, 0xFF1A2738);
	fill_rect(fb, 0, 0, fb.width, 28, 0xFF0C141E);
	fill_rect(fb, 0, fb.height.saturating_sub(56), fb.width, 56, 0xFF0C141E);
	let dock_x = fb.width / 2;
	fill_rect(fb, dock_x.saturating_sub(120), fb.height.saturating_sub(48), 240, 40, 0xFF1E66A8);
	// Push write-back directmap stores out to the scanout device.
	unsafe { core::arch::asm!("wbinvd", options(nostack, preserves_flags)) };
	crate::sys::serial::println(b"[DESKTOP] drawn");
}

pub(super) fn init_framebuffer(handoff: &BootHandoffV1) {
	let Some(fb) = handoff.framebuffer() else {
		return;
	};
	if fb.width == 0 || fb.height == 0 || fb.stride == 0 || fb.ptr == 0 {
		return;
	}
	let row_bytes = (fb.width as u64).saturating_mul(core::mem::size_of::<u32>() as u64);
	if (fb.stride as u64) < row_bytes {
		return;
	}
	let Some(frame_len) = (fb.stride as usize)
		.checked_mul(fb.height as usize)
	else {
		return;
	};
	let base = fb.ptr & !0xFFF;
	let offset = (fb.ptr - base) as usize;
	let fb_size = core::cmp::max(fb.size as usize, frame_len);
	let Some(map_len) = offset.checked_add(fb_size) else {
		return;
	};
	let base_va = match crate::memory::mmio::map_framebuffer(PhysAddr::new(base), map_len) {
		Ok(v) => v,
		Err(_) => {
			crate::sys::serial::println(b"[FBINIT] map_framebuffer failed; using directmap");
			VirtAddr::new(crate::memory::layout::DIRECTMAP_BASE + base)
		}
	};
	FRAMEBUFFER.call_once(|| KernelFramebuffer {
		width: fb.width,
		height: fb.height,
		stride: fb.stride,
		base_va,
		offset,
	});
}
