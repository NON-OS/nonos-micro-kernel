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

use super::gop_handle::try_gop_handle;
use crate::handoff::types::FramebufferInfo;
use uefi::proto::console::gop::GraphicsOutput;
use uefi::table::boot::BootServices;
use uefi::Identify;

/// Get framebuffer info from GOP. Tries all handles for multi-GPU systems (Optimus).
pub fn get_framebuffer_info(bs: &BootServices) -> FramebufferInfo {
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
