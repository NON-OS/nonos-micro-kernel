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
use crate::constants::VG_TARGET_TEXTURE_2D;

/// Bind flags are fixed at creation: the host picks its allocation from them,
/// so a resource not created scanout-capable can never be scanned out.
#[derive(Clone, Copy)]
pub struct Resource3d {
    pub resource_id: u32,
    pub target: u32,
    pub format: u32,
    pub bind: u32,
    pub width: u32,
    pub height: u32,
    pub depth: u32,
    pub array_size: u32,
    pub last_level: u32,
    pub nr_samples: u32,
}

impl Resource3d {
    pub fn render_target(resource_id: u32, format: u32, width: u32, height: u32, bind: u32) -> Self {
        Self {
            resource_id,
            target: VG_TARGET_TEXTURE_2D,
            format,
            bind,
            width,
            height,
            depth: 1,
            array_size: 1,
            last_level: 0,
            nr_samples: 0,
        }
    }

    pub(super) fn fields(&self) -> [u32; 12] {
        [
            self.resource_id,
            self.target,
            self.format,
            self.bind,
            self.width,
            self.height,
            self.depth,
            self.array_size,
            self.last_level,
            self.nr_samples,
            0,
            0,
        ]
    }
}
