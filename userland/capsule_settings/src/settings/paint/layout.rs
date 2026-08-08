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

pub const HEADER_H: u32 = 36;
pub const TAB_H: u32 = 28;
pub const STATUS_H: u32 = 24;
pub const BODY_TOP: u32 = HEADER_H + TAB_H;
pub const ROW_H: u32 = 24;
pub const PAD_X: u32 = 14;
pub const LABEL_LEFT: u32 = PAD_X;

/// Where the value column starts.
///
/// The longest label, "SMAP (supervisor access prevention)", is 35 glyphs at
/// 9px, so it runs to x=329 from `LABEL_LEFT`. At the old 320 it overlapped
/// the value. This leaves the longest label clear with a gap after it, and the
/// assert keeps that true if a longer label is added.
pub const VALUE_LEFT: u32 = 344;

const LONGEST_LABEL_GLYPHS: u32 = 35;
const GLYPH_ADVANCE: u32 = 9;
const _: () = assert!(
    LABEL_LEFT + LONGEST_LABEL_GLYPHS * GLYPH_ADVANCE < VALUE_LEFT,
    "the value column must start clear of the longest field label"
);
