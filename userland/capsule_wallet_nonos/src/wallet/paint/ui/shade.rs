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

// Nudge an opaque ARGB colour toward white or black by a small per-channel
// amount. Used to fake elevation with single-pixel highlight and shadow lines,
// since the framebuffer draws opaque rectangles with no alpha blending.
pub fn lighten(argb: u32, amt: u8) -> u32 {
    let a = argb & 0xFF00_0000;
    let r = ((argb >> 16) & 0xFF) as u16;
    let g = ((argb >> 8) & 0xFF) as u16;
    let b = (argb & 0xFF) as u16;
    let up = |v: u16| -> u32 { (v + amt as u16).min(255) as u32 };
    a | (up(r) << 16) | (up(g) << 8) | up(b)
}

pub fn darken(argb: u32, amt: u8) -> u32 {
    let a = argb & 0xFF00_0000;
    let r = ((argb >> 16) & 0xFF) as u8;
    let g = ((argb >> 8) & 0xFF) as u8;
    let b = (argb & 0xFF) as u8;
    let down = |v: u8| -> u32 { v.saturating_sub(amt) as u32 };
    a | (down(r) << 16) | (down(g) << 8) | down(b)
}
