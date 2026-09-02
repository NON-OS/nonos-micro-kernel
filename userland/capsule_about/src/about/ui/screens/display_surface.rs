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

use crate::about::data::display::primary_dimensions;
use crate::about::format::u64_decimal;

use super::super::card::{self, titled};
use super::super::kv::{kv, ROW_H};
use super::super::metrics::CARD_PAD;

pub const HEIGHT: u32 = card::OVERHEAD + ROW_H * 4;

// The dimensions are one query answered once. When it fails both axes say so
// rather than falling back to a plausible size the compositor never reported.
pub fn paint(fb: &mut PaintBuffer, y: i32, w: u32) {
    let top = titled(fb, 0, y, w, HEIGHT, b"Surface");
    let dims = primary_dimensions();
    let mut bw = [0u8; 20];
    let mut bh = [0u8; 20];
    let (width, height): (&[u8], &[u8]) = match dims {
        Some((dw, dh)) => (u64_decimal(dw as u64, &mut bw), u64_decimal(dh as u64, &mut bh)),
        None => (b"unavailable", b"unavailable"),
    };
    let rows: [(&[u8], &[u8], bool); 4] = [
        (b"Width (px)", width, dims.is_some()),
        (b"Height (px)", height, dims.is_some()),
        (b"Format", b"ARGB8888", false),
        (b"Query", b"nonos_display_dimensions(0)", false),
    ];
    for (i, (label, value, num)) in rows.into_iter().enumerate() {
        let row_y = top + (i as u32 * ROW_H) as i32;
        kv(fb, CARD_PAD, row_y, card::inner(w), label, value, num);
    }
}
