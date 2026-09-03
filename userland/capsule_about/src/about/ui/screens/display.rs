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

use nonos_app_skeleton::PaintBuffer;

use crate::about::state::State;

use super::super::card::{self, titled};
use super::super::chrome::Rect;
use super::super::kv::{kv, ROW_H};
use super::super::metrics::{CARD_GAP, CARD_PAD, CHAIN_H};
use super::{chain, display_surface};

const BACKEND: &[u8] = b"compositor + driver.virtio_gpu";
const PATH: [&[u8]; 3] = [b"capsule_about", b"compositor", b"driver.virtio_gpu"];
const PATH_GAP: u32 = 14;
const PATH_H: u32 = card::OVERHEAD + CHAIN_H + PATH_GAP + ROW_H;

pub fn content_h(_rect: &Rect) -> u32 {
    display_surface::HEIGHT + CARD_GAP + PATH_H
}

pub fn paint(state: &State, fb: &mut PaintBuffer, rect: &Rect) {
    let mut pane = fb.sub(rect.x, rect.y, rect.w, rect.h);
    let y = -(state.scroll as i32);
    display_surface::paint(&mut pane, y, rect.w);
    path(&mut pane, y + (display_surface::HEIGHT + CARD_GAP) as i32, rect.w);
}

// The chain and the row under it are the same fact drawn twice: the hops the pixel
// takes, then the backend string those hops add up to.
fn path(fb: &mut PaintBuffer, y: i32, w: u32) {
    let top = titled(fb, 0, y, w, PATH_H, b"Present path");
    chain::paint(fb, CARD_PAD, top, &PATH);
    let row_y = top + (CHAIN_H + PATH_GAP) as i32;
    kv(fb, CARD_PAD, row_y, card::inner(w), b"Backend", BACKEND, false);
}
