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

//! Transport-bar geometry. The painter and the hit-test both read `bar`, so a
//! click lands on the control that was actually drawn under the pointer.

use crate::ui::geometry::Rect;
use crate::ui::metrics::{S5, S6};

pub const THUMB: i32 = 46;
pub const PLAY_R: i32 = 21;
const VOL_W: i32 = 108;
const STEP: i32 = 44;

pub struct Bar {
    pub thumb: Rect,
    pub shuffle: Rect,
    pub prev: Rect,
    pub play: Rect,
    pub next: Rect,
    pub repeat: Rect,
    pub scrub: Rect,
    pub volume: Rect,
    pub speaker: Rect,
}

pub fn bar(r: Rect) -> Bar {
    let cy = r.cy();
    let hub = r.cx() - 40;
    let btn = |i: i32| Rect::new(hub + i * STEP - 16, cy - 16, 32, 32);
    let scrub_x = hub + 2 * STEP + S6 + 16;
    let vol_x = r.right() - S6 - VOL_W;
    Bar {
        thumb: Rect::new(r.x + S6, cy - THUMB / 2, THUMB, THUMB),
        shuffle: btn(-2),
        prev: btn(-1),
        play: Rect::new(hub - PLAY_R, cy - PLAY_R, PLAY_R * 2, PLAY_R * 2),
        next: btn(1),
        repeat: btn(2),
        scrub: Rect::new(scrub_x, cy - 3, (vol_x - S5 - 26 - scrub_x).max(0), 6),
        volume: Rect::new(vol_x, cy - 3, VOL_W, 6),
        speaker: Rect::new(vol_x - 26, cy - 11, 22, 22),
    }
}
