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
/// The context every in-OS 3D client shares until per-capsule contexts exist.
pub const RENDER_CTX_ID: u32 = 1;

/// Reserved for the boot proof so it cannot collide with a client's own
/// resources, which start above this.
pub const PROBE_RESOURCE_ID: u32 = 0x1000;
pub const PROBE_SURFACE_HANDLE: u32 = 0x1000;

pub const PROBE_WIDTH: u32 = 64;
pub const PROBE_HEIGHT: u32 = 64;
pub const PROBE_BYTES: u64 = (PROBE_WIDTH * PROBE_HEIGHT * 4) as u64;

/// Clear colour, and the same value as it must land in memory. The format is
/// B8G8R8A8_UNORM, so opaque blue is stored blue-first.
pub const PROBE_CLEAR_B: f32 = 1.0;
pub const PROBE_PIXEL: [u8; 4] = [0xFF, 0x00, 0x00, 0xFF];
