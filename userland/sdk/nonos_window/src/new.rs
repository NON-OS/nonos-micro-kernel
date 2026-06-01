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

use nonos_surface::{attach, create, SurfaceDescriptor, SURFACE_FORMAT_ARGB8888};

use super::types::Window;

impl Window {
    pub fn new(width: u32, height: u32) -> Option<Window> {
        let req = SurfaceDescriptor {
            width,
            height,
            stride: width * 4,
            format: SURFACE_FORMAT_ARGB8888,
            byte_len: width as u64 * height as u64 * 4,
            base_va: 0,
            flags: 0,
        };
        let handle = create(&req);
        if handle < 0 {
            return None;
        }
        let h = handle as u64;
        let mut mapped = SurfaceDescriptor::default();
        if attach(h, &mut mapped) < 0 || mapped.base_va == 0 {
            return None;
        }
        Some(Window {
            handle: h,
            base: mapped.base_va as *mut u32,
            pixels: (mapped.byte_len / 4) as usize,
            width: mapped.width,
            height: mapped.height,
        })
    }
}
