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

//! Settings geometry: two columns. Playback controls on the left, the facts
//! the capsule can answer for itself on the right, with the library counters
//! beneath them. The painter and the hit-test share every rect here.

use crate::ui::geometry::Rect;
use crate::ui::metrics::{line_h, BODY, PAGE, S3, S4, S5, S6, SECTION};
use crate::ui::widget::{permille, TILE_H};

const ROW_H: i32 = 52;
const ROWS: i32 = 4;
const PAD: i32 = S5 + S3;
pub const FACT_H: i32 = 36;
const METER_H: i32 = 72;
pub const TOGGLES: [&str; 3] = ["Shuffle", "Repeat", "Mute output"];

#[derive(Clone, Copy)]
pub enum Hit {
    Toggle(usize),
    Volume(u32),
    Section(usize),
}

fn head_h() -> i32 {
    S4 + line_h(SECTION) + S3
}

pub const NAV_W: i32 = 196;

fn nav_shown(r: Rect) -> bool {
    r.w > NAV_W * 3
}

fn body(r: Rect) -> Rect {
    let top = r.y + line_h(PAGE) + line_h(BODY) + S3 + S4;
    let dx = if nav_shown(r) { NAV_W + S6 } else { 0 };
    Rect::new(r.x + dx, top, r.w - dx, (r.bottom() - top).max(0))
}

pub fn nav_row(r: Rect, i: usize) -> Rect {
    let top = r.y + line_h(PAGE) + line_h(BODY) + S3 + S4;
    let w = if nav_shown(r) { NAV_W } else { 0 };
    Rect::new(r.x, top + i as i32 * 38, w, 34)
}

pub fn nav_at(r: Rect, x: i32, y: i32) -> Option<usize> {
    (0..9).find(|&i| nav_row(r, i).contains(x, y))
}

fn column(r: Rect, i: i32) -> Rect {
    let b = body(r);
    let w = (b.w - S6) / 2;
    Rect::new(b.x + i * (w + S6), b.y, w, b.h)
}

pub fn panel_rect(r: Rect) -> Rect {
    let c = column(r, 0);
    Rect::new(c.x, c.y, c.w, (c.h - S6 - METER_H).max(0))
}

pub fn facts_rect(r: Rect) -> Rect {
    let c = column(r, 1);
    Rect::new(c.x, c.y, c.w, (c.h - S6 - TILE_H).max(0))
}

pub fn tiles_rect(r: Rect) -> Rect {
    let f = facts_rect(r);
    Rect::new(f.x, f.bottom() + S6, f.w, TILE_H)
}

pub fn meter_rect(r: Rect) -> Rect {
    let p = panel_rect(r);
    Rect::new(p.x, p.bottom() + S6, p.w, METER_H)
}

pub fn toggle_row(r: Rect, i: usize) -> Rect {
    let p = panel_rect(r);
    let h = ((p.h - head_h() - S4) / ROWS).min(ROW_H).max(1);
    Rect::new(p.x + PAD, p.y + head_h() + i as i32 * h, p.w - PAD * 2, h)
}

pub fn volume_rect(r: Rect) -> Rect {
    let row = toggle_row(r, 3);
    let w = (row.w / 2).min(220);
    Rect::new(row.right() - w, row.cy() - 3, w, 6)
}

pub fn hit(r: Rect, x: i32, y: i32) -> Option<Hit> {
    if let Some(i) = nav_at(r, x, y) {
        return Some(Hit::Section(i));
    }
    let vr = volume_rect(r);
    if vr.inset(-12).contains(x, y) {
        return Some(Hit::Volume(permille(vr, x)));
    }
    (0..3).find(|&i| toggle_row(r, i).contains(x, y)).map(Hit::Toggle)
}
