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

use crate::pm::format;
use crate::pm::security::sensitive::{ADMIN, DEBUG, RAW_HW, SPAWN};
use crate::pm::theme::{ACCENT, AMBER, DANGER, MUTED, TRACK_BG};

use super::metrics::{NUM_PX, RISK_SLOT_GAP, RISK_SLOT_H, RISK_SLOT_W};
use super::text;

// The four sensitive classes, drawn as four fixed slots so a scanning eye can
// compare rows by position instead of reading labels. A held class fills its slot
// in the class colour; an unheld one leaves the track showing.
pub const CLASSES: [(u64, u32); 4] =
    [(ADMIN, DANGER), (RAW_HW, AMBER), (SPAWN, ACCENT), (DEBUG, MUTED)];

// Every colour above is opaque, which is the only reason `fill_rect` is correct
// over a row this capsule has already painted: it writes rather than blends, so
// an alpha-carrying token here would punch through the table body.
pub fn paint(fb: &mut PaintBuffer, x: u32, y: u32, caps: u64) -> u32 {
    let mut slot_x = x;
    for (mask, argb) in CLASSES {
        let fill = if caps & mask != 0 { argb } else { TRACK_BG };
        fb.fill_rect(slot_x, y, RISK_SLOT_W, RISK_SLOT_H, fill);
        slot_x += RISK_SLOT_W + RISK_SLOT_GAP;
    }
    let mut buf = [0u8; 4];
    let n = format::u32_decimal(caps.count_ones(), &mut buf);
    let top = text::centred_top(y, RISK_SLOT_H, NUM_PX);
    text::mono(fb, slot_x, top, &buf[..n], MUTED, NUM_PX);
    slot_x + text::mono_width(fb, &buf[..n], NUM_PX)
}
