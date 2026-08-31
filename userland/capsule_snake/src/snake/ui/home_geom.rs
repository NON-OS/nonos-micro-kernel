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

use nonos_toolkit::font::ttf::line_height;

use super::metrics::{BTN_GAP, BTN_H, BTN_W, GAP, GAP_WIDE, HUD_H, PX_BODY, PX_WORDMARK};
use super::rect::{self, Rect};

pub const ACTIONS: usize = 4;
pub const CARDS: usize = 2;

pub const LABELS: [&[u8]; ACTIONS] = [b"Play", b"Continue", b"Ranks", b"Settings"];
pub const CARD_LABELS: [&[u8]; CARDS] = [b"Daily challenge", b"Recent best"];

pub fn wordmark_h() -> u32 {
    line_height(PX_WORDMARK).max(1) as u32 + line_height(PX_BODY).max(1) as u32
}

fn stack_h() -> u32 {
    wordmark_h() + GAP_WIDE + BTN_H * 2 + BTN_GAP + GAP_WIDE + HUD_H
}

// The hub is one centred column: wordmark, a primary Play, a row of three, then
// the two summary cards. Everything below reads its origin from here.
pub fn stack(w: u32, h: u32) -> Rect {
    rect::centred(rect::content(w, h), BTN_W * 2 + GAP, stack_h())
}

pub fn wordmark(w: u32, h: u32) -> Rect {
    let s = stack(w, h);
    (s.0, s.1, s.2, wordmark_h())
}

pub fn action(w: u32, h: u32, index: usize) -> Rect {
    let s = stack(w, h);
    let top = s.1 + wordmark_h() + GAP_WIDE;
    if index == 0 {
        return (s.0, top, s.2, BTN_H);
    }
    let band = (s.0, top + BTN_H + BTN_GAP, s.2, BTN_H);
    rect::column(band, index - 1, ACTIONS - 1, BTN_GAP)
}

pub fn action_at(w: u32, h: u32, x: i32, y: i32) -> Option<usize> {
    rect::index_at(ACTIONS, x, y, |i| action(w, h, i))
}

pub fn card(w: u32, h: u32, index: usize) -> Rect {
    let s = stack(w, h);
    let band = (s.0, s.1 + s.3.saturating_sub(HUD_H), s.2, HUD_H);
    rect::column(band, index, CARDS, GAP)
}

pub fn card_at(w: u32, h: u32, x: i32, y: i32) -> Option<usize> {
    rect::index_at(CARDS, x, y, |i| card(w, h, i))
}
