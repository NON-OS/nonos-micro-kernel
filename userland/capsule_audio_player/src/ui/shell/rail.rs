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


//! The right rail: the cover of what is playing, its scrubber and transport,
//! the volume trim and the up-next queue. `tab_at` and `queue_row` are the one
//! geometry source, so the hit-test lands on exactly what the painter drew.

extern crate alloc;

use nonos_app_skeleton::PaintBuffer;

use crate::library::{Library, Queue};
use crate::transport::State;
use crate::model::PlayerView;
use crate::ui::art::cover;
use crate::ui::geometry::Rect;
use crate::ui::icon::{Glyph, Icons};
use crate::ui::metrics::{cap_h, pill, ITEM, LABEL, SECONDARY, S1, S2, S3, S4, S5, S6, SECTION};
use crate::ui::paint::{fill, stroke, text, text_centre, text_right};
use crate::ui::state::RAIL_TABS;
use crate::ui::text::{mmss, truncate_to_width};
use crate::ui::theme::{CYAN, CYAN_WASH, EDGE, INK, MID, MUTE, PANEL, VOID};
use crate::ui::widget::slider;

const QROW_H: i32 = 46;

fn body(r: Rect) -> Rect {
    r.pad(S5, S5)
}

fn reserved() -> i32 {
    S6 + cap_h(SECTION) * 4 + 6 + S5 + cap_h(LABEL) * 2 + 44 + S5 + 6 + S6 + 32 + S4 + QROW_H
}

fn art_rect(r: Rect) -> Rect {
    let b = body(r);
    let h = (b.h - reserved()).clamp(72, b.w);
    Rect::new(b.x, b.y, b.w, h)
}

fn seek_rect(r: Rect) -> Rect {
    let b = body(r);
    Rect::new(b.x, art_rect(r).bottom() + S6 + cap_h(SECTION) * 4, b.w, 6)
}

fn transport_row(r: Rect) -> Rect {
    let b = body(r);
    Rect::new(b.x, seek_rect(r).bottom() + S5 + cap_h(LABEL) * 2, b.w, 44)
}

fn vol_rect(r: Rect) -> Rect {
    let b = body(r);
    Rect::new(b.x + 26, transport_row(r).bottom() + S5, b.w - 26, 6)
}

pub fn tab_rect(r: Rect, i: usize) -> Rect {
    let b = body(r);
    let w = b.w / RAIL_TABS.len() as i32;
    Rect::new(b.x + i as i32 * w, vol_rect(r).bottom() + S6, w, 32)
}

pub fn tab_at(r: Rect, x: i32, y: i32) -> Option<usize> {
    (0..RAIL_TABS.len()).find(|&i| tab_rect(r, i).contains(x, y))
}

pub fn queue_row(r: Rect, i: usize) -> Rect {
    let b = body(r);
    Rect::new(b.x, tab_rect(r, 0).bottom() + S3 + i as i32 * QROW_H, b.w, QROW_H)
}

pub fn queue_visible(r: Rect) -> usize {
    let top = queue_row(r, 0).y;
    ((r.bottom() - S5 - top).max(0) / QROW_H) as usize
}

pub fn queue_at(r: Rect, x: i32, y: i32) -> Option<usize> {
    (0..queue_visible(r)).find(|&i| queue_row(r, i).contains(x, y))
}

fn round_btn(fb: &mut PaintBuffer, icons: &Icons, r: Rect, g: Glyph, ink: u32) {
    icons.centred(fb, r, 18, g, ink);
}

