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


//! Downloads: the offline-storage meter and the tracks kept on device.

extern crate alloc;

use nonos_app_skeleton::PaintBuffer;

use crate::library::Library;
use crate::ui::geometry::Rect;
use crate::ui::icon::Icons;
use crate::ui::metrics::{line_h, PAGE, SECONDARY, S4, S6};
use crate::ui::widget::{page_header, row, storage_meter, Flags};

const CARD_H: i32 = 92;

fn top(r: Rect) -> i32 {
    r.y + line_h(PAGE) + line_h(SECONDARY) + S6
}

pub fn list_top(r: Rect) -> i32 {
    top(r) + CARD_H + S6
}

pub fn row_h() -> i32 {
    line_h(SECONDARY) + S4 * 2
}

pub fn visible(r: Rect) -> usize {
    ((r.bottom() - list_top(r)).max(0) / row_h()) as usize
}

pub fn row_at(r: Rect, n: usize, x: i32, y: i32) -> Option<usize> {
    if x < r.x || x > r.right() {
        return None;
    }
    let i = ((y - list_top(r)).max(0) / row_h()) as usize;
    if y >= list_top(r) && i < n.min(visible(r)) {
        Some(i)
    } else {
        None
    }
}

pub fn paint(
    fb: &mut PaintBuffer,
    icons: &Icons,
    r: Rect,
    lib: &Library,
    step: u32,
    playing: Option<usize>,
) {
    let n = lib.tracks.len();
    page_header(fb, r, "Downloads", "Kept on device, ready offline.", "Stored locally");
    let card = Rect::new(r.x, top(r), r.w, CARD_H);
    storage_meter(fb, card, n as u32, n.max(1) as u32 * 2, "3.4 GB of 10 GB used");
    let shown = visible(r).min(n);
    for i in 0..shown {
        let t = &lib.tracks[i];
        let rr = Rect::new(r.x, list_top(r) + i as i32 * row_h(), r.w, row_h());
        let f = Flags { playing: playing == Some(i), queued: false, hover: false };
        row(fb, icons, rr, t, i + 1, &f, step);
    }
}
