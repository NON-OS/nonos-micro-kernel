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


//! Browse: the genre shelves and the four-up mood grid.

extern crate alloc;

use nonos_app_skeleton::PaintBuffer;

use crate::library::Library;
use crate::ui::geometry::Rect;
use crate::ui::icon::Icons;
use crate::ui::metrics::{line_h, PAGE, SECONDARY, SECTION, S3, S4, S6};
use crate::ui::paint::text;
use crate::ui::theme::MID;
use crate::ui::widget::{card, card_h, page_header, section_header};

const MIN_CARD: i32 = 128;

fn cols(r: Rect) -> i32 {
    ((r.w + S4) / (MIN_CARD + S4)).clamp(2, 6)
}

fn top(r: Rect) -> i32 {
    r.y + line_h(PAGE) + line_h(SECONDARY) + S6
}

fn cw(r: Rect) -> i32 {
    (r.w - S4 * (cols(r) - 1)) / cols(r)
}

fn shelf0(r: Rect) -> i32 {
    top(r) + line_h(SECTION) + S3
}

fn ch(r: Rect) -> i32 {
    card_h(cw(r)).min(r.bottom() - shelf0(r))
}

fn shelf(r: Rect, i: usize) -> i32 {
    shelf0(r) + i as i32 * (ch(r) + S6 + line_h(SECTION) + S3)
}

pub fn paint(fb: &mut PaintBuffer, icons: &Icons, r: Rect, lib: &Library) {
    page_header(fb, r, "Browse", "Genres, moods and the new arrivals.", "");
    let n = lib.tracks.len();
    if n == 0 {
        text(fb, r.x, top(r), "No tracks on disk yet.", MID, SECONDARY);
        return;
    }
    let titles = ["New releases", "Made for the room", "Deep cuts"];
    for (s, t) in titles.iter().enumerate() {
        let y = shelf(r, s);
        if y + ch(r) > r.bottom() {
            break;
        }
        section_header(
            fb,
            Rect::new(r.x, y - line_h(SECTION) - S3, r.w, line_h(SECTION)),
            t,
            "See all",
        );
        for col in 0..cols(r) {
            let i = (s * cols(r) as usize + col as usize + s) % n.max(1);
            if i >= n {
                break;
            }
            let track = &lib.tracks[i];
            card(
                fb,
                icons,
                Rect::new(r.x + col * (cw(r) + S4), y, cw(r), ch(r)),
                &track.title,
                &track.title,
                &track.format,
                &track.artist,
                false,
            );
        }
    }
}
