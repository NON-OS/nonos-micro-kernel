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

/// Normalize a GOP scanline stride to bytes for the handoff contract.
///
/// The spec says PixelsPerScanLine holds pixels, but firmware in the wild
/// disagrees: OVMF builds have been seen reporting bytes in that field,
/// while real laptop firmware reports pixels. The two ranges cannot
/// overlap for 32-bit modes: a byte stride is always at least width * 4,
/// and pixel padding never reaches four times the width. Below width the
/// value is garbage either way.
pub fn stride_to_bytes(stride: u32, width: u32) -> Option<u32> {
    let min_bytes = width.checked_mul(4)?;
    if stride >= min_bytes {
        return Some(stride);
    }
    if stride >= width {
        return stride.checked_mul(4);
    }
    None
}

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
        if let Some(stride_bytes) = stride_to_bytes(stride, width) {
            return FramebufferInfo {
                ptr,
                size: (stride_bytes as u64).saturating_mul(height as u64),
                width,
                height,
                stride: stride_bytes,
                pixel_format: if bgr { PIXEL_FORMAT_BGRX } else { PIXEL_FORMAT_RGBX },
                cursor_y: crate::display::get_cursor_y(),
                reserved: 0,
            };
        }
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
