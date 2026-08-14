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
pub mod attach_backing;
pub mod create_resource_2d;
pub mod ctx_create;
pub mod flush;
pub mod get_capset_info;
pub mod get_display_info;
pub mod get_edid;
pub mod hdr;
pub mod set_scanout;
pub mod submit_3d;
pub mod transfer_to_host_2d;
pub use attach_backing::attach_backing;
pub use create_resource_2d::create_resource_2d;
pub use ctx_create::ctx_create;
pub use flush::resource_flush;
pub use get_capset_info::get_capset_info;
pub use get_display_info::get_display_info;
pub use get_edid::get_edid;
pub use set_scanout::set_scanout;
pub use submit_3d::submit_3d;
pub use transfer_to_host_2d::transfer_to_host_2d;
