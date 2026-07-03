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

use crate::browser::css::{Computed, Position, Size};

// An absolutely positioned box leaves normal flow only when it pins at least
// one inset. With every inset auto it has no explicit position, so it stays in
// flow at its static spot rather than piling at the containing block corner.
pub(super) fn out_of_flow(s: &Computed) -> bool {
    s.position == Position::Absolute
        && (s.top != Size::Auto
            || s.right != Size::Auto
            || s.bottom != Size::Auto
            || s.left != Size::Auto)
}
