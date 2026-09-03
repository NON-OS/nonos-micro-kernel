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

pub const WIN_W: u32 = 1000;
pub const WIN_H: u32 = 680;
pub const WIN_X: u32 = 188;
pub const WIN_Y: u32 = 52;

pub const SIDEBAR_W: u32 = 196;
pub const NAV_TOP: u32 = 14;
pub const NAV_H: u32 = 36;
pub const NAV_GAP: u32 = 2;
pub const NAV_PAD_X: u32 = 12;
pub const NAV_RADIUS: u32 = 9;
pub const NAV_ICON: u32 = 18;
pub const NAV_LABEL_GAP: u32 = 11;
pub const SIDE_FOOT_H: u32 = 64;
pub const SIDE_FOOT_TOP: u32 = 12;
pub const SIDE_FOOT_LINE: u32 = 24;

pub const PANE_PAD_X: u32 = 20;
pub const PANE_PAD_TOP: u32 = 16;
pub const HEAD_H: u32 = 46;
pub const HEAD_META_GAP: u32 = 18;
pub const STATUS_H: u32 = 26;
pub const STATUS_PAD_X: u32 = 14;
pub const STATUS_GROUP_GAP: u32 = 22;

pub const CARD_RADIUS: u32 = 11;
pub const CARD_PAD: u32 = 15;
pub const CARD_HEAD_H: u32 = 32;
pub const CARD_GAP: u32 = 14;

pub const KV_ROW_H: u32 = 26;
pub const KV_LABEL_W: u32 = 172;
pub const KV_GAP: u32 = 12;

pub const CHIP_H: u32 = 28;
pub const CHIP_PAD_X: u32 = 11;
pub const CHIP_GAP: u32 = 8;
pub const CHIP_RADIUS: u32 = 8;
pub const CHIP_DOT: u32 = 6;
pub const CHIP_DOT_GAP: u32 = 8;

pub const TILE_H: u32 = 168;
pub const TILE_GAP: u32 = 14;
pub const TILE_RING_R: u32 = 36;
pub const TILE_RING_T: u32 = 7;
pub const TILE_RING_TOP: u32 = 16;
pub const TILE_SUB_GAP: u32 = 4;

// One wheel notch / arrow key. Scroll is in pixels, so a step is a size, not a
// row count: see state.rs.
pub const SCROLL_STEP: u32 = 24;

pub const TITLE_PX: f32 = 21.0;
pub const VALUE_PX: f32 = 27.0;
pub const BODY_PX: f32 = 17.0;
pub const NUM_PX: f32 = 17.0;

// Blocks the five screens lay out with. Every one is a real-pixel size at 1x, in
// the same space as the card and row metrics above.
pub const HERO_H: u32 = 124;
pub const HERO_MARK_R: u32 = 30;
pub const HERO_MARK_T: u32 = 3;
pub const HERO_TEXT_X: u32 = 96;
pub const HERO_TITLE_TOP: u32 = 20;
pub const HERO_SUB_TOP: u32 = 58;
pub const HERO_META_TOP: u32 = 84;
pub const PAIR_H: u32 = 50;
pub const PAIR_LINE: u32 = 24;
pub const CHAIN_H: u32 = 30;
pub const CHAIN_LINK: u32 = 26;
pub const LINE_STEP: u32 = 24;
pub const METER_H: u32 = 8;
