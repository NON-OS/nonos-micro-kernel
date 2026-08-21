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
use super::super::stream::{Stream, CCMD_CLEAR, OBJ_NULL};

pub struct ClearBuffers;

impl ClearBuffers {
    pub const COLOUR0: u32 = 1 << 2;
}

#[derive(Clone, Copy)]
pub struct Rgba {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

/// Clears the currently bound framebuffer. Depth is a double on the wire, so
/// it occupies two dwords between the colour and the stencil value.
pub fn clear(
    s: &mut Stream,
    buffers: u32,
    colour: Rgba,
    depth: f64,
    stencil: u32,
) -> Result<(), &'static str> {
    if buffers == 0 {
        return Err("virgl: clear with no buffers selected");
    }
    let depth_bits = depth.to_bits();
    s.push(
        CCMD_CLEAR,
        OBJ_NULL,
        &[
            buffers,
            colour.r.to_bits(),
            colour.g.to_bits(),
            colour.b.to_bits(),
            colour.a.to_bits(),
            depth_bits as u32,
            (depth_bits >> 32) as u32,
            stencil,
        ],
    );
    Ok(())
}
