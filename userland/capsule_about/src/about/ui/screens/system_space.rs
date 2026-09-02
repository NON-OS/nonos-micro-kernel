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

use crate::about::data::layout;

use super::super::card::{self, titled};
use super::super::kv::pair;
use super::super::metrics::{CARD_PAD, PAIR_H};

pub const HEIGHT: u32 = card::OVERHEAD + PAIR_H * 5;

// Addresses are label-over-value rather than a kv table: a canonical 64-bit
// pointer measures wider in the mono face than any fixed label column can leave
// for it, and half an address is worse than none.
pub fn paint(fb: &mut PaintBuffer, x: u32, y: i32, w: u32) {
    let top = titled(fb, x, y, w, HEIGHT, b"Address space");
    let rows: [(&[u8], &[u8], bool); 5] = [
        (b"Kernel base", layout::KERNEL_BASE, true),
        (b"Directmap", layout::DIRECTMAP, true),
        (b"Directmap size", layout::DIRECTMAP_SIZE, false),
        (b"Capsule ring", layout::CAPSULE_RING, false),
        (b"Image", layout::IMAGE_KIND, false),
    ];
    for (i, (label, value, num)) in rows.into_iter().enumerate() {
        let row_y = top + (i as u32 * PAIR_H) as i32;
        pair(fb, x + CARD_PAD, row_y, card::inner(w), label, value, num);
    }
}
