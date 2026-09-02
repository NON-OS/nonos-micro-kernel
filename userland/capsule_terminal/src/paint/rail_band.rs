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
/// A rail element placed in the scrolled column. `y` is the top edge in
/// rail-local pixels and goes negative once the column has been scrolled past
/// that element, which is why it is signed where a `Rect` is not.
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Band {
    pub x: u32,
    pub y: i32,
    pub w: u32,
    pub h: u32,
}

/// The part of a band the rail sub-buffer can address, as a `(top, height)`
/// pair. Only the top edge needs cutting: the sub-buffer's own height already
/// stops everything below it, whereas a negative `y` cannot be handed to the
/// unsigned fills at all.
pub fn clip(y: i32, h: u32) -> Option<(u32, u32)> {
    if y >= 0 {
        return Some((y as u32, h));
    }
    let cut = y.unsigned_abs();
    if cut >= h {
        return None;
    }
    Some((0, h - cut))
}

pub fn hits(b: &Band, x: u32, y: i32) -> bool {
    x >= b.x && x < b.x + b.w && y >= b.y && y < b.y + b.h as i32
}

/// Whether any part of a band falls inside a viewport `vh` pixels tall. Bands
/// that fail this are skipped rather than rasterized and thrown away.
pub fn visible(b: &Band, vh: u32) -> bool {
    b.y < vh as i32 && b.y + b.h as i32 > 0
}
