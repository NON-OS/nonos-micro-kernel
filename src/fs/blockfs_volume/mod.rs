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

mod create;
mod error;
mod format_volume;
mod key_to_array;
mod mount_volume;
mod read;
mod read_all;
mod remove;
mod stat;
mod state;
mod write;
mod write_or_create;

pub use create::create;
pub use error::VolumeError;
pub use format_volume::format_volume;
pub use mount_volume::mount_volume;
pub use read::read;
pub use read_all::read_all;
pub use remove::remove;
pub use stat::stat;
pub use write::write;
pub use write_or_create::write_or_create;
