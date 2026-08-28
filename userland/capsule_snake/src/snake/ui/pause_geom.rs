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

use super::metrics::{BTN_GAP, BTN_H, GAP_WIDE, MODAL_W, PAD, PX_TITLE};
use super::rect::{self, Rect};

pub const ACTIONS: usize = 4;

pub const LABELS: [&[u8]; ACTIONS] = [b"Resume", b"Restart", b"Settings", b"Quit"];

pub fn title_h() -> u32 {
    line_height(PX_TITLE).max(1) as u32
}

// The modal floats over the live board, so it is centred on the whole surface
// rather than on the content rect the other screens share. Its height follows
// the title and the action stack, which is all it ever holds.
pub fn modal(w: u32, h: u32) -> Rect {
    let span = BTN_H * ACTIONS as u32 + BTN_GAP * (ACTIONS as u32 - 1);
    rect::centred((0, 0, w, h), MODAL_W, PAD * 2 + title_h() + GAP_WIDE + span)
}

pub fn title(w: u32, h: u32) -> Rect {
    let inner = rect::inset(modal(w, h), PAD);
    (inner.0, inner.1, inner.2, title_h())
}

pub fn action(w: u32, h: u32, index: usize) -> Rect {
    let inner = rect::inset(modal(w, h), PAD);
    let span = BTN_H * ACTIONS as u32 + BTN_GAP * (ACTIONS as u32 - 1);
    let top = inner.1 + inner.3.saturating_sub(span);
    rect::row((inner.0, top, inner.2, span), index, BTN_H, BTN_GAP)
}

pub fn action_at(w: u32, h: u32, x: i32, y: i32) -> Option<usize> {
    rect::index_at(ACTIONS, x, y, |i| action(w, h, i))
}
