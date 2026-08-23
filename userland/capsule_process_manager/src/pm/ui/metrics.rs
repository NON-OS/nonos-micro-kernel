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


// Every layout size the restyle uses, in real pixels at 1x. Nothing here is
// sized below ttf::MIN_UI_PX (17.0), which clamps both draw and measure.

pub const WIN_W: u32 = 1240;
pub const WIN_H: u32 = 780;
pub const SIDEBAR_W: u32 = 196;
pub const NAV_TOP: u32 = 14;
pub const NAV_H: u32 = 36;
pub const NAV_GAP: u32 = 2;
pub const NAV_PAD_X: u32 = 12;
pub const NAV_RADIUS: u32 = 9;
pub const NAV_ICON: u32 = 18;
pub const NAV_LABEL_GAP: u32 = 11;
pub const SIDE_FOOT_H: u32 = 54;

pub const PANE_PAD_X: u32 = 20;
pub const PANE_PAD_TOP: u32 = 16;
pub const HEAD_H: u32 = 46;
pub const STATUS_H: u32 = 26;
pub const INSPECTOR_W: u32 = 280;
pub const INSPECTOR_PAD: u32 = 15;
pub const CARD_H: u32 = 108;
pub const CARD_GAP: u32 = 12;
pub const CARD_RADIUS: u32 = 11;
pub const CARD_PAD: u32 = 13;

pub const TOOLBAR_H: u32 = 32;
pub const TOOLBAR_GAP: u32 = 12;
pub const CHIP_H: u32 = 30;
pub const CHIP_PAD_X: u32 = 11;
pub const CHIP_GAP: u32 = 8;
pub const TBL_RADIUS: u32 = 11;
pub const TBL_HEAD_H: u32 = 30;
pub const ROW_H: u32 = 28;
pub const CELL_PAD_X: u32 = 12;

pub const PANEL_RADIUS: u32 = 11;
pub const PANEL_PAD: u32 = 15;
pub const PANEL_HEAD_H: u32 = 34;
pub const BAR_H: u32 = 7;
pub const BAR_ROW_H: u32 = 28;
pub const SPARK_H: u32 = 38;
pub const COL_PID_W: u32 = 64;
pub const COL_STATE_W: u32 = 96;
pub const COL_CPU_W: u32 = 108;
pub const COL_MEM_W: u32 = 92;
pub const COL_UPTIME_W: u32 = 88;
pub const COL_AUTH_W: u32 = 108;
pub const NAME_MIN_W: u32 = 120;
pub const RISK_SLOT_W: u32 = 7;
pub const RISK_SLOT_H: u32 = 14;
pub const RISK_SLOT_GAP: u32 = 3;

pub const TITLE_PX: f32 = 21.0;
pub const CARD_VALUE_PX: f32 = 27.0;
pub const BODY_PX: f32 = 17.0;
pub const NUM_PX: f32 = 17.0;
