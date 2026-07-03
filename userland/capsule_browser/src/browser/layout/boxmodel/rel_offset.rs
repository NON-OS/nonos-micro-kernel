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

use crate::browser::css::Computed;

use super::offset_px::offset_px;

// position:relative shift after normal flow: left/top win over right/bottom.
pub(super) fn rel_offset(s: &Computed, base_w: i32) -> (i32, i32) {
    let dx = match offset_px(s.left, base_w) {
        Some(l) => l,
        None => -offset_px(s.right, base_w).unwrap_or(0),
    };
    let dy = match offset_px(s.top, base_w) {
        Some(t) => t,
        None => -offset_px(s.bottom, base_w).unwrap_or(0),
    };
    (dx, dy)
}
