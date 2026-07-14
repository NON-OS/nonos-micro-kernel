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

use super::current::current_framebuffer_info;
use super::usable::mode_usable;
use crate::handoff::types::FramebufferInfo;
use uefi::prelude::*;
use uefi::proto::console::gop::GraphicsOutput;
use uefi::table::boot::BootServices;

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
