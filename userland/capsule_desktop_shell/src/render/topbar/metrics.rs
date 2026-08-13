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

//! Colours and measurements for the menu bar. The panel and border colours are
//! the dock's, so the two bars share one look.

pub(super) const BAR_BG: u32 = 0xFF1B_2030;
pub(super) const BAR_BORDER: u32 = 0xFF2A_3446;
pub(super) const TILE_BG: u32 = 0xFF23_2C3C;
pub(super) const TILE_BORDER: u32 = 0xFF2E_3A4C;
pub(super) const FG: u32 = 0xFFCF_E6E9;
pub(super) const WORDMARK: u32 = 0xFFE6_F0FA;

pub(super) const LOGO_X: u32 = 12;
pub(super) const LOGO_SIZE: u32 = 20;
pub(super) const WORDMARK_X: u32 = 40;
/// Right edge of the clickable brand region (logo plus wordmark).
pub(super) const BRAND_RIGHT: u32 = 118;

pub(super) const RIGHT_MARGIN: u32 = 12;
pub(super) const TILE_H: u32 = crate::render::layout::MENUBAR_TILE_H;
pub(super) const PAD_X: u32 = 12;
pub(super) const GAP: u32 = 10;

pub(super) const BATT_GLYPH_W: u32 = 24;
pub(super) const NET_GLYPH_W: u32 = 14;
pub(super) const DOT: u32 = 8;
