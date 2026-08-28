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

use super::convert_geom::{chip, row, swap, CHIPS};
use crate::calc::convert::{list, Category};

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ConvertHit {
    Chip(usize),
    From(usize),
    To(usize),
    Swap,
}

fn inside(rect: (i32, i32, i32, i32), x: i32, y: i32) -> bool {
    let (rx, ry, rw, rh) = rect;
    rw > 0 && rh > 0 && x >= rx && x < rx + rw && y >= ry && y < ry + rh
}

pub fn at(cat: Category, win_w: i32, x: i32, y: i32) -> Option<ConvertHit> {
    for i in 0..CHIPS {
        if inside(chip(win_w, i), x, y) {
            return Some(ConvertHit::Chip(i));
        }
    }
    if inside(swap(win_w), x, y) {
        return Some(ConvertHit::Swap);
    }
    for i in 0..list(cat).len() {
        if inside(row(win_w, true, i), x, y) {
            return Some(ConvertHit::From(i));
        }
        if inside(row(win_w, false, i), x, y) {
            return Some(ConvertHit::To(i));
        }
    }
    None
}
