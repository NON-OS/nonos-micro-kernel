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


//! Request and event opcodes, transcribed from the protocol definitions.

pub mod req {
    pub const DISPLAY_SYNC: u16 = 0;
    pub const DISPLAY_GET_REGISTRY: u16 = 1;
    pub const REGISTRY_BIND: u16 = 0;

    pub const COMPOSITOR_CREATE_SURFACE: u16 = 0;

    pub const SHM_CREATE_POOL: u16 = 0;
    pub const POOL_CREATE_BUFFER: u16 = 0;
    pub const POOL_DESTROY: u16 = 1;
    pub const BUFFER_DESTROY: u16 = 0;

    pub const SURFACE_DESTROY: u16 = 0;
    pub const SURFACE_ATTACH: u16 = 1;
    pub const SURFACE_DAMAGE: u16 = 2;
    pub const SURFACE_FRAME: u16 = 3;
    pub const SURFACE_COMMIT: u16 = 6;
    pub const SURFACE_DAMAGE_BUFFER: u16 = 9;

    pub const XDG_BASE_GET_SURFACE: u16 = 2;
    pub const XDG_BASE_PONG: u16 = 3;
    pub const XDG_SURFACE_GET_TOPLEVEL: u16 = 1;
    pub const XDG_SURFACE_SET_GEOMETRY: u16 = 3;
    pub const XDG_SURFACE_ACK_CONFIGURE: u16 = 4;
    pub const TOPLEVEL_SET_TITLE: u16 = 2;
    pub const TOPLEVEL_SET_APP_ID: u16 = 3;

    pub const SEAT_GET_POINTER: u16 = 0;
    pub const SEAT_GET_KEYBOARD: u16 = 1;
}

pub mod ev {
    pub const CALLBACK_DONE: u16 = 0;
    pub const SHM_FORMAT: u16 = 0;
    pub const BUFFER_RELEASE: u16 = 0;
    pub const XDG_SURFACE_CONFIGURE: u16 = 0;
    pub const TOPLEVEL_CONFIGURE: u16 = 0;
}

/// wl_shm formats. Zero and one are the two every client understands.
pub const FORMAT_ARGB8888: u32 = 0;
pub const FORMAT_XRGB8888: u32 = 1;
