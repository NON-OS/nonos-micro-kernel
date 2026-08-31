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

use super::metrics::{CARD_PAD_X, SLIDER_W, SWITCH_H, SWITCH_W};

pub fn right_edge(card_x: u32, card_w: u32) -> u32 {
    card_x + card_w - CARD_PAD_X
}

pub fn switch_rect(card_x: u32, card_w: u32, row_y: i32, row_h: u32) -> (u32, i32) {
    let x = right_edge(card_x, card_w).saturating_sub(SWITCH_W);
    let y = row_y + ((row_h - SWITCH_H) / 2) as i32;
    (x, y)
}

pub fn slider_x(card_x: u32, card_w: u32) -> u32 {
    right_edge(card_x, card_w).saturating_sub(SLIDER_W)
}

/// Whether a pointer at `x` is over the control column of a row, which is what
/// separates "toggle this" from "select this row".
pub fn in_control(x: i32, card_x: u32, card_w: u32) -> bool {
    x >= slider_x(card_x, card_w) as i32 && x <= right_edge(card_x, card_w) as i32
}
