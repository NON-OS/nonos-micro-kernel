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

//! Panel geometry.
//!
//! Every size here is a real pixel: apps paint at 1x and the AA text facade
//! clamps at `ttf::MIN_UI_PX` (17.0) for draw *and* measure. A caption asked for
//! at 13px renders at 17px but measures at 17px too, so rows sized for a smaller
//! caption would overflow the box they were measured into. The scale below
//! starts at the floor and the rows are tall enough to hold two lines of it.

pub const SIDEBAR_W: u32 = 212;
pub const NAV_TOP: u32 = 14;
pub const NAV_H: u32 = 38;
pub const NAV_GAP: u32 = 2;
pub const NAV_PAD_X: u32 = 12;
pub const NAV_RADIUS: u32 = 9;
pub const NAV_ICON: u32 = 18;
pub const NAV_LABEL_GAP: u32 = 12;

pub const PANE_PAD_X: u32 = 28;
pub const PANE_PAD_TOP: u32 = 24;

pub const HEAD_ICON: u32 = 30;
pub const HEAD_H: u32 = 72;

pub const CARD_RADIUS: u32 = 10;
pub const CARD_GAP: u32 = 16;
pub const CARD_PAD_X: u32 = 16;
pub const CARD_HEAD_H: u32 = 48;
pub const CARD_HEAD_NOTE_H: u32 = 74;

pub const ROW_H: u32 = 44;
pub const ROW_NOTE_H: u32 = 66;

pub const SWITCH_W: u32 = 42;
pub const SWITCH_H: u32 = 24;
pub const KNOB_INSET: u32 = 2;

pub const SLIDER_W: u32 = 150;
pub const SLIDER_TRACK_H: u32 = 4;
pub const SLIDER_KNOB_R: u32 = 7;

pub const PILL_H: u32 = 26;
pub const PILL_PAD_X: u32 = 11;

pub const TITLE_PX: f32 = 26.0;
pub const SUBTITLE_PX: f32 = 17.0;
pub const CARD_TITLE_PX: f32 = 18.0;
pub const BODY_PX: f32 = 17.0;
pub const NOTE_PX: f32 = 17.0;
