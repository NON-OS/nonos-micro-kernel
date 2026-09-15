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


//! The Home banner: an eyebrow, the feature title, one line of copy, a pair of
//! actions and the stamp block pinned to the top right, over a cover wash that
//! bleeds in from the right edge.

extern crate alloc;

use nonos_app_skeleton::PaintBuffer;

use crate::ui::art::cover;
use crate::ui::geometry::Rect;
use crate::ui::icon::{Glyph, Icons};
use crate::ui::metrics::{cap_h, line_h, BODY, HERO, LABEL, R_CARD, S3, S4, S5, S6, S7};
use crate::ui::paint::{fill, stroke, text, text_right};
use crate::ui::text::{truncate_to_width, upper};
use crate::ui::theme::{CYAN, EDGE, INK, MID, MUTE, RAISED};
use super::button::{button, Variant};

pub const HERO_H: i32 = 196;

pub fn action_rect(r: Rect, i: usize) -> Rect {
    let y = r.bottom() - S6 - 42;
    let x = r.x + S7 + i as i32 * 168;
    Rect::new(x, y, 156, 42)
}

pub fn action_at(r: Rect, x: i32, y: i32) -> Option<usize> {
    (0..2).find(|&i| action_rect(r, i).contains(x, y))
}

pub fn hero(
    fb: &mut PaintBuffer,
    icons: &Icons,
    r: Rect,
    eyebrow: &str,
    title: &str,
    copy: &str,
    stamp: (&str, &str),
) {
    fill(fb, r, R_CARD, RAISED);
    let wash = Rect::new(r.x + r.w * 42 / 100, r.y, r.w - r.w * 42 / 100, r.h);
    cover(fb, wash, title, R_CARD);
    fill(fb, Rect::new(r.x, r.y, r.w * 62 / 100, r.h), R_CARD, RAISED);
    stroke(fb, r, R_CARD, 1, EDGE);

    let x = r.x + S7;
    let tw = r.w * 58 / 100 - S7;
    let mut y = r.y + S6;
    text(fb, x, y, &upper(eyebrow), CYAN, LABEL);
    y += line_h(LABEL) + S3;
    text(fb, x, y, &truncate_to_width(title, HERO, tw), INK, HERO);
    y += line_h(HERO) - S3;
    text(fb, x, y, &truncate_to_width(copy, BODY, tw), MID, BODY);

    let head = Rect::new(r.x + r.w / 2, r.y + S6, r.w / 2 - S7, cap_h(LABEL) * 2);
    text_right(fb, head, stamp.0, INK, LABEL);
    let sub = Rect::new(head.x, head.bottom() + S4, head.w, cap_h(LABEL) * 2);
    text_right(fb, sub, stamp.1, MUTE, LABEL);
    fill(fb, Rect::new(sub.right() - 38, sub.bottom() + S4, 38, 1), 0, CYAN);

    button(fb, icons, action_rect(r, 0), "Play", Some(Glyph::Play), Variant::Solid);
    button(fb, icons, action_rect(r, 1), "Shuffle", Some(Glyph::Shuffle), Variant::Ghost);
    let _ = (S5, icons);
}
