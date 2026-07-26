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

use crate::library::{Library, Queue};
use crate::library::Track;
use crate::ui::geometry::{list_row_h, Rect};
use crate::ui::sprite::note;

use super::palette::{rgb24, ACCENT, GROOVE, MUTED, PANEL, TEXT};

pub fn paint_list(fb: &mut PaintBuffer, lib: &Library, q: &Queue, current: Option<usize>, r: &Rect) {
    let rh = list_row_h(r);
    fb.text_ttf(r.x as i32, r.y as i32, "Library", TEXT, (rh as f32) * 0.5);
    let top = r.y + rh;
    for (i, t) in lib.tracks.iter().enumerate() {
        let ry = top + i as u32 * rh;
        if ry + rh > r.y + r.h {
            break;
        }
        row(fb, t, current == Some(i), q.contains(i), r.x, ry, r.w, rh);
    }
    if lib.tracks.is_empty() {
        fb.text(r.x, top, b"No tracks in /audio", MUTED);
    }
}

fn row(fb: &mut PaintBuffer, t: &Track, playing: bool, queued: bool, x: u32, y: u32, w: u32, h: u32) {
    let bg = if playing { PANEL } else { GROOVE };
    fb.fill_rect(x, y + 3, w, h.saturating_sub(6), bg);
    let ic = if playing { ACCENT } else { MUTED };
    let is = h * 45 / 100;
    let g = note(48, rgb24(ic));
    fb.blit_rgba8_scaled(x + h / 4, y + h.saturating_sub(is) / 2, is, is, &g.rgba, g.w, g.h);
    let tx = x + h;
    fb.text(tx, y + h / 3 - 4, t.title.as_bytes(), TEXT);
    fb.text(tx, y + h * 2 / 3 - 4, t.format.as_bytes(), MUTED);
    if queued {
        fb.text(x + w.saturating_sub(h), y + h / 2 - 4, b"Q", ACCENT);
    }
}
