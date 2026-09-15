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

//! The transport bar across the foot of the window: cover thumb and metadata
//! on the left, the play cluster at the centre, scrubber and volume on the
//! right.

use nonos_app_skeleton::PaintBuffer;

use crate::model::PlayerView;
use crate::transport::State;
use crate::ui::art::cover;
use crate::ui::geometry::Rect;
use crate::ui::icon::{Glyph, Icons};
use crate::ui::metrics::{pill, ITEM, LABEL, R_THUMB, S2, S3, S4, S5};
use crate::ui::paint::{disc, fill, ring, stroke, text, text_right};
use crate::ui::text::{mmss, truncate_to_width};
use crate::ui::theme::{alpha, CYAN, EDGE, INK, MID, MUTE, PANEL, RED, VOID};
use crate::ui::widget::slider;

use super::bar::{bar, Bar, PLAY_R};

fn toggle(fb: &mut PaintBuffer, icons: &Icons, r: Rect, g: Glyph, on: bool) {
    if on {
        fill(fb, r, pill(r.h), alpha(CYAN, 0x28));
    }
    icons.centred(fb, r, 18, g, if on { CYAN } else { MID });
}

fn meta(fb: &mut PaintBuffer, b: &Bar, v: &PlayerView, id: &str) {
    cover(fb, b.thumb, id, R_THUMB);
    stroke(fb, b.thumb, R_THUMB, 1, EDGE);
    let tx = b.thumb.right() + S4;
    let tw = b.shuffle.x - S5 - tx;
    let title = truncate_to_width(&v.title, ITEM, tw);
    text(fb, tx, b.thumb.y + S2, &title, INK, ITEM);
    let sub = truncate_to_width(&v.artist, LABEL, tw);
    text(fb, tx, b.thumb.y + S2 + 25, &sub, MUTE, LABEL);
}

fn cluster(fb: &mut PaintBuffer, icons: &Icons, b: &Bar, v: &PlayerView) {
    toggle(fb, icons, b.shuffle, Glyph::Shuffle, v.shuffle);
    icons.centred(fb, b.prev, 20, Glyph::Prev, INK);
    icons.centred(fb, b.next, 20, Glyph::Next, INK);
    toggle(fb, icons, b.repeat, Glyph::Repeat, v.repeat);
    disc(fb, b.play.cx(), b.play.cy(), PLAY_R, CYAN);
    ring(fb, b.play.cx(), b.play.cy(), PLAY_R + 4, 1, alpha(CYAN, 0x50));
    let g = if v.state == State::Playing { Glyph::Pause } else { Glyph::Play };
    icons.centred(fb, b.play, 20, g, VOID);
}

fn scrubber(fb: &mut PaintBuffer, b: &Bar, r: Rect, v: &PlayerView) {
    let stamps = Rect::new(b.scrub.x, r.y + S3, b.scrub.w, 20);
    text(fb, stamps.x, stamps.y, &mmss(v.pos_ms), MUTE, LABEL);
    text_right(fb, stamps, &mmss(v.dur_ms), MUTE, LABEL);
    slider(fb, b.scrub, v.pos_ms as u64, v.dur_ms.max(1) as u64, true);
}

pub fn transport(fb: &mut PaintBuffer, icons: &Icons, r: Rect, v: &PlayerView, id: &str) {
    fill(fb, r, 0, PANEL);
    fill(fb, Rect::new(r.x, r.y, r.w, 1), 0, EDGE);
    let b = bar(r);
    meta(fb, &b, v, id);
    cluster(fb, icons, &b, v);
    scrubber(fb, &b, r, v);
    icons.centred(fb, b.speaker, 17, Glyph::Speaker, if v.muted { RED } else { MID });
    let vol = if v.muted { 0 } else { v.volume_q15.max(0) as u64 };
    slider(fb, b.volume, vol, 32768, false);
}
