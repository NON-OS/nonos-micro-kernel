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

use super::metrics::{
    FOOT_BTN_H, FOOT_BTN_W, FOOT_H, GAP, GAP_TIGHT, HUD_CARD_GAP, PAD_TIGHT, ROW_H,
};
use super::play_geom::{foot_band, hud_band, rail};
use super::rect::{self, Rect};

pub const HUD_CARDS: usize = 4;
pub const FOOT_BTNS: usize = 4;
pub const RAIL_ROWS: usize = 4;

pub const FOOT_LABELS: [&[u8]; FOOT_BTNS] = [b"Pause", b"Restart", b"Sound", b"Quit"];
pub const RAIL_HEADS: [&[u8]; RAIL_ROWS] = [b"Mode", b"Level", b"Next level", b"Tip"];

pub fn hud(w: u32, h: u32, index: usize) -> Rect {
    rect::column(hud_band(w, h), index, HUD_CARDS, HUD_CARD_GAP)
}

pub fn hud_at(w: u32, h: u32, x: i32, y: i32) -> Option<usize> {
    rect::index_at(HUD_CARDS, x, y, |i| hud(w, h, i))
}

// The footer row is right-aligned so the primary action sits under the rail
// rather than drifting with the board width.
pub fn foot(w: u32, h: u32, index: usize) -> Rect {
    let band = foot_band(w, h);
    let span = FOOT_BTN_W * FOOT_BTNS as u32 + GAP * (FOOT_BTNS as u32 - 1);
    let x = band.0 + band.2.saturating_sub(span) + index as u32 * (FOOT_BTN_W + GAP);
    let y = band.1 + FOOT_H.saturating_sub(FOOT_BTN_H) / 2;
    (x, y, FOOT_BTN_W, FOOT_BTN_H)
}

pub fn foot_at(w: u32, h: u32, x: i32, y: i32) -> Option<usize> {
    rect::index_at(FOOT_BTNS, x, y, |i| foot(w, h, i))
}

pub fn rail_inner(w: u32, h: u32) -> Rect {
    rect::inset(rail(w, h), PAD_TIGHT)
}

pub fn rail_row(w: u32, h: u32, index: usize) -> Rect {
    rect::row(rail_inner(w, h), index, ROW_H, GAP_TIGHT)
}
