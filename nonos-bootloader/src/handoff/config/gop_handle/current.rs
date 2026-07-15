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

use super::usable::mode_usable;
use crate::display::get_cursor_y;
use crate::handoff::config::framebuffer::stride_to_bytes;
use crate::handoff::types::FramebufferInfo;
use uefi::proto::console::gop::GraphicsOutput;

pub(super) fn current_framebuffer_info(gop: &mut GraphicsOutput) -> Option<FramebufferInfo> {
    let mode_info = gop.current_mode_info();
    let pixel_format = mode_usable(&mode_info)?;
    let (width, height) = mode_info.resolution();
    let stride = stride_to_bytes(mode_info.stride() as u32, width as u32)? as usize;
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
