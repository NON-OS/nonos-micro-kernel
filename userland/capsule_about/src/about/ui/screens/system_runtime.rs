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

use crate::about::data::runtime::{sample, Runtime};
use crate::about::format::u64_decimal;

use super::super::card::{self, titled};
use super::super::kv::{kv, ROW_H};
use super::super::metrics::{CARD_PAD, METER_H};
use super::runtime_meter::meter;
use super::runtime_text::{loads, memory};

pub const HEIGHT: u32 = card::OVERHEAD + ROW_H * 3 + METER_H + 12;

const LABELS: [&[u8]; 3] = [b"Capsules", b"Memory", b"Load 1/5/15"];

// What the kernel is running right now, read through the ungated proc-stat call.
// When the call fails every row says so and the meter is not drawn: a zero-length
// bar would read as an idle machine rather than as an unanswered question.
pub fn paint(fb: &mut PaintBuffer, x: u32, y: i32, w: u32) {
    let top = titled(fb, x, y, w, HEIGHT, b"Runtime");
    match sample() {
        Some(r) => live(fb, x, top, w, &r),
        None => {
            for (i, label) in LABELS.into_iter().enumerate() {
                let row_y = top + (i as u32 * ROW_H) as i32;
                kv(fb, x + CARD_PAD, row_y, card::inner(w), label, b"unavailable", false);
            }
        }
    }
}

fn live(fb: &mut PaintBuffer, x: u32, top: i32, w: u32, r: &Runtime) {
    let mut count = [0u8; 20];
    let mut mem = [0u8; 64];
    let mut load = [0u8; 48];
    let values: [&[u8]; 3] = [
        u64_decimal(r.capsules as u64, &mut count),
        memory(r, &mut mem),
        loads(r, &mut load),
    ];
    for (i, value) in values.into_iter().enumerate() {
        let row_y = top + (i as u32 * ROW_H) as i32;
        kv(fb, x + CARD_PAD, row_y, card::inner(w), LABELS[i], value, true);
    }
    meter(fb, x, top + (ROW_H * 3 + 8) as i32, card::inner(w), r);
}
