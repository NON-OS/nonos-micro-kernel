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

use crate::clock::angles::{hand_end, hour_angle, minute_angle, second_angle};
use crate::clock::geom::{line, ring};
use crate::clock::state::State;
use crate::clock::theme;

pub fn paint(state: &State, fb: &mut PaintBuffer, cx: i32, cy: i32, radius: i32) {
    ring(fb, cx, cy, radius, theme::DIM);
    let mut tick = 0;
    while tick < 12 {
        let a = tick * 30;
        let (x0, y0) = hand_end(cx, cy, a, radius - 10);
        let (x1, y1) = hand_end(cx, cy, a, radius);
        line(fb, x0, y0, x1, y1, 2, theme::DIM);
        tick += 1;
    }
    let r = &state.rtc;
    let (hx, hy) = hand_end(cx, cy, hour_angle(r.hour, r.minute), radius / 2);
    line(fb, cx, cy, hx, hy, 4, theme::FG);
    let (mx, my) = hand_end(cx, cy, minute_angle(r.minute, r.second), radius * 3 / 4);
    line(fb, cx, cy, mx, my, 3, theme::FG);
    let (sx, sy) = hand_end(cx, cy, second_angle(r.second), radius * 4 / 5);
    line(fb, cx, cy, sx, sy, 2, theme::ACCENT);
}
