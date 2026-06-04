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

use super::layer::{Layer, MAX_LAYERS};
use super::table::SceneTable;

impl SceneTable {
    pub const fn new() -> Self {
        Self {
            entries: [Layer {
                owner_pid: 0,
                surface_handle: 0,
                x: 0,
                y: 0,
                width: 0,
                height: 0,
                z: 0,
                in_use: false,
                miss_count: 0,
            }; MAX_LAYERS],
            count: 0,
        }
    }
}
