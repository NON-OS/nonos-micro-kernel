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

use nonos_cap::{
    CAP_CORE_EXEC, CAP_DEBUG, CAP_GRAPHICS_DISPLAY_QUERY, CAP_GRAPHICS_PRESENT,
    CAP_GRAPHICS_SURFACE_CREATE, CAP_GRAPHICS_SURFACE_MAP, CAP_MEMORY,
};

pub const SDK_CAPS: u64 = CAP_CORE_EXEC
    | CAP_MEMORY
    | CAP_DEBUG
    | CAP_GRAPHICS_DISPLAY_QUERY
    | CAP_GRAPHICS_SURFACE_CREATE
    | CAP_GRAPHICS_SURFACE_MAP
    | CAP_GRAPHICS_PRESENT;
