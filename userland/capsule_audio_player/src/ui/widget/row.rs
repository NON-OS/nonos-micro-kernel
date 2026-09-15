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

//! Track table: the column solver, the header rule and one row.

extern crate alloc;

use alloc::string::String;
use nonos_app_skeleton::PaintBuffer;

use crate::library::Track;
use crate::ui::art::cover;
use crate::ui::geometry::Rect;
use crate::ui::icon::{Glyph, Icons};
use crate::ui::metrics::{line_h, DATA, ITEM, LABEL, R_THUMB, S3, S4, SECONDARY};
use crate::ui::paint::{fill, text, text_mid, text_right};
use crate::ui::text::{mmss, push_u32, truncate_to_width, upper};
use crate::ui::theme::{CYAN, EDGE, GREEN, HOVER, INK, MID, MUTE};

use super::eq::equalizer;

pub const ROW_H: i32 = 54;

pub struct Cols {
    pub index: Rect,
    pub art: Rect,
    pub title: Rect,
    pub format: Rect,
    pub time: Rect,
    pub pip: Rect,
}

pub fn cols(r: Rect) -> Cols {
    let art_s = ROW_H - S4;
    let ix = Rect::new(r.x + S3, r.y, 34, r.h);
    let art = Rect::new(ix.right() + S3, r.cy() - art_s / 2, art_s, art_s);
    let pip = Rect::new(r.right() - 34, r.y, 26, r.h);
    let time = Rect::new(pip.x - 80, r.y, 70, r.h);
    let format = Rect::new(time.x - 96, r.y, 86, r.h);
    Cols { index: ix, art, title: Rect::new(art.right() + S4, r.y, format.x - art.right() - S4 * 2, r.h), format, time, pip }
}

pub fn header(fb: &mut PaintBuffer, r: Rect) {
    let c = cols(r);
    text_mid(fb, c.index, "#", MUTE, LABEL);
    text_mid(fb, c.title, &upper("Title"), MUTE, LABEL);
    text_mid(fb, c.format, &upper("Format"), MUTE, LABEL);
    text_right(fb, c.time, &upper("Time"), MUTE, LABEL);
    fill(fb, Rect::new(r.x, r.bottom() - 1, r.w, 1), 0, EDGE);
}

pub struct Flags {
    pub playing: bool,
    pub queued: bool,
    pub hover: bool,
}

pub fn row(fb: &mut PaintBuffer, icons: &Icons, r: Rect, t: &Track, n: usize, f: &Flags, step: u32) {
    let (playing, queued) = (f.playing, f.queued);
    let c = cols(r);
    if f.hover && !playing {
        fill(fb, r, 8, HOVER);
    }
    if playing {
        fill(fb, r, 8, HOVER);
        equalizer(fb, c.index.centred(16, 16), true, step);
    } else {
        let mut s = String::new();
        push_u32(&mut s, n as u32, 2);
        text_mid(fb, c.index, &s, MUTE, DATA);
    }
    cover(fb, c.art, &t.path, R_THUMB as u32);
    let fg = if playing { CYAN } else { INK };
    let ty = r.cy() - (line_h(ITEM) + line_h(SECONDARY)) / 2;
    let title = truncate_to_width(&t.title, ITEM, c.title.w);
    text(fb, c.title.x, ty, &title, fg, ITEM);
    let artist = truncate_to_width(&t.artist, SECONDARY, c.title.w);
    text(fb, c.title.x, ty + line_h(ITEM), &artist, MID, SECONDARY);
    text_mid(fb, c.format, &t.format, MID, DATA);
    text_right(fb, c.time, &mmss(t.dur_ms), MID, DATA);
    if queued {
        icons.centred(fb, c.pip, 18, Glyph::Check, GREEN);
    }
}
