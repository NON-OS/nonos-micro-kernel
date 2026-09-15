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


//! Radio: the always-on stations, three to a row.

extern crate alloc;

use nonos_app_skeleton::PaintBuffer;

use crate::ui::geometry::Rect;
use crate::ui::icon::Icons;
use crate::ui::metrics::{line_h, PAGE, SECONDARY, S4, S6};
use crate::ui::widget::{card, page_header};

const COLS: i32 = 3;
const MIN_CARD_H: i32 = 96;

const STATIONS: [(&str, &str); 6] = [
    ("Focus Radio", "Deep work, deeper sound"),
    ("Night Drive Radio", "Late roads, neon lights"),
    ("Chill Radio", "Relax and recharge"),
    ("Discover Radio", "New music, always"),
    ("Ambient Radio", "Peace in every frequency"),
    ("Retro Radio", "Synth, always"),
];

fn top(r: Rect) -> i32 {
    r.y + line_h(PAGE) + line_h(SECONDARY) + S6
}

fn card_h(r: Rect) -> i32 {
    let rows = (STATIONS.len() as i32 + COLS - 1) / COLS;
    ((r.bottom() - top(r) - S4 * (rows - 1)) / rows).max(MIN_CARD_H)
}

pub fn tile(r: Rect, i: usize) -> Rect {
    let w = (r.w - S4 * (COLS - 1)) / COLS;
    let h = card_h(r);
    let col = i as i32 % COLS;
    let row = i as i32 / COLS;
    Rect::new(r.x + col * (w + S4), top(r) + row * (h + S4), w, h)
}

pub fn tile_at(r: Rect, x: i32, y: i32) -> Option<usize> {
    (0..STATIONS.len()).find(|&i| tile(r, i).contains(x, y))
}

pub fn paint(fb: &mut PaintBuffer, icons: &Icons, r: Rect) {
    page_header(fb, r, "Radio", "Always-on stations, tuned to the moment.", "Lossless where available");
    for (i, (name, sub)) in STATIONS.iter().enumerate() {
        let t = tile(r, i);
        if t.bottom() > r.bottom() {
            break;
        }
        card(fb, icons, t, name, name, "Live", sub, false);
    }
}
