// NØNOS Operating System
// Copyright (C) 2026 NØNOS Contributors
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

mod backend;
mod capacity;
mod error;
mod flush;
mod geometry;
mod map_ahci;
mod map_nvme;
mod map_virtio;
mod read;
mod select;
mod types;
mod write;

pub use backend::Backend;
pub use capacity::capacity;
pub use error::BlockDeviceError;
pub use flush::flush;
pub use geometry::geometry;
pub use read::read;
pub use select::selected;
pub use types::BlockGeometry;
pub use write::write;
