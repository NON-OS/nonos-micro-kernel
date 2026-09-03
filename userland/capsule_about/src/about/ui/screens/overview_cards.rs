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

use crate::about::data::{license, product};

use super::super::card::{self, titled};
use super::super::kv::pair;
use super::super::metrics::{CARD_GAP, CARD_PAD, PAIR_H};

pub const ROWS: u32 = 3;
pub const HEIGHT: u32 = card::OVERHEAD + PAIR_H * ROWS;

// Label-over-value rather than the kv table used elsewhere: at half the pane a
// fixed label column would leave a homepage or a licence name nothing to be drawn
// in, and a truncated URL is worse than a second line.
pub fn paint(fb: &mut PaintBuffer, y: i32, w: u32) {
    let col = (w.saturating_sub(CARD_GAP)) / 2;
    let right = col + CARD_GAP;
    identity(fb, 0, y, col);
    terms(fb, right, y, w.saturating_sub(right));
}

fn identity(fb: &mut PaintBuffer, x: u32, y: i32, w: u32) {
    let top = titled(fb, x, y, w, HEIGHT, b"Identity");
    let inner = w.saturating_sub(CARD_PAD * 2);
    let cx = x + CARD_PAD;
    pair(fb, cx, top, inner, b"Product", product::NAME, false);
    pair(fb, cx, top + PAIR_H as i32, inner, b"Homepage", product::HOMEPAGE, false);
    pair(fb, cx, top + (PAIR_H * 2) as i32, inner, b"Copyright", product::COPYRIGHT, false);
}

fn terms(fb: &mut PaintBuffer, x: u32, y: i32, w: u32) {
    let top = titled(fb, x, y, w, HEIGHT, b"Terms");
    let inner = w.saturating_sub(CARD_PAD * 2);
    let cx = x + CARD_PAD;
    pair(fb, cx, top, inner, b"Licence", license::NAME, false);
    pair(fb, cx, top + PAIR_H as i32, inner, b"Version", license::VERSION, false);
    pair(fb, cx, top + (PAIR_H * 2) as i32, inner, b"Full text", b"Licenses section", false);
}
