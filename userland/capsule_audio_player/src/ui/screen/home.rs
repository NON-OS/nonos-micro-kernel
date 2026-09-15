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


//! Home: the feature banner, then the six-up shelves. `grid_top` and `card_at`
//! are the one geometry source, so the hit-test lands on exactly the tile the
//! painter drew.

extern crate alloc;

use nonos_app_skeleton::PaintBuffer;

use crate::library::Library;
use crate::ui::geometry::Rect;
use crate::ui::icon::Icons;
use crate::ui::metrics::{line_h, SECTION, S3, S4, S5, S6};
use crate::ui::widget::{card, card_h, hero, section_header, HERO_H};

const MIN_CARD: i32 = 128;
const GAP: i32 = S4;

fn cols(r: Rect) -> i32 {
    ((r.w + GAP) / (MIN_CARD + GAP)).clamp(2, 6)
}

pub fn card_w(r: Rect) -> i32 {
    (r.w - GAP * (cols(r) - 1)) / cols(r)
}

pub fn grid_top(r: Rect) -> i32 {
    r.y + HERO_H + S6 + line_h(SECTION) + S3
}

fn shelf_h(r: Rect) -> i32 {
    card_h(card_w(r)).min(r.bottom() - grid_top(r))
}

fn shelf_top(r: Rect, shelf: usize) -> i32 {
    grid_top(r) + shelf as i32 * (shelf_h(r) + S6 + line_h(SECTION) + S3)
}

pub fn shelves(r: Rect) -> usize {
    let mut n = 0;
    while n < 3 && shelf_top(r, n) + shelf_h(r) <= r.bottom() {
        n += 1;
    }
    n
}

pub fn card_rect(r: Rect, shelf: usize, col: i32) -> Rect {
    let w = card_w(r);
    Rect::new(r.x + col * (w + GAP), shelf_top(r, shelf), w, shelf_h(r))
}

pub fn card_at(r: Rect, n: usize, x: i32, y: i32) -> Option<usize> {
    for shelf in 0..shelves(r) {
        for col in 0..cols(r) {
            let i = shelf * cols(r) as usize + col as usize;
            if i < n && card_rect(r, shelf, col).contains(x, y) {
                return Some(i);
            }
        }
    }
    None
}

const SHELF_TITLES: [(&str, &str); 3] =
    [("Made for you", "See all"), ("Recently added", "See all"), ("Jump back in", "See all")];

pub fn paint(
    fb: &mut PaintBuffer,
    icons: &Icons,
    r: Rect,
    lib: &Library,
    playing: Option<usize>,
) {
    let banner = Rect::new(r.x, r.y, r.w, HERO_H);
    let feature = lib.tracks.first();
    let title = feature.map(|t| t.title.as_str()).unwrap_or("Your library");
    hero(
        fb,
        icons,
        banner,
        "Featured today",
        title,
        "Picked up where you left off, tuned to the hour and the room you are in.",
        ("NONOS AUDIO", "LOSSLESS"),
    );

    let n = lib.tracks.len();
    for shelf in 0..shelves(r) {
        let head = Rect::new(r.x, shelf_top(r, shelf) - line_h(SECTION) - S3, r.w, line_h(SECTION));
        let (t, a) = SHELF_TITLES[shelf.min(2)];
        section_header(fb, head, t, a);
        for col in 0..cols(r) {
            let i = shelf * cols(r) as usize + col as usize;
            if i >= n {
                break;
            }
            let track = &lib.tracks[i];
            card(
                fb,
                icons,
                card_rect(r, shelf, col),
                &track.title,
                &track.title,
                &track.format,
                &track.artist,
                playing == Some(i),
            );
        }
    }
    let _ = S5;
}
