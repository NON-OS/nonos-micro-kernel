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
use super::super::stream::{Stream, CCMD_SET_VIEWPORT_STATE, OBJ_NULL};

/// Gallium viewports are a scale and a translate, not a rectangle. Half the
/// extent scales clip space onto the surface; the translate centres it.
pub fn set_viewport(s: &mut Stream, width: u32, height: u32) -> Result<(), &'static str> {
    if width == 0 || height == 0 {
        return Err("virgl: zero viewport extent");
    }
    let half_w = width as f32 / 2.0;
    let half_h = height as f32 / 2.0;
    s.push(
        CCMD_SET_VIEWPORT_STATE,
        OBJ_NULL,
        &[
            0,
            half_w.to_bits(),
            (-half_h).to_bits(),
            0.5f32.to_bits(),
            half_w.to_bits(),
            half_h.to_bits(),
            0.5f32.to_bits(),
        ],
    );
    Ok(())
}
