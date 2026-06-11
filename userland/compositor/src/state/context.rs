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

use super::{AttachCache, CursorTracker, DamageAccumulator, FocusTable, SceneTable};

pub struct Context {
    pub gfx_port: u32,
    pub resource_id: u32,
    pub width: u32,
    pub height: u32,
    pub stride: u32,
    pub backing_len: u64,
    pub backing_va: u64,
    // GOP mode presents through MkSurfacePresent (kernel blit to the UEFI
    // framebuffer) instead of the virtio-gpu transfer/scanout/flush ops.
    pub gop_mode: bool,
    pub surface_handle: u64,
    pub first_scanout_done: bool,
    pub scanout_error_reported: bool,
    pub next_request_id: u32,
    pub scene: SceneTable,
    pub damage: DamageAccumulator,
    pub focus: FocusTable,
    pub cursor: CursorTracker,
    pub attach: AttachCache,
}

impl Context {
    pub fn issue_request_id(&mut self) -> u32 {
        let id = self.next_request_id;
        self.next_request_id = id.wrapping_add(1).max(1);
        id
    }
}
