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

//! Media card: cover, title, subtitle, and the play affordance the playing
//! card carries in place of hover.

use nonos_app_skeleton::PaintBuffer;

use crate::ui::art::{cover_scrimmed, tint};
use crate::ui::geometry::Rect;
use crate::ui::icon::{Glyph, Icons};
use crate::ui::metrics::{line_h, ITEM, R_COVER, S2, S3, SECONDARY};
use crate::ui::paint::{disc, shadow, text};
use crate::ui::text::truncate_to_width;

use super::chip::{measure as chip_w, stamp};
use crate::ui::theme::{alpha, rgb, INK, MID, VOID};

pub fn card_h(w: i32) -> i32 {
    w + S3 + line_h(ITEM) + line_h(SECONDARY)
}

pub fn art_rect(r: Rect) -> Rect {
    let room = r.h - S3 - line_h(ITEM) - line_h(SECONDARY);
    Rect::new(r.x, r.y, r.w, r.w.min(room).max(0))
}

pub fn card(
    fb: &mut PaintBuffer,
    icons: &Icons,
    r: Rect,
    id: &str,
    title: &str,
    badge: &str,
    sub: &str,
    playing: bool,
) {
    let art = art_rect(r);
    cover_scrimmed(fb, art, id, R_COVER);
    stamp(fb, Rect::new(art.x + S3, art.y + S3, chip_w(badge), 22), badge);
    if playing {
        let s = (art.w / 4).max(20);
        let c = Rect::new(art.right() - s - S3, art.bottom() - s - S3, s, s);
        let accent = tint(id);
        shadow(fb, c, (s / 2) as u32, (s / 3) as u32, alpha(rgb(accent), 0x55));
        disc(fb, c.cx(), c.cy(), s / 2, accent);
        icons.centred(fb, c, s / 2, Glyph::Play, VOID);
    }
    let ty = art.bottom() + S3;
    let t = truncate_to_width(title, ITEM, r.w);
    text(fb, r.x, ty, &t, INK, ITEM);
    let s = truncate_to_width(sub, SECONDARY, r.w);
    text(fb, r.x, ty + line_h(ITEM) - S2, &s, MID, SECONDARY);
}
