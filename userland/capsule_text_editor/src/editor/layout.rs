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

pub(super) const FIRST_LINE_Y: u32 = 76;
pub(super) const GLYPH_ADVANCE: u32 = 9;
pub(super) const LINE_HEIGHT: u32 = 20;
pub(super) const TEXT_LEFT: u32 = 16;
pub(super) fn wrap_cols(width: u32) -> u32 {
    width.saturating_sub(TEXT_LEFT * 2).saturating_div(GLYPH_ADVANCE).clamp(32, 160)
}
