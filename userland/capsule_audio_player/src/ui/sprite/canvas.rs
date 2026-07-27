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

//! RGBA8 sprite buffer with allocation and alpha pixel writes.

extern crate alloc;

use alloc::vec::Vec;

pub struct Sprite {
    pub rgba: Vec<u8>,
    pub w: u32,
    pub h: u32,
}

impl Sprite {
    pub fn blank(px: u32) -> Sprite {
        Sprite { rgba: alloc::vec![0u8; (px * px * 4) as usize], w: px, h: px }
    }

    pub fn set(&mut self, x: u32, y: u32, rgb: u32, a: u8) {
        if x >= self.w || y >= self.h {
            return;
        }
        let i = ((y * self.w + x) * 4) as usize;
        self.rgba[i] = (rgb >> 16) as u8;
        self.rgba[i + 1] = (rgb >> 8) as u8;
        self.rgba[i + 2] = rgb as u8;
        self.rgba[i + 3] = a;
    }
}
