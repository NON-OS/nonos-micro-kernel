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
use alloc::vec::Vec;

use super::super::stream::{Stream, CCMD_SET_FRAMEBUFFER_STATE, OBJ_NULL};

/// Binds the surfaces a later clear or draw writes into. A zero depth handle
/// means no depth attachment, which is what a plain colour pass wants.
pub fn set_framebuffer(
    s: &mut Stream,
    colour: &[u32],
    depth: u32,
) -> Result<(), &'static str> {
    if colour.is_empty() && depth == 0 {
        return Err("virgl: framebuffer with no attachments");
    }
    if colour.iter().any(|&h| h == 0) {
        return Err("virgl: colour attachment handle 0");
    }
    let mut payload = Vec::with_capacity(2 + colour.len());
    payload.push(colour.len() as u32);
    payload.push(depth);
    payload.extend_from_slice(colour);
    s.push(CCMD_SET_FRAMEBUFFER_STATE, OBJ_NULL, &payload);
    Ok(())
}
