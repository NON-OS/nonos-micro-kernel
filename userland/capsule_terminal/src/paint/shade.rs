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

//! Derive chrome shades from the active background so a theme is complete: the
//! command blocks, header, footer and input bar all track `state.bg` instead of
//! staying a fixed dark. The nudge preserves the alpha channel, so translucent
//! profiles keep letting the desktop show through.

/// Nudge `bg` toward readable contrast by `mag` per channel, keeping its alpha.
/// A dark theme lifts (chrome sits just above the body); a light theme deepens.
pub fn elevate(bg: u32, mag: u8) -> u32 {
    let a = bg & 0xFF00_0000;
    let r = ((bg >> 16) & 0xFF) as i32;
    let g = ((bg >> 8) & 0xFF) as i32;
    let b = (bg & 0xFF) as i32;
    let luma = (r * 30 + g * 59 + b * 11) / 100;
    let d = if luma < 128 { mag as i32 } else { -(mag as i32) };
    let clamp = |v: i32| v.clamp(0, 255) as u32;
    a | (clamp(r + d) << 16) | (clamp(g + d) << 8) | clamp(b + d)
}
