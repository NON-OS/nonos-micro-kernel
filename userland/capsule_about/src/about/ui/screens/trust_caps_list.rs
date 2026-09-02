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

use crate::about::data::caps::{is_granted, ALL_CAPS, MASK};
use crate::about::format::{hex_u64, u64_decimal};

use super::super::card;
use super::super::chip_wrap::wrap_h;
use super::super::kv::ROW_H;
use super::tile_text::push;

pub const CAP_SLOTS: usize = 21;
pub const ROLE_GAP: u32 = 12;
pub const DENIED_GAP: u32 = 14;

// The grid is driven off the descriptor table, never off a list typed here, so a
// capability added to the kernel's mask appears on the correct side by itself.
pub fn names(dst: &mut [&'static [u8]; CAP_SLOTS], granted: bool) -> usize {
    let mut n = 0usize;
    for cap in ALL_CAPS {
        if is_granted(cap.bit) == granted && n < dst.len() {
            dst[n] = cap.name;
            n += 1;
        }
    }
    n
}

// "Granted 5 of 21   mask 0x1819", assembled from the table's own arithmetic so
// the count and the mask cannot drift apart.
pub fn summary<'a>(dst: &'a mut [u8; 48], granted: bool, mask: bool) -> &'a [u8] {
    let mut slots: [&'static [u8]; CAP_SLOTS] = [b""; CAP_SLOTS];
    let count = names(&mut slots, granted) as u64;
    let mut a = [0u8; 20];
    let mut b = [0u8; 20];
    let mut n = push(dst, 0, if granted { b"Granted " } else { b"Denied " });
    n = push(dst, n, u64_decimal(count, &mut a));
    n = push(dst, n, b" of ");
    n = push(dst, n, u64_decimal(ALL_CAPS.len() as u64, &mut b));
    if mask {
        let mut hex = [0u8; 20];
        n = push(dst, n, b"    mask ");
        n = push(dst, n, hex_u64(MASK, &mut hex));
    }
    &dst[..n]
}

pub fn height(inner: u32) -> u32 {
    let mut slots: [&'static [u8]; CAP_SLOTS] = [b""; CAP_SLOTS];
    let granted = names(&mut slots, true);
    let chips = wrap_h(&slots[..granted], inner);
    let denied = names(&mut slots, false);
    let denied_chips = wrap_h(&slots[..denied], inner);
    let roles = granted as u32 * ROW_H;
    card::OVERHEAD + ROW_H + chips + ROLE_GAP + roles + DENIED_GAP + ROW_H + denied_chips
}
