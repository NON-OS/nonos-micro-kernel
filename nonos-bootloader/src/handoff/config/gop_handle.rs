// NØNOS Operating System
// Copyright (C) 2026 NØNOS Contributors
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

use crate::display::get_cursor_y;
use crate::handoff::types::FramebufferInfo;
use uefi::prelude::*;
use uefi::proto::console::gop::{GraphicsOutput, ModeInfo, PixelFormat};
use uefi::table::boot::BootServices;

pub const PIXEL_FORMAT_RGBX: u32 = 2;
pub const PIXEL_FORMAT_BGRX: u32 = 3;

/// Try to get framebuffer info from a GOP handle. Returns None if invalid.
pub fn try_gop_handle(bs: &BootServices, handle: Handle, _idx: usize) -> Option<FramebufferInfo> {
    let mut gop = bs.open_protocol_exclusive::<GraphicsOutput>(handle).ok()?;
    if let Some(info) = current_framebuffer_info(&mut gop) {
        return Some(info);
    }
    let mode_count = gop.modes().len();
    for idx in 0..mode_count {
        let mode = match gop.query_mode(idx as u32) {
            Ok(mode) => mode,
            Err(_) => continue,
        };
        if mode_usable(mode.info()).is_none() {
            continue;
        }
        if gop.set_mode(&mode).is_ok() {
            if let Some(info) = current_framebuffer_info(&mut gop) {
                return Some(info);
            }
        }
    }
    None
}

fn current_framebuffer_info(gop: &mut GraphicsOutput) -> Option<FramebufferInfo> {
    let mode_info = gop.current_mode_info();
    let pixel_format = mode_usable(&mode_info)?;
    let (width, height) = mode_info.resolution();
    let stride =
        super::framebuffer::stride_to_bytes(mode_info.stride() as u32, width as u32)? as usize;
    let mut fb = gop.frame_buffer();
    let fb_addr = fb.as_mut_ptr() as u64;
    if fb_addr == 0 {
        return None;
    }
    let fb_size = fb.size() as u64;
    if fb_size == 0 {
        return None;
    }
    if (fb_size as usize) < stride.checked_mul(height)? {
        return None;
    }
    Some(FramebufferInfo {
        ptr: fb_addr,
        size: fb_size,
        width: width as u32,
        height: height as u32,
        stride: stride as u32,
        pixel_format,
        cursor_y: get_cursor_y(),
        reserved: 0,
    })
}

fn mode_usable(info: &ModeInfo) -> Option<u32> {
    let (width, height) = info.resolution();
    if width == 0 || height == 0 || info.stride() < width {
        return None;
    }
    match info.pixel_format() {
        PixelFormat::Rgb => Some(PIXEL_FORMAT_RGBX),
        PixelFormat::Bgr => Some(PIXEL_FORMAT_BGRX),
        PixelFormat::Bitmask | PixelFormat::BltOnly => None,
    }
}
