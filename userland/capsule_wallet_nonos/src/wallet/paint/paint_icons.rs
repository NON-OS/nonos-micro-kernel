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

use crate::wallet::theme::BG;

fn r(fb: &mut PaintBuffer, x: i32, y: i32, w: u32, h: u32, c: u32) {
    if x >= 0 && y >= 0 {
        fb.fill_rect(x as u32, y as u32, w, h, c);
    }
}

fn disc(fb: &mut PaintBuffer, cx: i32, cy: i32, rad: i32, c: u32) {
    let d = (2 * rad) as u32;
    r(fb, cx - rad, cy - rad, d, d, c);
    r(fb, cx - rad, cy - rad, 2, 2, BG());
    r(fb, cx + rad - 2, cy - rad, 2, 2, BG());
    r(fb, cx - rad, cy + rad - 2, 2, 2, BG());
    r(fb, cx + rad - 2, cy + rad - 2, 2, 2, BG());
}

pub fn theme(fb: &mut PaintBuffer, bx: u32, by: u32, light: bool, c: u32) {
    let (cx, cy) = ((bx + 17) as i32, (by + 15) as i32);
    if light {
        disc(fb, cx - 1, cy, 7, c);
        disc(fb, cx + 4, cy - 2, 7, BG());
    } else {
        disc(fb, cx, cy, 4, c);
        r(fb, cx - 1, cy - 9, 2, 4, c);
        r(fb, cx - 1, cy + 5, 2, 4, c);
        r(fb, cx - 9, cy - 1, 4, 2, c);
        r(fb, cx + 5, cy - 1, 4, 2, c);
    }
}

pub fn cmd(fb: &mut PaintBuffer, bx: u32, by: u32, c: u32) {
    let (x, y) = ((bx + 11) as i32, (by + 9) as i32);
    for (dx, dy) in [(0, 0), (7, 0), (0, 7), (7, 7)] {
        r(fb, x + dx, y + dy, 5, 5, c);
    }
}

pub fn bell(fb: &mut PaintBuffer, bx: u32, by: u32, c: u32, badge: u32) {
    let cx = (bx + 17) as i32;
    r(fb, cx - 6, by as i32 + 16, 12, 3, c);
    r(fb, cx - 5, by as i32 + 11, 10, 5, c);
    r(fb, cx - 3, by as i32 + 8, 6, 3, c);
    r(fb, cx - 1, by as i32 + 6, 2, 2, c);
    r(fb, cx - 1, by as i32 + 19, 2, 2, c);
    disc(fb, cx + 6, by as i32 + 8, 4, badge);
}

pub fn lock(fb: &mut PaintBuffer, bx: u32, by: u32, c: u32) {
    let cx = (bx + 17) as i32;
    r(fb, cx - 4, by as i32 + 6, 2, 6, c);
    r(fb, cx + 2, by as i32 + 6, 2, 6, c);
    r(fb, cx - 4, by as i32 + 6, 8, 2, c);
    r(fb, cx - 6, by as i32 + 12, 12, 9, c);
    r(fb, cx - 1, by as i32 + 15, 2, 3, BG());
}

pub fn account(fb: &mut PaintBuffer, bx: u32, by: u32, c: u32) {
    let cx = (bx + 14) as i32;
    disc(fb, cx, by as i32 + 11, 4, c);
    r(fb, cx - 6, by as i32 + 17, 12, 5, c);
    r(fb, cx - 6, by as i32 + 17, 2, 2, BG());
    r(fb, cx + 4, by as i32 + 17, 2, 2, BG());
    let chx = (bx + 34) as i32;
    r(fb, chx - 3, by as i32 + 13, 6, 2, c);
    r(fb, chx - 2, by as i32 + 15, 4, 2, c);
    r(fb, chx - 1, by as i32 + 17, 2, 2, c);
}
