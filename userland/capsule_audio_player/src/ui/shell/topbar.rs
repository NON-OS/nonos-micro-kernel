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

//! The top strip over the content column: a search field that mirrors the live
//! query buffer, and a right-hand chip naming the mount the library was
//! scanned from. Clicking the field routes to the Search screen.

use nonos_app_skeleton::PaintBuffer;

use crate::ui::geometry::Rect;
use crate::ui::icon::{Glyph, Icons};
use crate::ui::metrics::{pill, ITEM, LABEL, S2, S3, S4, S5, S6};
use crate::ui::paint::{fill, stroke, text_mid, text_right};
use crate::ui::theme::{alpha, CYAN, DEEP, EDGE, INK, MID, MUTE, PRESS, RAISED};

const FIELD_H: i32 = 34;
const CHIP_W: i32 = 132;
const BELL_W: i32 = 30;

pub fn clear(r: Rect) -> Rect {
    let f = field(r);
    Rect::new(f.right() - 26, f.y, 22, f.h)
}

pub fn field(r: Rect) -> Rect {
    Rect::new(r.x + S5, r.cy() - FIELD_H / 2, (r.w - S5 * 2 - CHIP_W - BELL_W - S5 * 2).min(420), FIELD_H)
}

pub fn topbar(fb: &mut PaintBuffer, icons: &Icons, r: Rect, query: &str, focused: bool) {
    fill(fb, Rect::new(r.x, r.bottom() - 1, r.w, 1), 0, EDGE);

    let f = field(r);
    fill(fb, f, pill(f.h), DEEP);
    if focused {
        fill(fb, f, pill(f.h), PRESS);
    }
    stroke(fb, f, pill(f.h), 1, if focused { alpha(CYAN, 0xB0) } else { EDGE });
    icons.centred(fb, Rect::new(f.x + S3, f.y, 22, f.h), 16, Glyph::Search, MID);
    let tx = f.x + S3 + 22 + S3;
    let (label, ink) = if query.is_empty() { ("Search the library", MUTE) } else { (query, INK) };
    let text_w = f.right() - tx - S4 - if query.is_empty() { 0 } else { 24 };
    text_mid(fb, Rect::new(tx, f.y, text_w, f.h), label, ink, ITEM);
    if !query.is_empty() {
        icons.centred(fb, clear(r), 14, Glyph::Close, MID);
    }

    let bell = Rect::new(r.right() - S6 - BELL_W, r.cy() - BELL_W / 2, BELL_W, BELL_W);
    icons.centred(fb, bell, 17, Glyph::Bell, MID);

    let chip = Rect::new(bell.x - S4 - CHIP_W, r.cy() - FIELD_H / 2, CHIP_W, FIELD_H);
    fill(fb, chip, pill(chip.h), RAISED);
    stroke(fb, chip, pill(chip.h), 1, EDGE);
    icons.centred(fb, Rect::new(chip.x + S3, chip.y, 20, chip.h), 15, Glyph::Download, CYAN);
    icons.centred(fb, Rect::new(chip.right() - 22, chip.y, 20, chip.h), 13, Glyph::Chevron, MID);
    text_right(fb, Rect::new(chip.x, chip.y, chip.w - 26 - S2, chip.h), "/audio", MID, LABEL);
}