pub fn rail(
    fb: &mut PaintBuffer,
    icons: &Icons,
    r: Rect,
    lib: &Library,
    queue: &Queue,
    view: &PlayerView,
    tab: usize,
) {
    if r.w <= 0 {
        return;
    }
    fill(fb, r, 0, PANEL);
    fill(fb, Rect::new(r.x, r.y, 1, r.h), 0, EDGE);

    let b = body(r);
    let art = art_rect(r);
    let title = view.title.as_str();
    let artist = view.artist.as_str();
    cover(fb, art, title, 12);

    let ty = art.bottom() + S5;
    let head = truncate_to_width(title, SECTION, b.w - 34);
    text(fb, b.x, ty, &head, INK, SECTION);
    let sub = truncate_to_width(artist, SECONDARY, b.w - 34);
    text(fb, b.x, ty + cap_h(SECTION) * 2, &sub, MID, SECONDARY);
    icons.centred(fb, Rect::new(b.right() - 30, ty, 30, 30), 17, Glyph::Heart, MUTE);

    let seek = seek_rect(r);
    slider(fb, seek, view.pos_ms as u64, view.dur_ms.max(1) as u64, true);
    let times = Rect::new(b.x, seek.bottom() + S2, b.w, cap_h(LABEL) * 2);
    text(fb, times.x, times.y, &mmss(view.pos_ms), MUTE, LABEL);
    text_right(fb, times, &mmss(view.dur_ms), MUTE, LABEL);

    let tr = transport_row(r);
    let big = Rect::new(tr.cx() - 22, tr.y, 44, 44);
    fill(fb, big, pill(44), CYAN);
    let g = if view.state == State::Playing { Glyph::Pause } else { Glyph::Play };
    icons.centred(fb, big, 19, g, VOID);
    round_btn(fb, icons, Rect::new(big.x - 44 - S3, tr.y + 7, 30, 30), Glyph::Prev, INK);
    round_btn(fb, icons, Rect::new(big.right() + S3, tr.y + 7, 30, 30), Glyph::Next, INK);
    let sh = if view.shuffle { CYAN } else { MUTE };
    let rp = if view.repeat { CYAN } else { MUTE };
    round_btn(fb, icons, Rect::new(b.x, tr.y + 7, 30, 30), Glyph::Shuffle, sh);
    round_btn(fb, icons, Rect::new(b.right() - 30, tr.y + 7, 30, 30), Glyph::Repeat, rp);

    let vol = vol_rect(r);
    icons.centred(fb, Rect::new(b.x, vol.cy() - 11, 22, 22), 16, Glyph::Speaker, MID);
    let level = if view.muted { 0 } else { view.volume_q15.max(0) as u64 };
    slider(fb, vol, level, 32768, false);

    for (i, label) in RAIL_TABS.iter().enumerate() {
        let t = tab_rect(r, i);
        let on = i == tab;
        text_centre(fb, t, label, if on { INK } else { MUTE }, SECONDARY);
        if on {
            fill(fb, Rect::new(t.x + S3, t.bottom() - 2, t.w - S3 * 2, 2), 1, CYAN);
        }
    }
    fill(fb, Rect::new(b.x, tab_rect(r, 0).bottom() - 1, b.w, 1), 0, EDGE);

    let items = queue.items();
    let shown = queue_visible(r).min(items.len());
    for i in 0..shown {
        let row = queue_row(r, i);
        let Some(t) = lib.get(items[i]) else { continue };
        if lib.get(items[i]).is_some_and(|t| t.title == view.title) {
            fill(fb, row, 8, CYAN_WASH);
            stroke(fb, row, 8, 1, EDGE);
        }
        let thumb = Rect::new(row.x + S2, row.cy() - 16, 32, 32);
        cover(fb, thumb, &t.title, 7);
        let tx = thumb.right() + S3;
        let tw = row.right() - S3 - tx;
        text(
            fb,
            tx,
            row.cy() - cap_h(SECONDARY) - S1,
            &truncate_to_width(&t.title, SECONDARY, tw),
            INK,
            SECONDARY,
        );
        text(
            fb,
            tx,
            row.cy() + S1,
            &truncate_to_width(&t.artist, LABEL, tw),
            MUTE,
            LABEL,
        );
    }
    if shown == 0 {
        let empty = Rect::new(b.x, queue_row(r, 0).y, b.w, QROW_H);
        text_centre(fb, empty, "Queue is empty", MUTE, ITEM);
    }
    let _ = (S1, S4);
}
