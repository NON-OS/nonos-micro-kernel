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

use super::constants::pixel_format;

#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct FramebufferInfo {
    pub ptr: u64,
    pub size: u64,
    pub width: u32,
    pub height: u32,
    /// Row pitch in bytes, never pixels.
    pub stride: u32,
    pub pixel_format: u32,
    pub cursor_y: u32,
    pub reserved: u32,
}

impl FramebufferInfo {
    #[inline]
    pub fn is_valid(&self) -> bool {
        self.ptr != 0 && self.width > 0 && self.height > 0 && self.stride > 0
    }

    #[inline]
    pub fn bytes_per_pixel(&self) -> u32 {
        match self.pixel_format {
            pixel_format::RGB | pixel_format::BGR => 3,
            pixel_format::RGBX | pixel_format::BGRX => 4,
            _ => 4,
        }
    }
}
