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

use crate::about::data::license::TEXT;
use crate::about::theme::FOREGROUND;

use super::super::card::{self, titled};
use super::super::metrics::{BODY_PX, CARD_PAD, LINE_STEP};
use super::super::text::line;

pub fn lines() -> usize {
    TEXT.lines().count()
}

pub fn height() -> u32 {
    card::OVERHEAD + lines() as u32 * LINE_STEP
}

// The whole licence is thousands of pixels tall, so only the rows the pane can
// show are walked: the first index comes from how far the card top sits above the
// pane, and one extra row on each side covers the partially visible ones.
pub fn paint(fb: &mut PaintBuffer, y: i32, w: u32) {
    let top = titled(fb, 0, y, w, height(), b"License text");
    let first = if top < 0 { (-top) as u32 / LINE_STEP } else { 0 };
    let count = fb.height / LINE_STEP + 2;
    let body = TEXT.lines().enumerate().skip(first as usize).take(count as usize);
    for (i, row) in body {
        let row_y = top + (i as u32 * LINE_STEP) as i32;
        line(fb, CARD_PAD, row_y, row.as_bytes(), FOREGROUND, BODY_PX);
    }
}
