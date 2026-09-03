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

use crate::about::data::license::{NAME, SPDX, URL, VERSION};
use crate::about::theme::TITLE;

use super::super::card;
use super::super::kv::{kv, ROW_H};
use super::super::metrics::{CARD_PAD, TITLE_PX};
use super::super::text::line;

const NAME_H: u32 = 30;
const NAME_GAP: u32 = 10;
pub const HEIGHT: u32 = CARD_PAD * 2 + NAME_H + NAME_GAP + ROW_H * 3;

// The banner states the licence this image ships under before the text of it, so
// the identifier a reader actually needs is not buried in the scroll body.
pub fn paint(fb: &mut PaintBuffer, y: i32, w: u32) {
    let top = card::panel(fb, 0, y, w, HEIGHT);
    line(fb, CARD_PAD, top, NAME, TITLE, TITLE_PX);
    let rows: [(&[u8], &[u8], bool); 3] = [
        (b"Version", VERSION, false),
        (b"SPDX", SPDX, true),
        (b"Text", URL, false),
    ];
    let first = top + (NAME_H + NAME_GAP) as i32;
    for (i, (label, value, num)) in rows.into_iter().enumerate() {
        kv(fb, CARD_PAD, first + (i as u32 * ROW_H) as i32, card::inner(w), label, value, num);
    }
}
