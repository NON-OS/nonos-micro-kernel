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
use super::super::stream::{Stream, CCMD_CREATE_OBJECT, OBJ_SURFACE};

/// A surface is the renderable view of a resource. Nothing can be drawn into
/// a resource directly; the framebuffer takes surface handles.
pub fn create_surface(
    s: &mut Stream,
    handle: u32,
    res_handle: u32,
    format: u32,
) -> Result<(), &'static str> {
    if handle == 0 {
        return Err("virgl: surface handle 0 is reserved");
    }
    if res_handle == 0 {
        return Err("virgl: surface over resource 0");
    }
    // level 0, layers 0: the first and only layer of a plain 2D texture.
    s.push(CCMD_CREATE_OBJECT, OBJ_SURFACE, &[handle, res_handle, format, 0, 0]);
    Ok(())
}
