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

use nonos_app_skeleton::PaintBuffer;

use super::super::metrics::{BODY_PX, PAIR_LINE};
use super::super::text::line;
use super::prose_wrap::split;

// How tall the wrapped run will be, walked with the same splitter the painter
// uses so a screen's reported extent and its drawn height cannot disagree.
pub fn height(b: &[u8], w: u32) -> u32 {
    let mut rest = b;
    let mut rows = 0u32;
    while !rest.is_empty() {
        let (_, next) = split(rest, w);
        rows += 1;
        rest = &rest[next.max(1)..];
    }
    rows * PAIR_LINE
}

pub fn paint(fb: &mut PaintBuffer, x: u32, y: i32, w: u32, b: &[u8], argb: u32) -> u32 {
    let mut rest = b;
    let mut rows = 0u32;
    while !rest.is_empty() {
        let (end, next) = split(rest, w);
        line(fb, x, y + (rows * PAIR_LINE) as i32, &rest[..end], argb, BODY_PX);
        rows += 1;
        rest = &rest[next.max(1)..];
    }
    rows * PAIR_LINE
}
