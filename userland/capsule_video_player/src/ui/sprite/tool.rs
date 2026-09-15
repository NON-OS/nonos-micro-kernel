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

//! Settings gear plus the video and disc source marks.

use super::canvas::Sprite;
use super::prim::{disc as fill_disc, ring};
use super::shape::tri;
use super::stroke::line;
use super::unit::{blank, frame, W};

const TEETH: [(i32, i32); 6] =
    [(1000, 0), (500, 866), (-500, 866), (-1000, 0), (-500, -866), (500, -866)];

pub fn gear(px: u32, rgb: u32) -> Sprite {
    let (mut s, m) = blank(px);
    let (c, band) = (m(50), m(16));
    ring(&mut s, c, c, m(38), band, rgb);
    let (t, lo, hi) = (m(14), m(32), m(41));
    for (cx, cy) in TEETH {
        let a = (c + lo * cx / 1000, c + lo * cy / 1000);
        let b = (c + hi * cx / 1000, c + hi * cy / 1000);
        line(&mut s, a, b, t, rgb);
    }
    s
}

pub fn video(px: u32, rgb: u32) -> Sprite {
    let (mut s, m) = blank(px);
    frame(&mut s, &m, [12, 24, 88, 76], m(W), rgb);
    tri(&mut s, [(m(42), m(38)), (m(42), m(62)), (m(64), m(50))], rgb);
    s
}

pub fn disc(px: u32, rgb: u32) -> Sprite {
    let (mut s, m) = blank(px);
    ring(&mut s, m(50), m(50), m(38), m(W), rgb);
    fill_disc(&mut s, m(50), m(50), m(7), rgb);
    s
}
