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
use crate::constants::VG_FORMAT_B8G8R8A8_UNORM;
use crate::state::{Resource, ResourceTable, Scanout};
pub fn insert(
    resources: &ResourceTable,
    resource_id: u32,
    scanout: Scanout,
    backing_addr: u64,
    backing_len: u32,
) -> Result<(), &'static str> {
    resources.insert(Resource {
        resource_id,
        owner_pid: 0,
        width: scanout.width,
        height: scanout.height,
        format: VG_FORMAT_B8G8R8A8_UNORM,
        backing_addr,
        backing_len,
        in_use: true,
    })
}
