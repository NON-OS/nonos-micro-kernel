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

use super::announce::announce;
use super::backing::alloc_backing;
use super::register::register_share;
use super::require_peers::require_peers;
use super::types::DesktopWindow;
use nonos_runtime::munmap;
use nonos_surface::destroy;

impl DesktopWindow {
    pub(crate) fn open(width: u32, height: u32, window_id: u32) -> Option<DesktopWindow> {
        let (base, stride, byte_len) = alloc_backing(width, height)?;
        let handle = register_share(base, width, height, stride, byte_len)?;
        let peers = require_peers()?;
        if !announce(&peers, window_id, handle, width, height) {
            let _ = destroy(handle);
            let _ = munmap(base as *mut u8, byte_len as usize);
            return None;
        }
        Some(DesktopWindow {
            handle,
            backing: base,
            pixels: (byte_len / 4) as usize,
            byte_len,
            width,
            height,
            window_id,
            peers,
            rid: 4,
        })
    }
}
