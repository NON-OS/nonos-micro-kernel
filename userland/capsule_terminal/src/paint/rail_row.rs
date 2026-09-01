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

use super::rail_text::{lh, RAIL_GAP};
use crate::layout::Rect;

/// Row geometry for the left rail's lists. The painter and the hit-test both
/// call `row_rect`, so a click can never land on a row other than the one drawn
/// under the pointer.
pub fn row_h() -> u32 {
    lh() * 2 + RAIL_GAP
}

pub fn row_rect(i: u32, rail: Rect) -> Rect {
    Rect { x: rail.x, y: rail.y + i * row_h(), w: rail.w, h: row_h() }
}

/// Rows the list can show whole. A row clipped by the rail's lower edge is not
/// drawn and not clickable, which is why both sides ask this one question.
pub fn rows_fit(rail: Rect) -> u32 {
    rail.h / row_h().max(1)
}

pub fn inside(r: Rect, x: u32, y: u32) -> bool {
    x >= r.x && x < r.x + r.w && y >= r.y && y < r.y + r.h
}

pub fn row_at(list: Rect, count: u32, x: u32, y: u32) -> Option<u32> {
    let visible = count.min(rows_fit(list));
    (0..visible).find(|&i| inside(row_rect(i, list), x, y))
}

/// Last path component, or the whole path when it names the root.
pub fn base_name(path: &str) -> &str {
    match path.rsplit('/').next() {
        Some(tail) if !tail.is_empty() => tail,
        _ => path,
    }
}
