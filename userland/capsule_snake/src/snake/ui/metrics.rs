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

// Every layout size the rebuild uses, in real pixels at 1x. No type size here
// sits below ttf::MIN_UI_PX (17.0), which clamps both draw and measure.

pub const PAD: u32 = 20;
pub const PAD_TIGHT: u32 = 12;
pub const GAP: u32 = 14;
pub const GAP_TIGHT: u32 = 8;
pub const GAP_WIDE: u32 = 22;

pub const RADIUS_PANEL: u32 = 14;
pub const RADIUS_CARD: u32 = 11;
pub const RADIUS_BTN: u32 = 9;
pub const RADIUS_PILL: u32 = 15;
pub const RADIUS_CELL: u32 = 4;

pub const RAIL_W: u32 = 292;
pub const HUD_H: u32 = 92;
pub const HUD_CARD_GAP: u32 = 12;
pub const FOOT_H: u32 = 56;
pub const FOOT_BTN_W: u32 = 168;
pub const FOOT_BTN_H: u32 = 38;

pub const BTN_H: u32 = 44;
pub const BTN_W: u32 = 268;
pub const BTN_GAP: u32 = 10;
pub const TOGGLE_W: u32 = 52;
pub const TOGGLE_H: u32 = 28;
pub const TOGGLE_KNOB: u32 = 22;
pub const CHIP_H: u32 = 34;
pub const CHIP_PAD_X: u32 = 14;
pub const CHIP_GAP: u32 = 9;

pub const MODAL_W: u32 = 420;
pub const MODAL_H: u32 = 336;
pub const ROW_H: u32 = 34;
pub const TABLE_HEAD_H: u32 = 30;
pub const RANK_ROWS: usize = 10;

pub const ICON_SM: u32 = 14;
pub const ICON_MD: u32 = 18;
pub const ICON_LG: u32 = 26;

pub const PX_WORDMARK: f32 = 52.0;
pub const PX_TITLE: f32 = 30.0;
pub const PX_HEAD: f32 = 22.0;
pub const PX_BODY: f32 = 18.0;
pub const PX_LABEL: f32 = 17.0;
pub const PX_STAT: f32 = 27.0;
