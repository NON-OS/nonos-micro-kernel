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

//! Search: the live query echoed as a heading, then the filtered rows. With an
//! empty query the screen names what it searches rather than pretending to
//! have suggestions.

extern crate alloc;

use alloc::string::String;
use nonos_app_skeleton::PaintBuffer;

use crate::library::{Library, Queue};
use crate::ui::geometry::Rect;
use crate::ui::icon::Icons;
use crate::ui::metrics::{line_h, BODY, PAGE, S3, S4};
use crate::ui::text::count;
use crate::ui::icon::Glyph;
use crate::ui::widget::{button, button_w, page_header, row, table_header, Flags, Variant, ROW_H};

const CLEAR: &str = "Clear";
const PLAY: &str = "Play now";

pub fn clear_rect(r: Rect) -> Rect {
    let w = button_w(CLEAR, true);
    Rect::new(r.right() - w, r.y + (line_h(PAGE) - 38) / 2, w, 38)
}

pub fn play_rect(r: Rect) -> Rect {
    let c = clear_rect(r);
    let w = button_w(PLAY, true);
    Rect::new(c.x - S4 - w, c.y, w, c.h)
}

pub fn play_at(r: Rect, hits: &[usize], x: i32, y: i32) -> Option<usize> {
    if hits.is_empty() || !play_rect(r).contains(x, y) {
        return None;
    }
    hits.first().copied()
}

fn head_rect(r: Rect) -> Rect {
    Rect::new(r.x, r.y + line_h(PAGE) + line_h(BODY) + S3 + S4, r.w, 28)
}

pub fn list_rect(r: Rect) -> Rect {
    let top = head_rect(r).bottom() + S3;
    Rect::new(r.x, top, r.w, (r.bottom() - top).max(0))
}

pub fn visible(r: Rect) -> usize {
    (list_rect(r).h / ROW_H).max(0) as usize
}

pub fn row_at(r: Rect, scroll: usize, len: usize, x: i32, y: i32) -> Option<usize> {
    let l = list_rect(r);
    if !l.contains(x, y) {
        return None;
    }
    let idx = scroll + ((y - l.y) / ROW_H) as usize;
    if idx < len { Some(idx) } else { None }
}

pub fn paint(
    fb: &mut PaintBuffer,
    icons: &Icons,
    r: Rect,
    lib: &Library,
    queue: &Queue,
    hits: &[usize],
    query: &str,
    scroll: usize,
    playing: Option<usize>,
    hover: Option<usize>,
    step: u32,
) {
    let title = if query.is_empty() {
        String::from("Search")
    } else {
        let mut s = String::from("\u{201c}");
        s.push_str(query);
        s.push('\u{201d}');
        s
    };
    let sub = if query.is_empty() {
        count(lib.tracks.len(), "track indexed", "tracks indexed")
    } else {
        count(hits.len(), "match", "matches")
    };
    page_header(fb, r, &title, &sub, "");
    if !query.is_empty() {
        button(fb, icons, clear_rect(r), CLEAR, Some(Glyph::Close), Variant::Ghost);
        if !hits.is_empty() {
            button(fb, icons, play_rect(r), PLAY, Some(Glyph::Play), Variant::Glow);
        }
    }
    table_header(fb, head_rect(r));
    let l = list_rect(r);
    for slot in 0..visible(r) {
        let Some(&idx) = hits.get(scroll + slot) else { break };
        let Some(t) = lib.get(idx) else { continue };
        let rr = Rect::new(l.x, l.y + slot as i32 * ROW_H, l.w, ROW_H);
        let f = Flags {
            playing: playing == Some(idx),
            queued: queue.contains(idx),
            hover: hover == Some(idx),
        };
        row(fb, icons, rr, t, idx + 1, &f, step);
    }
}
