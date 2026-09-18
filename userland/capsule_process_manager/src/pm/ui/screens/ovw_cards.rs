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

//! What every stat card shares: the meter at its foot, the load tint and
//! the in-place subcaption builder. The cards themselves are in
//! `ovw_cpu_mem.rs` and `ovw_activity.rs`.

use nonos_app_skeleton::PaintBuffer;

use crate::pm::format::u32_decimal;
use crate::pm::theme::{ACCENT, AMBER, DANGER};

use super::super::bars;
use super::super::metrics::{BAR_H, CARD_H, CARD_PAD};

/// The meter at the card's foot: `n` of `d`, tinted.
pub(super) fn meter(fb: &mut PaintBuffer, at: (u32, u32, u32), band: u32, n: u64, d: u64, t: u32) {
    let bar_y = (at.1 + CARD_H).saturating_sub(CARD_PAD + BAR_H).max(band);
    let w = at.2.saturating_sub(CARD_PAD * 2);
    bars::hbar(fb, at.0 + CARD_PAD, bar_y, w, BAR_H, n, d, t);
}

pub(super) fn load_tint(pct: u32) -> u32 {
    match pct {
        0..=50 => ACCENT,
        51..=85 => AMBER,
        _ => DANGER,
    }
}

/// "head 42 tail" built in place: there is no formatter.
pub(super) fn sub_n(out: &mut [u8], head: &[u8], v: u32, tail: &[u8]) -> usize {
    let mut i = head.len().min(out.len());
    out[..i].copy_from_slice(&head[..i]);
    i += u32_decimal(v, &mut out[i..]);
    let end = (i + tail.len()).min(out.len());
    out[i..end].copy_from_slice(&tail[..end - i]);
    end
}
