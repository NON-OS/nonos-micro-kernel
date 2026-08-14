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
#[derive(Clone, Copy)]
pub struct Box3d {
    pub x: u32,
    pub y: u32,
    pub z: u32,
    pub w: u32,
    pub h: u32,
    pub d: u32,
}

impl Box3d {
    pub fn whole_2d(width: u32, height: u32) -> Self {
        Self { x: 0, y: 0, z: 0, w: width, h: height, d: 1 }
    }

    pub(super) fn words(&self) -> [u32; 6] {
        [self.x, self.y, self.z, self.w, self.h, self.d]
    }

    pub(super) fn is_empty(&self) -> bool {
        self.w == 0 || self.h == 0 || self.d == 0
    }
}
