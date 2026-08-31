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

use nonos_app_skeleton::paint::PaintBuffer;

pub fn plus(fb: &mut PaintBuffer, x: u32, y: u32, s: u32, argb: u32) {
    let t = 1 + s / 8;
    let n = s.saturating_sub(s / 5);
    let o = s.saturating_sub(n) / 2;
    fb.fill_rect(x + o, y + s / 2 - t / 2, n, t, argb);
    fb.fill_rect(x + s / 2 - t / 2, y + o, t, n, argb);
}

pub fn close(fb: &mut PaintBuffer, x: u32, y: u32, s: u32, argb: u32) {
    let t = 1 + s / 10;
    let o = s / 10;
    let n = s.saturating_sub(t + s / 5);
    for i in 0..=n {
        fb.fill_rect(x + o + i, y + o + i, t, t, argb);
        fb.fill_rect(x + o + i, y + o + n - i, t, t, argb);
    }
}

pub fn check(fb: &mut PaintBuffer, x: u32, y: u32, s: u32, argb: u32) {
    let t = 1 + s / 10;
    let a = s / 5;
    let b = s / 2;
    let px = x + s / 8;
    let py = y + s / 2;
    for i in 0..=a {
        fb.fill_rect(px + i, py + i, t, t, argb);
    }
    for i in 0..=b {
        fb.fill_rect(px + a + i, (py + a).saturating_sub(i), t, t, argb);
    }
}

pub fn dots(fb: &mut PaintBuffer, x: u32, y: u32, s: u32, argb: u32) {
    let d = 1 + s / 8;
    let g = 1 + s / 8;
    let top = y + s.saturating_sub(d * 3 + g * 2) / 2;
    for i in 0..3 {
        fb.fill_rect(x + s.saturating_sub(d) / 2, top + i * (d + g), d, d, argb);
    }
}
