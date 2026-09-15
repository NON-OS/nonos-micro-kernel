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

//! Now Playing: the cover at display size, the metadata block beside it, and
//! the decoded waveform under both. The transport controls stay in the shell
//! bar rather than being duplicated here.

use nonos_app_skeleton::PaintBuffer;

use crate::model::PlayerView;
use crate::ui::art::cover;
use crate::ui::geometry::Rect;
use crate::ui::metrics::{line_h, DISPLAY, HERO, ITEM, LABEL, R_CARD, S3, S4, S6};
use crate::ui::paint::{fill, stroke, text};
use crate::ui::text::{mmss, truncate_to_width, upper};
use crate::ui::theme::{CYAN, EDGE, INK, MID, MUTE, VIOLET_WASH};
use crate::ui::widget::{chip_w, chip, waveform};
use crate::waveform::Waveform;

const ART: i32 = 300;

fn art_side(r: Rect) -> i32 {
    (r.w * 42 / 100).clamp(180, ART)
}

pub fn art_rect(r: Rect) -> Rect {
    let s = art_side(r);
    Rect::new(r.x, r.y + S4, s, s)
}

pub fn wave_rect(r: Rect) -> Rect {
    let a = art_rect(r);
    Rect::new(r.x, a.bottom() + S6, r.w, (r.bottom() - a.bottom() - S6 - S4).clamp(0, 120))
}

fn stat(fb: &mut PaintBuffer, r: Rect, label: &str, value: &str) {
    text(fb, r.x, r.y, &upper(label), MUTE, LABEL);
    text(fb, r.x, r.y + line_h(LABEL), value, INK, HERO);
}

pub fn paint(fb: &mut PaintBuffer, r: Rect, v: &PlayerView, w: &Waveform, id: &str) {
    let a = art_rect(r);
    fill(fb, a.inset(-S3), R_CARD + 4, VIOLET_WASH);
    cover(fb, a, id, R_CARD);
    stroke(fb, a, R_CARD, 1, EDGE);

    let tx = a.right() + S6;
    let tw = r.right() - tx;
    text(fb, tx, a.y + S3, &upper("Now playing"), CYAN, LABEL);
    let title = truncate_to_width(&v.title, DISPLAY, tw);
    text(fb, tx, a.y + S3 + line_h(LABEL), &title, INK, DISPLAY);
    let artist = truncate_to_width(&v.artist, ITEM, tw);
    text(fb, tx, a.y + S3 + line_h(LABEL) + line_h(DISPLAY), &artist, MID, ITEM);

    let cy = a.y + S3 + line_h(LABEL) + line_h(DISPLAY) + line_h(ITEM) + S4;
    let cw = chip_w(&v.format);
    let c = Rect::new(tx, cy, cw, 30);
    chip(fb, c, &v.format, false);

    let sy = cy + 30 + S6;
    stat(fb, Rect::new(tx, sy, tw / 2, 60), "Elapsed", &mmss(v.pos_ms));
    stat(fb, Rect::new(tx + tw / 2, sy, tw / 2, 60), "Duration", &mmss(v.dur_ms));

    let wr = wave_rect(r);
    if wr.h > 20 {
        text(fb, wr.x, wr.y, &upper("Waveform"), MUTE, LABEL);
        let bars = Rect::new(wr.x, wr.y + line_h(LABEL), wr.w, wr.h - line_h(LABEL));
        waveform(fb, bars, &w.buckets, v.pos_ms as u64, v.dur_ms.max(1) as u64);
    }
}
