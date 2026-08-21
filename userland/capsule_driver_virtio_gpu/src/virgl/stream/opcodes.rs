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
pub const CCMD_CREATE_OBJECT: u8 = 1;
pub const CCMD_SET_VIEWPORT_STATE: u8 = 4;
pub const CCMD_SET_FRAMEBUFFER_STATE: u8 = 5;
pub const CCMD_CLEAR: u8 = 7;

pub const OBJ_NULL: u8 = 0;
pub const OBJ_SURFACE: u8 = 8;

pub const MAX_PAYLOAD_DWORDS: usize = 0xFFFF;
