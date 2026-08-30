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

//! The Settings palette. Two variants of each control style: the live one, and
//! a dimmed one for the rows that have no implementation behind them yet, so a
//! dead control never reads as a working one.

use crate::editor::widget::{DropdownStyle, NavStyle};

pub(super) const RAIL_BG: u32 = 0xFF08111D;
pub(super) const HAIRLINE: u32 = 0xFF16273B;
pub(super) const CARD_BG: u32 = 0xFF0B1524;
pub(super) const TEXT: u32 = 0xFFE4ECF5;
pub(super) const MUTED: u32 = 0xFF9BB0C7;
pub(super) const DIM: u32 = 0xFF4A5D72;
pub(super) const DIM_SOFT: u32 = 0xFF3C4C60;
pub(super) const SWITCH: (u32, u32, u32) = (0xFF17BED9, 0xFF1C2F47, 0xFFE4ECF5);

pub(super) fn nav_live() -> NavStyle {
    NavStyle {
        accent: 0xFF0C4C5D,
        ring: 0x4017BED9,
        label: MUTED,
        label_sel: TEXT,
        radius: 9,
        pad_x: 12,
    }
}

pub(super) fn nav_dim() -> NavStyle {
    NavStyle { accent: 0x600C4C5D, ring: 0x2017BED9, label: DIM, label_sel: MUTED, ..nav_live() }
}

pub(super) fn drop_dim() -> DropdownStyle {
    DropdownStyle {
        bg: 0xFF132539,
        border: 0xFF1C2F47,
        radius: 9,
        text: DIM,
        chevron: DIM_SOFT,
        pad_x: 12,
    }
}
