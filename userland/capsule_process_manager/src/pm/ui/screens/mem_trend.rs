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

use crate::pm::format::mem_human;
use crate::pm::state::{Ring, State};
use crate::pm::theme::{AMBER, CARD_BG, CARD_BORDER, MUTED, OK, TITLE};

use super::super::metrics::{BODY_PX, NUM_PX, PANEL_HEAD_H, PANEL_PAD, PANEL_RADIUS};
use super::super::{spark, text};

// Falling resident memory is the healthy direction, so the sign picks the tint
// rather than the size: OK when the newest sample sits below the oldest one the
// ring still holds, AMBER when it has grown, MUTED before there is a history.
pub fn delta(ring: &Ring) -> (i64, u32) {
    if ring.is_empty() {
        return (0, MUTED);
    }
    let newest = ring.mem_at(ring.len() - 1) as i64;
    let moved = newest - ring.oldest_mem() as i64;
    (moved, if moved > 0 { AMBER } else { OK })
}

pub fn paint(state: &State, fb: &mut PaintBuffer, x: u32, y: u32, w: u32, h: u32) {
    fb.fill_round(x, y, w, h, PANEL_RADIUS, CARD_BG);
    fb.stroke_round(x, y, w, h, PANEL_RADIUS, 1, CARD_BORDER);
    let top = text::centred_top(y, PANEL_HEAD_H, BODY_PX);
    text::left(fb, x + PANEL_PAD, top, b"RESIDENT TREND", TITLE, BODY_PX);
    let ring = &state.history.total;
    let (moved, tint) = delta(ring);
    let mut buf = [0u8; 20];
    buf[0] = if moved < 0 { b'-' } else { b'+' };
    let n = 1 + mem_human(moved.unsigned_abs(), &mut buf[1..]);
    text::mono_right(fb, x + w.saturating_sub(PANEL_PAD), top, &buf[..n], tint, NUM_PX);
    let plot_h = h.saturating_sub(PANEL_HEAD_H + PANEL_PAD);
    spark::mem(
        fb,
        x + PANEL_PAD,
        y + PANEL_HEAD_H,
        w.saturating_sub(PANEL_PAD * 2),
        plot_h,
        ring,
        tint,
    );
}
