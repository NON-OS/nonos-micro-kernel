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

//! The figures on the load chart's caption line: mean, peak and the user
//! share, placed from the right edge by measurement so a three-digit peak
//! never shoves the caption that sits on the same line.

use nonos_app_skeleton::PaintBuffer;

use crate::pm::state::State;
use crate::pm::theme::{MUTED, OK};

use super::super::chrome::Rect;
use super::super::metrics::{NUM_PX, PANEL_HEAD_H, PANEL_PAD};
use super::super::text;
use super::ovw_cards::sub_n;

pub(super) fn legend(state: &State, fb: &mut PaintBuffer, r: &Rect) {
    let ring = &state.history.total;
    let n = ring.len().max(1) as u32;
    let mean = (0..ring.len()).map(|i| ring.cpu_at(i) as u32).sum::<u32>() / n;
    let top = text::centred_top(r.y, PANEL_HEAD_H, NUM_PX);
    let right = r.x + r.w.saturating_sub(PANEL_PAD);
    let mut buf = [0u8; 24];
    let len = sub_n(&mut buf, b"mean ", mean, b"%");
    text::mono_right(fb, right, top, &buf[..len], MUTED, NUM_PX);
    let mut used = text::mono_width(fb, &buf[..len], NUM_PX) + PANEL_PAD;
    let len = sub_n(&mut buf, b"peak ", ring.peak_cpu() as u32, b"%");
    text::mono_right(fb, right.saturating_sub(used), top, &buf[..len], MUTED, NUM_PX);
    used += text::mono_width(fb, &buf[..len], NUM_PX) + PANEL_PAD;
    let len = sub_n(&mut buf, b"user ", state.sys.user_pct as u32, b"%");
    text::mono_right(fb, right.saturating_sub(used), top, &buf[..len], OK, NUM_PX);
}
