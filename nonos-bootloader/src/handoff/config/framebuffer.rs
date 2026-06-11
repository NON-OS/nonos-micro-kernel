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

use super::gop_handle::{try_gop_handle, PIXEL_FORMAT_BGRX, PIXEL_FORMAT_RGBX};
use crate::handoff::types::FramebufferInfo;
use uefi::proto::console::gop::GraphicsOutput;
use uefi::table::boot::BootServices;
use uefi::Identify;

/// Get framebuffer info for the kernel handoff.
///
/// Prefer the framebuffer the display module already latched and drew the
/// splash to. It is the proven-good linear framebuffer for this machine and
/// its MMIO address stays valid across ExitBootServices, whereas re-opening
/// GOP here is fragile after mode setup and yields no usable framebuffer on
/// some firmware. The GOP re-query remains as a fallback for the headless
/// path where the splash never initialized.
pub fn get_framebuffer_info(bs: &BootServices) -> FramebufferInfo {
    if let Some((ptr, width, height, stride, bgr)) = crate::display::gop::latched_linear_fb() {
        return FramebufferInfo {
            ptr,
            size: (stride as u64).saturating_mul(height as u64),
            width,
            height,
            stride,
            pixel_format: if bgr { PIXEL_FORMAT_BGRX } else { PIXEL_FORMAT_RGBX },
            cursor_y: crate::display::get_cursor_y(),
            reserved: 0,
        };
    }
    if let Ok(handles) =
        bs.locate_handle_buffer(uefi::table::boot::SearchType::ByProtocol(&GraphicsOutput::GUID))
    {
        for (idx, &handle) in handles.iter().enumerate() {
            if let Some(fb) = try_gop_handle(bs, handle, idx) {
                return fb;
            }
        }
    }
    if let Ok(gop_handle) = bs.get_handle_for_protocol::<GraphicsOutput>() {
        if let Some(fb) = try_gop_handle(bs, gop_handle, 0) {
            return fb;
        }
    }
    FramebufferInfo {
        ptr: 0,
        size: 0,
        width: 0,
        height: 0,
        stride: 0,
        pixel_format: 0,
        cursor_y: 0,
        reserved: 0,
    }
}
