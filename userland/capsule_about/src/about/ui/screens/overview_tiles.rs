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

use crate::about::data::caps::{is_granted, ALL_CAPS};
use crate::about::data::display::primary_dimensions;
use crate::about::data::uptime::read_millis;
use crate::about::format::u64_decimal;
use super::super::gauge;
use super::super::metrics::TILE_GAP;
use super::tile_text::{dims, ratio, uptime};

const DAY_S: u64 = 86_400;

// The third column absorbs the rounding remainder so the row ends flush with the
// cards above and below it rather than a pixel short.
pub fn paint(fb: &mut PaintBuffer, y: i32, w: u32) {
    if y < 0 || y + gauge::HEIGHT as i32 > fb.height as i32 {
        return;
    }
    let y = y as u32;
    let col = (w.saturating_sub(TILE_GAP * 2)) / 3;
    let last = w.saturating_sub((col + TILE_GAP) * 2);
    uptime_tile(fb, 0, y, col);
    caps_tile(fb, col + TILE_GAP, y, col);
    display_tile(fb, (col + TILE_GAP) * 2, y, last);
}

// The ring is the position inside the current day, which is the only fraction the
// clock can honestly offer: there is no total to divide an uptime by.
fn uptime_tile(fb: &mut PaintBuffer, x: u32, y: u32, w: u32) {
    let mut value = [0u8; 24];
    let mut raw = [0u8; 20];
    match read_millis() {
        Some(ms) => {
            let v = uptime(ms, &mut value);
            let sub = u64_decimal(ms, &mut raw);
            gauge::tile(fb, x, y, w, b"Uptime", v, sub, (ms / 1000) % DAY_S, DAY_S);
        }
        None => gauge::tile(fb, x, y, w, b"Uptime", b"unavailable", b"mk_time_millis", 0, 1),
    }
}

fn caps_tile(fb: &mut PaintBuffer, x: u32, y: u32, w: u32) {
    let total = ALL_CAPS.len() as u64;
    let granted = ALL_CAPS.iter().filter(|c| is_granted(c.bit)).count() as u64;
    let mut value = [0u8; 24];
    let v = ratio(granted, total, &mut value);
    gauge::tile(fb, x, y, w, b"Capabilities", v, b"granted to this capsule", granted, total);
}

fn display_tile(fb: &mut PaintBuffer, x: u32, y: u32, w: u32) {
    let mut value = [0u8; 24];
    match primary_dimensions() {
        Some((dw, dh)) => {
            let v = dims(dw, dh, &mut value);
            gauge::tile(fb, x, y, w, b"Display", v, b"ARGB8888", 1, 1);
        }
        None => gauge::tile(fb, x, y, w, b"Display", b"unavailable", b"ARGB8888", 0, 1),
    }
}
