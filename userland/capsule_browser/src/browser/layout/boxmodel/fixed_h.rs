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

// A definite border-box height, or None when the box sizes to its content.
// A pixel or calc height is always definite. A percentage height resolves
// only when the containing block has a definite height; otherwise the box
// falls back to content sizing, as CSS specifies.
pub(super) fn fixed_h(style: &Computed, cb_h: Option<i32>) -> Option<i32> {
    if let Some(px) = style.height.definite_px() {
        return Some(px);
    }
    match cb_h {
        Some(base) => style.height.resolve(base),
        None => None,
    }
}
