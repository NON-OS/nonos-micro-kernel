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

//! The Home screen palette. Home is a fixed-identity surface: it paints the
//! NONOS Docs navy and cyan regardless of the active editor theme, so its
//! colours are named here rather than read from `theme`.

pub(super) const BG: u32 = 0xFF06_0C15;
pub(super) const RAIL_BG: u32 = 0xFF08_111D;
pub(super) const RAIL_LINE: u32 = 0xFF16_273B;
pub(super) const BRAND_A: u32 = 0xFF17_BED9;
pub(super) const BRAND_B: u32 = 0xFF3A_DCF0;
pub(super) const BRAND_MARK: u32 = 0xFF06_0C15;
pub(super) const TITLE: u32 = 0xFFE4_ECF5;
pub(super) const LABEL: u32 = 0xFF9B_B0C7;
pub(super) const MUTED: u32 = 0xFF61_788F;
pub(super) const ACCENT: u32 = 0xFF17_BED9;
pub(super) const NAV_ACCENT: u32 = 0xFF0C_4C5D;
pub(super) const NAV_RING: u32 = 0x4017_BED9;
pub(super) const FIELD_BG: u32 = 0xFF0B_1524;
pub(super) const FIELD_LINE: u32 = 0xFF1C_2F47;
pub(super) const ICON_BG: u32 = 0xFF13_2539;
pub(super) const CARD_BG: u32 = 0xFF0B_1524;

/// Sink an opaque colour part-way back into the page, marking a control that
/// is drawn but has nothing wired behind it yet.
pub(super) fn dim(argb: u32) -> u32 {
    let mix = |c: u32, b: u32| (c * 11 + b * 9) / 20;
    let r = mix((argb >> 16) & 0xFF, (BG >> 16) & 0xFF);
    let g = mix((argb >> 8) & 0xFF, (BG >> 8) & 0xFF);
    let b = mix(argb & 0xFF, BG & 0xFF);
    0xFF00_0000 | (r << 16) | (g << 8) | b
}
