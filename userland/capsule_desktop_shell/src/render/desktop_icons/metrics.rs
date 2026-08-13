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

//! Grid metrics for the desktop icons.

pub(super) const ICON: u32 = 48;
pub(super) const CELL_W: u32 = 104;
pub(super) const CELL_H: u32 = 96;
pub(super) const LEFT: u32 = 24;

/// Width of the rename caret, which is a rule rather than a glyph cell.
pub(super) const CARET_W: u32 = 2;

/// Leave room at the bottom for the floating dock plus a little breathing space.
pub(super) const BOTTOM_RESERVE: u32 = 120;
